use std::io::{Read, Error as ioError};
use std::error::Error;
use sha2::{Sha256, Digest};
use transaction::{Input, Output, Transaction, Amount, Txid};
mod transaction;


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, ioError> {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size)?;
   

    match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        }
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        }
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer))
        }
    }
}

fn read_u32(transaction_bytes: &mut &[u8]) -> Result<u32, ioError> {
    let mut buffer = [0; 4];
    transaction_bytes.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, ioError> {
    let mut buffer = [0; 8];
    transaction_bytes.read_exact(&mut buffer).unwrap();
    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}

fn read_txid(transaction_hex: &mut &[u8]) -> Result<Txid, ioError> {
    let mut buffer = [0; 32];
    transaction_hex.read(&mut buffer).unwrap();
    buffer.reverse();
    Ok(Txid::from_bytes(buffer))
}

fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, ioError> {
    let script_size = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer)?;
    Ok(hex::encode(buffer))
}

fn hash_raw_transaction(raw_transaction: &[u8]) -> Txid {
    let mut hasher = Sha256::new();
    hasher.update(&raw_transaction);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hasher2 = hasher.finalize();

    Txid::from_bytes(hasher2.into())
}

pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>>{
    let transaction_bytes = hex::decode(transaction_hex)?;
    let mut bytes_slice = transaction_bytes.as_slice();

    // 1. Version
    let version = read_u32(&mut bytes_slice)?;

    // 2. SegWit marker & flag
    let mut marker_and_flag = [0u8; 2];
    bytes_slice.read_exact(&mut marker_and_flag)?;

    if marker_and_flag != [0x00, 0x01] {
        panic!("Not a SegWit transaction");
    }

    // 3. Input count (CompactSize)
    let input_count = read_compact_size(&mut bytes_slice)?;

    // 4. Read inputs
    let mut inputs = vec![];
    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;

        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence,
        });
    }

    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = vec![];
    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;
        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }

    let locktime = read_u32(&mut bytes_slice)?;
    let transaction_id = hash_raw_transaction(&transaction_bytes);
    
    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        locktime   
    };

    Ok(serde_json::to_string_pretty(&transaction)?)
}





// Testing
#[cfg(test)] // <-- Only compile this block when testing
mod tests {
    use super::read_compact_size;
    use std::error::Error;

    #[test] // <-- This is a test function
    fn test_read_compact_size() -> Result<(), Box<dyn Error>> {
        let mut bytes = [1_u8].as_slice();
        let input_count = read_compact_size(&mut bytes)?;
        assert_eq!(input_count, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let input_count = read_compact_size(&mut bytes)?;
        assert_eq!(input_count, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let input_count = read_compact_size(&mut bytes)?;
        assert_eq!(input_count, 16777216_u64);

        Ok(())
    }
}


