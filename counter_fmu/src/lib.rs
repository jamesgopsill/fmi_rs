use fmi_rs::{
    fmi2::{FMI2, FMI2CallbackFunctions, FMI2Status, FMI2Type},
    generate_fmi2_ffi,
};

#[derive(Default)]
struct Counter {
    count: f64,
}

impl FMI2 for Counter {
    fn instantiate<'a>(
        _instance_name: &'a str,
        _fmu_type: FMI2Type,
        _guid: &'a str,
        _resource_location: &'a str,
        _functions: *const FMI2CallbackFunctions,
        _visible: bool,
        _logging_on: bool,
    ) -> Self {
        Self::default()
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> FMI2Status {
        self.count += communication_step_size;
        FMI2Status::Ok
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> FMI2Status {
        match vr {
            0 => *value = self.count,
            _ => return FMI2Status::Error,
        }
        FMI2Status::Ok
    }
}

generate_fmi2_ffi!(Counter);
