const INITIAL_WAIT_FRAME: usize = 10;
pub const SPINNING_FRAME_TIME: usize = 20;
const STOPPED_FRAME_TIME: usize = 50;
const SHOW_WINNINGS_FRAME_TIME: usize = 50;
pub const LEVER_PULL_FRAME_TIME: usize = 5;

pub const ANIMATION_TIME_FLOOR: usize = INITIAL_WAIT_FRAME
    + SPINNING_FRAME_TIME
    + STOPPED_FRAME_TIME
    + LEVER_PULL_FRAME_TIME
    + SHOW_WINNINGS_FRAME_TIME;
pub const ANIMATION_TIME_MACHINE_FACTOR: usize = INITIAL_WAIT_FRAME + LEVER_PULL_FRAME_TIME;

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
}

impl AnimationState {
    pub fn new(machine_num: usize) -> Self {
        Self {
            animation_type: AnimationType::Wait,
            frames_remaining: (machine_num + 1) * INITIAL_WAIT_FRAME,
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
            AnimationType::Spinning => (AnimationType::ShowWinnings, SHOW_WINNINGS_FRAME_TIME),
            AnimationType::ShowWinnings => (AnimationType::Stopped, STOPPED_FRAME_TIME),
            AnimationType::Stopped => (AnimationType::Stopped, STOPPED_FRAME_TIME),
        }
    }
}
