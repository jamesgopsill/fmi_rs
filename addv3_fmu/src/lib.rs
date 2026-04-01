use fmi_rs::{fmi3::*, generate_fmi3_ffi};

#[derive(Default)]
pub struct Add {
    a: f64,
    b: f64,
    c: f64,
}

impl FMI3 for Add {
    fn instantiate_co_simulation<'a>(
        _instance_name: &'a str,
        _instantiation_token: &'a str,
        _resource_path: &'a str,
        _visible: bool,
        _logging_on: bool,
        _event_mode_used: bool,
        _early_return_allowed: bool,
        _intermediate_variables: &'a [u32],
        _instance_environment: *mut std::ffi::c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut std::ffi::c_void,
            status: std::ffi::c_int,
            category: *const std::ffi::c_char,
            message: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_void,
        _intermediate_update: *const extern "C" fn(
            instance_enivronment: *mut std::ffi::c_void,
        ) -> *mut std::ffi::c_void,
    ) -> Option<Self> {
        Some(Self::default())
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
        self.c = self.a + self.b;
        Status::Ok
    }

    fn get_float64(&mut self, vr: u32, value: &mut f64) -> Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.c,
            _ => return Status::Error,
        }
        Status::Ok
    }

    fn set_float64(&mut self, vr: u32, value: f64) -> Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            2 => self.c = value,
            _ => return Status::Error,
        }
        Status::Ok
    }
}

generate_fmi3_ffi!(Add);
