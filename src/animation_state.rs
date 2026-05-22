const INITIAL_WAIT_FRAME: usize = 10;
pub const SPINNING_FRAME_TIME: usize = 35;
const STOPPED_FRAME_TIME: usize = 5;
pub const FRAMES_PER_PAYLINE: usize = 25;
const MIN_SHOW_WINNINGS_FRAMES: usize = 10;
pub const LEVER_PULL_FRAME_TIME: usize = 5;

pub fn show_winnings_frames(max_paylines: usize) -> usize {
    let dynamic = FRAMES_PER_PAYLINE * max_paylines;
    if dynamic < MIN_SHOW_WINNINGS_FRAMES {
        MIN_SHOW_WINNINGS_FRAMES
    } else {
        dynamic
    }
}

/// Total frames for the entire animation across all machines.
/// Earlier machines absorb the stagger into a longer ShowWinnings phase
/// so all machines exit ShowWinnings at the same global frame.
pub fn total_animation_frames(total_machines: usize, max_paylines: usize) -> usize {
    (total_machines * INITIAL_WAIT_FRAME)
        + LEVER_PULL_FRAME_TIME
        + SPINNING_FRAME_TIME
        + show_winnings_frames(max_paylines)
        + STOPPED_FRAME_TIME
}

pub enum AnimationType {
    Wait,
    LeverPull,
    Spinning,
    ShowWinnings,
    Stopped,
}

/// State representing current animation
/// Parameter is frames remaining
pub struct AnimationState {
    pub animation_type: AnimationType,
    pub frames_remaining: usize,
    pub show_winnings_duration: usize,
}

impl AnimationState {
    pub fn new(machine_num: usize, total_machines: usize, max_paylines: usize) -> Self {
        // Earlier machines enter ShowWinnings sooner due to the staggered start.
        // They get extra ShowWinnings frames so all machines exit ShowWinnings
        // at the same global frame. The last machine gets the base duration.
        let show_winnings_duration = show_winnings_frames(max_paylines)
            + (total_machines - 1 - machine_num) * INITIAL_WAIT_FRAME;

        Self {
            animation_type: AnimationType::Wait,
            frames_remaining: (machine_num + 1) * INITIAL_WAIT_FRAME,
            show_winnings_duration,
        }
    }

    pub fn tick(&mut self) {
        if self.frames_remaining == 0 {
            let (animation_type, new_frames_remaining) = self.get_next_state();
            self.animation_type = animation_type;
            self.frames_remaining = new_frames_remaining;
        } else {
            self.frames_remaining -= 1;
        }
    }

    fn get_next_state(&self) -> (AnimationType, usize) {
        match self.animation_type {
            AnimationType::Wait => (AnimationType::LeverPull, LEVER_PULL_FRAME_TIME),
            AnimationType::LeverPull => (AnimationType::Spinning, SPINNING_FRAME_TIME),
            AnimationType::Spinning => (AnimationType::ShowWinnings, self.show_winnings_duration),
            AnimationType::ShowWinnings => (AnimationType::Stopped, STOPPED_FRAME_TIME),
            AnimationType::Stopped => (AnimationType::Stopped, STOPPED_FRAME_TIME),
        }
    }
}
