pub struct Rom {
    bytes: Vec<u8>,
}

#[allow(dead_code)]
impl Rom {
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Rom { bytes }
    }

    pub fn read(&self, addr: usize) -> u8 {
        *self.bytes.get(addr).unwrap_or(&0)
    }

    pub fn read_word(&self, addr: usize) -> u16 {
        ((self.read(addr) as u16) << 8) + self.read(addr + 1) as u16
    }

    pub fn read_long(&self, addr: usize) -> u32 {
        ((self.read(addr) as u32) << 16) + self.read(addr + 1) as u32
    }

    pub fn read_string(&self, range: std::ops::Range<usize>) -> String {
        if range.end >= self.bytes.len() {
            return format!("end {} >= length {}", range.end, self.bytes.len())
        }
        String::from_utf8_lossy(&self.bytes[range]).to_string()
    }

    pub fn vectors(&self) -> Vec<u32> {
        let mut i = 0;
        let mut vtrs = Vec::new();
        while i < 0x100 {
            vtrs.push(self.read_long(i));
            i += 4;
        }
        vtrs
    }

    pub fn entry_point(&self) -> u32 {
        self.read_long(0x4)
    }

    pub fn system_type(&self) -> String {
        self.read_string(0x100..0x110)
    }

}