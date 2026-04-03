use fmi_rs::{fmi2::*, generate_fmi2_ffi};

#[derive(Default)]
pub struct Add {
    a: Fmi2Real,
    b: Fmi2Real,
}

impl Fmi2 for Add {
    fn instantiate(
        _instance_name: Fmi2Str,
        _fmu_type: Fmi2Type,
        _guid: Fmi2Str,
        _resource_location: Fmi2Str,
        _functions: *const CallbackFunctions,
        _visible: Fmi2Bool,
        _logging_on: Fmi2Bool,
    ) -> Self {
        Self::default()
    }

    fn get_real(&mut self, vr: Fmi2Uint, value: &mut Fmi2Real) -> Fmi2Status {
        match vr {
            0 => *value = self.a,
            1 => *value = self.b,
            2 => *value = self.a + self.b,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn set_real(&mut self, vr: Fmi2Uint, value: Fmi2Real) -> Fmi2Status {
        match vr {
            0 => self.a = value,
            1 => self.b = value,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }
}

generate_fmi2_ffi!(Add);
