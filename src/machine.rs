pub struct Machine {
    pub reels: [Reel; 5],
    pub _name: String,
}

impl Machine {
    pub fn create_n_machines(count: i32) -> Vec<Self> {
        let mut machines: Vec<Machine> = Vec::new();
        for i in 1..=count {
            machines.push(Machine::new(i));
        }
        machines
    }

    pub fn new(_machine_num: i32) -> Self {
        // todo: make a new machine, maybe somehow pass a seed here for semi random machines?
        // also set the initial pointer location here
        // good principle here would be to share that out so the animation module
        // can properly display the correct start of the reel before the spin starts
        todo!();
    }

    pub fn spin(&mut self) {
        for reel in &mut self.reels {
            reel.spin();
        }
    }
}

pub struct Reel {
    symbols: Vec<char>,
    ptr: usize,
}

impl Reel {
    pub fn spin(&mut self) {
        // todo: spin the individual reel
        todo!()
    }
}
