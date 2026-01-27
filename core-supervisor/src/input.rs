use embassy_time::Instant;

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
    ShortPress { continuing: bool, count: u16 },
    LongPress { continuing: bool, level: u16 },
}

// TODO: Make it support different "limits" for short and long presses.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct InputMethod {
    button: ButtonRole,
    pressed: bool,
    state: InputMethodState,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
enum InputMethodState {
    Idle,
    /// The button was just pressed for the first time in a while, unknown which state
    /// path will it go to
    FirstPress {
        pressed_at: u64,
    },
    /// Button was let go in enough time to constitute it as a short press, but no idea
    /// if more presses will come.
    ShortRelease {
        released_at_ms: u64,
        count: u16,
    },
    /// Button was pressed again before the state machine went back to idle, aka before the multi
    /// short press sequence was ended
    ContinuingPress {
        pressed_at_ms: u64,
        multi_short_press_count: u16,
    },
    LongPress {
        started_at_ms: u64,
        /// The long press "level" we're currently at.
        level: u16,
    },
}

impl InputMethod {
    pub fn new(button: ButtonRole) -> Self {
        InputMethod {
            button,
            pressed: false,
            state: InputMethodState::Idle,
        }
    }
}

/// This function is supposed to be called in a periodic task, ideally every 5 or 10 ms
pub fn process_input_methods(methods: &mut [InputMethod]) {
    for method in methods {
        let instant = Instant::now().as_millis();

        if method.state == InputMethodState::Idle && !method.pressed {
            continue;
        }

        if method.state == InputMethodState::Idle && method.pressed {
            method.state = InputMethodState::FirstPress {
                pressed_at: instant,
            };
            continue;
        }

        // If released before the long press trigger
        if let InputMethodState::FirstPress { pressed_at } = method.state
            && !method.pressed
            && pressed_at + LONG_PRESS_DURATION_MS > instant
        {
            method.state = InputMethodState::ShortRelease {
                released_at_ms: instant,
                count: 1,
            };
            on_event(
                method.button,
                ButtonGesture::ShortPress {
                    continuing: true,
                    count: 1,
                },
            );
            continue;
        }

        // If the button was pressed down for long enough
        if let InputMethodState::FirstPress { pressed_at } = method.state
            && method.pressed
            && pressed_at + LONG_PRESS_DURATION_MS < instant
        {
            method.state = InputMethodState::LongPress {
                started_at_ms: pressed_at,
                level: 1,
            };
            on_event(
                method.button,
                ButtonGesture::LongPress {
                    continuing: true,
                    level: 1,
                },
            );
            continue;
        }

        // It's already at the long press state, but the button is continuing to be held down
        if let InputMethodState::LongPress {
            started_at_ms,
            level,
        } = method.state
            && method.pressed
        {
            let current_level = {
                let held_ms = instant - started_at_ms; // total hold time since initial press
                let beyond_long = held_ms.saturating_sub(LONG_PRESS_DURATION_MS); // time after 1s threshold

                // because int division always rounds down, it will only increment the level
                // if it passed the threshold
                1 + (beyond_long / SUPER_LONG_PRESS_DURATION_MS).min(u16::MAX as u64)
                    as u16
            };

            if level != current_level {
                method.state = InputMethodState::LongPress {
                    started_at_ms,
                    level: current_level,
                };
                on_event(
                    method.button,
                    ButtonGesture::LongPress {
                        continuing: true,
                        level: current_level,
                    },
                );
            }
            continue;
        }

        // If the button was let go while long pressing
        if let InputMethodState::LongPress {
            started_at_ms: _,
            level,
        } = method.state
            && !method.pressed
        {
            on_event(
                method.button,
                ButtonGesture::LongPress {
                    continuing: false,
                    level,
                },
            );
            method.state = InputMethodState::Idle;
            continue;
        }

        // If there are no more presses recorded before the timeout, end the multi short press
        // sequence
        if let InputMethodState::ShortRelease {
            released_at_ms,
            count,
        } = method.state
            && !method.pressed
            && released_at_ms + MULTI_SHORT_PRESS_TIMEOUT_MS < instant
        {
            method.state = InputMethodState::Idle;
            on_event(
                method.button,
                ButtonGesture::ShortPress {
                    continuing: false,
                    count,
                },
            );
            continue;
        }

        // If the button was pressed again before the multi press sequence ended
        if let InputMethodState::ShortRelease {
            released_at_ms: _,
            count,
        } = method.state
            && method.pressed
        {
            method.state = InputMethodState::ContinuingPress {
                pressed_at_ms: instant,
                multi_short_press_count: count,
            };
            continue;
        }

        // If the button was short pressed again after another short press
        if let InputMethodState::ContinuingPress {
            pressed_at_ms,
            multi_short_press_count,
        } = method.state
            && !method.pressed
            && pressed_at_ms + LONG_PRESS_DURATION_MS > instant
        {
            method.state = InputMethodState::ShortRelease {
                released_at_ms: instant,
                count: multi_short_press_count + 1,
            };
            on_event(
                method.button,
                ButtonGesture::ShortPress {
                    continuing: true,
                    count: multi_short_press_count + 1,
                },
            );
            continue;
        }

        // If the button was pressed again after a short press, and this time it passed
        // the long press threshold
        if let InputMethodState::ContinuingPress {
            pressed_at_ms,
            multi_short_press_count,
        } = method.state
            && method.pressed
            && pressed_at_ms + LONG_PRESS_DURATION_MS <= instant
        {
            on_event(
                method.button,
                ButtonGesture::ShortPress {
                    continuing: false,
                    count: multi_short_press_count,
                },
            );
            on_event(
                method.button,
                ButtonGesture::LongPress {
                    continuing: true,
                    level: 1,
                },
            );
            method.state = InputMethodState::LongPress {
                started_at_ms: pressed_at_ms,
                level: 1,
            };
            continue;
        }
    }
}

fn on_event(_button: ButtonRole, _gesture: ButtonGesture) {
    // TODO: To be implemented
}
