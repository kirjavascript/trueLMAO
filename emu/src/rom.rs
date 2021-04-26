pub struct ROM {
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for ROM {
    fn from(bytes: Vec<u8>) -> Self {
        ROM { bytes }
    }
}

#[allow(dead_code)]
impl ROM {
    pub fn read_byte(&self, addr: u32) -> u8 {
        *self.bytes.get(addr as usize).unwrap_or(&0)
    }

    pub fn read_word(&self, addr: u32) -> u16 {
        ((self.read_byte(addr) as u16) << 8) | self.read_byte(addr + 1) as u16
    }

    pub fn read_long(&self, addr: u32) -> u32 {
        ((self.read_word(addr) as u32) << 16) | self.read_word(addr + 2) as u32
    }

    pub fn read_string(&self, range: std::ops::Range<usize>) -> String {
        if range.end >= self.bytes.len() {
            return format!("end {} >= length {}", range.end, self.bytes.len())
        }
        String::from_utf8_lossy(&self.bytes[range]).to_string()
    }

    // pub fn vectors(&self) -> Vec<u32> {
    //     let mut i = 0;
    //     let mut vtrs = Vec::new();
    //     while i < 0x100 {
    //         vtrs.push(self.read_long(i));
    //         i += 4;
    //     }
    //     vtrs
    // }

    pub fn stack_pointer(&self) -> u32 { self.read_long(0x0) }
    pub fn entry_point(&self) -> u32 { self.read_long(0x4) }

    pub fn system_type(&self) -> String { self.read_string(0x100..0x110) }
    pub fn copyright(&self) -> String { self.read_string(0x110..0x120) }
    pub fn domestic_name(&self) -> String { self.read_string(0x120..0x150) }
    pub fn overseas_name(&self) -> String { self.read_string(0x150..0x180) }
    pub fn serial_number(&self) -> String { self.read_string(0x180..0x18E) }
    pub fn checksum(&self) -> u16 { self.read_word(0x18E) }
    pub fn device_support(&self) -> String { self.read_string(0x190..0x1A0) }

}
