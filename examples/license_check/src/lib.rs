#![allow(unused)]
use fmi_rs::fmi2::*;

#[derive(Default, Fmi2Ffi)]
pub struct MeaningOfLife {
    lic: String,
}

impl Fmi2 for MeaningOfLife {
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

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Fmi2Status {
        match vr {
            0 => *value = 42.0,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn set_string(&mut self, vr: u32, value: Fmi2Str) -> Fmi2Status {
        match vr {
            1 => {
                let lic = match value.to_str() {
                    Ok(l) => l,
                    Err(e) => return e,
                };
                self.lic = lic.to_string_lossy().into_owned();
            }
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn enter_initialization_mode(&mut self) -> Fmi2Status {
        match self.lic.as_str() {
            "my-license" => Fmi2Status::OK,
            _ => Fmi2Status::ERROR,
        }
    }
}
