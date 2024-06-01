#![allow(unused)]

mod garbled_circuit;
mod symmetric_key_encryption;
mod util;
use anyhow::Result;

fn main() -> Result<()> {
    // client side
    let base_table = [1, 0]; // NOT circuit
    let (input2, output2, table2) = garbled_circuit::generate2()?;

    let input = &input2.0;

    // server side
    let output = garbled_circuit::evaluate2(input, &table2)?;

    // client side
    let result = garbled_circuit::decode2(&output, &output2, &base_table)?;
    println!("{}", result);

    Ok(())
}
