use anyhow::Result;
use async_trait::async_trait;
use std::{
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    time::Duration,
};
use tokio::{
    sync::{
        mpsc::{
            self,
            Receiver,
        },
        oneshot::{
            self,
            Sender,
        },
    },
    time,
};
use tracing::{
    debug,
    info,
    info_span,
    instrument,
};
use tracing_futures::Instrument;

// Sensor

pub trait Sensor<T> {
    fn get_channels(self) -> SensorChannels<T>;
}

// Sensor Channels

pub struct SensorChannels<T> {
    read: Pin<Box<dyn Future<Output = Result<()>>>>,
    receiver: Receiver<T>,
    sender: Sender<()>,
}

impl<T> SensorChannels<T> {
    #[instrument(skip(self))]
    pub fn decompose(self) -> (impl Future<Output = Result<()>>, Receiver<T>, Sender<()>) {
        debug!("returning decomposed fields of SensorChannels.");
        (self.read, self.receiver, self.sender)
    }
}

// Pull Sensor

#[async_trait]
pub trait PullSensor<T> {
    async fn read(&mut self) -> Result<T>;
}

#[derive(Debug)]
pub struct FromPullSensor<S, T>
where
    S: PullSensor<T>,
{
    interval: Duration,
    sensor: S,
    _t: PhantomData<T>,
}

impl<S, T> Sensor<T> for FromPullSensor<S, T>
where
    S: PullSensor<T> + Debug + Send + 'static,
    T: Debug + Send + Sync + 'static,
{
    #[instrument]
    fn get_channels(self) -> SensorChannels<T> {
        debug!("creating data and kill signal channels.");
        let (data_tx, data_rx) = mpsc::channel(100);
        let (kill_tx, kill_rx) = oneshot::channel();

        debug!("creating polling loop.");
        let poll = async move {
            let interval = self.interval;
            let mut kill_rx = Box::pin(kill_rx);
            let mut sensor = self.sensor;

            loop {
                tokio::select! {
                    _ = time::sleep(interval) => {
                        debug!("reading data from sensor.");
                        let data = sensor.read().await?;
                        debug!("sending data.");
                        let _ = data_tx.send(data).await?;
                    },
                    _ = &mut kill_rx => {
                        info!("received kill signal. exiting with ok.");
                        return Ok(());
                    },
                }
            }
        };

        debug!("creating instrumented polling loop.");
        let poll = poll.instrument(info_span!("polling_loop"));

        debug!("returning new SensorChannels.");
        SensorChannels {
            read: Box::pin(poll),
            receiver: data_rx,
            sender: kill_tx,
        }
    }
}

#[instrument]
pub fn from_pull_sensor<S, T>(sensor: S, interval: Duration) -> FromPullSensor<S, T>
where
    S: PullSensor<T> + Debug,
{
    debug!("returning new FromPullSensor.");
    FromPullSensor::<S, T> {
        interval,
        sensor,
        _t: PhantomData::<T>,
    }
}
