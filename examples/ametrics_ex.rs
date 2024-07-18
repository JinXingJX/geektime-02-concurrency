use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
use std::thread;

fn main() -> Result<()> {
    let task_name = [
        "call.thread.worker.0",
        "call.thread.worker.1",
        "call.thread.worker.2",
        "call.thread.worker.3",
        "call.thread.worker.4",
        "call.thread.worker.5",
    ];
    let page_name = ["page.view.1", "page.view.2", "page.view.0"];
    let mut metric_names = Vec::new();
    metric_names.extend_from_slice(&task_name);
    metric_names.extend_from_slice(&page_name);
    let metrics = AmapMetrics::new(metric_names.as_slice());
    for i in 0..5 {
        task_worker(i, metrics.clone())?; // Metrics {data: Arc::clone(&self.data)}
    }
    for i in 0..2 {
        request_worker(i, metrics.clone())?;
    }
    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("{:?}", metrics);
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..1000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    let mut rng = rand::thread_rng();
    thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
    metrics.inc(format!("page.view.{}", idx))?;
    Ok(())
}
