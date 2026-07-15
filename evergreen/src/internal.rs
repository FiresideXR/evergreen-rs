

/// Wraps a [tokio::task] and pair of [mpsc::Sender] and [mpsc::Receiver]
/// 
/// For internal use only :P
pub struct TaskWrapper<A, B, C> {
    pub cancel_token: tokio_util::sync::CancellationToken,
    pub task_handle: tokio::task::JoinHandle<A>,
    pub incoming_queue: tokio::sync::mpsc::Receiver<B>,
    pub outgoing_queue: tokio::sync::mpsc::Sender<C>,
}

impl <A, B, C> TaskWrapper<A, B, C> {

    #[inline(always)]
    pub fn is_done(&self) -> bool {
        self.task_handle.is_finished() || self.incoming_queue.is_closed() || self.outgoing_queue.is_closed()
    }

    /// Errors if the receiver has been dropped or the channel has been closed
    #[inline(always)]
    pub async fn send(&self, value: C) -> Result<(), tokio::sync::mpsc::error::SendError<C>> {
        self.outgoing_queue.send(value).await
    }

    #[inline(always)]
    pub async fn recv(&mut self) -> Option<B> {
        self.incoming_queue.recv().await
    }

    #[inline(always)]
    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }
}

