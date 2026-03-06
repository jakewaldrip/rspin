use rand::{Rng, RngExt, seq::SliceRandom};

pub const REEL_STRIPS: [[char; 24]; 5] = [
    [
        'o', '#', '$', 'o', '*', '@', 'o', '#', '$', '7', '%', '#', '$', '*', '@', '&', 'o', '#',
        '$', 'o', '*', '@', '#', '$',
    ],
    [
        '#', 'o', '$', '#', '*', '@', '#', 'o', '$', '&', '#', 'o', '$', '*', '@', '7', '#', 'o',
        '$', '#', '*', '%', 'o', '$',
    ],
    [
        '$', '#', 'o', '$', '%', '@', '$', '#', 'o', '7', '$', '#', 'o', '*', '@', '&', '$', '#',
        'o', '$', '*', '@', '#', 'o',
    ],
    [
        'o', '$', '#', 'o', '@', '*', 'o', '$', '#', '&', 'o', '$', '#', '@', '*', '%', 'o', '$',
        '#', 'o', '@', '*', '7', '$',
    ],
    [
        '#', '$', 'o', '#', '@', '*', '#', '$', '%', '&', '#', '$', 'o', '@', '*', '7', '#', '$',
        'o', '#', '@', '*', 'o', '$',
    ],
];

#[derive(Debug)]
pub struct Machine {
    pub reels: [Reel; 5],
    pub name: String,
}

impl Machine {
    pub fn create_n_machines(count: i32) -> Vec<Self> {
        (1..=count).map(Self::new).collect()
    }

    /// Make a new machine with all the reels set
    pub fn new(machine_num: i32) -> Self {
        Self {
            reels: std::array::from_fn(Reel::new),
            name: format!("Machine {machine_num}"),
        }
    }

    pub fn spin(&mut self) {
        let mut rng = rand::rng();
        for reel in &mut self.reels {
            reel.spin(&mut rng);
        }
    }
}

#[derive(Debug)]
pub struct Reel {
    symbols: Vec<char>,
    ptr: usize,
}

impl Reel {
    /// Make a new reel and shuffle it
    pub fn new(reel_num: usize) -> Self {
        let mut reel_set = REEL_STRIPS[reel_num].to_vec();
        reel_set.shuffle(&mut rand::rng());

        Reel {
            symbols: reel_set,
            ptr: 0,
        }
    }

    /// Spin the individual reel,
    pub fn spin<R: Rng>(&mut self, rng: &mut R) {
        self.ptr = rng.random_range(0..self.symbols.len());
    }
}
