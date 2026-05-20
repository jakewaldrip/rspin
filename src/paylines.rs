use crate::symbols::Symbols;

#[derive(Debug, Clone)]
pub enum Paylines {
    HorSM(Symbols, Vec<(usize, usize)>),
    AboveSM(Symbols, Vec<(usize, usize)>),
    BelowSM(Symbols, Vec<(usize, usize)>),
    ZigSM(Symbols, Vec<(usize, usize)>),
    ZagSM(Symbols, Vec<(usize, usize)>),
    HorXL(Symbols, Vec<(usize, usize)>),
    Zig(Symbols, Vec<(usize, usize)>),
    Zag(Symbols, Vec<(usize, usize)>),
    Above(Symbols, Vec<(usize, usize)>),
    Below(Symbols, Vec<(usize, usize)>),
    Eye(Symbols, Vec<(usize, usize)>),
}

impl Paylines {
    pub fn get_payout(&self, bet: i32) -> i32 {
        match self {
            Paylines::HorSM(symbol, _) => symbol.get_value() * bet,
            Paylines::AboveSM(symbol, _) => symbol.get_value() * bet,
            Paylines::BelowSM(symbol, _) => symbol.get_value() * bet,
            Paylines::ZigSM(symbol, _) => symbol.get_value() * 2 * bet,
            Paylines::ZagSM(symbol, _) => symbol.get_value() * 2 * bet,
            Paylines::HorXL(symbol, _) => symbol.get_value() * 3 * bet,
            Paylines::Zig(symbol, _) => symbol.get_value() * 4 * bet,
            Paylines::Zag(symbol, _) => symbol.get_value() * 4 * bet,
            Paylines::Above(symbol, _) => symbol.get_value() * 8 * bet,
            Paylines::Below(symbol, _) => symbol.get_value() * 8 * bet,
            Paylines::Eye(symbol, _) => symbol.get_value() * 10 * bet,
        }
    }
}

// pass a slice of rows (0=top, 1=middle, 2=bottom),
// each row containing 5 symbols (one per reel, columns 0-4)
fn check_payline(visible_reels: &[&[Symbols]], positions: &[(usize, usize)]) -> Option<Symbols> {
    // Find the first non-wild symbol to use as the target
    let target = positions
        .iter()
        .map(|&(row, col)| &visible_reels[row][col])
        .find(|s| **s != Symbols::Wild);

    // All positions are wild gives no payout (Wild has no value on its own)
    // Consider revisiting this. I don't want to overcomplicate the logic here,
    // but getting a wild payout would feel nice
    let target = target?;

    if positions.iter().all(|&(row, col)| {
        visible_reels[row][col] == *target || visible_reels[row][col] == Symbols::Wild
    }) {
        Some(target.clone())
    } else {
        None
    }
}

pub type PaylineCheckerFn = fn(&[&[Symbols]]) -> Option<Paylines>;

///
/// . - - - .
///
pub fn check_hor_sm(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 3] = [(1, 1), (1, 2), (1, 3)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::HorSM(symbols, positions_vec))
}

/// . - - - .
///
///
pub fn check_above_sm(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 3] = [(0, 1), (0, 2), (0, 3)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions)
        .map(|symbols| Paylines::AboveSM(symbols, positions_vec))
}

///
///
/// . - - - .
pub fn check_below_sm(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 3] = [(2, 1), (2, 2), (2, 3)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions)
        .map(|symbols| Paylines::BelowSM(symbols, positions_vec))
}

///     -
///   -
/// -
pub fn check_zig_sm(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 3] = [(2, 1), (1, 2), (0, 3)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::ZigSM(symbols, positions_vec))
}

/// -
///   -
///     -
pub fn check_zag_sm(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 3)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::ZagSM(symbols, positions_vec))
}

///
/// -----
///
pub fn check_hor_xl(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 5] = [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::HorXL(symbols, positions_vec))
}

///   -
///  - -
/// -   -
pub fn check_zig(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 5] = [(2, 0), (1, 1), (0, 2), (1, 3), (2, 4)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::Zig(symbols, positions_vec))
}

/// -   -
///  - -
///   -
pub fn check_zag(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 5] = [(0, 0), (1, 1), (2, 2), (1, 3), (0, 4)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::Zag(symbols, positions_vec))
}

/// -----
///
///
pub fn check_above(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 5] = [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::Above(symbols, positions_vec))
}

///
///
/// -----
pub fn check_below(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 5] = [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)];
    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::Below(symbols, positions_vec))
}

///  ---
/// -   -
///  ---
pub fn check_eye(visible_reels: &[&[Symbols]]) -> Option<Paylines> {
    let positions: [(usize, usize); 8] = [
        (1, 0),
        (0, 1),
        (2, 1),
        (0, 2),
        (2, 2),
        (0, 3),
        (2, 3),
        (1, 4),
    ];

    let positions_vec = positions.to_vec();
    check_payline(visible_reels, &positions).map(|symbols| Paylines::Eye(symbols, positions_vec))
}
