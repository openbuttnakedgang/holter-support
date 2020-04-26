
use crate::hasher::Hasher32;

pub struct Crc32Mpeg2 {
    crc: u32,
}

impl Crc32Mpeg2 {
    pub fn new() -> Self {
        Self {
            crc: !0,
        }
    }
}

impl Hasher32 for Crc32Mpeg2 {
    fn write(&mut self, bytes: &[u8]) {
        let mut msb;
        for i in 0 .. bytes.len() {
            self.crc ^= (bytes[i] as u32) << 24;
            for _ in 0 .. 8 {
                msb = self.crc >> 31;
                self.crc <<= 1;
                self.crc ^= (0 - msb as i32) as u32 & 0x04C11DB7;
            }
        }
    }
    fn write32(&mut self, words: &[u32]) {
        for w in words {
            let bytes = w.to_le_bytes();
            self.write(&bytes);
        }
    }

    fn finish(&mut self) -> u32 {
        self.crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_mpeg() { 
        {
            let mut crc = Crc32Mpeg2::new();
            crc.write(&[0xA5;4]);
            
            let res = crc.finish();
            assert_eq!(res, 0x29928E70);
        }
        {
            let mut crc = Crc32Mpeg2::new();
            crc.write(&[0xA5;1]);
            crc.write(&[0xA5;1]);
            crc.write(&[0xA5;1]);
            crc.write(&[0xA5;1]);
            
            let res = crc.finish();
            assert_eq!(res, 0x29928E70);
        }
        {
            let mut crc = Crc32Mpeg2::new();
            crc.write(&[0xA5;1]);
            
            let res = crc.finish();
            assert_eq!(res, 0xA8E282D1);
        }
        {
            let mut crc = Crc32Mpeg2::new();
            crc.write32(&[0xA5A5A5A5;1]);
            
            let res = crc.finish();
            assert_eq!(res, 0x29928E70);
        }
        {
            let mut crc = Crc32Mpeg2::new();
            crc.write32(&[0xA5A5A5A5;2]);
            
            let res = crc.finish();
            assert_eq!(res, 0xDE33FC32);
        }
    }

}
