// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

use crate::promise::{Promise, PromiseError};
use crate::{PromiseResult, StructureRef};

#[derive(Debug)]
pub struct PromiseFuture(
    pub(crate) Promise,
    pub(crate) futures_channel::oneshot::Receiver<()>,
);

pub struct PromiseReply(Promise);

impl std::future::Future for PromiseFuture {
    type Output = Result<Option<PromiseReply>, PromiseError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        match Pin::new(&mut self.1).poll(context) {
            Poll::Ready(Err(_)) => panic!("Sender dropped before callback was called"),
            Poll::Ready(Ok(())) => {
                let res = match self.0.wait() {
                    PromiseResult::Replied => {
                        if self.0.get_reply().is_none() {
                            Ok(None)
                        } else {
                            Ok(Some(PromiseReply(self.0.clone())))
                        }
                    }
                    PromiseResult::Interrupted => Err(PromiseError::Interrupted),
                    PromiseResult::Expired => Err(PromiseError::Expired),
                    PromiseResult::Pending => {
                        panic!("Promise resolved but returned Pending");
                    }
                    err => Err(PromiseError::Other(err)),
                };
                Poll::Ready(res)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl futures_core::future::FusedFuture for PromiseFuture {
    fn is_terminated(&self) -> bool {
        self.1.is_terminated()
    }
}

impl Deref for PromiseReply {
    type Target = StructureRef;

    #[inline]
    fn deref(&self) -> &StructureRef {
        self.0.get_reply().expect("Promise without reply")
    }
}

impl std::fmt::Debug for PromiseReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_tuple("PromiseReply");

        match self.0.get_reply() {
            Some(reply) => debug.field(reply),
            None => debug.field(&"<no reply>"),
        }
        .finish()
    }
}
