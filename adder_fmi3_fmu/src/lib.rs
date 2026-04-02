use fmi_rs::{fmi3::*, generate_fmi3_ffi};

#[derive(Default)]
pub struct Add {
    a: f64,
    b: f64,
}

impl FMI3 for Add {
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
        ),
    ) -> Option<Self> {
        Some(Self::default())
    }

    fn get_float64(&mut self, vr: u32, value: &mut f64) -> Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.a + self.b,
            _ => return Status::Error,
        }
        Status::Ok
    }

    fn set_float64(&mut self, vr: u32, value: f64) -> Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            _ => return Status::Error,
        }
        Status::Ok
    }
}

generate_fmi3_ffi!(Add);
