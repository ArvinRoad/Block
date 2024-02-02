use bincode;
use serde::{ Deserialize, Serialize };
use crypto::digest::Digest;
use crypto::sha3::Sha3;

// TODO: 序列化
pub fn block_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let seialized = bincode::serialize(value).unwrap();
    seialized
}
// TODO: 反序列化
pub fn block_deserialize<'a, T>(bytes: &'a[u8]) -> T 
    where T: Deserialize<'a>,
{
    let deserialize = bincode::deserialize(bytes).unwrap();
    deserialize
}

// TODO: 取哈希
pub fn get_hash(value: &[u8]) -> String
{
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

// 测试
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Point
{
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{block_serialize, block_deserialize};

    #[test]
    fn coder_works() {
        let point = Point{ x: 1, y: 1 };
        let se = block_serialize(&point);
        let de: Point = block_deserialize(&se);
        assert_eq!(de, point);
    }
}