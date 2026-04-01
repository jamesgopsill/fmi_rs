use fmi_rs::{fmi2::*, generate_fmi2_ffi};

#[derive(Default)]
pub struct Add {
    a: f64,
    b: f64,
    c: f64,
}

impl FMI2 for Add {
    fn instantiate<'a>(
        _instance_name: &'a str,
        _fmu_type: Kind,
        _guid: &'a str,
        _resource_location: &'a str,
        _functions: *const CallbackFunctions,
        _visible: bool,
        _logging_on: bool,
    ) -> Self {
        Self::default()
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: bool,
    ) -> Status {
        self.c = self.a + self.b;
        Status::Ok
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.c,
            _ => return Status::Error,
        }
        Status::Ok
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            2 => self.c = value,
            _ => return Status::Error,
        }
        Status::Ok
    }
}

generate_fmi2_ffi!(Add);
