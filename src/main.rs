mod blockchain;
use blockchain::blockchain::Blockchain;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut blockchain: Blockchain = Blockchain::new().unwrap();
    let _ = blockchain.add_block("data1".to_string());
    let _ = blockchain.add_block("data2".to_string());
    let _ = blockchain.add_block("data3".to_string());

    for block in blockchain.iter() {
        println!("{:?}", block);
    }

    Ok(())
}
