use grid_printer::GridPrinterErr;
use std::error::Error;

fn main() {
    let err = GridPrinterErr::DimensionErr;
    println!("{}", err);
    println!("{:?}", err);

    println!("err.source = {:?}", err.source());
    // println!("err.backtrace = {:?}", err.backtrace());
}
