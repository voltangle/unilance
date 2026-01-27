use core::future::Future;
use core::pin::{Pin, pin};
use embassy_time::Instant;
use heapless::Vec;

const LONG_PRESS_DURATION_MS: u64 = 1000;
/// There are multiple levels to long presses, so this const denotes the amount of time
/// it takes to advance to the next long press "level". So, if you press for 3 seconds,
/// it will be a second level long press, if for 5, then third level, and so on.
const SUPER_LONG_PRESS_DURATION_MS: u64 = 2000;
/// How much time it takes for the state machine to save the current amount of short presses
const MULTI_SHORT_PRESS_TIMEOUT_MS: u64 = 300;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum ButtonRole {
    Power,
    Park,
    Next,
    Headlight,
    Ok,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum ButtonGesture {
    ShortPress { continuing: bool, count: u8 },
    LongPress { continuing: bool, level: u8 },
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct InputMethodState {
    button: ButtonRole,
    pressed: bool,
    /// This timestamp has two roles:
    /// - If it's a number, the button was pressed down and we're counting for how long.
    /// - If it's zero, the button was let go and the associated action was handled.
    last_press_timestamp_ms: u64,
    /// When the last short press was recorded
    last_short_press_timestamp_ms: u64,
    short_press_count: u8,
    long_press_level: u8,
}

enum InputMethodStates {
    Idle,
    /// The button was just pressed for the first time in a while, unknown which state
    /// path will it go to
    FirstPress,
    /// Button was let go in enough time to constitute it as a short press, but no idea
    /// if more presses will come.
    ShortRelease {
        released_at_ms: u64,
        count: u16
    },
    /// Button was pressed again before the state machine went back to idle.
    ContinuingPress {
        pressed_at_ms: u64,
        previous_state: InputMethodStates
    },
    LongPress {
        started_at_ms: u64,
        /// The long press "level" we're currently at.
        level: u16,
    }
}

impl InputMethodState {
    pub fn new(button: ButtonRole) -> Self {
        InputMethodState {
            button,
            pressed: false,
            last_press_timestamp_ms: 0,
            last_short_press_timestamp_ms: 0,
            short_press_count: 0,
            long_press_level: 0,
        }
    }
}

/// This function is supposed to be called in an async task
pub async fn process_input_methods(states: &mut [InputMethodState]) {
    for state in states {
        let instant = Instant::now().as_millis();

        // If pressed just now
        if state.pressed && state.last_press_timestamp_ms == 0 {
            state.last_press_timestamp_ms = instant;
        }

        if state.pressed && state.last_short_press_timestamp_ms != 0 {
            state.short_press_count += 1;
        }

        // Short press handling
        if !state.pressed
            && state.last_press_timestamp_ms + LONG_PRESS_DURATION_MS > instant
        {
            // Button was let go for long enough
            if state.last_short_press_timestamp_ms + MULTI_SHORT_PRESS_TIMEOUT_MS
                < instant
            {
                state.last_short_press_timestamp_ms = 0;
                state.last_press_timestamp_ms = 0;
                on_event(
                    state.button,
                    ButtonGesture::ShortPress {
                        continuing: false,
                        count: state.short_press_count,
                    },
                );
                continue;
            }
        }

        if !state.pressed && state.long_press_level > 1 {
            on_event(state.button, ButtonGesture::LongPress { continuing: false, level: state.long_press_level });
        }

        if state.pressed
            && state.last_press_timestamp_ms + LONG_PRESS_DURATION_MS < instant
        {
            // Count the long press levels
            let level = (((instant - state.last_press_timestamp_ms
                + LONG_PRESS_DURATION_MS)
                / SUPER_LONG_PRESS_DURATION_MS)
                + 1) as u8;
            if state.long_press_level != level {
                on_event(state.button, ButtonGesture::LongPress { continuing: true, level: level });
                state.long_press_level = level;
                continue;
            }
        }
    }
}

fn on_event(_button: ButtonRole, _gesture: ButtonGesture) {
    // TODO: To be implemented
}
