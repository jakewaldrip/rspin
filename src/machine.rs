use rand::{Rng, RngExt, seq::SliceRandom};

use crate::{
    paylines::{
        PaylineCheckerFn, Paylines, check_above, check_below, check_eye, check_hor_xl, check_zag,
        check_zig,
    },
    symbols::{REEL_STRIPS, Symbols},
};

#[derive(Debug)]
pub struct Machine {
    pub reels: [Reel; 5],
    pub _name: String,
    pub paylines: Vec<Paylines>,
}

impl Machine {
    /// Create a specific number of machines with default impl
    pub fn create_n_machines(count: i32) -> Vec<Self> {
        (1..=count).map(Self::new).collect()
    }

    /// Make a new machine with all the reels set
    pub fn new(machine_num: i32) -> Self {
        Self {
            reels: std::array::from_fn(Reel::new),
            _name: format!("Machine {machine_num}"),
            paylines: Vec::new(),
        }
    }

    /// Spin the reels on the machine
    pub fn spin(&mut self) {
        let mut rng = rand::rng();
        for reel in &mut self.reels {
            reel.spin(&mut rng);
        }
    }

    pub fn get_all_paylines(&mut self, _lines: i32) {
        const PAYLINES_TO_CHECK: [PaylineCheckerFn; 6] = [
            check_hor_xl,
            check_zig,
            check_zag,
            check_above,
            check_below,
            check_eye,
        ];

        let visible_symbols = self.get_visible_symbols();
        for payline_checker in &PAYLINES_TO_CHECK {
            if let Some(payline) = payline_checker(
                &visible_symbols
                    .iter()
                    .map(|v| v.as_slice())
                    .collect::<Vec<_>>(),
            ) {
                self.paylines.push(payline);
            }
        }
    }

    fn get_visible_symbols(&self) -> Vec<Vec<Symbols>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Reel {
    symbols: Vec<Symbols>,
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
