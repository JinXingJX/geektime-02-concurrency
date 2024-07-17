use anyhow::Result;
use concurrency::Matrix;

fn main() -> Result<()> {
    let a = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let b = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let c = a * b;
    println!("{:?}", c);
    Ok(())
}
