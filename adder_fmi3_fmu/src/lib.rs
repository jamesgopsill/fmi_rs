use std::ffi::c_void;

use fmi_rs::fmi3::*;

// TODO.

#[derive(Default, Fmi3Ffi)]
pub struct Add {
    _a: f64,
    _b: f64,
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
}
