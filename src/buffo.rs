pub struct Buffo{
    mem : [u8;10]
}

impl Buffo {
    pub fn ppp(mut self) -> Buffo{
        println!("PPP {}", self.mem[1]);
        return self;
    }

    pub fn new() -> Buffo {
        Buffo {
            mem : [0;10]
        }
    }

    pub fn get_mem(&mut self) -> &mut [u8] {
        &mut self.mem
    }

}