use core::blockchain;
use std::thread;
use std::time::Duration;

fn main()
{
	let mut bc = blockchain::BlockChain::new_blockchain();

	thread::sleep(Duration::from_secs(5));

	bc.add_block(String::from("a -> b: 5btc"));

	thread::sleep(Duration::from_secs(5));
	bc.add_block(String::from("c -> d: 1btc"));

	for b in bc.blocks
	{
		println!("+++++++++++++++++++++++++++++++++++++++");
		println!("{:#?}", b);
		println!("");
	}
}