use fmi_rs::fmi2::*;

#[derive(Default, Fmi2Ffi)]
pub struct Counter {
    count: f64,
}

type Type = Fmi2Str;

impl Fmi2 for Counter {
    fn instantiate(
        _instance_name: Fmi2Str,
        _fmu_type: Fmi2Type,
        _guid: Type,
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
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: Fmi2Bool,
    ) -> Fmi2Status {
        self.count += communication_step_size;
        Fmi2Status::OK
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Fmi2Status {
        match vr {
            0 => *value = self.count,
            _ => return Fmi2Status::FATAL,
        }
        Fmi2Status::OK
    }
}
