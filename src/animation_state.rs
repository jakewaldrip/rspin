const INITIAL_WAIT_FRAME: usize = 20;
const SPINNING_FRAME_TIME: usize = 20;
const STOPPED_FRAME_TIME: usize = 100;

pub enum AnimationType {
    Wait,
    Spinning,
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
            AnimationType::Wait => (AnimationType::Spinning, SPINNING_FRAME_TIME),
            AnimationType::Spinning => (AnimationType::Stopped, STOPPED_FRAME_TIME),
            AnimationType::Stopped => (AnimationType::Stopped, STOPPED_FRAME_TIME),
        }
    }
}
