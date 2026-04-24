use fmi_rs::fmi2::*;

#[derive(Default, Fmi2Ffi)]
pub struct Multiplier {
    input: f64,
    multiplier: f64,
}

impl Fmi2 for Multiplier {
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
            2 => *value = self.input * self.multiplier,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Fmi2Status {
        match vr {
            0 => self.input = value,
            1 => self.multiplier = value,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }
}
