use chrono::prelude::*;
use utils::coder;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct BlockHeader
{
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block 
{
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block
{
    // TODO: 求区块哈希
    fn set_hash(&mut self)
    {
        let header = coder::block_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    // TODO: 创建区块
    pub fn new_block(data: String, pre_hash: String) -> Block
    {
        let transactions = coder::block_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

        let time = Utc::now().timestamp();
        let mut block = Block
        {
            header: BlockHeader 
            {
                time: time,
                tx_hash: tx_hash,
                pre_hash: pre_hash,
            },
            hash: "".to_string(),
            data: data,
        };
        block.set_hash();
        block
    }
}