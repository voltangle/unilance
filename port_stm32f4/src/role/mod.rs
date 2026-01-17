#[cfg(not(any(feature = "role_control", feature = "role_supervisor")))]
compile_error!("Enable at least one role: role_control or role_supervisor (or both)!")
