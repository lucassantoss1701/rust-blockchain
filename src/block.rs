pub mod block {
    use bincode::serialize;
    use crypto::digest::Digest;
    use crypto::sha2::Sha256;
    use log::info;
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;

    const TARGET_HEXS: usize = 4;
    pub type Result<T> = std::result::Result<T, failure::Error>;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Block {
        timestamp: u128,
        transactions: String,
        prev_block_hash: String,
        hash: String,
        nonce: i32,
        height: i32,
    }

    impl Block {
        pub fn new_genesis_block() -> Block {
            Block::new_block(String::from("Genesis Block"), String::from(""), 0).unwrap()
        }

        pub fn get_hash(&self) -> String {
            self.hash.clone()
        }

        pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
            let timestamp: u128 = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let mut block = Block {
                timestamp,
                transactions: data,
                prev_block_hash,
                hash: String::from(""),
                nonce: 0,
                height: height as i32,
            };

            //pprof_of_work
            let _ = block.run_proof_of_work();

            Ok(block)
        }

        fn prepare_hash_data(&self) -> Result<Vec<u8>> {
            let content: (String, String, u128, usize, i32) = (
                self.prev_block_hash.clone(),
                self.transactions.clone(),
                self.timestamp,
                TARGET_HEXS,
                self.nonce,
            );

            let bytes = serialize(&content)?;

            Ok(bytes)
        }

        fn validate(&self) -> Result<bool> {
            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher = Sha256::new();
            hasher.input(&data[..]);
            let mut vec1: Vec<u8> = Vec::new();
            vec1.resize(TARGET_HEXS, '0' as u8);

            Ok(&hasher.result_str()[0..TARGET_HEXS] == String::from_utf8(vec1)?)
        }

        fn run_proof_of_work(&mut self) -> Result<()> {
            info!("mining block!");
            while !self.validate()? {
                self.nonce += 1;
            }

            let data: Vec<u8> = self.prepare_hash_data()?;
            let mut hasher: Sha256 = Sha256::new();
            hasher.input(&data[..]);
            self.hash = hasher.result_str();
            Ok(())
        }

        pub fn get_prev_block_hash(&self) -> String {
            self.prev_block_hash.clone()
        }
    }
}
