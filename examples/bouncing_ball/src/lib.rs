use fmi_rs::fmi2::*;

#[derive(Fmi2Ffi)]
pub struct BouncingBall {
    h: f64,
    g: f64,
    e: f64,
    v: f64,
}

impl Default for BouncingBall {
    fn default() -> Self {
        Self {
            h: 1.0,
            g: -9.81,
            e: 0.7,
            v: 0.0,
        }
    }
}

impl Fmi2 for BouncingBall {
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
            0 => *value = self.h,
            2 => *value = self.v,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Fmi2Status {
        match vr {
            0 => self.h = value,
            2 => self.v = value,
            4 => self.g = value,
            5 => self.e = value,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn get_continuous_states(&mut self, x: &mut [f64]) -> Fmi2Status {
        x[0] = self.h;
        x[1] = self.v;
        Fmi2Status::OK
    }

    fn set_continuous_states(&mut self, x: &[f64]) -> Fmi2Status {
        self.h = x[0];
        self.v = x[1];
        Fmi2Status::OK
    }

    /// Set the state space derivative for the bouncing ball.
    fn get_derivatives(&mut self, dx: &mut [f64]) -> Fmi2Status {
        dx[0] = self.v;
        dx[1] = self.g;
        Fmi2Status::OK
    }

    /// Activates on an event
    fn new_discrete_states(&mut self, info: &mut EventInfo) -> Fmi2Status {
        if self.h <= 0. && self.v < 0. {
            self.h = 0.01;
            self.v = -self.v * self.e;
            info.values_of_continuous_states_changed = Fmi2Bool::TRUE;
        } else {
            info.values_of_continuous_states_changed = Fmi2Bool::FALSE;
        }
        info.nominals_of_continuous_states_changed = Fmi2Bool::FALSE;
        info.terminate_simulation = Fmi2Bool::FALSE;
        info.next_event_time_defined = Fmi2Bool::FALSE;
        Fmi2Status::OK
    }

    /// Provide the event indicator to keep track of. In this case
    /// it is the height crossing zero.
    fn get_event_indicators(&mut self, ei: &mut [f64]) -> Fmi2Status {
        ei[0] = self.h;
        Fmi2Status::OK
    }
}
