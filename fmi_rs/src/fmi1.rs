#![allow(clippy::too_many_arguments)]
use std::ffi::*;

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum Status {
    Ok = 0,
    Warning = 1,
    Discard = 2,
    Error = 3,
    Fatal = 4,
    Pending = 5,
}

pub trait FMI1: Sized {
    fn instantiate_fmu(
        instance_name: &str,
        guid: &str,
        fmu_location: &str,
        mime_type: &str,
        timeout: f64,
        interactive: bool,
        functions: *const CallbackFunctions,
        logging_on: bool,
    ) -> Self;

    fn initialize(&mut self) -> Status {
        Status::Ok
    }

    fn terminate(&mut self) -> Status {
        Status::Ok
    }

    fn reset(&mut self) -> Status {
        Status::Ok
    }

    fn set_debug_logging(&mut self, _logging_on: bool) -> Status {
        Status::Ok
    }

    fn set_real(&mut self, _vr: u32, _value: f64) -> Status {
        Status::Error
    }

    fn set_integer(&mut self, _vr: u32, _value: i32) -> Status {
        Status::Error
    }

    fn set_boolean(&mut self, _vr: u32, _value: bool) -> Status {
        Status::Error
    }

    fn set_string(&mut self, _vr: u32, _value: &str) -> Status {
        Status::Ok
    }

    fn get_real(&mut self, _vr: u32, _value: &mut f64) -> Status {
        Status::Error
    }

    fn get_integer(&mut self, _vr: u32, _value: &mut i32) -> Status {
        Status::Error
    }

    fn get_boolean(&mut self, _vr: u32, _value: &mut i8) -> Status {
        Status::Error
    }

    fn get_string(&mut self, _vr: u32, _value: &mut c_char) -> Status {
        Status::Error
    }

    fn get_real_output_derivative(&mut self, _vr: u32, _order: &i32, _value: &mut f64) -> Status {
        Status::Error
    }

    fn do_step(
        &mut self,
        _current_communication_point: f64,
        _communication_step_size: f64,
        _new_step: bool,
    ) -> Status {
        Status::Ok
    }

    fn get_status(&mut self, _status_kind: i32, _status: &mut i32) -> Status {
        Status::Error
    }

    fn get_real_status(&mut self, _status_kind: i32, _status: &mut f64) -> Status {
        Status::Error
    }

    fn get_integer_status(&mut self, _status_kind: i32, _status: &mut i32) -> Status {
        Status::Error
    }

    fn get_boolean_status(&mut self, _status_kind: i32, _status: &mut i8) -> Status {
        Status::Error
    }

    fn get_string_status(&mut self, _status_kind: i32, _status: *mut c_char) -> Status {
        Status::Error
    }
}

#[repr(C)]
pub struct CallbackFunctions {
    pub logger: extern "C" fn(
        c: *mut c_void,
        instance_name: *const c_char,
        status: *const c_int,
        category: *const c_char,
        message: *const c_char,
        ...
    ),
    pub step_finished: extern "C" fn(c: *mut c_void, status: *const c_char),
    pub allocate_memory: extern "C" fn(nobj: usize, size: usize),
    pub free_memory: extern "C" fn(),
}

#[macro_export]
macro_rules! generate_fmi1_ffi {
    ($t: ty) => {
        use $crate::utils::*;
        use std::slice::{from_raw_parts, from_raw_parts_mut};
        use std::iter::zip;

        const _: () = {
            const fn assert_impl<T: FMI1>() {}
            assert_impl::<$t>();
        };

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiGetTypesPlatform() -> *const c_char {
            c"standard32".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiGetVersion() -> *const c_char {
            c"1.0".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiInstantiateSlave(
            instance_name: *const c_char,
            guid: *const c_char,
            fmu_location: *const c_char,
            mime_type: *const c_char,
            timeout: c_double,
            interactive: c_char, // i8
            functions: *const CallbackFunctions,
            logging_on: c_char,
        ) -> *mut c_void {
            if functions.is_null()
                || instance_name.is_null()
                || guid.is_null()
                || fmu_location.is_null()
                || mime_type.is_null()
            {
                return std::ptr::null_mut();
            }
            let Some(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(guid) = guid.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(fmu_location) = fmu_location.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(mime_type) = mime_type.to_str() else {
                return std::ptr::null_mut();
            };
            let Some(interactive) = interactive.to_bool() else {
                return std::ptr::null_mut();
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return std::ptr::null_mut();
            };
            let fmu = <$t>::instantiate_fmu(
                instance_name,
                guid,
                fmu_location,
                mime_type,
                timeout,
                interactive,
                functions,
                logging_on,
            );
            let instance = Box::new(fmu);
            Box::into_raw(instance) as *mut c_void
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiInitializeSlave(fmu: *mut c_void) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            fmu.initialize()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiTerminateSlave(fmu: *mut c_void) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            fmu.terminate()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiResetSlave(fmu: *mut c_void) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            fmu.reset()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiFreeSlaveInstance(fmu: *mut c_void) {
            if !fmu.is_null() {
                let _ = unsafe { Box::from_raw(fmu as *mut $t) };
            }
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiSetDebugLogging(
            fmu: *mut c_void,
            logging_on: c_char,
        ) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            let Some(logging_on) = logging_on.to_bool() else {
                return Status::Fatal;
            };
            fmu.set_debug_logging(logging_on)
        }


        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident,$t_c:ty, $t_rust:ty,  $to_rust:expr) => {


                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut c_void,
                    vrs: *const c_uint,
                    nvr: usize,
                    values: *mut $t_c,
                ) -> Status {
                    let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                        return Status::Fatal;
                    };
                    if vrs.is_null() || values.is_null() {
                        return Status::Fatal;
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts_mut(values, nvr) };
                    for (vr, value) in zip(vrs, values) {
                        let status = fmu.$trait_get(*vr, value);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    fmu: *mut c_void,
                    vrs: *const c_uint,
                    nvr: usize,
                    values: *const $t_c,
                ) -> Status {
                    let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                        return Status::Fatal;
                    };
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts(values, nvr) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let rv: $t_rust = $to_rust(value);
                        let status = fmu.$trait_set(*vr, rv);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }
            };
        }


        generate_get_set!(
            fmiGetInteger,
            fmiSetInteger,
            get_integer,
            set_integer,
            c_int,
            i32,
            |v: *const c_int| unsafe { *v }
        );
        generate_get_set!(
            fmiGetReal,
            fmiSetReal,
            get_real,
            set_real,
            c_double,
            f64,
            |v: *const c_double| unsafe { *v }
        );
        generate_get_set!(
            fmiGetBoolean,
            fmiSetBoolean,
            get_boolean,
            set_boolean,
            c_char,
            bool,
            // OPTION: Put error when it is not 1 or 0?
            |v: *const c_char| unsafe { *v != 0 }
        );
        generate_get_set!(
            fmiGetString,
            fmiSetString,
            get_string,
            set_string,
            c_char,
            &str,
            |v: *const c_char| v.to_str().unwrap_or("")
        );


        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiGetRealOutputDerivatives(
            fmu: *mut c_void,
            vr: *const c_uint,
            nvr: usize,
            order: *const c_int,
            value: *mut c_double,
        ) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts_mut(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.get_real_output_derivative(vrs[i], &orders[i], &mut values[i]);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }


        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiDoStep(
            fmu: *mut c_void,
            current_communication_point: c_double,
            communication_step_size: c_double,
            new_step: c_char,
        ) -> Status {
            let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                return Status::Fatal;
            };
            let Some(n) = new_step.to_bool() else {
                return Status::Fatal;
            };
            fmu.do_step(current_communication_point, communication_step_size, n)
        }

        macro_rules! generate_get_status {
            ($get_fn:ident, $trait_get:ident, $t_c:ty, $t_rust:ty, $to_rust:expr) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut c_void,
                    status_kind: *const c_int,
                    value: *mut $t_c,
                ) -> Status {
                    let Some(fmu) = fmu_from_ptr::<$t>(fmu) else {
                        return Status::Fatal;
                    };
                    if status_kind.is_null() || value.is_null() {
                        return Status::Fatal;
                    }
                    let Some(rv) = $to_rust(value) else {
                        return Status::Fatal;
                    };
                    fmu.$trait_get(unsafe { *status_kind }, rv)
                }
            }
        }

        generate_get_status!(
            fmiGetStatus,
            get_status,
            c_int,
            i32,
            |v: *mut c_int| unsafe { v.cast::<i32>().as_mut() }
        );

        generate_get_status!(
            fmiGetRealStatus,
            get_real_status,
            c_double,
            f64,
            |v: *mut c_double| unsafe { v.cast::<f64>().as_mut() }
        );

        generate_get_status!(
            fmiGetIntegerStatus,
            get_integer_status,
            c_int,
            i32,
            |v: *mut c_int| unsafe { v.cast::<i32>().as_mut() }
        );

        generate_get_status!(
            fmiGetBooleanStatus,
            get_boolean_status,
            c_char,
            i8,
            |v: *mut c_char| unsafe { v.cast::<i8>().as_mut() }
        );

        generate_get_status!(
            fmiGetStringStatus,
            get_string_status,
            c_char,
            *mut c_char,
            |v: *mut c_char| Some(v)
        );
    };
}

#[cfg(test)]
mod cargo_check {
    use super::*;
    // Usesd to get type checking on the macro.
    #[derive(Default)]
    struct Model {}
    impl FMI1 for Model {
        fn instantiate_fmu(
            _instance_name: &str,
            _guid: &str,
            _fmu_location: &str,
            _mime_type: &str,
            _timeout: f64,
            _interactive: bool,
            _functions: *const CallbackFunctions,
            _logging_on: bool,
        ) -> Self {
            Self::default()
        }
    }
    generate_fmi1_ffi!(Model);
}
