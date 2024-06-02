use crate::symmetric_key_encryption::{decrypt, encrypt};
use crate::util::generate_os_rand;
use anyhow::{anyhow, Result};

pub struct GarbledInputLabel2(pub Vec<u8>, pub Vec<u8>);
pub struct GarbledOutputLabel2(pub Vec<u8>, pub Vec<u8>);

pub struct GarbledTable {
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl GarbledTable {
    pub fn as_tuple(&self) -> (&Vec<u8>, &Vec<u8>) {
        (&self.nonce, &self.ciphertext)
    }
}

impl From<(Vec<u8>, Vec<u8>)> for GarbledTable {
    fn from((nonce, ciphertext): (Vec<u8>, Vec<u8>)) -> Self {
        Self { nonce, ciphertext }
    }
}

type GarbledTable2 = (GarbledTable, GarbledTable);

pub fn generate2() -> Result<(GarbledInputLabel2, GarbledOutputLabel2, GarbledTable2)> {
    let garbled_input_label2 = GarbledInputLabel2(generate_os_rand(128), generate_os_rand(128));
    let garbled_output_label2 = GarbledOutputLabel2(generate_os_rand(128), generate_os_rand(128));

    // TODO shuffle
    let garbled_table2 = (
        GarbledTable::from(encrypt(&garbled_input_label2.0, &garbled_output_label2.0)?),
        GarbledTable::from(encrypt(&garbled_input_label2.1, &garbled_output_label2.1)?),
    );

    Ok((garbled_input_label2, garbled_output_label2, garbled_table2))
}

pub fn evaluate2(garbled_input_label: &[u8], garbled_table: &GarbledTable2) -> Result<Vec<u8>> {
    let tables = [&garbled_table.0, &garbled_table.1].into_iter();

    for table in tables {
        if let Ok(garbled_output_label) = decrypt(garbled_input_label, table.as_tuple()) {
            return Ok(garbled_output_label);
        }
    }

    Err(anyhow!("Evaluation failed"))
}

pub fn decode2(
    output_label: &[u8],
    output_label2: &GarbledOutputLabel2,
    base_table: &[u8],
) -> Result<u8> {
    let labels = [&output_label2.0, &output_label2.1].into_iter();

    for (i, label) in labels.enumerate() {
        if output_label == label {
            return Ok(base_table[i]);
        }
    }

    Err(anyhow!("Decoding failed"))
}
