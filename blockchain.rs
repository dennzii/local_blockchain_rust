use crate::block::check_diff;

use super::*;

pub struct Blockchain{
    pub blocks : Vec<Block>//blocklardan oluşan bir vector

}

impl Blockchain{

    pub fn verify(&self) -> bool
    {
        for (i,block) in self.blocks.iter().enumerate()
        {//enumerate nedir??
            
            
            /*
            Blockchaindeki her bloğun valid olması için gereken koşullar burada sıralanmalı
            Bitcoin'de verification için 19 bakılacak şey var. burada kullandıklarım:
            1-Index
            2-Difficulty değeri hash'de sağlanmış olamalı
            3-Timestamp
             */

            //index match?
            if block.index != i as u32{
                println!("Index mismatch block {}",&i);
                return false;
            }
            else if !check_diff(&block.hash(), block.difficulty)
            {
                println!("Difficulty mismatch!");
                return false;
            }
            else if i != 0 //not genesis block
            {
                /*
                Blockchainde genesis bloğundan başladığımızda diğer blokların timestampleri artmak zorundadır.
                Mantıken bir blok kendisinden önceki bloktan daha önce zincire eklenmiş OLAMAZ.
                 */
                
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp
                {
                    println!("Time not increased! {} >! {}",&block.timestamp,&prev_block.timestamp);
                    return false;
                }
                else if prev_block.hash != block.previous_block_hash
                {
                    println!("Hash mismatch! {:?} != {:?}",&block.previous_block_hash,&prev_block.hash);
                    return false;
                }
            }
            println!("{}. Block verified",&i);
            
        }
        
        return true;
    }
}