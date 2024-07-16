use anyhow::anyhow;
use anyhow::Result;
use std::time::Duration;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rc) = mpsc::channel();
    //创建producer线程
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(tx, i));
    }

    //创建consumer线程
    let consumer = thread::spawn(move || {
        for msg in rc {
            println!("msg: {:?}", msg);
        }
    });
    consumer
        .join()
        .map_err(|e| anyhow!("consumer thread panicked: {:?}", e))?;
    Ok(())
}

fn producer(tx: mpsc::Sender<Msg>, i: usize) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(i, value))?;
        thread::sleep(Duration::from_millis(1000));
    }
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
