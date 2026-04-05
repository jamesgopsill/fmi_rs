use fmi_rs::fmi2::*;

#[derive(Default, Fmi2Ffi)]
pub struct Multiplier {
    input: f64,
    multiplier: f64,
    output: f64,
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

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: Fmi2Bool,
    ) -> Fmi2Status {
        self.output = self.input * self.multiplier;
        Fmi2Status::OK
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Fmi2Status {
        match vr {
            0 => *value = self.input,
            1 => *value = self.multiplier,
            2 => *value = self.output,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Fmi2Status {
        match vr {
            0 => self.input = value,
            1 => self.multiplier = value,
            2 => self.output = value,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }
}
