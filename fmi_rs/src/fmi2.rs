use std::ffi::{c_char, c_double, c_int, c_void};

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum FMI2Status {
    Ok = 0,
    Warning = 1,
    Discard = 2,
    Error = 3,
    Fatal = 4,
    Pending = 5,
}

impl TryFrom<c_int> for FMI2Status {
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

#[repr(i32)]
pub enum FMI2Type {
    ModelExchange = 0,
    CoSimulation = 1,
}

impl TryFrom<c_int> for FMI2Type {
    type Error = FMI2Status;
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ModelExchange),
            1 => Ok(Self::CoSimulation),
            _ => Err(FMI2Status::Fatal),
        }
    }
}

#[repr(i32)]
pub enum FMI2StatusKind {
    DoStepStatus = 0,
    PendingStatus = 1,
    LastSuccessfulTime = 2,
    Terminated = 3,
}

impl TryFrom<c_int> for FMI2StatusKind {
    type Error = FMI2Status;
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DoStepStatus),
            1 => Ok(Self::PendingStatus),
            2 => Ok(Self::LastSuccessfulTime),
            3 => Ok(Self::Terminated),
            _ => Err(FMI2Status::Error),
        }
    }
}

pub trait FMI2 {
    fn instantiate<'a>(
        instance_name: &'a str,
        fmu_type: FMI2Type,
        guid: &'a str,
        resource_location: &'a str,
        functions: *const FMI2CallbackFunctions,
        visible: bool,
        logging_on: bool,
    ) -> Self;

    /// # Safety
    unsafe fn from_ptr<'a>(ptr: *mut c_void) -> Result<&'a mut Self, FMI2Status>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            return Err(FMI2Status::Fatal);
        }
        unsafe { Ok(&mut *(ptr as *mut Self)) }
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_real(&mut self, _vr: u32, _value: &mut f64) -> FMI2Status {
        FMI2Status::Error
    }

    fn get_integer(&mut self, _vr: u32, _value: &mut i32) -> FMI2Status {
        FMI2Status::Error
    }

    fn get_boolean(&mut self, _vr: u32, _value: &mut i32) -> FMI2Status {
        FMI2Status::Error
    }

    fn get_string(&mut self, _vr: u32, _value: &mut c_char) -> FMI2Status {
        FMI2Status::Error
    }

    fn setup_experiment(
        &mut self,
        _tolerance_defined: bool,
        _tolerance: f64,
        _start_time: f64,
        _stop_time_defined: bool,
        _stop_time: f64,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn enter_initialization_mode(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn exit_initialization_mode(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn terminate(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn reset(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_debug_logging(&mut self, _logging_on: bool, _categories: Vec<String>) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_real(&mut self, _vr: u32, _value: f64) -> FMI2Status {
        FMI2Status::Error
    }

    fn set_integer(&mut self, _vr: u32, _value: i32) -> FMI2Status {
        FMI2Status::Error
    }

    fn set_boolean(&mut self, _vr: u32, _value: bool) -> FMI2Status {
        FMI2Status::Error
    }

    fn set_string(&mut self, _vr: u32, _value: &str) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_status(&mut self, _status_kind: FMI2StatusKind, _value: *mut FMI2Status) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_real_status(&mut self, _status_kind: FMI2StatusKind, _value: *mut f64) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_integer_status(&mut self, _status_kind: FMI2StatusKind, _value: *mut i32) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_boolean_status(&mut self, _status_kind: FMI2StatusKind, _value: *mut i32) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_string_status(
        &mut self,
        _status_kind: FMI2StatusKind,
        _value: *mut c_char,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn cancel_step(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_real_input_derivative(&self, _vr: u32, _order: i32, _value: f64) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_real_output_derivative(
        &mut self,
        _vr: u32,
        _order: &i32,
        _value: &mut f64,
    ) -> FMI2Status {
        FMI2Status::Error
    }

    fn serialized_fmu_state_size(
        &mut self,
        _state: *mut std::ffi::c_void,
        _size: *mut usize,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn serialize_fmu_state(
        &mut self,
        _state: *mut std::ffi::c_void,
        _serialized_state: &[u8],
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn deserialized_fmu_state(
        &mut self,
        _buffer: &[u8],
        _size: usize,
        _state: *mut *mut std::ffi::c_void,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_fmu_state(&mut self, _state: *mut c_void) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_fmu_state(&mut self, _state: *mut *mut c_void) -> FMI2Status {
        FMI2Status::Ok
    }

    fn free_fmu_state(&mut self, _state: *mut c_void) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_directional_derivative(
        &mut self,
        _v_known: &[u32],
        _v_unknown: &[u32],
        _dv_known: &[f64],
        _dv_unknown: &mut [f64],
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn enter_event_mode(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn new_discrete_states(&mut self, _info: *mut FMI2EventInfo) -> FMI2Status {
        FMI2Status::Ok
    }

    fn enter_continuous_time_mode(&mut self) -> FMI2Status {
        FMI2Status::Ok
    }

    fn completed_integrator_step(
        &mut self,
        _no_prior: i32,
        _enter_event: *mut i32,
        _term: *mut i32,
    ) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_time(&mut self, _time: f64) -> FMI2Status {
        FMI2Status::Ok
    }

    fn set_continuous_states(&mut self, _x: &[f64]) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_derivatives(&mut self, _dx: &mut [f64]) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_event_indicators(&mut self, _ei: &mut [f64]) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_continuous_states(&mut self, _x: &mut [f64]) -> FMI2Status {
        FMI2Status::Ok
    }

    fn get_nominals_of_continuous_states(&mut self, _x: &mut [f64]) -> FMI2Status {
        FMI2Status::Ok
    }
}

#[repr(C)]
pub struct FMI2CallbackFunctions {
    pub logger: extern "C" fn(
        component_environment: *mut c_void,
        instance_name: *const c_char,
        status: c_int,
        category: *const c_char,
        message: *const c_char,
        ...
    ),
    pub allocate_memory: extern "C" fn(n_obj: usize, size: usize) -> *mut c_void,
    pub free_memory: extern "C" fn(obj: *mut c_void),
    pub step_finished: extern "C" fn(component_environment: *mut c_void, status: c_int),
    pub component_environment: *mut c_void,
}

#[repr(C)]
pub struct FMI2EventInfo {
    pub new_discrete_states_needed: c_int,
    pub terminate_simulation: c_int,
    pub nominals_of_continuous_states_changed: c_int,
    pub values_of_continuous_states_changed: c_int,
    pub next_event_time_defined: c_int,
    pub next_event_time: c_double,
}
