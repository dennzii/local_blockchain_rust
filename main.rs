mod block;
mod hashable;
mod lib;
use block::Block;
use hashable::Hashable;
mod blockchain;
use blockchain::Blockchain;
use lib::*;

pub fn main()
{
    let difficulty = 0x00ffffffffffffffffffffffffffffff;//16 bayt most significant 16 byte in hash

    let mut block = Block::new(0,now(),vec![0;32],
        1,"genesis".to_owned(),difficulty);

        /*
        LITTLE ENDIAN ve BIG ENDIAN
        42u32 hex : 0x0000002a
        Big endian: 00 00 00 2a
        Little endian: 2a 00 00 00
        Çıktıda most significant bytelar sonda çünkü Little Endian gösteriminde gösterilmiş. 
         */
    


    let h = block.hash();//hashleme yapılır.
    
    println!("{:?}",&hex::encode(&h));//little endian gösteriminde döndürür.

    block.mine();
    println!("Genesis block mined! new hash:{:?}",&hex::encode( &block.hash()));
    
    let mut last_block_hash = block.hash.clone();//"block" değişkenini blockchaine ödünç vermeden hash'i klonla

    let mut blockchain = Blockchain
    {
        blocks: vec![block] //first block is genesis
    };


    //10 blok eklenir.
    for i in 1..=10
    {
        let mut tmp_block = Block::new(i,now(),last_block_hash,1,"another block".to_owned(),difficulty);
        tmp_block.mine();
        println!("{}. Block mined! Nonce: {}  hash:{:?}",&tmp_block.index,&tmp_block.nonce,&hex::encode( &tmp_block.hash()),);

        last_block_hash = tmp_block.hash.clone();
        blockchain.blocks.push(tmp_block);
    }

    println!("Verifying Blocks..");
    blockchain.verify();


}

