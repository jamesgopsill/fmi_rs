use std::ffi::c_void;

use fmi_rs::fmi3::*;

#[derive(Default, Fmi3Ffi)]
pub struct Add {
    a: f64,
    b: f64,
}

impl Fmi3 for Add {
    fn instantiate_model_exchange(
        _instance_name: Fmi3Str,
        _instantiation_token: Fmi3Str,
        _resource_path: Fmi3Str,
        _visible: Fmi3Bool,
        _logging_on: Fmi3Bool,
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: Fmi3Status,
            category: Fmi3Str,
            message: Fmi3Str,
        ),
    ) -> Option<Self> {
        Some(Self::default())
    }

    fn set_float64(&mut self, vr: u32, value: f64) -> Fmi3Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            _ => return Fmi3Status::ERROR,
        }
        Fmi3Status::OK
    }

    fn get_float64(&mut self, vr: u32, value: &mut f64) -> Fmi3Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.a + self.b,
            _ => return Fmi3Status::ERROR,
        }
        Fmi3Status::OK
    }
}
