

pub enum Error {
    StackOverflow,
    StackUnderflow,
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        match e {
            Error::StackOverflow => "Stack Overflow".to_string(),
            Error::StackUnderflow => "Stack Underflow".to_string(),
        }
    }
}



pub struct Stack {
    stack: Vec<u8>,
}


impl Stack {
    pub fn new(stack_size: usize) -> Stack {
        Stack {
            stack: Vec::with_capacity(stack_size),
        }
    }

    pub fn push(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.stack.len() + data.len() > self.stack.capacity() {
            return Err(Error::StackOverflow);
        }
        for &byte in data {
            self.stack.push(byte);
        }
        Ok(())
    }

    pub fn pop(&mut self, size: u8) -> Result<&[u8], Error> {
        if self.stack.is_empty() {
            return Err(Error::StackUnderflow);
        }
        if size as usize > self.stack.len() {
            return Err(Error::StackUnderflow);
        }
        let start = self.stack.len() - size as usize;
        let end = self.stack.len();
        Ok(&self.stack[start..end])
    }

}
