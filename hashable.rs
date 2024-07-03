pub trait Hashable{
    fn bytes(&self)->Vec<u8>;//Bu body'siz tanımlandı. block.rs'de devamı implemente edildi.

    fn hash(&self) -> Vec<u8>//bytes() fonksiyonundan gelen değeri SHA-256 algoritmasından geçirir.
    {
       return crypto_hash::digest(crypto_hash::Algorithm::SHA256,&self.bytes())
    }
}


