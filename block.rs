
type BlockHash = Vec<u8>;
use crate::lib::*;
use crate::hashable::Hashable;
use super::*;

pub struct Block{
    pub index: u32,//Bloğun numarısını simgeler. Unsigned integer olmalıdır.
    pub timestamp: u128,
    pub hash : Vec<u8>,//bytelardan oluşan (çünkü unsigned 8-bit) bir vector
    pub previous_block_hash : Vec<u8>,
    pub nonce : u64,//number only used once demek. Difficulty değeri sağlanana kadar yapılan iterasyon.
    pub payload : String,
    pub difficulty : u128//hash 32 byte, difficulty most significant 16 byte üzerinden belirlenir.
}

impl Block{//implementing
    pub fn new( index: u32,  timestamp: u128, previous_block_hash : Vec<u8>,nonce : u64,  payload : String,difficulty:u128) -> Self //self burda return type
    {
        return Block{
            index,
            timestamp,
            hash : vec![0;32],//32 BAYT 256 BİT EDER Kİ SHA256 İÇİN GEREKEN DE BUDUR. bu bir vector constructor'u gibi ve buna makro deniyor
            previous_block_hash,
            nonce,
            payload,
            difficulty
        }
    }

    pub fn mine(&mut self)
    {
        for nonce_attempt in 0..(u64::MAX)//nonce başlar 1'den başlar 2^64 değer alabilir.
        {
            self.nonce = nonce_attempt;//nonce değeri son iterasyon olur.
            let mut h = self.hash();//bu kısmın işlem cost'u yüksek

            if check_diff(&h, self.difficulty)//eğer bulunan hash valid ise.
            {
                self.hash = h;//bulunan hash. hash olarak belirlenir.
                return;//end loop
            }
        }
    }
}

//difficulty hash'İn 32 baytının most significant 16 baytını kapsar kalanına gerek yoktur.
pub fn check_diff(hash : &BlockHash,diff : u128) -> bool
{
   return diff > lib::difficulty_bytes_as_u128(&hash);
   //diff eğer 0x0000fff.. gibi bir değerse hash'in ondan küçük olması için most significant 2 byte'ının 0 olması lazım.
   //bu da bir işlem gücü gerektiriyor.
}

impl Hashable for Block{
    fn bytes(&self) -> Vec<u8>//Block nesnesinin attributelarını byte cinsine çevirir ve onları bir byte cinsinden bir arrayın içine koyar
    {
        let mut bytes = vec![];
        bytes.extend(&u32_to_bytes(&self.index));
        bytes.extend(&u128_to_bytes(&self.timestamp));
        bytes.extend(&u64_to_bytes(&self.nonce));
        bytes.extend(&self.previous_block_hash);
        bytes.extend(&*self.payload.as_bytes());
        bytes.extend(&u128_to_bytes(&self.difficulty));

        
        return bytes;//bunu hashlenmek üzere hash() fonksiyonuna gönderir.
    }
}
