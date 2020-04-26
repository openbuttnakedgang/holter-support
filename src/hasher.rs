
pub trait Hasher32 {
    fn write(&mut self, bytes: &[u8]);
    fn write32(&mut self, bytes: &[u32]);
    fn finish(&mut self) -> u32;
}
