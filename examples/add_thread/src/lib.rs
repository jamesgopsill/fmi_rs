use std::{
    ffi::c_void,
    sync::{Mutex, mpsc},
    thread,
    time::Duration,
};

use fmi_rs::fmi3::*;

#[derive(Fmi3Ffi)]
pub struct Add {
    a: f64,
    b: f64,
    tx: Option<mpsc::Sender<(f64, f64)>>,
    rx: Mutex<mpsc::Receiver<f64>>,
    thread_handle: Mutex<Option<thread::JoinHandle<()>>>,
}

impl Default for Add {
    fn default() -> Self {
        let (tx_in, rx_in) = mpsc::channel::<(f64, f64)>();
        let (tx_out, rx_out) = mpsc::channel::<f64>();
        let handle = thread::spawn(move || {
            while let Ok((a, b)) = rx_in.recv() {
                let _ = tx_out.send(a + b);
            }
        });

        Self {
            a: 0.0,
            b: 0.0,
            tx: Some(tx_in),
            rx: Mutex::new(rx_out),
            thread_handle: Mutex::new(Some(handle)),
        }
    }
}

impl Drop for Add {
    fn drop(&mut self) {
        drop(self.tx.take());

        if let Ok(mut lock) = self.thread_handle.lock()
            && let Some(handle) = lock.take()
        {
            let _ = handle.join();
        }
    }
}

impl Fmi3 for Add {
    fn instantiate_co_simulation(
        _instance_name: Fmi3Str,
        _instantiation_token: Fmi3Str,
        _resource_path: Fmi3Str,
        _visible: Fmi3Bool,
        _logging_on: Fmi3Bool,
        _event_mode_used: Fmi3Bool,
        _early_return_allowed: Fmi3Bool,
        _intermediate_variables: &[u32],
        _instance_environment: *mut c_void,
        _log_message: *const extern "C" fn(
            instance_environment: *mut c_void,
            status: Fmi3Status,
            category: Fmi3Str,
            message: Fmi3Str,
        ),
        _intermediate_update: *const extern "C" fn(instance_enivronment: *mut c_void),
    ) -> Option<Self> {
        Some(Self::default())
    }

    fn set_float64(&mut self, vrs: &[u32], values: &[f64]) -> Fmi3Status {
        let mut current_index = 0;
        for vr in vrs {
            match vr {
                0 => {
                    self.a = values[current_index];
                    current_index += 1;
                }
                1 => {
                    self.b = values[current_index];
                    current_index += 1;
                }
                _ => return Fmi3Status::ERROR,
            }
        }
        Fmi3Status::OK
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _no_set_fmu_state_prior: Fmi3Bool,
        _event_encountered: &mut Fmi3Bool,
        _terminate: &mut Fmi3Bool,
        _early_return: &mut Fmi3Bool,
        _last_successful_time: &mut f64,
    ) -> Fmi3Status {
        if let Some(tx) = self.tx.as_ref() {
            match tx.send((self.a, self.b)) {
                Ok(_) => Fmi3Status::OK,
                Err(_) => Fmi3Status::ERROR,
            }
        } else {
            Fmi3Status::ERROR
        }
    }

    fn get_float64(&mut self, vrs: &[u32], values: &mut [f64]) -> Fmi3Status {
        let mut current_index = 0;
        for vr in vrs {
            match vr {
                2 => {
                    match self.rx.lock().unwrap().recv_timeout(Duration::from_secs(1)) {
                        Ok(v) => values[current_index] = v,
                        Err(_) => return Fmi3Status::WARNING,
                    }
                    current_index += 1;
                }
                _ => return Fmi3Status::ERROR,
            }
        }
        Fmi3Status::OK
    }
}
