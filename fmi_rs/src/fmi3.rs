#![allow(clippy::too_many_arguments)]

use std::ffi::{c_char, c_void};

/// Represents the status returned by FMI functions.
#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum Status {
    /// All well.
    Ok = 0,
    /// Things are not quite right, but the computation can continue.
    Warning = 1,
    /// The FMU decided to skip this step.
    Discard = 2,
    /// An error occurred that can be recovered from.
    Error = 3,
    /// A global error occurred; the simulation cannot continue.
    Fatal = 4,
}

impl TryFrom<i32> for Status {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::Warning),
            2 => Ok(Self::Discard),
            3 => Ok(Self::Error),
            4 => Ok(Self::Fatal),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum DependencyKind {
    Independent = 0,
    Constant = 1,
    Fixed = 2,
    Tunable = 3,
    Discrete = 4,
    Dependent = 5,
}

impl TryFrom<i32> for DependencyKind {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(Self::Independent),
            1 => Ok(Self::Constant),
            2 => Ok(Self::Fixed),
            3 => Ok(Self::Tunable),
            4 => Ok(Self::Discrete),
            5 => Ok(Self::Dependent),
            _ => Err(Status::Fatal),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum IntervalQualifier {
    IntervalNotYetKnown = 0,
    IntervalUnchanged = 1,
    IntervalChanged = 2,
}

impl TryFrom<i32> for IntervalQualifier {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(Self::IntervalNotYetKnown),
            1 => Ok(Self::IntervalUnchanged),
            2 => Ok(Self::IntervalChanged),
            _ => Err(Status::Fatal),
        }
    }
}

pub trait FMI3: Sized {
    fn instantiate_model_exchange(
        _instance_name: &str,
        _instantiation_token: &str,
        _resource_path: &str,
        _visible: bool,
        _logging_on: bool,
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: i32,
            category: *const c_char,
            message: *const c_char,
        ) -> *mut c_void,
    ) -> Option<Self> {
        None
    }

    fn instantiate_co_simulation(
        _instance_name: &str,
        _instantiation_token: &str,
        _resource_path: &str,
        _visible: bool,
        _logging_on: bool,
        _event_mode_used: bool,
        _early_return_allowed: bool,
        _intermediate_variables: &[u32],
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: i32,
            category: *const c_char,
            message: *const c_char,
        ) -> *mut c_void,
        _intermediate_update: *const extern "C" fn(
            instance_enivronment: *mut c_void,
        ) -> *mut c_void,
    ) -> Option<Self> {
        None
    }

    fn instantiate_scheduled_execution(
        _instance_name: &str,
        _instantiation_token: &str,
        _resource_path: &str,
        _visible: bool,
        _logging_on: bool,
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: i32,
            category: *const c_char,
            message: *const c_char,
        ) -> *mut c_void,
        _lock_preemption: *const extern "C" fn() -> *mut c_void,
        _unlock_preemption: *const extern "C" fn() -> *mut c_void,
    ) -> Option<Self> {
        None
    }

    fn set_debug_logging(&mut self, _logging_on: bool, _categories: Vec<&str>) -> Status {
        Status::Ok
    }

    fn enter_configuration_mode(&mut self) -> Status {
        Status::Ok
    }

    fn exit_configuration_mode(&mut self) -> Status {
        Status::Ok
    }

    fn enter_initialization_mode(
        &mut self,
        _tolerance_defined: bool,
        _tolerance: f64,
        _start_time: f64,
        _stop_time_defined: bool,
        _stop_time: f64,
    ) -> Status {
        Status::Ok
    }

    fn exit_initialization_mode(&mut self) -> Status {
        Status::Ok
    }

    fn enter_event_mode(
        &mut self,
        _step_event: bool,
        _state_event: bool,
        _roots: &[i32],
        _time_event: bool,
    ) -> Status {
        Status::Ok
    }

    fn terminate(&mut self) -> Status {
        Status::Ok
    }

    fn reset(&mut self) -> Status {
        Status::Ok
    }

    fn get_float64(&mut self, _vr: u32, _value: &mut f64) -> Status {
        Status::Error
    }

    fn set_float64(&mut self, _vr: u32, _value: f64) -> Status {
        Status::Error
    }

    fn get_float32(&mut self, _vr: u32, _value: &mut f32) -> Status {
        Status::Error
    }

    fn set_float32(&mut self, _vr: u32, _value: f32) -> Status {
        Status::Error
    }

    fn get_int8(&mut self, _vr: u32, _value: &mut i8) -> Status {
        Status::Error
    }

    fn set_int8(&mut self, _vr: u32, _value: i8) -> Status {
        Status::Error
    }

    fn get_int16(&mut self, _vr: u32, _value: &mut i16) -> Status {
        Status::Error
    }

    fn set_int16(&mut self, _vr: u32, _value: i16) -> Status {
        Status::Error
    }

    fn get_int32(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    fn set_int32(&mut self, _vr: u32, _value: i32) -> Status {
        Status::Error
    }

    fn get_int64(&mut self, _vr: u32, _value: &mut i64) -> Status {
        Status::Error
    }

    fn set_int64(&mut self, _vr: u32, _value: i64) -> Status {
        Status::Error
    }

    fn get_uint8(&mut self, _vr: u32, _value: &mut u8) -> Status {
        Status::Error
    }

    fn set_uint8(&mut self, _vr: u32, _value: u8) -> Status {
        Status::Error
    }

    fn get_uint16(&mut self, _vr: u32, _value: &mut u16) -> Status {
        Status::Error
    }

    fn set_uint16(&mut self, _vr: u32, _value: u16) -> Status {
        Status::Error
    }

    fn get_uint32(&mut self, _vr: u32, _value: &mut u32) -> Status {
        Status::Error
    }

    fn set_uint32(&mut self, _vr: u32, _value: u32) -> Status {
        Status::Error
    }

    fn get_uint64(&mut self, _vr: u32, _value: &mut u64) -> Status {
        Status::Error
    }

    fn set_uint64(&mut self, _vr: u32, _value: u64) -> Status {
        Status::Error
    }

    fn get_boolean(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    fn set_boolean(&mut self, _vr: u32, _value: bool) -> Status {
        Status::Error
    }

    fn get_string(&mut self, _vr: u32, _value: &mut *const c_char) -> Status {
        Status::Error
    }

    fn set_string(&mut self, _vr: u32, _value: &str) -> Status {
        Status::Error
    }

    fn get_binary(&mut self, _vr: u32, _size: &mut usize, _value: &mut *const u8) -> Status {
        Status::Error
    }

    fn set_binary(&mut self, _vr: u32, _value: &[u8]) -> Status {
        Status::Error
    }

    fn get_clock(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    fn set_clock(&mut self, _vr: u32, _value: bool) -> Status {
        Status::Error
    }

    /// Returns the required buffer size for the serialized state.
    fn serialized_fmu_state_size(&mut self, _state: *mut c_void, _size: *mut usize) -> Status {
        Status::Ok
    }

    /// Serializes the FMU state into a byte buffer.
    fn serialize_fmu_state(&mut self, _state: *mut c_void, _serialized_state: &[u8]) -> Status {
        Status::Ok
    }

    /// Deserializes the FMU state from a byte buffer.
    fn deserialized_fmu_state(
        &mut self,
        _buffer: &[u8],
        _size: usize,
        _state: *mut *mut c_void,
    ) -> Status {
        Status::Ok
    }

    /// Sets the internal state of the FMU.
    fn set_fmu_state(&mut self, _state: *mut c_void) -> Status {
        Status::Ok
    }

    /// Captures the internal state of the FMU.
    fn get_fmu_state(&mut self, _state: *mut *mut c_void) -> Status {
        Status::Ok
    }

    /// Frees a previously captured FMU state.
    fn free_fmu_state(&mut self, _state: *mut c_void) -> Status {
        Status::Ok
    }

    /// Computes partial derivatives (directional derivatives).
    fn get_directional_derivative(
        &mut self,
        _v_known: &[u32],
        _v_unknown: &[u32],
        _dv_known: &[f64],
        _dv_unknown: &mut [f64],
    ) -> Status {
        Status::Ok
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior: bool,
        _event_encountered: &mut bool,
        _terminate: &mut bool,
        _early_return: &mut bool,
        _last_successful_time: &mut f64,
    ) -> Status {
        Status::Fatal
    }

    fn enter_step_mode(&mut self) -> Status {
        Status::Ok
    }

    fn set_time(&mut self, _time: f64) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Sets new continuous state values.
    fn set_continuous_states(&mut self, _x: &[f64]) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Retrieves the state derivatives.
    fn get_derivatives(&mut self, _dx: &mut [f64]) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Retrieves the event indicators (zero-crossing functions).
    fn get_event_indicators(&mut self, _ei: &mut [f64]) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Retrieves current continuous states.
    fn get_continuous_states(&mut self, _x: &mut [f64]) -> Status {
        Status::Ok
    }

    fn new_discrete_states(
        &mut self,
        _discrete_states_need_update: &mut bool,
        _terminate_simulation: &mut bool,
        _nominals_of_continuous_states_changed: &mut bool,
        _values_of_continuous_states_changed: &mut bool,
        _next_event_time_defined: &mut bool,
        _next_event_time: *mut f64,
    ) -> Status {
        Status::Ok
    }

    fn set_interval_decimal(&mut self, _vr: u32, _interval: f64) -> Status {
        Status::Ok
    }

    fn get_interval_decimal(&mut self, _vr: u32, _interval: &mut f64) -> Status {
        Status::Ok
    }

    fn set_interval_fraction(&mut self, _vr: u32, _interval: f64) -> Status {
        Status::Ok
    }

    fn get_interval_fraction(&mut self, _vr: u32, _interval: &mut f64) -> Status {
        Status::Ok
    }

    fn set_shift_decimal(&mut self, _vr: u32, _interval: f64) -> Status {
        Status::Ok
    }

    fn get_shift_decimal(&mut self, _vr: u32, _interval: &mut f64) -> Status {
        Status::Ok
    }

    fn set_shift_fraction(&mut self, _vr: u32, _counter: u64, _resolution: u64) -> Status {
        Status::Ok
    }

    fn get_shift_fraction(
        &mut self,
        _vr: u32,
        _counter: &mut u64,
        _resolution: &mut u64,
    ) -> Status {
        Status::Ok
    }
}

#[macro_export]
macro_rules! generate_fmi3_ffi {
    ($t: ty) => {
        use $crate::utils::*;
        use std::ffi::{c_char, c_void};
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};

        // -- TRAIT BOUND CHECK --
        const _: () = {
            const fn assert_impl<T: FMI3>() {}
            assert_impl::<$t>();
        };

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetVersion() -> *const c_char {
            c"3.0".as_ptr() as *const c_char
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateModelExchange(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: i32,
            logging_on: i32,
            instance_environment: *mut c_void,
            log_message: *const extern "C" fn(
                instance_environment: *mut c_void,
                status: i32,
                category: *const c_char,
                message: *const c_char,
            ) -> *mut c_void,
        ) -> *mut c_void {
            let Some(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(instantiation_token) = instantiation_token.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(resource_path) = resource_path.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(visible) = visible.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(instance) = <$t>::instantiate_model_exchange(
                instance_name,
                instantiation_token,
                resource_path,
                visible,
                logging_on,
                instance_environment,
                log_message,
            ) else {
                return std::ptr::null_mut();
            };
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateCoSimulation(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: i32,
            logging_on: i32,
            event_mode_used: i32,
            early_return_allowed: i32,
            required_intermediate_variables: *const u32,
            n_required_intermediate_variables: usize,
            instance_environment: *mut c_void,
            log_message: *const extern "C" fn(
                instance_environment: *mut c_void,
                status: i32,
                category: *const c_char,
                message: *const c_char,
            ) -> *mut c_void,
            intermediate_update: *const extern "C" fn(
                instance_enivronment: *mut c_void,
            ) -> *mut c_void,
        ) -> *mut c_void {
            let Some(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(instantiation_token) = instantiation_token.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(resource_path) = resource_path.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(visible) = visible.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(event_mode_used) = event_mode_used.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(early_return_allowed) = early_return_allowed.to_bool() else {
                return std::ptr::null_mut();
            };
            let rivs = unsafe {
                 from_raw_parts(
                     required_intermediate_variables,
                     n_required_intermediate_variables,
                 )
            };
            let Some(instance) = <$t>::instantiate_co_simulation(
                instance_name,
                instantiation_token,
                resource_path,
                visible,
                logging_on,
                event_mode_used,
                early_return_allowed,
                rivs,
                instance_environment,
                log_message,
                intermediate_update,
            ) else {
                return std::ptr::null_mut();
            };
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateScheduledExecution(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: i32,
            logging_on: i32,
            instance_environment: *mut c_void,
            log_message: *const extern "C" fn(
                instance_environment: *mut c_void,
                status: i32,
                category: *const c_char,
                message: *const c_char,
            ) -> *mut c_void,
            lock_preemption: *const extern "C" fn() -> *mut c_void,
            unlock_preemption: *const extern "C" fn() -> *mut c_void,
        ) -> *mut c_void {
            let Some(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(instantiation_token) = instantiation_token.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(resource_path) = resource_path.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(visible) = visible.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(instance) = <$t>::instantiate_scheduled_execution(
                instance_name,
                instantiation_token,
                resource_path,
                visible,
                logging_on,
                instance_environment,
                log_message,
                lock_preemption,
                unlock_preemption,
            ) else {
                return std::ptr::null_mut();
            };
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3FreeInstance(fmu: *mut $t) {
            if !fmu.is_null() {
                let _ = unsafe { Box::from_raw(fmu) };
            }
        }

        // -- LOGGING & DEBUG --
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetDebugLogging(
            fmu: *mut $t,
            logging_on: i32,
            n_categories: usize,
            categories: *const *const c_char,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return Status::Fatal;
            };
            let mut cats = Vec::with_capacity(n_categories);
            let cat_ptrs = unsafe { from_raw_parts(categories, n_categories) };
            for &p in cat_ptrs {
                match p.to_str() {
                    Some(s) => cats.push(s),
                    None => return Status::Fatal,
                }
            }
            fmu.set_debug_logging(logging_on, cats)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterConfigurationMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.enter_configuration_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3ExitConfigurationMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.exit_configuration_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterInitializationMode(
            fmu: *mut $t,
            tolerance_defined: i32,
            tolerance: f64,
            start_time: f64,
            stop_time_defined: i32,
            stop_time: f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let Some(tolerance_defined) = tolerance_defined.to_bool() else {
                return Status::Fatal;
            };
            let Some(stop_time_defined) = stop_time_defined.to_bool() else {
                return Status::Fatal;
            };
            fmu.enter_initialization_mode(
                tolerance_defined,
                tolerance,
                start_time,
                stop_time_defined,
                stop_time,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3ExitInitializationMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.exit_initialization_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterEventMode(
            fmu: *mut $t,
            step_event: i32,
            state_event: i32,
            roots_found: *const i32,
            n_event_indicators: usize,
            time_event: i32,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let Some(step_event) = step_event.to_bool() else {
                return Status::Fatal;
            };
            let Some(state_event) = state_event.to_bool() else {
                return Status::Fatal;
            };
            let Some(time_event) = time_event.to_bool() else {
                return Status::Fatal;
            };
            let roots = unsafe { from_raw_parts(roots_found, n_event_indicators) };
            fmu.enter_event_mode(step_event, state_event, roots, time_event)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3Terminate(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.terminate()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3Reset(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.reset()
        }

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident, $t_val:ty, $to_rust: expr) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvrs: usize,
                    values: *mut $t_val,
                    nvals: usize,
                ) -> Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Status::Fatal,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Status::Fatal
                    }
                    // TODO: Different to FMI2. Could these be different lengths?
                    let vrs = unsafe { from_raw_parts(vrs, nvrs) };
                    let values = unsafe { from_raw_parts_mut(values, nvals) };
                    for (vr, value) in zip(vrs, values) {
                        let status = fmu.$trait_get(*vr, value);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvrs: usize,
                    values: *const $t_val,
                    nvals: usize,
                ) -> Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Status::Fatal,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Status::Fatal
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvrs) };
                    let values = unsafe { from_raw_parts(values, nvals) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let rv = $to_rust(value);
                        let status = fmu.$trait_set(*vr, rv);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }
            };
        }


        generate_get_set!(
            fmi3GetFloat32,
            fmi3SetFloat32,
            get_float32,
            set_float32,
            f32,
            |v: *const f32| unsafe { *v }
        );
        generate_get_set!(
            fmi3GetFloat64,
            fmi3SetFloat64,
            get_float64,
            set_float64,
            f64,
            |v: *const f64| unsafe { *v }
        );

        generate_get_set!(fmi3GetInt8, fmi3SetInt8, get_int8, set_int8, i8, |v: *const i8| unsafe { *v });
        generate_get_set!(fmi3GetInt16, fmi3SetInt16, get_int16, set_int16, i16, |v: *const i16| unsafe { *v });
        generate_get_set!(fmi3GetInt32, fmi3SetInt32, get_int32, set_int32, i32, |v: *const i32| unsafe { *v });
        generate_get_set!(fmi3GetInt64, fmi3SetInt64, get_int64, set_int64, i64, |v: *const i64| unsafe { *v });
        generate_get_set!(fmi3GetUInt8, fmi3SetUInt8, get_uint8, set_uint8, u8, |v: *const u8| unsafe { *v });
        generate_get_set!(fmi3GetUInt16, fmi3SetUInt16, get_uint16, set_uint16, u16, |v: *const u16| unsafe { *v });
        generate_get_set!(fmi3GetUInt32, fmi3SetUInt32, get_uint32, set_uint32, u32, |v: *const u32| unsafe { *v });
        generate_get_set!(fmi3GetUInt64, fmi3SetUInt64, get_uint64, set_uint64, u64, |v: *const u64| unsafe { *v });


        generate_get_set!(
            fmi3GetBoolean,
            fmi3SetBoolean,
            get_boolean,
            set_boolean,
            i32,
            // OPTION: Put error when it is not 1 or 0?
            |v: *const i32| unsafe { *v != 0 }
        );
        generate_get_set!(
            fmi3GetString,
            fmi3SetString,
            get_string,
            set_string,
            *const c_char,
            |v: *const *const c_char| {
                let v = unsafe { *v };
                v.to_str().unwrap_or("")
            }
        );

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetBinary(
            fmu: *mut $t,
            vrs: *const u32,
            nvrs: usize,
            sizes: *mut usize,
            values: *mut *const u8,
            nvals: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vrs.is_null() || sizes.is_null() || values.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vrs, nvrs) };
            let sizes = unsafe { from_raw_parts_mut(sizes, nvals) };
            let values = unsafe { from_raw_parts_mut(values, nvals) };
            for ((vr, size), value) in zip(zip(vrs, sizes), values) {
                let status = fmu.get_binary(*vr, size, value);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetBinary(
            fmu: *mut $t,
            vrs: *const u32,
            nvrs: usize,
            sizes: *const usize,
            values: *const u8,
            nvals: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vrs.is_null() || sizes.is_null() || values.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vrs, nvrs) };
            let sizes = unsafe { from_raw_parts(sizes, nvals) };
            let values = unsafe { from_raw_parts(values, nvals) };
            for ((vr, size), value) in zip(zip(vrs, sizes), values) {
                let slice = unsafe { from_raw_parts(value, *size) };
                let status = fmu.set_binary(*vr, slice);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }


        // -- CLOCKS --
        generate_get_set!(
            fmi3GetClock,
            fmi3SetClock,
            get_clock,
            set_clock,
            i32,
            // OPTION: Put error when it is not 1 or 0?
            |v: *const i32| unsafe { *v != 0 }
        );

        macro_rules! generate_get_set_fmi2_format {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident, $t_val:ty,  $to_rust:expr) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvr: usize,
                    values: *mut $t_val,
                ) -> Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Status::Fatal,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Status::Fatal;
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts_mut(values, nvr) };
                    for (vr, value) in zip(vrs, values) {
                        let status = fmu.$trait_get(*vr, value);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvr: usize,
                    values: *const $t_val,
                ) -> Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Status::Fatal,
                    };
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts(values, nvr) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let rv  = $to_rust(value);
                        let status = fmu.$trait_set(*vr, rv);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }
            };
        }


        generate_get_set_fmi2_format!(
            fmi3GetIntervalDecimal,
            fmi3SetIntervalDecimal,
            get_interval_decimal,
            set_interval_decimal,
            f64,
            |v: *const f64| unsafe { *v }
        );

        generate_get_set_fmi2_format!(
            fmi3GetIntervalFraction,
            fmi3SetIntervalFraction,
            get_interval_fraction,
            set_interval_fraction,
            f64,
            |v: *const f64| unsafe { *v }
        );

        generate_get_set_fmi2_format!(
            fmi3GetShiftDecimal,
            fmi3SetShiftDecimal,
            get_shift_decimal,
            set_shift_decimal,
            f64,
            |v: *const f64| unsafe { *v }
        );


        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetShiftFraction(
            fmu: *mut $t,
            vrs: *const u32,
            n: usize,
            counters: *mut u64,
            resolutions: *mut u64
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vrs.is_null() || counters.is_null() || resolutions.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vrs, n) };
            let counters = unsafe { from_raw_parts_mut(counters, n) };
            let resolutions = unsafe { from_raw_parts_mut(resolutions, n) };
            for ((vr, counter), resolution) in zip(zip(vrs, counters), resolutions) {
                let status = fmu.get_shift_fraction(*vr, counter, resolution);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetShiftFraction(
            fmu: *mut $t,
            vrs: *const u32,
            n: usize,
            counters: *const u64,
            resolutions: *const u64
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vrs.is_null() || counters.is_null() || resolutions.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vrs, n) };
            let counters = unsafe { from_raw_parts(counters, n) };
            let resolutions = unsafe { from_raw_parts(resolutions, n) };
            for ((vr, counter), resolution) in zip(zip(vrs, counters), resolutions) {
                let status = fmu.set_shift_fraction(*vr, *counter, *resolution);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }



        // -- STATE MANAGEMENT --

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SerializedFMUstateSize(
            fmu: *mut $t,
            state: *mut c_void,
            size: *mut usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.serialized_fmu_state_size(state, size)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SerializeFMUstate(
            fmu: *mut $t,
            state: *mut c_void,
            serialized_state: *mut u8, // fmi2Byte is u8
            size: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if state.is_null() || serialized_state.is_null() {
                return Status::Fatal;
            }
            let buffer = unsafe { from_raw_parts_mut(serialized_state, size) };
            fmu.serialize_fmu_state(state, buffer)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3DeSerializeFMUstate(
            fmu: *mut $t,
            serialized_state: *const u8, // fmi2Byte is u8
            size: usize,
            state: *mut *mut c_void,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if state.is_null() || serialized_state.is_null() {
                return Status::Fatal;
            }
            let buffer = unsafe { std::slice::from_raw_parts(serialized_state, size) };
            fmu.deserialized_fmu_state(buffer, size, state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetFMUstate(
            fmu: *mut $t,
            state: *mut *mut c_void,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            fmu.get_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetFMUstate(fmu: *mut $t, state: *mut c_void) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            fmu.set_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3FreeFMUstate(
            fmu: *mut $t,
            state: *mut c_void,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            fmu.free_fmu_state(state)
        }


        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetDirectionalDerivative(
            fmu: *mut $t,
            v_unknown_ptr: *const u32,
            n_unknown: usize,
            v_known_ptr: *const u32,
            n_known: usize,
            dv_known_ptr: *const f64,
            n_dv_known: usize,
            dv_unknown_mut_ptr: *mut f64,
            n_dv_unknown: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if v_unknown_ptr.is_null()
                || v_known_ptr.is_null()
                || dv_known_ptr.is_null()
                || dv_unknown_mut_ptr.is_null()
            {
                return Status::Fatal;
            }
            let v_unknown = unsafe { from_raw_parts(v_unknown_ptr, n_unknown) };
            let dv_unknown = unsafe { from_raw_parts_mut(dv_unknown_mut_ptr, n_dv_unknown) };
            let v_known = unsafe { from_raw_parts(v_known_ptr, n_known) };
            let dv_known = unsafe { from_raw_parts(dv_known_ptr, n_dv_known) };

            fmu.get_directional_derivative(v_known, v_unknown, dv_known, dv_unknown)
        }



        // -- CO-SIMULATION / STEPPING --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3DoStep(
            fmu: *mut $t,
            current_communication_point: f64,
            communication_step_size: f64,
            no_set_fmu_state_prior: i32,
            event_encountered: *mut i32,
            terminate: *mut i32,
            early_return: *mut i32,
            last_successful_time: *mut f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if event_encountered.is_null() || terminate.is_null() || early_return.is_null() {
                return Status::Fatal;
            }
            let Some(no_set_fmu_state_prior) = no_set_fmu_state_prior.to_bool() else {
                return Status::Fatal;
            };

            let mut ev = false;
            let mut term = false;
            let mut er = false;

            let status = fmu.do_step(
                current_communication_point,
                communication_step_size,
                no_set_fmu_state_prior,
                &mut ev,
                &mut term,
                &mut er,
                unsafe { &mut *last_successful_time },
            );

            match (ev) {
                true => unsafe { *event_encountered = 1 },
                false => unsafe { *event_encountered = 0}
            }
            match (term) {
                true => unsafe { *terminate = 1 },
                false => unsafe { *terminate = 0 }
            }
            match (er) {
                true => unsafe { *early_return = 1 },
                false => unsafe { *early_return = 0 }
            }
            status
        }


        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterStepMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.enter_step_mode()
        }

        // -- MODEL EXCHANGE SPECIFIC --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetTime(fmu: *mut $t, time: f64) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.set_time(time)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetContinuousStates(
            fmu: *mut $t,
            x: *const f64,
            nx: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if x.is_null() {
                return Status::Fatal;
            }
            let x = unsafe { from_raw_parts(x, nx) };
            fmu.set_continuous_states(x)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetDerivatives(
            fmu: *mut $t,
            dx: *mut f64,
            nx: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if dx.is_null() {
                return Status::Fatal;
            }
            let dx = unsafe { from_raw_parts_mut(dx, nx) };
            fmu.get_derivatives(dx)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetEventIndicators(
            fmu: *mut $t,
            ei: *mut f64,
            ni: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if ei.is_null() {
                return Status::Fatal;
            }
            let ei = unsafe { from_raw_parts_mut(ei, ni) };
            fmu.get_event_indicators(ei)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetContinuousStates(
            fmu: *mut $t,
            x: *mut f64,
            nx: usize,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if x.is_null() {
                return Status::Fatal;
            }
            let x = unsafe { from_raw_parts_mut(x, nx) };
            fmu.get_continuous_states(x)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3NewDiscreteStates(
            fmu: *mut $t,
            discrete_states_need_update: *mut i32,
            terminate_simulation: *mut i32,
            nominals_of_continuous_states_changed: *mut i32,
            values_of_continuous_states_changed: *mut i32,
            next_event_time_defined: *mut i32,
            next_event_time: *mut f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if discrete_states_need_update.is_null() ||
                terminate_simulation.is_null() ||
                nominals_of_continuous_states_changed.is_null() ||
                values_of_continuous_states_changed.is_null() ||
                next_event_time_defined.is_null() ||
                next_event_time.is_null() {
                return Status::Fatal;
            }
            let mut dsnu = false;
            let mut ts = false;
            let mut nocc = false;
            let mut vocc = false;
            let mut netd = false;
            let status = fmu.new_discrete_states(
                &mut dsnu,
                &mut ts,
                &mut nocc,
                &mut vocc,
                &mut netd,
                unsafe { &mut *next_event_time },
            );
            match (dsnu) {
                true => unsafe { *discrete_states_need_update = 1 },
                false => unsafe { *discrete_states_need_update = 0 }
            }
            match (ts) {
                true => unsafe { *terminate_simulation = 1 },
                false => unsafe { *terminate_simulation = 0 }
            }
            match nocc {
                true => unsafe { *nominals_of_continuous_states_changed = 1 },
                false => unsafe { *nominals_of_continuous_states_changed = 0 }
            }
            match vocc {
                true => unsafe { *values_of_continuous_states_changed = 1 },
                false => unsafe { *values_of_continuous_states_changed = 0 }
            }
            match netd {
                true => unsafe { *next_event_time_defined = 1 },
                false => unsafe { *next_event_time_defined = 0 }
            }
            status
        }

    };
}

#[cfg(test)]
mod cargo_check {
    // Usesd to get type checking on the macro.
    use crate::fmi3::*;
    #[derive(Default)]
    pub struct Model {
        _n: f64,
    }
    impl FMI3 for Model {
        fn instantiate_model_exchange<'a>(
            _instance_name: &'a str,
            _instantiation_token: &'a str,
            _resource_path: &'a str,
            _visible: bool,
            _logging_on: bool,
            _instance_environment: *mut std::ffi::c_void,
            _log_message: *const extern "C" fn(
                instance_environment: *mut std::ffi::c_void,
                status: i32,
                category: *const std::ffi::c_char,
                message: *const std::ffi::c_char,
            ) -> *mut std::ffi::c_void,
        ) -> Option<Model> {
            Some(Self::default())
        }
    }
    generate_fmi3_ffi!(Model);
}
