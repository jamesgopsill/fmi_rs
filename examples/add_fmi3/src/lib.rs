use std::ffi::c_void;

use fmi_rs::fmi3::*;

#[derive(Default, Fmi3Ffi)]
pub struct Add {
    a: f64,
    b: f64,
}

impl Fmi3 for Add {
    fn instantiate_co_simulation(
        _instance_name: Fmi3Str,
        _instantiation_token: Fmi3Str,
        _resource_path: Fmi3Str,
        _visible: Fmi3Bool,
        _logging_on: Fmi3Bool,
        _event_mode_used: Fmi3Bool,
        _early_return_allowed: Fmi3Bool,
        _intermediate_variables: &[u32],
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: Fmi3Status,
            category: Fmi3Str,
            message: Fmi3Str,
        ),
        _intermediate_update: *const extern "C" fn(instance_enivronment: *mut c_void),
    ) -> Option<Self> {
        Some(Self::default())
    }

    fn set_float64(&mut self, vrs: &[u32], values: &[f64]) -> Fmi3Status {
        let mut current_index = 0;
        for vr in vrs {
            match vr {
                0 => {
                    self.a = values[current_index];
                    current_index += 1;
                }
                1 => {
                    self.b = values[current_index];
                    current_index += 1;
                }
                _ => return Fmi3Status::ERROR,
            }
        }
        Fmi3Status::OK
    }

    fn get_float64(&mut self, vrs: &[u32], values: &mut [f64]) -> Fmi3Status {
        let mut current_index = 0;
        for vr in vrs {
            match vr {
                2 => {
                    values[current_index] = self.a + self.b;
                    current_index += 1;
                }
                _ => return Fmi3Status::ERROR,
            }
        }
        Fmi3Status::OK
    }
}
