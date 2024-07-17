use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::thread;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    for i in 0..5 {
        task_worker(i, metrics.clone()); // Metrics {data: Arc::clone(&self.data)}
    }
    for _ in 0..2 {
        request_worker(metrics.clone());
    }
    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..1000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
}

fn request_worker(metrics: Metrics) {
    let mut rng = rand::thread_rng();
    thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
    let page = rng.gen_range(1..10);
    metrics.inc(format!("page.view.{}", page)).unwrap();
}
