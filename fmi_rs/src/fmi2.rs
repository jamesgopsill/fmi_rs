use std::ffi::{CStr, c_char, c_void};

pub use fmi_rs_macros::Fmi2Ffi;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Fmi2Status(i32);

impl Fmi2Status {
    pub const OK: Self = Self(0);
    pub const WARNING: Self = Self(1);
    pub const DISCARD: Self = Self(2);
    pub const ERROR: Self = Self(3);
    pub const FATAL: Self = Self(4);
    pub const PENDING: Self = Self(5);
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Fmi2Type(i32);

impl Fmi2Type {
    pub const MODEL_EXCHANGE: Self = Self(0);
    pub const CO_SIMULATION: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct Fmi2StatusType(i32);

impl Fmi2StatusType {
    pub const DO_STEP_STATUS: Self = Self(0);
    pub const PENDING_STATUS: Self = Self(1);
    pub const LAST_SUCCESSFUL_TIME: Self = Self(2);
    pub const TERMINATED: Self = Self(3);
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Fmi2Bool(i32);

impl Fmi2Bool {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Fmi2Str(*const c_char);

impl Fmi2Str {
    pub fn to_str(&self) -> Result<&CStr, Fmi2Status> {
        if self.0.is_null() {
            return Err(Fmi2Status::FATAL);
        }
        Ok(unsafe { CStr::from_ptr(self.0) })
    }
}

#[repr(C)]
pub struct Fmi2CallbackFunctions {
    /// Pointer to the logger function for reporting messages to the master.
    pub logger: extern "C" fn(
        component_environment: *mut c_void,
        instance_name: Fmi2Str,
        status: Fmi2Status,
        category: Fmi2Str,
        message: Fmi2Str,
        ...
    ),
    /// Function to allocate memory (use this instead of `malloc` for FMI compliance).
    pub allocate_memory: extern "C" fn(n_obj: usize, size: usize) -> *mut c_void,
    /// Function to free memory.
    pub free_memory: extern "C" fn(obj: *mut c_void),
    /// Callback for asynchronous step completion.
    pub step_finished: extern "C" fn(component_environment: *mut c_void, status: Fmi2Status),
    /// Pointer to the master's private environment data.
    pub component_environment: *mut c_void,
}

/// Structure containing information about events (Model Exchange).
#[repr(C)]
pub struct EventInfo {
    pub new_discrete_states_needed: Fmi2Bool,
    pub terminate_simulation: Fmi2Bool,
    pub nominals_of_continuous_states_changed: Fmi2Bool,
    pub values_of_continuous_states_changed: Fmi2Bool,
    pub next_event_time_defined: Fmi2Bool,
    pub next_event_time: f64,
}

/// Implementing the trait enables the struct to conform
/// to the FMI2 spec and have the FFI automatically generated
/// by the accompanying macro `generate_fmi2_ffi`.
pub trait Fmi2: Sized {
    /// Informs the FMU to allocate a new instance of the model.
    fn instantiate(
        instance_name: Fmi2Str,
        fmu_type: Fmi2Type,
        guid: Fmi2Str,
        resource_location: Fmi2Str,
        functions: &Fmi2CallbackFunctions,
        visible: Fmi2Bool,
        logging_on: Fmi2Bool,
    ) -> Self;

    /// Perform the respective actions for the FMU
    /// during a simulation step.
    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: Fmi2Bool,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// The simulation requests the f64 associated with
    /// the value reference declared in the .xml file.
    fn get_real(&mut self, _vr: u32, _value: &mut f64) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// The simulation requests the f64 associated with
    /// the value reference declared in the .xml file.
    fn get_integer(&mut self, _vr: u32, _value: &mut i32) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// The simulation requests the bool (i32) associated with
    /// the value reference declared in the .xml file.
    fn get_boolean(&mut self, _vr: u32, _value: &mut Fmi2Bool) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// The simulation requests the ptr to the string associated with
    /// the value reference declared in the .xml file.
    fn get_string(&mut self, _vr: u32, _value: &mut Fmi2Str) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// Provides details of the simulation times and tolerances
    /// to the FMU.
    fn setup_experiment(
        &mut self,
        _tolerance_defined: Fmi2Bool,
        _tolerance: f64,
        _start_time: f64,
        _stop_time_defined: Fmi2Bool,
        _stop_time: f64,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Informs the FMU that it is entering the Initialization Mode.
    fn enter_initialization_mode(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Informs the FMU that it is exiting the Initialization Mode.
    fn exit_initialization_mode(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Terminates the simulation.
    fn terminate(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Resets the FMU to the state immediately after instantiation.
    fn reset(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Configure the FMU's debug logging settings.
    fn set_debug_logging(&mut self, _logging_on: Fmi2Bool, _categories: &[Fmi2Str]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Sets a given f64 value reference in the FMU.
    fn set_real(&mut self, _vr: u32, _value: f64) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// Sets a given i32 value reference in the FMU.
    fn set_integer(&mut self, _vr: u32, _value: i32) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// Sets a given bool value reference in the FMU.
    fn set_boolean(&mut self, _vr: u32, _value: Fmi2Bool) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// Sets a given string value reference in the FMU. The FMU should
    /// take a copy.
    fn set_string(&mut self, _vr: u32, _value: Fmi2Str) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Queries status of asynchronous operations.
    fn get_status(&mut self, _status_kind: Fmi2StatusType, _value: &mut Fmi2Status) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Queries status of a specific f64 value.
    fn get_real_status(&mut self, _status_type: Fmi2StatusType, _value: &mut f64) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Queries status of a specific Integer value.
    fn get_integer_status(&mut self, _status_type: Fmi2StatusType, _value: &mut i32) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Queries status of a specific Boolean value.
    fn get_boolean_status(
        &mut self,
        _status_type: Fmi2StatusType,
        _value: &mut Fmi2Bool,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Queries status of a specific String value.
    fn get_string_status(
        &mut self,
        _status_type: Fmi2StatusType,
        _value: &mut Fmi2Str,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Co-Simulation only] Signals the slave to stop the current `do_step` computation.
    fn cancel_step(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Sets the n-th derivative of a real input.
    fn set_real_input_derivative(&self, _vr: u32, _order: i32, _value: f64) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Gets the n-th derivative of a real output.
    fn get_real_output_derivative(
        &mut self,
        _vr: u32,
        _order: &i32,
        _value: &mut f64,
    ) -> Fmi2Status {
        Fmi2Status::ERROR
    }

    /// Returns the required buffer size for the serialized state.
    fn serialized_fmu_state_size(&mut self, _state: &mut c_void, _size: &mut usize) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Serializes the FMU state into a byte buffer.
    fn serialize_fmu_state(&mut self, _state: &mut c_void, _serialized_state: &[u8]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Deserializes the FMU state from a byte buffer.
    fn deserialized_fmu_state(
        &mut self,
        _buffer: &[u8],
        _size: usize,
        _state: &mut *mut c_void,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Sets the internal state of the FMU.
    fn set_fmu_state(&mut self, _state: &mut c_void) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Captures the internal state of the FMU.
    fn get_fmu_state(&mut self, _state: &mut *mut c_void) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Frees a previously captured FMU state.
    fn free_fmu_state(&mut self, _state: &mut c_void) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// Computes partial derivatives (directional derivatives).
    fn get_directional_derivative(
        &mut self,
        _v_known: &[u32],
        _v_unknown: &[u32],
        _dv_known: &[f64],
        _dv_unknown: &mut [f64],
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Enters Event Mode.
    fn enter_event_mode(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Computes new discrete states.
    fn new_discrete_states(&mut self, _info: &mut EventInfo) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Enters Continuous-Time Mode.
    fn enter_continuous_time_mode(&mut self) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Notifies the FMU that the integrator step is complete.
    fn completed_integrator_step(
        &mut self,
        _no_prior: i32,
        _enter_event: &mut i32,
        _term: &mut i32,
    ) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Sets a new time point.
    fn set_time(&mut self, _time: f64) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Sets new continuous state values.
    fn set_continuous_states(&mut self, _x: &[f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    fn set_derivatives(&mut self, _dx: &[f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    fn set_event_indicators(&mut self, _ei: &[f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    fn set_nominals_of_continuous_states(&mut self, _x: &[f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Retrieves the state derivatives.
    fn get_derivatives(&mut self, _dx: &mut [f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Retrieves the event indicators (zero-crossing functions).
    fn get_event_indicators(&mut self, _ei: &mut [f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Retrieves current continuous states.
    fn get_continuous_states(&mut self, _x: &mut [f64]) -> Fmi2Status {
        Fmi2Status::OK
    }

    /// [Model Exchange only] Retrieves nominal values of continuous states for scaling.
    fn get_nominals_of_continuous_states(&mut self, _x: &mut [f64]) -> Fmi2Status {
        Fmi2Status::OK
    }
}

#[macro_export]
macro_rules! generate_fmi2_ffi {
    ($t: ty) => {
        use std::ffi::{c_char, c_void};
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};
        // use $crate::fmi2::*;

        // -- TRAIT BOUND CHECK --
        const _: () = {
            const fn assert_impl<T: Fmi2>() {}
            assert_impl::<$t>();
        };

        // -- THE MACRO --
        /// # Safety
        /// I have manually checked I do not define these names anywhere else and
        /// the macros is called once (could there be a compile time check?)
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetTypesPlatform() -> *const c_char {
            c"default".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetVersion() -> *const c_char {
            c"2.0".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Instantiate(
            instance_name: Fmi2Str,
            fmu_type: Fmi2Type,
            guid: Fmi2Str,
            resource_location: Fmi2Str,
            functions: *const Fmi2CallbackFunctions,
            visible: Fmi2Bool,
            logging_on: Fmi2Bool,
        ) -> *mut c_void {
            let functions = match unsafe { functions.as_ref() } {
                Some(f) => f,
                None => return std::ptr::null_mut(),
            };
            let instance = <$t>::instantiate(
                instance_name,
                fmu_type,
                guid,
                resource_location,
                functions,
                visible,
                logging_on,
            );
            let instance = Box::new(instance);
            Box::into_raw(instance) as *mut c_void
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2DoStep(
            fmu: *mut $t,
            current_communication_point: f64,
            communication_step_size: f64,
            no_set_fmu_state_prior_to_current_point: Fmi2Bool,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            fmu.do_step(
                current_communication_point,
                communication_step_size,
                no_set_fmu_state_prior_to_current_point,
            )
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2FreeInstance(fmu: *mut $t) {
            if !fmu.is_null() {
                let _ = unsafe { Box::from_raw(fmu) };
            }
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetupExperiment(
            fmu: *mut $t,
            tolerance_defined: Fmi2Bool,
            tolerance: f64,
            start_time: f64,
            stop_time_defined: Fmi2Bool,
            stop_time: f64,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            fmu.setup_experiment(
                tolerance_defined,
                tolerance,
                start_time,
                stop_time_defined,
                stop_time,
            )
        }

        macro_rules! generate_no_arg_fcn {
            ($get_fn:ident, $trait_fn:ident) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(fmu: *mut $t) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    fmu.$trait_fn()
                }
            };
        }

        generate_no_arg_fcn!(fmi2EnterInitializationMode, enter_initialization_mode);
        generate_no_arg_fcn!(fmi2ExitInitializationMode, exit_initialization_mode);
        generate_no_arg_fcn!(fmi2Terminate, terminate);
        generate_no_arg_fcn!(fmi2Reset, reset);
        generate_no_arg_fcn!(fmi2CancelStep, cancel_step);
        generate_no_arg_fcn!(fmi2EnterEventMode, enter_event_mode);
        generate_no_arg_fcn!(fmi2EnterContinuousTimeMode, enter_continuous_time_mode);

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetDebugLogging(
            fmu: *mut $t,
            logging_on: Fmi2Bool,
            n_categories: usize,
            categories: *const Fmi2Str,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            if categories.is_null() {
                return Fmi2Status::FATAL;
            }
            let categories = unsafe { from_raw_parts(categories, n_categories) };
            fmu.set_debug_logging(logging_on, categories)
        }

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident, $t_val:ty) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvr: usize,
                    values: *mut $t_val,
                ) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Fmi2Status::FATAL;
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts_mut(values, nvr) };
                    for (vr, value) in zip(vrs, values) {
                        let status = fmu.$trait_get(*vr, value);
                        if status != Fmi2Status::OK {
                            return status;
                        }
                    }
                    Fmi2Status::OK
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    fmu: *mut $t,
                    vrs: *const u32,
                    nvr: usize,
                    values: *const $t_val,
                ) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts(values, nvr) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let status = fmu.$trait_set(*vr, *value);
                        if status != Fmi2Status::OK {
                            return status;
                        }
                    }
                    Fmi2Status::OK
                }
            };
        }

        generate_get_set!(
            fmi2GetInteger,
            fmi2SetInteger,
            get_integer,
            set_integer,
            i32
        );

        generate_get_set!(fmi2GetReal, fmi2SetReal, get_real, set_real, f64);

        generate_get_set!(
            fmi2GetBoolean,
            fmi2SetBoolean,
            get_boolean,
            set_boolean,
            Fmi2Bool
        );

        generate_get_set!(
            fmi2GetString,
            fmi2SetString,
            get_string,
            set_string,
            Fmi2Str
        );

        macro_rules! generate_get_status_fcn {
            ($get_fn:ident, $trait_fn:ident, $type:ty) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    status_type: Fmi2StatusType,
                    value: *mut $type,
                ) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    let value = match unsafe { value.as_mut() } {
                        Some(v) => v,
                        None => return Fmi2Status::FATAL,
                    };
                    fmu.$trait_fn(status_type, value)
                }
            };
        }

        generate_get_status_fcn!(fmi2GetStatus, get_status, Fmi2Status);
        generate_get_status_fcn!(fmi2GetRealStatus, get_real_status, f64);
        generate_get_status_fcn!(fmi2GetIntegerStatus, get_integer_status, i32);
        generate_get_status_fcn!(fmi2GetBooleanStatus, get_boolean_status, Fmi2Bool);
        generate_get_status_fcn!(fmi2GetStringStatus, get_string_status, Fmi2Str);

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetRealInputDerivatives(
            fmu: *mut $t,
            vr: *const u32,
            nvr: usize,
            order: *const i32,
            value: *const f64,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Fmi2Status::FATAL;
            }
            let vrs = unsafe { std::slice::from_raw_parts(vr, nvr) };
            let orders = unsafe { std::slice::from_raw_parts(order, nvr) };
            let values = unsafe { std::slice::from_raw_parts(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.set_real_input_derivative(vrs[i], orders[i], values[i]);
                if status != Fmi2Status::OK {
                    return status;
                }
            }
            Fmi2Status::OK
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetRealOutputDerivatives(
            fmu: *mut $t,
            vr: *const u32,
            nvr: usize,
            order: *const i32,
            value: *mut f64,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Fmi2Status::FATAL;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts_mut(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.get_real_output_derivative(vrs[i], &orders[i], &mut values[i]);
                if status != Fmi2Status::OK {
                    return status;
                }
            }
            Fmi2Status::OK
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SerializedFMUstateSize(
            fmu: *mut $t,
            state: *mut c_void,
            size: *mut usize,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            let size = match unsafe { size.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            fmu.serialized_fmu_state_size(state, size)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SerializeFMUstate(
            fmu: *mut $t,
            state: *mut c_void,
            serialized_state: *mut u8,
            size: usize,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            let buffer = unsafe { from_raw_parts_mut(serialized_state, size) };
            fmu.serialize_fmu_state(state, buffer)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2DeSerializeFMUstate(
            fmu: *mut $t,
            serialized_state: *const u8,
            size: usize,
            state: *mut *mut c_void,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            let buffer = unsafe { from_raw_parts(serialized_state, size) };
            fmu.deserialized_fmu_state(buffer, size, state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetFMUstate(
            fmu: *mut $t,
            state: *mut *mut c_void,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            fmu.get_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetFMUstate(fmu: *mut $t, state: *mut c_void) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            fmu.set_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2FreeFMUstate(fmu: *mut $t, state: *mut c_void) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let state = match unsafe { state.as_mut() } {
                Some(s) => s,
                None => return Fmi2Status::FATAL,
            };
            fmu.free_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetDirectionalDerivative(
            fmu: *mut $t,
            v_unknown_ptr: *const u32,
            n_unknown: usize,
            v_known_ptr: *const u32,
            n_known: usize,
            dv_known_ptr: *const f64,
            dv_unknown_mut_ptr: *mut f64,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            if v_unknown_ptr.is_null()
                || v_known_ptr.is_null()
                || dv_known_ptr.is_null()
                || dv_unknown_mut_ptr.is_null()
            {
                return Fmi2Status::FATAL;
            }
            let v_unknown = unsafe { from_raw_parts(v_unknown_ptr, n_unknown) };
            let dv_unknown = unsafe { from_raw_parts_mut(dv_unknown_mut_ptr, n_unknown) };
            let v_known = unsafe { from_raw_parts(v_known_ptr, n_known) };
            let dv_known = unsafe { from_raw_parts(dv_known_ptr, n_known) };
            fmu.get_directional_derivative(v_known, v_unknown, dv_known, dv_unknown)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2NewDiscreteStates(
            fmu: *mut $t,
            info: *mut EventInfo,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let info = match unsafe { info.as_mut() } {
                Some(i) => i,
                None => return Fmi2Status::FATAL,
            };
            fmu.new_discrete_states(info)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2CompletedIntegratorStep(
            fmu: *mut $t,
            no_prior: i32,
            enter_event: *mut i32,
            term: *mut i32,
        ) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            let ee = match unsafe { enter_event.as_mut() } {
                Some(e) => e,
                None => return Fmi2Status::FATAL,
            };
            let t = match unsafe { term.as_mut() } {
                Some(t) => t,
                None => return Fmi2Status::FATAL,
            };
            fmu.completed_integrator_step(no_prior, ee, t)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetTime(fmu: *mut $t, time: f64) -> Fmi2Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi2Status::FATAL,
            };
            fmu.set_time(time)
        }

        macro_rules! generate_slice_fcns {
            ($ffi_get:ident, $trait_get:ident, $ffi_set: ident, $trait_set: ident, $type:ty) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $ffi_get(
                    fmu: *mut $t,
                    x: *mut $type,
                    nx: usize,
                ) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    if x.is_null() {
                        return Fmi2Status::FATAL;
                    }
                    let x = unsafe { from_raw_parts_mut(x, nx) };
                    fmu.$trait_get(x)
                }
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $ffi_set(
                    fmu: *mut $t,
                    x: *const $type,
                    nx: usize,
                ) -> Fmi2Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi2Status::FATAL,
                    };
                    if x.is_null() {
                        return Fmi2Status::FATAL;
                    }
                    let x = unsafe { from_raw_parts(x, nx) };
                    fmu.$trait_set(x)
                }
            };
        }

        generate_slice_fcns!(
            fmi2GetContinuousStates,
            get_continuous_states,
            fmi2SetContinuousStates,
            set_continuous_states,
            f64
        );

        generate_slice_fcns!(
            fmi2GetDerivatives,
            get_derivatives,
            fmi2SetDerivatives,
            set_derivatives,
            f64
        );

        generate_slice_fcns!(
            fmi2GetEventIndicators,
            get_event_indicators,
            fmi2SetEventIndicators,
            set_event_indicators,
            f64
        );

        generate_slice_fcns!(
            fmi2GetNominalsOfContinuousStates,
            get_nominals_of_continuous_states,
            fmi2SetNominalsOfContinuousStates,
            set_nominals_of_continuous_states,
            f64
        );
    };
}

#[cfg(test)]
mod cargo_check {
    use super::*;
    // Usesd to get type checking on the macro.
    #[derive(Default)]
    pub struct Fmu {
        count: f64,
    }
    impl Fmi2 for Fmu {
        fn instantiate(
            _instance_name: Fmi2Str,
            _fmu_type: Fmi2Type,
            _guid: Fmi2Str,
            _resource_location: Fmi2Str,
            _functions: &Fmi2CallbackFunctions,
            _visible: Fmi2Bool,
            _logging_on: Fmi2Bool,
        ) -> Self {
            Self::default()
        }

        fn do_step(
            &mut self,
            _current_communication_point: f64,
            communication_step_size: f64,
            _no_set_fmu_state_prior_to_current_point: Fmi2Bool,
        ) -> Fmi2Status {
            self.count += communication_step_size;
            Fmi2Status::OK
        }

        fn get_real(&mut self, vr: u32, value: &mut f64) -> Fmi2Status {
            match vr {
                0 => *value = self.count,
                _ => return Fmi2Status::ERROR,
            }
            Fmi2Status::OK
        }
    }
    generate_fmi2_ffi!(Fmu);
}
