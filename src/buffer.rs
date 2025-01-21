use tokio::io::AsyncWriteExt;

pub struct Buffer {
    pub data: Vec<u8>,
    index: usize
}

impl Buffer {
    pub fn from(data: Vec<u8>) -> Buffer {
        return Buffer {
            data,
            index: 0,
        }
    }

    pub fn new() -> Buffer {
        return Buffer {
            data: Vec::new(),
            index: 0
        }
    }

    pub fn read_byte(mut self) -> u8 {
        let val = self.data[self.index];
        self.index += 1;
        return val;
    }

    pub fn read_short(&mut self) -> i16 {
        let data = self.data[self.index..self.index+2].try_into().unwrap();
        let val = i16::from_be_bytes(data);
        self.index += 2;
        return val;
    }

    pub fn read_int(mut self) -> i32 {
        let data: [u8; 4] = self.data[self.index..self.index+4].try_into().unwrap();
        let val = i32::from_be_bytes(data);
        self.index += 4;
        return val;
    }

    pub fn read_int_le(mut self) -> i32 {
        let data: [u8; 4] = self.data[self.index..self.index+4].try_into().unwrap();
        let val = i32::from_le_bytes(data);
        self.index += 4;
        return val;
    }

    pub fn read_long(mut self) -> i64 {
        let data: [u8; 8] = self.data[self.index..self.index+8].try_into().unwrap();
        let val = i64::from_be_bytes(data);
        self.index += 8;
        return val;
    }

    pub fn read_string(&mut self) -> String {
        let len = self.read_short();
        let pos = self.index;
        self.index += len as usize;
        let utf_bytes = &self.data[pos..self.index];
        return String::from_utf8(utf_bytes.to_vec()).unwrap();
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.data.insert(self.index, byte);
        self.index += 1;
    }

    pub fn write_short(&mut self, byte: i16) {
        for x in byte.to_be_bytes() {
            self.write_byte(x);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        self.write_short(string.len() as i16);
        for x in string.as_bytes() {
            self.write_byte(*x);
        }
    }
}