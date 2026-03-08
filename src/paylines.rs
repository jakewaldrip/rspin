use crate::symbols::Symbols;

#[derive(Debug)]
pub enum Paylines {
    HorXL(Symbols),
    Zig(Symbols),
    Zag(Symbols),
    Above(Symbols),
    Below(Symbols),
    Eye(Symbols),
}

impl Paylines {
    pub fn get_payout(&self) -> i32 {
        match self {
            Paylines::HorXL(symbol) => symbol.get_value() * 3,
            Paylines::Zig(symbol) => symbol.get_value() * 4,
            Paylines::Zag(symbol) => symbol.get_value() * 4,
            Paylines::Above(symbol) => symbol.get_value() * 8,
            Paylines::Below(symbol) => symbol.get_value() * 8,
            Paylines::Eye(symbol) => symbol.get_value() * 10,
        }
    }
}

// todo, consider putting fn pointers to the below in a slice and executing the whole slice
// something eventually returning Option<Paylines> for simplicity
// caller could just extract reels into the visible slice (fn on reels to do this)
// machines.map(|machine| let x = machine.reels.map(extract); check_fn_ptrs.map(|fn_ptr| fn_ptr(x)))

/// Pass a slice of reels, with a slice representing the visible area
/// 0 is top, 1 is middle, 2 is bottom
/// 0 - 4 is outer
pub fn check_hor_xl(_reels: &[&[Symbols]]) -> bool {
    todo!()
}

pub fn check_zig(_reels: &[&[Symbols]]) -> bool {
    todo!()
}

pub fn check_zag(_reels: &[&[Symbols]]) -> bool {
    todo!()
}

pub fn check_above(_reels: &[&[Symbols]]) -> bool {
    todo!()
}

pub fn check_below(_reels: &[&[Symbols]]) -> bool {
    todo!()
}

pub fn check_eye(_reels: &[&[Symbols]]) -> bool {
    todo!()
}
