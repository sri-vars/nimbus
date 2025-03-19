use std::sync::{Arc, OnceLock};

use smol::{Executor, Task, future};

pub struct NimbusRt {
    executor: Arc<Executor<'static>>,
}

impl NimbusRt {
    pub fn instance() -> &'static NimbusRt {
        static INSTANCE: OnceLock<NimbusRt> = OnceLock::new();
        INSTANCE.get_or_init(|| NimbusRt {
            executor: Arc::new(Executor::new()),
        })
    }

    pub fn spawn<F>(future: F) -> Task<F::Output>
    where
        F: std::future::Future + 'static + Send,
        F::Output: 'static + Send,
    {
        let rt = NimbusRt::instance();
        rt.executor.spawn(future)
    }

    pub fn run<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        future::block_on(self.executor.run(future));
    }
}
