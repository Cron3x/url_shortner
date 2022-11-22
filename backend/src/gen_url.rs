use crc::{Crc, CRC_32_ISCSI};
pub fn gen_url_32(url:&str) -> String{
    let x = Crc::<u32>::new(&CRC_32_ISCSI);
    let a = x.checksum(url.as_bytes());
    
    format!("l{:x}",a)
}