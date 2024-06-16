

pub(crate) trait Load<I: Sized> {
    fn load(&mut self, index: I);
}
    


pub(crate) struct Register {
    #[cfg(target_pointer_width = "64")]
    register: [u8; 8],
    #[cfg(target_pointer_width = "32")]
    register: [u8; 4],
    #[cfg(target_pointer_width = "16")]
    register: [u8; 2],
}

impl Register {
    pub fn new() -> Register {
        Register {
            #[cfg(target_pointer_width = "64")]
            register: [0; 8],
            #[cfg(target_pointer_width = "32")]
            register: [0; 4],
            #[cfg(target_pointer_width = "16")]
            register: [0; 2],
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.register
    }

    pub fn get_mut(&mut self) -> &mut [u8] {
        &mut self.register
    }

    pub fn set(&mut self, value: &[u8]) {
        self.register.copy_from_slice(value);
    }
}

impl Load<u8> for Register {
    fn load(&mut self, value: u8) {
        self.register[0] = value;
    }
}

impl Load<u16> for Register {
    fn load(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.register[0] = bytes[0];
        self.register[1] = bytes[1];
    }
}

#[cfg(target_pointer_width = "32")]
impl Load<u32> for Register {
    fn load(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.register[0] = bytes[0];
        self.register[1] = bytes[1];
        self.register[2] = bytes[2];
        self.register[3] = bytes[3];
    }
}

#[cfg(target_pointer_width = "64")]
impl Load<u32> for Register {
    fn load(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.register[0] = bytes[0];
        self.register[1] = bytes[1];
        self.register[2] = bytes[2];
        self.register[3] = bytes[3];
    }
}


#[cfg(target_pointer_width = "64")]
impl Load<u64> for Register {
    fn load(&mut self, value: u64) {
        let bytes = value.to_le_bytes();
        self.register[0] = bytes[0];
        self.register[1] = bytes[1];
        self.register[2] = bytes[2];
        self.register[3] = bytes[3];
        self.register[4] = bytes[4];
        self.register[5] = bytes[5];
        self.register[6] = bytes[6];
        self.register[7] = bytes[7];
    }
}
