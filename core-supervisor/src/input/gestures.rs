use defmt::Format;
use embassy_time::Instant;

const LONG_PRESS_DURATION_MS: u64 = 1000;
/// There are multiple levels to long presses, so this const denotes the amount of time
/// it takes to advance to the next long press "level". So, if you press for 3 seconds,
/// it will be a second level long press, if for 5, then third level, and so on.
const SUPER_LONG_PRESS_DURATION_MS: u64 = 2000;
/// How much time it takes for the state machine to save the current amount of short presses
const MULTI_SHORT_PRESS_TIMEOUT_MS: u64 = 300;

// TODO: also support long press to short press transitions without breaking the gesture
// apart
#[derive(Clone, Copy, PartialEq, PartialOrd, Format)]
pub(crate) enum ButtonGesture {
    ShortPress {
        continuing: bool,
        count: u16,
    },
    LongPress {
        continuing: bool,
        level: u16,
    },
    /// If you do a short press and immediately follow it with a long press, this state
    /// will be emitted. Before, it was replaced by two state emissions in the form of
    /// ShortPress with continuing = false and immediately followed by LongPress with
    /// level 1 and continuing = true.
    ShortLongTransition {
        short_count: u16,
    },
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Format, Default)]
pub(crate) enum InputMethodState {
    #[default]
    Idle,
    /// The button was just pressed for the first time in a while, unknown which state
    /// path will it go to
    FirstPress { pressed_at: u64 },
    /// Button was let go in enough time to constitute it as a short press, but no idea
    /// if more presses will come.
    ShortRelease { released_at_ms: u64, count: u16 },
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

/// Processes button gestures, like long press, multi press, etc. Returns the current
/// gesture (if there is anything new), and modifies [state] with the new current state.
pub fn process_button_gestures(
    state: &mut InputMethodState,
    pressed: bool,
) -> Option<ButtonGesture> {
    let instant = Instant::now().as_millis();

    if *state == InputMethodState::Idle && !pressed {
        return None;
    }

    if *state == InputMethodState::Idle && pressed {
        *state = InputMethodState::FirstPress {
            pressed_at: instant,
        };
        return None;
    }

    // If released before the long press trigger
    if let InputMethodState::FirstPress { pressed_at } = *state
        && !pressed
        && pressed_at + LONG_PRESS_DURATION_MS > instant
    {
        *state = InputMethodState::ShortRelease {
            released_at_ms: instant,
            count: 1,
        };
        return Some(ButtonGesture::ShortPress {
            continuing: true,
            count: 1,
        });
    }

    // If the button was pressed down for long enough
    if let InputMethodState::FirstPress { pressed_at } = *state
        && pressed
        && pressed_at + LONG_PRESS_DURATION_MS < instant
    {
        *state = InputMethodState::LongPress {
            started_at_ms: pressed_at,
            level: 1,
        };
        return Some(ButtonGesture::LongPress {
            continuing: true,
            level: 1,
        });
    }

    // It's already at the long press state, but the button is continuing to be held down
    if let InputMethodState::LongPress {
        started_at_ms,
        level,
    } = *state
        && pressed
    {
        let current_level = {
            let held_ms = instant - started_at_ms; // total hold time since initial press
            let beyond_long = held_ms.saturating_sub(LONG_PRESS_DURATION_MS); // time after 1s threshold

            // because int division always rounds down, it will only increment the level
            // if it passed the threshold
            1 + (beyond_long / SUPER_LONG_PRESS_DURATION_MS).min(u16::MAX as u64) as u16
        };

        if level != current_level {
            *state = InputMethodState::LongPress {
                started_at_ms,
                level: current_level,
            };
            return Some(ButtonGesture::LongPress {
                continuing: true,
                level: current_level,
            });
        }
    }

    // If the button was let go while long pressing
    if let InputMethodState::LongPress {
        started_at_ms: _,
        level,
    } = *state
        && !pressed
    {
        *state = InputMethodState::Idle;
        return Some(ButtonGesture::LongPress {
            continuing: false,
            level,
        });
    }

    // If there are no more presses recorded before the timeout, end the multi short press
    // sequence
    if let InputMethodState::ShortRelease {
        released_at_ms,
        count,
    } = *state
        && !pressed
        && released_at_ms + MULTI_SHORT_PRESS_TIMEOUT_MS < instant
    {
        *state = InputMethodState::Idle;
        return Some(ButtonGesture::ShortPress {
            continuing: false,
            count,
        });
    }

    // If the button was pressed again before the multi press sequence ended
    if let InputMethodState::ShortRelease {
        released_at_ms: _,
        count,
    } = *state
        && pressed
    {
        *state = InputMethodState::ContinuingPress {
            pressed_at_ms: instant,
            multi_short_press_count: count,
        };
        return None;
    }

    // If the button was short pressed again after another short press
    if let InputMethodState::ContinuingPress {
        pressed_at_ms,
        multi_short_press_count,
    } = *state
        && !pressed
        && pressed_at_ms + LONG_PRESS_DURATION_MS > instant
    {
        *state = InputMethodState::ShortRelease {
            released_at_ms: instant,
            count: multi_short_press_count + 1,
        };
        return Some(ButtonGesture::ShortPress {
            continuing: true,
            count: multi_short_press_count + 1,
        });
    }

    // If the button was pressed again after a short press, and this time it passed
    // the long press threshold
    if let InputMethodState::ContinuingPress {
        pressed_at_ms,
        multi_short_press_count,
    } = *state
        && pressed
        && pressed_at_ms + LONG_PRESS_DURATION_MS <= instant
    {
        *state = InputMethodState::LongPress {
            started_at_ms: pressed_at_ms,
            level: 1,
        };
        return Some(ButtonGesture::ShortLongTransition {
            short_count: multi_short_press_count,
        });
    }

    None
}
