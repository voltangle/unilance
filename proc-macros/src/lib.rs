use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Nothing, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, Item, ItemStruct, LitStr, Result, Token, parse_macro_input};

struct Args {
    roles: Vec<LitStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let roles: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        if roles.is_empty() {
            return Err(Error::new(
                input.span(),
                "Expected at least one role string literal.",
            ));
        }
        Ok(Self {
            roles: roles.into_iter().collect(),
        })
    }
}

fn cfg_for_role(role_lit: &LitStr) -> Result<proc_macro2::TokenStream> {
    let role = role_lit.value();
    let ts = match role.as_str() {
        "control" => quote! {
            feature = "role_control"
        },
        "control_exclusive" => quote! {
            all(feature = "role_control", not(feature = "role_supervisor"))
        },
        "supervisor" => quote! {
            feature = "role_supervisor"
        },
        "supervisor_exclusive" => quote! {
            all(not(feature = "role_control"), feature = "role_supervisor")
        },
        "combined" => quote! {
            all(feature = "role_control", feature = "role_supervisor")
        },
        "either" => quote! {
            all(
                any(feature = "role_control", feature = "role_supervisor"),
                not(all(feature = "role_control", feature = "role_supervisor"))
            )
        },
        _ => {
            return Err(Error::new_spanned(
                role_lit,
                "Invalid role. Use: \"control\", \"supervisor\", \"either\", or \"combined\".",
            ));
        }
    };
    Ok(ts)
}

fn validate_registration_item(item: &ItemStruct, macro_name: &str) -> Result<()> {
    if !item.generics.params.is_empty() || item.generics.where_clause.is_some() {
        return Err(Error::new_spanned(
            &item.generics,
            format!("`{macro_name}` does not support generic structs."),
        ));
    }

    Ok(())
}

/// Bind a MESC [`::mesc::Hal`] implementation to the exported C hooks.
///
/// Apply this to the marker struct that implements [`::mesc::Hal`]. The macro keeps the
/// struct in place and emits the required `#[unsafe(no_mangle)] extern "C"` wrappers.
///
/// ```ignore
/// #[mesc::global_hal]
/// struct MotorHal;
///
/// impl mesc::Hal for MotorHal {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn global_hal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as Nothing);
    let item_ast = parse_macro_input!(item as ItemStruct);

    if let Err(error) = validate_registration_item(&item_ast, "global_hal") {
        return error.to_compile_error().into();
    }

    let ident = &item_ast.ident;

    quote! {
        #item_ast

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESC_getHallState() -> i32 {
            <#ident as ::mesc::Hal>::get_hall_state() as i32
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESCfoc_getRawADC() {
            <#ident as ::mesc::Hal>::refresh_adc();
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESCfoc_getRawADCVph() {
            <#ident as ::mesc::Hal>::refresh_adc_for_vphase();
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setDeadtimeNs(motor: &mut ::mesc::MESC_motor_typedef, ns: u16) {
            <#ident as ::mesc::Hal>::set_deadtime(motor, ns);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_break(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_a_break(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_break(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_b_break(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_break(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_c_break(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_enable(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_a_enable(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_enable(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_b_enable(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_enable(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::phase_c_enable(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_enableOutput(motor: &mut ::mesc::MESC_motor_typedef) {
            <#ident as ::mesc::Hal>::enable_output(motor);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_setDuty(motor: &mut ::mesc::MESC_motor_typedef, duty: u16) {
            <#ident as ::mesc::Hal>::phase_a_set_duty(motor, duty);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_setDuty(motor: &mut ::mesc::MESC_motor_typedef, duty: u16) {
            <#ident as ::mesc::Hal>::phase_b_set_duty(motor, duty);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_setDuty(motor: &mut ::mesc::MESC_motor_typedef, duty: u16) {
            <#ident as ::mesc::Hal>::phase_c_set_duty(motor, duty);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phD_setDuty(motor: &mut ::mesc::MESC_motor_typedef, duty: u16) {
            <#ident as ::mesc::Hal>::phase_d_set_duty(motor, duty);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_getDuty(motor: &mut ::mesc::MESC_motor_typedef) -> u16 {
            <#ident as ::mesc::Hal>::phase_a_get_duty(motor)
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_getDuty(motor: &mut ::mesc::MESC_motor_typedef) -> u16 {
            <#ident as ::mesc::Hal>::phase_b_get_duty(motor)
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_getDuty(motor: &mut ::mesc::MESC_motor_typedef) -> u16 {
            <#ident as ::mesc::Hal>::phase_c_get_duty(motor)
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_getMaxDuty(motor: &mut ::mesc::MESC_motor_typedef) -> u16 {
            <#ident as ::mesc::Hal>::get_max_duty(motor)
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setPWMFrequency(motor: &mut ::mesc::MESC_motor_typedef, freq: u32) {
            <#ident as ::mesc::Hal>::set_pwm_frequency(motor, freq);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setIRQ(motor: &mut ::mesc::MESC_motor_typedef, state: bool) {
            <#ident as ::mesc::Hal>::set_irq(motor, state);
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_isTimerCountingDown(motor: &mut ::mesc::MESC_motor_typedef) -> bool {
            <#ident as ::mesc::Hal>::is_tim_counting_down(motor)
        }
    }
    .into()
}

/// Bind a MESC [`::mesc::CoreHal`] implementation to the exported C hooks.
///
/// Apply this to the marker struct that implements [`::mesc::CoreHal`]. The macro keeps the
/// struct in place and emits the required `#[unsafe(no_mangle)] extern "C"` wrappers.
///
/// ```ignore
/// #[mesc::global_core_hal]
/// struct MescImpl;
///
/// impl mesc::CoreHal for MescImpl {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn global_core_hal(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as Nothing);
    let item_ast = parse_macro_input!(item as ItemStruct);

    if let Err(error) = validate_registration_item(&item_ast, "global_core_hal") {
        return error.to_compile_error().into();
    }

    let ident = &item_ast.ident;

    quote! {
        #item_ast

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_delayMs(ms: u32) {
            <#ident as ::mesc::CoreHal>::delay_ms(ms)
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_getCPUCycles() -> u32 {
            <#ident as ::mesc::CoreHal>::get_cpu_cycles()
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTrace(string: *const ::core::ffi::c_char) {
            <#ident as ::mesc::CoreHal>::log_trace(unsafe {
                ::core::ffi::CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTraceDouble(
            msg: *const ::core::ffi::c_char,
            num: ::core::ffi::c_double,
        ) {
            <#ident as ::mesc::CoreHal>::log_trace_double(
                unsafe { ::core::ffi::CStr::from_ptr(msg).to_str().unwrap() },
                num as f64,
            )
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTraceInt(
            msg: *const ::core::ffi::c_char,
            num: ::core::ffi::c_uint,
        ) {
            <#ident as ::mesc::CoreHal>::log_trace_int(
                unsafe { ::core::ffi::CStr::from_ptr(msg).to_str().unwrap() },
                num as u32,
            )
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logDebug(string: *const ::core::ffi::c_char) {
            <#ident as ::mesc::CoreHal>::log_debug(unsafe {
                ::core::ffi::CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logInfo(string: *const ::core::ffi::c_char) {
            <#ident as ::mesc::CoreHal>::log_info(unsafe {
                ::core::ffi::CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logWarn(string: *const ::core::ffi::c_char) {
            <#ident as ::mesc::CoreHal>::log_warn(unsafe {
                ::core::ffi::CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logError(string: *const ::core::ffi::c_char) {
            <#ident as ::mesc::CoreHal>::log_error(unsafe {
                ::core::ffi::CStr::from_ptr(string).to_str().unwrap()
            })
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn for_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);

    // Parse the annotated thing as an Item so we can re-emit it cleanly.
    let item_ast = parse_macro_input!(item as Item);

    let mut cfgs = Vec::with_capacity(args.roles.len());
    for role_lit in &args.roles {
        match cfg_for_role(role_lit) {
            Ok(cfg) => cfgs.push(cfg),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    let cfg_expr = if cfgs.len() == 1 {
        // Single role => no need for any(...)
        let one = &cfgs[0];
        quote!(#one)
    } else {
        // Multiple roles => any(role1, role2, ...)
        quote!(any(#(#cfgs),*))
    };

    let expanded = quote! {
        #[cfg(#cfg_expr)]
        #item_ast
    };

    expanded.into()
}
