pub struct Buffo{
    mem : [u8;10]
}

impl Buffo {
    pub fn new() -> Buffo {
        Buffo {
            mem : [0;10]
        }
    }

    pub fn get_mem(&mut self) -> &mut [u8] {
        &mut self.mem
    }

    pub fn put_mem(&mut self, data : u8) -> u8 {
        self.mem[0] = data;
        return 1;
    }
}