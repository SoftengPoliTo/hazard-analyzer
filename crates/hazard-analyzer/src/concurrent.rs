use crossbeam::channel::{Receiver, Sender};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use code_certifier::error::{Error, ErrorKind, Result};

// Defines a framework for a *producer-consumers-composer* pattern.
pub(crate) trait ConcurrentRunner<'a> {
    // Items received by the `producer`
    type Items: Sync + Send;

    // Item sent from `producer` to `consumer`.
    type ProducerItem: Sync + Send;

    // Item sent from `consumer` to `composer`.
    type ConsumerItem: Sync + Send;

    // Output returned by the `composer`.
    type Output: Sync + Send;

    // Sends items to the `consumer`.
    fn producer(&self, items: Self::Items, sender: Sender<Self::ProducerItem>) -> Result<()>;

    // Receivs items from the `producer`, processes them,
    // and sends the results to the `composer`.
    fn consumer(
        &self,
        receiver: Receiver<Self::ProducerItem>,
        sender: Sender<Self::ConsumerItem>,
    ) -> Result<()>;

    // Receivs items from the `consumer`, computes an `Output`, and returns it.
    fn composer(&self, receiver: Receiver<Self::ConsumerItem>) -> Result<Self::Output>;

    // Executes the producer-consumers pattern.
    fn run(self, items: Self::Items, n_threads: usize) -> Result<Self::Output>
    where
        Self: Sync + Sized,
    {
        let (producer_sender, consumer_receiver) = crossbeam::channel::bounded(n_threads);
        let (consumer_sender, composer_receiver) = crossbeam::channel::bounded(n_threads);

        crossbeam::thread::scope(|scope| {
            // Producer
            scope.spawn(|_| self.producer(items, producer_sender));

            // Composer
            let composer = scope.spawn(|_| self.composer(composer_receiver));

            // Consumer.
            (0..n_threads).into_par_iter().try_for_each(|_| {
                self.consumer(consumer_receiver.clone(), consumer_sender.clone())
            })?;

            // The Sender between consumers and composer must be dropped so that shared channels can be closed.
            // Otherwise, the composer will eternally await data from the consumers.
            drop(consumer_sender);

            // Result produced by the composer.
            composer
                .join()
                .map_err(|_| Error::new(ErrorKind::Concurrent, "Error during composer join"))?
        })
        .map_err(|_| Error::new(ErrorKind::Concurrent, "Concurrent runner scope error"))?
    }
}
