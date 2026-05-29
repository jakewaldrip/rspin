# rspin

A terminal-based slot machine simulator with animated reels. Built in Rust.

<!-- screenshot -->
<p align="center">
  <img src="" alt="rspin gameplay screenshot" width="600">
</p>

## Installation

### From crates.io

```bash
cargo install rspin
```

### From source

```bash
git clone https://github.com/jakewaldrip/rspin.git
cd rspin
cargo build --release
./target/release/rspin play 10
```

> Requires Rust 1.85 or later.

## Usage

```bash
rspin play <bet> [count]   # Spin! Wager <bet> credits on [count] machines (default: 1)
rspin balance              # Check your credit balance
rspin cheat <amount>       # Add (or subtract) credits — no judgement here
```

### Examples

```bash
rspin play 10       # Bet 10 credits on a single machine
rspin play 5 3      # Bet 5 credits each on 3 machines at once (15 total)
rspin balance       # See how you're doing
rspin cheat 500     # Feeling generous with yourself? Go for it
```

## How to Play

You start with **1,000 credits**. Place a bet, pull the lever, and hope the reels land in your favor. It's that simple.

Each machine has **5 reels** with **3 visible rows**. Match symbols across a payline to win.

### Symbols

| Symbol | Display | Tier | Payout |
|--------|---------|------|--------|
| Circle | `O` | Low | 1x |
| Hashtag | `#` | Low | 1x |
| Dollar | `$` | Low | 1x |
| At Sign | `@` | Low | 1x |
| Seven | `7` | Medium | 2x |
| Asterisk | `*` | Medium | 2x |
| Ampersand | `&` | Medium | 2x |
| Jackpot | `!` | High | 7x |
| Wild | `X` | Special | Matches any symbol |

### Paylines

There are **11 paylines** the game checks on every spin. They come in two flavors:

| Type | Patterns | Multiplier |
|------|----------|------------|
| **3-wide** (reels 1-3) | Horizontal (top, middle, bottom), diagonal up, diagonal down | 1-2x |
| **5-wide** (all reels) | Horizontal (top, middle, bottom), zigzag, zagzag | 3-8x |
| **Eye** (all reels) | Diamond shape across the grid | 10x |

Multiple paylines can hit on the same spin — the more matches, the bigger the payday.

Your final payout for each payline is: **symbol value x payline multiplier x your bet**.

### Wild Symbols

The Wild (`X`) substitutes for any symbol on a payline. A line of all Wilds pays nothing though — you need at least one real symbol in the mix.

### Multi-Machine Play

Feeling bold? Spin multiple machines at once with `rspin play <bet> <count>`. They all animate simultaneously with staggered starts. Your total wager is `bet x count`, and winnings from all machines are added together.

## Save Data

Your balance and spin count are saved automatically between sessions.

| Platform | Location |
|----------|----------|
| macOS | `~/Library/Application Support/com.rspin-database.rspin/state.toml` |
| Linux | `~/.local/share/rspin/state.toml` |

## License

[MIT](https://opensource.org/licenses/MIT)
