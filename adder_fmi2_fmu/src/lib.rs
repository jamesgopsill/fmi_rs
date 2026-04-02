use fmi_rs::{fmi2::*, generate_fmi2_ffi};

#[derive(Default)]
pub struct Add {
    a: f64,
    b: f64,
}

impl FMI2 for Add {
    fn instantiate(
        _instance_name: &str,
        _fmu_type: Kind,
        _guid: &str,
        _resource_location: &str,
        _functions: *const CallbackFunctions,
        _visible: bool,
        _logging_on: bool,
    ) -> Self {
        Self::default()
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.a + self.b,
            _ => return Status::Error,
        }
        Status::Ok
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            _ => return Status::Error,
        }
        Status::Ok
    }
}

generate_fmi2_ffi!(Add);
