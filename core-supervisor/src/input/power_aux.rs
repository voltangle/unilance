use defmt::{Format, info};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;

use crate::input::gestures::{self, ButtonGesture, process_button_gestures};
use crate::input::{ButtonRole, unilance_input_is_pressed};

#[derive(Clone, Format, Default)]
pub struct State {
    power: gestures::InputMethodState,
    aux: gestures::InputMethodState,
}

pub async fn input_handler(state: &'static Mutex<impl RawMutex, crate::State>) {
    let power_gesture =
        process_button_gestures(&mut state.lock().await.input.power, unsafe {
            unilance_input_is_pressed(ButtonRole::Power)
        });
    let aux_gesture =
        process_button_gestures(&mut state.lock().await.input.aux, unsafe {
            unilance_input_is_pressed(ButtonRole::Aux)
        });

    if let Some(gesture) = power_gesture {
        match gesture {
            ButtonGesture::ShortPress {
                continuing: _,
                count: _,
            } => {
                info!("Power button short pressed");
            }
            ButtonGesture::LongPress {
                continuing: _,
                level,
            } => {
                // only trigger once
                if level == 1 {
                    info!("Power button long pressed; has to trigger a shutdown");
                }
            }
            _ => {}
        }
    }

    if let Some(gesture) = aux_gesture {
        match gesture {
            ButtonGesture::ShortPress {
                continuing: _,
                count,
            } => {
                if count == 2 {
                    info!("Aux button pressed twice; needs to trigger parking mode");
                }
            }
            ButtonGesture::LongPress { continuing, level } => {
                if continuing {
                    info!("Aux button long pressing; at profile index {}", level);
                } else {
                    info!("Aux button long pressed; needs to set profile {}", level);
                }
            }
            _ => {}
        }
    }
}
