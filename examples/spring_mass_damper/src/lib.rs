#![doc = include_str!("../README.md")]
#![allow(non_snake_case)]
use fmi_rs::fmi2::*;
use nalgebra::{Matrix2, Matrix2x1, Vector1, Vector2};

#[derive(Fmi2Ffi)]
pub struct SpringMassDamper {
    mass: f64,
    spring_stiffness: f64,
    damping_coefficient: f64,
    initial_extension: f64,
    /// $$ F $$
    u: Vector1<f64>,
    /// $$ \begin{bmatrix} 0 & 1 \\\\ \frac{-k}{m} & \frac{1}{m} \end{bmatrix} $$
    A: Matrix2<f64>,
    /// $$ \begin{bmatrix} 0 & \frac{1}{m} \\\\ \end{bmatrix} $$
    B: Matrix2x1<f64>,
    /// $$ \begin{bmatrix} x_1 \\\\ x_2 \end{bmatrix} = \begin{bmatrix} x \\\\ \dot{x} \end{bmatrix} $$
    x: Vector2<f64>,
}

impl Default for SpringMassDamper {
    fn default() -> Self {
        let mass = 1.0;
        let spring_stiffness = 10.0;
        let damping_coefficient = 0.0;
        let initial_extension = 0.75;
        let u = Vector1::new(0.0);
        let A = Matrix2::new(
            0.0,
            1.0,
            -spring_stiffness / mass,
            -damping_coefficient / mass,
        );
        let B = Matrix2x1::new(0.0, 1.0 / mass);
        let x = Vector2::new(0.0, initial_extension);
        Self {
            mass,
            spring_stiffness,
            damping_coefficient,
            initial_extension,
            u,
            A,
            B,
            x,
        }
    }
}

impl Fmi2 for SpringMassDamper {
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

    fn setup_experiment(
        &mut self,
        _tolerance_defined: Fmi2Bool,
        _tolerance: f64,
        _start_time: f64,
        _stop_time_defined: Fmi2Bool,
        _stop_time: f64,
    ) -> Fmi2Status {
        let A = Matrix2::new(
            0.0,
            1.0,
            -self.spring_stiffness / self.mass,
            -self.damping_coefficient / self.mass,
        );
        let B = Matrix2x1::new(0.0, 1.0 / self.mass);
        let x = Vector2::new(self.initial_extension, 0.0);
        self.A = A;
        self.B = B;
        self.x = x;
        Fmi2Status::OK
    }

    fn set_real(&mut self, vr: u32, value: f64) -> Fmi2Status {
        match vr {
            0 => self.mass = value,
            1 => self.spring_stiffness = value,
            2 => self.damping_coefficient = value,
            3 => self.initial_extension = value,
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn get_real(&mut self, vr: u32, value: &mut f64) -> Fmi2Status {
        match vr {
            4 => *value = self.x[1], // velocity
            5 => *value = self.x[0], // position
            _ => return Fmi2Status::ERROR,
        }
        Fmi2Status::OK
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        communication_step_size: f64,
        _no_set_fmu_state_prior_to_current_point: Fmi2Bool,
    ) -> Fmi2Status {
        // Compute the derivative for the state space
        let derivative = self.A * self.x + self.B * self.u;
        // Euler Integration
        self.x += derivative * communication_step_size;
        Fmi2Status::OK
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_iteration() {
        let mut smd = SpringMassDamper::default();
        println!("{} {}", smd.A, smd.x);
        for i in 1..5 {
            smd.do_step(i as f64, 1.0, Fmi2Bool::FALSE);
            println!("{}", smd.x);
        }
    }
}
