use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

#[derive(Debug, Clone)]
pub struct CancelToken {
    canceled: Arc<AtomicBool>
}

impl CancelToken {
    pub fn new() -> Self {
        Self {
            canceled: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled.load(Ordering::SeqCst)
    }

    pub fn cancel(&self) {
        self.canceled.store(true, Ordering::SeqCst);
    }

    pub fn check_cancel(&self) -> Result<(), ()> {
        if self.is_canceled() {
            Err(())
        } else {
            Ok(())
        }
    }
}