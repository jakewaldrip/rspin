use rand::{Rng, RngExt, seq::SliceRandom};

use crate::{
    paylines::{
        PaylineCheckerFn, Paylines, check_above, check_above_sm, check_below, check_below_sm,
        check_eye, check_hor_sm, check_hor_xl, check_zag, check_zag_sm, check_zig, check_zig_sm,
    },
    symbols::{REEL_STRIPS, Symbols},
};

#[derive(Debug)]
pub struct Machine {
    pub reels: [Reel; 5],
    pub name: String,
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
            name: format!("Machine {machine_num}"),
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

    pub fn get_all_paylines(&mut self) {
        const PAYLINES_TO_CHECK: [PaylineCheckerFn; 11] = [
            check_hor_sm,
            check_above_sm,
            check_below_sm,
            check_zig_sm,
            check_zag_sm,
            check_hor_xl,
            check_zig,
            check_zag,
            check_above,
            check_below,
            check_eye,
        ];

        let visible_symbols = self.get_visible_symbols();
        let slices: Vec<&[Symbols]> = visible_symbols.iter().map(|v| v.as_slice()).collect();
        for payline_checker in &PAYLINES_TO_CHECK {
            if let Some(payline) = payline_checker(&slices) {
                self.paylines.push(payline);
            }
        }
    }

    fn get_visible_symbols(&self) -> Vec<Vec<Symbols>> {
        let mut rows = vec![Vec::new(), Vec::new(), Vec::new()];

        for reel in &self.reels {
            let len = reel.symbols.len();
            let mid = reel.ptr;
            let above = (mid + len - 1) % len;
            let below = (mid + 1) % len;

            rows[0].push(reel.symbols[above].clone());
            rows[1].push(reel.symbols[mid].clone());
            rows[2].push(reel.symbols[below].clone());
        }

        rows
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
