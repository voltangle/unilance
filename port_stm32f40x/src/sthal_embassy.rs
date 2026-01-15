use core::{
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};
use embassy_time::Duration;
use unilance_mesc::c_bind::{
    HAL_StatusTypeDef, HAL_StatusTypeDef_HAL_ERROR, HAL_StatusTypeDef_HAL_OK,
    TIM_HandleTypeDef,
};

// Set from main
pub static HCLK_HZ: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn HAL_Delay(ms: u32) {
    // Blocking delay implemented by Embassy's time driver.
    // This does not require async/await at the call site.
    embassy_time::block_for(Duration::from_millis(ms as u64));
}

#[unsafe(no_mangle)]
pub extern "C" fn HAL_RCC_GetHCLKFreq() -> u32 {
    HCLK_HZ.load(Ordering::Relaxed)
}

#[unsafe(no_mangle)]
pub extern "C" fn HAL_TIM_Base_Start(htim: *mut TIM_HandleTypeDef) -> HAL_StatusTypeDef {
    if htim.is_null() {
        return HAL_StatusTypeDef_HAL_ERROR;
    }

    unsafe {
        let tim = (*htim).Instance;
        if tim.is_null() {
            return HAL_StatusTypeDef_HAL_ERROR;
        }

        let cr1_ptr: *mut u32 = ptr::addr_of_mut!((*tim).CR1);
        let cr1 = cr1_ptr.read_volatile();
        cr1_ptr.write_volatile(cr1 | 1);
    }

    HAL_StatusTypeDef_HAL_OK
}
