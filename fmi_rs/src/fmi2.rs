use std::ffi::{c_char, c_double, c_int, c_void};

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

impl TryFrom<c_int> for Status {
    type Error = ();
    fn try_from(value: c_int) -> Result<Self, ()> {
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

impl TryFrom<c_int> for Kind {
    type Error = Status;
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
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

impl TryFrom<c_int> for StatusKind {
    type Error = Status;
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
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
pub trait FMI2 {
    /// Informs the FMU to allocate a new instance of the model.
    fn instantiate<'a>(
        instance_name: &'a str,
        fmu_type: Kind,
        guid: &'a str,
        resource_location: &'a str,
        functions: *const CallbackFunctions,
        visible: bool,
        logging_on: bool,
    ) -> Self;

    /// Helper to convert a raw C component pointer back into a Rust reference.
    ///
    /// # Safety
    /// The `ptr` must be a valid pointer to a type implementing `FMI2` that was
    /// created during `instantiate`.
    unsafe fn from_ptr<'a>(ptr: *mut c_void) -> Result<&'a mut Self, Status>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            return Err(Status::Fatal);
        }
        unsafe { Ok(&mut *(ptr as *mut Self)) }
    }

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
    fn get_string(&mut self, _vr: u32, _value: &mut c_char) -> Status {
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
    fn get_status(&mut self, _status_kind: StatusKind, _value: *mut Status) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific f64 value.
    fn get_real_status(&mut self, _status_kind: StatusKind, _value: *mut f64) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific Integer value.
    fn get_integer_status(&mut self, _status_kind: StatusKind, _value: *mut i32) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific Boolean value.
    fn get_boolean_status(&mut self, _status_kind: StatusKind, _value: *mut i32) -> Status {
        Status::Ok
    }

    /// [Co-Simulation only] Queries status of a specific String value.
    fn get_string_status(&mut self, _status_kind: StatusKind, _value: *mut c_char) -> Status {
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
        status: c_int,
        category: *const c_char,
        message: *const c_char,
        ...
    ),
    /// Function to allocate memory (use this instead of `malloc` for FMI compliance).
    pub allocate_memory: extern "C" fn(n_obj: usize, size: usize) -> *mut c_void,
    /// Function to free memory.
    pub free_memory: extern "C" fn(obj: *mut c_void),
    /// Callback for asynchronous step completion.
    pub step_finished: extern "C" fn(component_environment: *mut c_void, status: c_int),
    /// Pointer to the master's private environment data.
    pub component_environment: *mut c_void,
}

/// Structure containing information about events (Model Exchange).
#[repr(C)]
pub struct EventInfo {
    pub new_discrete_states_needed: c_int,
    pub terminate_simulation: c_int,
    pub nominals_of_continuous_states_changed: c_int,
    pub values_of_continuous_states_changed: c_int,
    pub next_event_time_defined: c_int,
    pub next_event_time: c_double,
}
