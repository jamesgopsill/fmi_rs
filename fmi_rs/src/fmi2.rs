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
    /// The result is not yet available (used in asynchronous Co-Simulation).
    Pending = 5,
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
            5 => Ok(Self::Pending),
            _ => Err(()),
        }
    }
}

/// Defines whether the FMU is being used for
/// Model Exchange or Co-Simulation.
#[repr(i32)]
pub enum Kind {
    ModelExchange = 0,
    CoSimulation = 1,
}

impl TryFrom<i32> for Kind {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ModelExchange),
            1 => Ok(Self::CoSimulation),
            _ => Err(Status::Fatal),
        }
    }
}

/// Defines the kind of status being
/// queried in `get_status` methods.
#[repr(i32)]
pub enum StatusKind {
    /// Query the status of a `do_step` call.
    DoStepStatus = 0,
    /// Query the status of a pending asynchronous operation.
    PendingStatus = 1,
    /// Query the last successful simulation time.
    LastSuccessfulTime = 2,
    /// Query if the FMU was terminated by the environment.
    Terminated = 3,
}

impl TryFrom<i32> for StatusKind {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DoStepStatus),
            1 => Ok(Self::PendingStatus),
            2 => Ok(Self::LastSuccessfulTime),
            3 => Ok(Self::Terminated),
            _ => Err(Status::Error),
        }
    }
}

/// Implementing the trait enables the struct to conform
/// to the FMI2 spec and have the FFI automatically generated
/// by the accompanying macro `generate_fmi2_ffi`.
pub trait FMI2: Sized {
    /// Informs the FMU to allocate a new instance of the model.
    fn instantiate(
        instance_name: &str,
        fmu_type: Kind,
        guid: &str,
        resource_location: &str,
        functions: *const CallbackFunctions,
        visible: bool,
        logging_on: bool,
    ) -> Self;

    /// Perform the respective actions for the FMU
    /// during a simulation step.
    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Status {
        Status::Ok
    }

    /// The simulation requests the f64 associated with
    /// the value reference declared in the .xml file.
    fn get_real(&mut self, _vr: u32, _value: &mut f64) -> Status {
        Status::Error
    }

    /// The simulation requests the f64 associated with
    /// the value reference declared in the .xml file.
    fn get_integer(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    /// The simulation requests the bool (i32) associated with
    /// the value reference declared in the .xml file.
    fn get_boolean(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    /// The simulation requests the ptr to the string associated with
    /// the value reference declared in the .xml file.
    fn get_string(&mut self, _vr: u32, _value: &mut *const c_char) -> Status {
        Status::Error
    }

    /// Provides details of the simulation times and tolerances
    /// to the FMU.
    fn setup_experiment(
        &mut self,
        _tolerance_defined: bool,
        _tolerance: f64,
        _start_time: f64,
        _stop_time_defined: bool,
        _stop_time: f64,
    ) -> Status {
        Status::Ok
    }

    /// Informs the FMU that it is entering the Initialization Mode.
    fn enter_initialization_mode(&mut self) -> Status {
        Status::Ok
    }

    /// Informs the FMU that it is exiting the Initialization Mode.
    fn exit_initialization_mode(&mut self) -> Status {
        Status::Ok
    }

    /// Terminates the simulation.
    fn terminate(&mut self) -> Status {
        Status::Ok
    }

    /// Resets the FMU to the state immediately after instantiation.
    fn reset(&mut self) -> Status {
        Status::Ok
    }

    /// Configure the FMU's debug logging settings.
    fn set_debug_logging(&mut self, _logging_on: bool, _categories: Vec<String>) -> Status {
        Status::Ok
    }

    /// Sets a given f64 value reference in the FMU.
    fn set_real(&mut self, _vr: u32, _value: f64) -> Status {
        Status::Error
    }

    /// Sets a given i32 value reference in the FMU.
    fn set_integer(&mut self, _vr: u32, _value: i32) -> Status {
        Status::Error
    }

    /// Sets a given bool value reference in the FMU.
    fn set_boolean(&mut self, _vr: u32, _value: bool) -> Status {
        Status::Error
    }

    /// Sets a given string value reference in the FMU. The FMU should
    /// take a copy.
    fn set_string(&mut self, _vr: u32, _value: &str) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of asynchronous operations.
    fn get_status(&mut self, _status_kind: StatusKind, _value: &mut Status) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific f64 value.
    fn get_real_status(&mut self, _status_kind: StatusKind, _value: &mut f64) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific Integer value.
    fn get_integer_status(&mut self, _status_kind: StatusKind, _value: &mut i32) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific Boolean value.
    fn get_boolean_status(&mut self, _status_kind: StatusKind, _value: &mut bool) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific String value.
    fn get_string_status(
        &mut self,
        _status_kind: StatusKind,
        _value: *mut *const c_char,
    ) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Signals the slave to stop the current `do_step` computation.
    fn cancel_step(&mut self) -> Status {
        Status::Ok
    }

    /// Sets the n-th derivative of a real input.
    fn set_real_input_derivative(&self, _vr: u32, _order: i32, _value: f64) -> Status {
        Status::Ok
    }

    /// Gets the n-th derivative of a real output.
    fn get_real_output_derivative(&mut self, _vr: u32, _order: &i32, _value: &mut f64) -> Status {
        Status::Error
    }

    /// Returns the required buffer size for the serialized state.
    fn serialized_fmu_state_size(
        &mut self,
        _state: *mut std::ffi::c_void,
        _size: *mut usize,
    ) -> Status {
        Status::Ok
    }

    /// Serializes the FMU state into a byte buffer.
    fn serialize_fmu_state(
        &mut self,
        _state: *mut std::ffi::c_void,
        _serialized_state: &[u8],
    ) -> Status {
        Status::Ok
    }

    /// Deserializes the FMU state from a byte buffer.
    fn deserialized_fmu_state(
        &mut self,
        _buffer: &[u8],
        _size: usize,
        _state: *mut *mut std::ffi::c_void,
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

    /// [Model Exchange only] Enters Event Mode.
    fn enter_event_mode(&mut self) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Computes new discrete states.
    fn new_discrete_states(&mut self, _info: *mut EventInfo) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Enters Continuous-Time Mode.
    fn enter_continuous_time_mode(&mut self) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Notifies the FMU that the integrator step is complete.
    fn completed_integrator_step(
        &mut self,
        _no_prior: i32,
        _enter_event: *mut i32,
        _term: *mut i32,
    ) -> Status {
        Status::Ok
    }

    /// [Model Exchange only] Sets a new time point.
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

    /// [Model Exchange only] Retrieves nominal values of continuous states for scaling.
    fn get_nominals_of_continuous_states(&mut self, _x: &mut [f64]) -> Status {
        Status::Ok
    }
}

#[repr(C)]
pub struct CallbackFunctions {
    /// Pointer to the logger function for reporting messages to the master.
    pub logger: extern "C" fn(
        component_environment: *mut c_void,
        instance_name: *const c_char,
        status: i32,
        category: *const c_char,
        message: *const c_char,
        ...
    ),
    /// Function to allocate memory (use this instead of `malloc` for FMI compliance).
    pub allocate_memory: extern "C" fn(n_obj: usize, size: usize) -> *mut c_void,
    /// Function to free memory.
    pub free_memory: extern "C" fn(obj: *mut c_void),
    /// Callback for asynchronous step completion.
    pub step_finished: extern "C" fn(component_environment: *mut c_void, status: i32),
    /// Pointer to the master's private environment data.
    pub component_environment: *mut c_void,
}

/// Structure containing information about events (Model Exchange).
#[repr(C)]
pub struct EventInfo {
    pub new_discrete_states_needed: i32,
    pub terminate_simulation: i32,
    pub nominals_of_continuous_states_changed: i32,
    pub values_of_continuous_states_changed: i32,
    pub next_event_time_defined: i32,
    pub next_event_time: f64,
}

#[macro_export]
macro_rules! generate_fmi2_ffi {
    ($t: ty) => {
        use $crate::utils::*;
        use $crate::fmi2::*;
        use std::ffi::*;
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};

        // -- TRAIT BOUND CHECK --
        const _: () = {
            const fn assert_impl<T: $crate::fmi2::FMI2>() {}
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
            instance_name: *const c_char,
            fmu_type: c_int,
            guid: *const c_char,
            resource_location: *const c_char,
            functions: *const CallbackFunctions,
            visible: i32,
            logging_on: i32,
        ) -> *mut c_void {
            if functions.is_null() {
                return std::ptr::null_mut();
            }
            let Some(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(guid) = guid.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(resource_location) = resource_location.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(fmu_type) = Kind::try_from(fmu_type) else {
                return std::ptr::null_mut();
            };
            let Some(visible) = visible.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return std::ptr::null_mut();
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
            current_communication_point: c_double,
            communication_step_size: c_double,
            no_set_fmu_state_prior_to_current_point: c_int,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let Some(n) = no_set_fmu_state_prior_to_current_point.to_bool() else {
                return Status::Fatal;
            };
            fmu.do_step(current_communication_point, communication_step_size, n)
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
            fmu.setup_experiment(
                tolerance_defined,
                tolerance,
                start_time,
                stop_time_defined,
                stop_time,
            )
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterInitializationMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.enter_initialization_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2ExitInitializationMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.exit_initialization_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Terminate(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.terminate()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Reset(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.reset()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetDebugLogging(
            fmu: *mut $t,
            logging_on: i32,
            n_categories: usize,
            categories: *const *const c_char,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if categories.is_null() {
                return Status::Fatal;
            }
            let categories = unsafe { from_raw_parts(categories, n_categories) };
            let mut cats: Vec<String> = Vec::new();
            for c in categories {
                if c.is_null() {
                    return Status::Fatal;
                }
                let cs = unsafe { CStr::from_ptr(*c) };
                match cs.to_str() {
                    Ok(s) => cats.push(s.to_owned()),
                    Err(_) => return Status::Fatal,
                }
            }
            let Some(logging_on) = logging_on.to_bool() else {
                return Status::Fatal;
            };
            fmu.set_debug_logging(logging_on, cats)
        }

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident, $t_val:ty, $to_rust:expr) => {
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
            fmi2GetInteger,
            fmi2SetInteger,
            get_integer,
            set_integer,
            i32,
            |v: *const i32| unsafe { *v }
        );

        generate_get_set!(
            fmi2GetReal,
            fmi2SetReal,
            get_real,
            set_real,
            f64,
            |v: *const f64| unsafe { *v }
        );

        generate_get_set!(
            fmi2GetBoolean,
            fmi2SetBoolean,
            get_boolean,
            set_boolean,
            i32,
            |v: *const i32| unsafe { *v != 0 }
        );

        generate_get_set!(
            fmi2GetString,
            fmi2SetString,
            get_string,
            set_string,
            *const c_char,
            |v: *const *const c_char| {
                let v = unsafe { *v };
                v.to_str().unwrap_or("")
            }
        );

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetStatus(
            fmu: *mut $t,
            status_kind: i32,
            value: *mut Status,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            let mut status = Status::Ok;
            let res = fmu.get_status(status_kind, &mut status);
            if res == Status::Ok {
               unsafe { *value  = status }
            }
            res
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetRealStatus(
            fmu: *mut $t,
            status_kind: i32,
            value: *mut f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            let value = match unsafe { value.as_mut() } {
                Some(v) => v,
                None => return Status::Fatal,
            };
            fmu.get_real_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetIntegerStatus(
            fmu: *mut $t,
            status_kind: i32,
            value: *mut i32,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            let value = match unsafe { value.as_mut() } {
                Some(v) => v,
                None => return Status::Fatal,
            };
            fmu.get_integer_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetBooleanStatus(
            fmu: *mut $t,
            status_kind: i32,
            value: *mut i32,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            if value.is_null() {
                return Status::Fatal;
            }
            let mut b = false;
            let res = fmu.get_boolean_status(status_kind, &mut b);
            match res {
                Status::Ok => {
                    match b {
                        true => unsafe { *value = 1 },
                        false => unsafe { *value = 0 }
                    }
                    res
                },
                _ => res
            }
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetStringStatus(
            fmu: *mut $t,
            status_kind: i32,
            value: *mut *const c_char,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            if value.is_null() {
                return Status::Fatal;
            }
            fmu.get_string_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2CancelStep(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.cancel_step()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetRealInputDerivatives(
            fmu: *mut $t,
            vr: *const u32,
            nvr: usize,
            order: *const i32,
            value: *const f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { std::slice::from_raw_parts(vr, nvr) };
            let orders = unsafe { std::slice::from_raw_parts(order, nvr) };
            let values = unsafe { std::slice::from_raw_parts(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.set_real_input_derivative(vrs[i], orders[i], values[i]);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetRealOutputDerivatives(
            fmu: *mut $t,
            vr: *const u32,
            nvr: usize,
            order: *const i32,
            value: *mut f64,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts_mut(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.get_real_output_derivative(vrs[i], &orders[i], &mut values[i]);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SerializedFMUstateSize(
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
        pub unsafe extern "C" fn fmi2SerializeFMUstate(
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
        pub unsafe extern "C" fn fmi2DeSerializeFMUstate(
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
        pub unsafe extern "C" fn fmi2GetFMUstate(
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
        pub unsafe extern "C" fn fmi2SetFMUstate(fmu: *mut $t, state: *mut c_void) -> Status {
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
        pub unsafe extern "C" fn fmi2FreeFMUstate(
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
            let dv_unknown = unsafe { from_raw_parts_mut(dv_unknown_mut_ptr, n_unknown) };
            let v_known = unsafe { from_raw_parts(v_known_ptr, n_known) };
            let dv_known = unsafe { from_raw_parts(dv_known_ptr, n_known) };

            fmu.get_directional_derivative(v_known, v_unknown, dv_known, dv_unknown)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterEventMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.enter_event_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2NewDiscreteStates(
            fmu: *mut $t,
            info: *mut EventInfo,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.new_discrete_states(info)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterContinuousTimeMode(fmu: *mut $t) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.enter_continuous_time_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2CompletedIntegratorStep(
            fmu: *mut $t,
            no_prior: i32,
            enter_event: *mut i32,
            term: *mut i32,
        ) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            if enter_event.is_null() || term.is_null() {
                return Status::Fatal;
            }
            fmu.completed_integrator_step(no_prior, enter_event, term)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetTime(fmu: *mut $t, time: f64) -> Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Status::Fatal,
            };
            fmu.set_time(time)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetContinuousStates(
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
        pub unsafe extern "C" fn fmi2GetDerivatives(
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
        pub unsafe extern "C" fn fmi2GetEventIndicators(
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
        pub unsafe extern "C" fn fmi2GetContinuousStates(
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

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetNominalsOfContinuousStates(
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

            fmu.get_nominals_of_continuous_states(x)
        }
    };
}

#[cfg(test)]
mod cargo_check {
    // Usesd to get type checking on the macro.
    use crate::fmi2::{CallbackFunctions, FMI2, Kind};
    #[derive(Default)]
    pub struct Fmu {
        count: f64,
    }
    impl FMI2 for Fmu {
        fn instantiate<'a>(
            _instance_name: &'a str,
            _fmu_type: Kind,
            _guid: &'a str,
            _resource_location: &'a str,
            _functions: *const CallbackFunctions,
            _visible: bool,
            _logging_on: bool,
        ) -> Self {
            Self::default()
        }

        fn do_step(
            &mut self,
            _current_communication_point: f64,
            communication_step_size: f64,
            _no_set_fmu_state_prior_to_current_point: bool,
        ) -> Status {
            self.count += communication_step_size;
            Status::Ok
        }

        fn get_real(&mut self, vr: u32, value: &mut f64) -> Status {
            match vr {
                0 => *value = self.count,
                _ => return Status::Error,
            }
            Status::Ok
        }
    }
    generate_fmi2_ffi!(Fmu);
}
