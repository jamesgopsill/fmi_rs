#![allow(clippy::too_many_arguments)]
use std::ffi::*;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Fmi1Status(i32);

impl Fmi1Status {
    pub const OK: Self = Self(0);
    pub const WARNING: Self = Self(1);
    pub const DISCARD: Self = Self(2);
    pub const ERROR: Self = Self(3);
    pub const FATAL: Self = Self(4);
    pub const PENDING: Self = Self(5);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Fmi1StatusKind(i32);

impl Fmi1StatusKind {
    pub const DO_STEP: Self = Self(0);
    pub const PENDING: Self = Self(1);
    pub const LAST_SUCCESSFUL_TIME: Self = Self(2);
}

pub type Fmi1Real = f64;
pub type Fmi1Int = i32;
pub type Fmi1Uint = u32;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Fmi1Bool(u8);

impl Fmi1Bool {
    pub const FALSE: Self = Self(0);
    pub const TRUE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Fmi1Str(*const c_char);

impl Fmi1Str {
    pub fn to_str(&self) -> Result<&CStr, Fmi1Status> {
        if self.0.is_null() {
            return Err(Fmi1Status::FATAL);
        }
        Ok(unsafe { CStr::from_ptr(self.0) })
    }
}

#[repr(C)]
pub struct Fmi1CallbackFunctions {
    pub logger: extern "C" fn(
        c: *mut c_void,
        instance_name: Fmi1Str,
        status: Fmi1Status,
        category: Fmi1Str,
        message: Fmi1Str,
        ...
    ),
    pub step_finished: extern "C" fn(c: *mut c_void, status: *const c_char),
    pub allocate_memory: extern "C" fn(nobj: usize, size: usize),
    pub free_memory: extern "C" fn(),
}

pub trait Fmi1: Sized {
    fn instantiate_fmu(
        instance_name: Fmi1Str,
        guid: Fmi1Str,
        fmu_location: Fmi1Str,
        mime_type: Fmi1Str,
        timeout: Fmi1Real,
        visible: Fmi1Bool,
        interactive: Fmi1Bool,
        functions: *const Fmi1CallbackFunctions,
        logging_on: Fmi1Bool,
    ) -> Self;

    fn initialize(&mut self) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn terminate(&mut self) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn reset(&mut self) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn set_debug_logging(&mut self, _logging_on: Fmi1Bool) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn set_real(&mut self, _vr: Fmi1Uint, _value: Fmi1Real) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn set_integer(&mut self, _vr: Fmi1Uint, _value: Fmi1Int) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn set_boolean(&mut self, _vr: Fmi1Uint, _value: Fmi1Bool) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn set_string(&mut self, _vr: Fmi1Uint, _value: Fmi1Str) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn get_real(&mut self, _vr: Fmi1Uint, _value: &mut Fmi1Real) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_integer(&mut self, _vr: Fmi1Uint, _value: &mut Fmi1Int) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_boolean(&mut self, _vr: Fmi1Uint, _value: &mut Fmi1Bool) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_string(&mut self, _vr: Fmi1Uint, _value: &mut Fmi1Str) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_real_output_derivative(
        &mut self,
        _vr: Fmi1Uint,
        _order: Fmi1Int,
        _value: &mut Fmi1Real,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn set_real_output_derivative(
        &mut self,
        _vr: Fmi1Uint,
        _order: Fmi1Int,
        _value: Fmi1Real,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn do_step(
        &mut self,
        _current_communication_point: Fmi1Real,
        _communication_step_size: Fmi1Real,
        _new_step: Fmi1Bool,
    ) -> Fmi1Status {
        Fmi1Status::OK
    }

    fn get_status(&mut self, _status_kind: Fmi1StatusKind, _status: &mut Fmi1Status) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_real_status(
        &mut self,
        _status_kind: Fmi1StatusKind,
        _status: &mut Fmi1Real,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_integer_status(
        &mut self,
        _status_kind: Fmi1StatusKind,
        _status: &mut Fmi1Int,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_boolean_status(
        &mut self,
        _status_kind: Fmi1StatusKind,
        _status: &mut Fmi1Bool,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }

    fn get_string_status(
        &mut self,
        _status_kind: Fmi1StatusKind,
        _status: *mut Fmi1Str,
    ) -> Fmi1Status {
        Fmi1Status::ERROR
    }
}

#[macro_export]
macro_rules! generate_fmi1_ffi {
    ($t: ty) => {
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};
        // use $crate::fmi1::*;

        const _: () = {
            const fn assert_impl<T: Fmi1>() {}
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
            instance_name: Fmi1Str,
            fmu_guid: Fmi1Str,
            fmu_location: Fmi1Str,
            mime_type: Fmi1Str,
            timeout: Fmi1Real,
            visible: Fmi1Bool,
            interactive: Fmi1Bool,
            functions: *const Fmi1CallbackFunctions,
            logging_on: Fmi1Bool,
        ) -> *mut $t {
            if functions.is_null() {
                return std::ptr::null_mut();
            }
            let fmu = <$t>::instantiate_fmu(
                instance_name,
                fmu_guid,
                fmu_location,
                mime_type,
                timeout,
                visible,
                interactive,
                functions,
                logging_on,
            );
            let instance = Box::new(fmu);
            Box::into_raw(instance) as *mut $t
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiFreeSlaveInstance(fmu: *mut $t) {
            if !fmu.is_null() {
                let _ = unsafe { Box::from_raw(fmu) };
            }
        }

        macro_rules! generate_no_arg_fcn {
            ($ffi_fn:ident, $trait_fn:ident) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $ffi_fn(fmu: *mut $t) -> Fmi1Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi1Status::FATAL,
                    };
                    fmu.$trait_fn()
                }
            };
        }

        generate_no_arg_fcn!(fmiInitializeSlave, initialize);
        generate_no_arg_fcn!(fmiTerminateSlave, terminate);
        generate_no_arg_fcn!(fmiResetSlave, reset);

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiSetDebugLogging(
            fmu: *mut $t,
            logging_on: Fmi1Bool,
        ) -> Fmi1Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi1Status::FATAL,
            };
            fmu.set_debug_logging(logging_on)
        }

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident, $t_val:ty) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    vrs: *const Fmi1Uint,
                    nvr: usize,
                    values: *mut $t_val,
                ) -> Fmi1Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi1Status::FATAL,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Fmi1Status::FATAL;
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts_mut(values, nvr) };
                    for (vr, value) in zip(vrs, values) {
                        let status = fmu.$trait_get(*vr, value);
                        if status != Fmi1Status::OK {
                            return status;
                        }
                    }
                    Fmi1Status::OK
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    fmu: *mut $t,
                    vrs: *const Fmi1Uint,
                    nvr: usize,
                    values: *const $t_val,
                ) -> Fmi1Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi1Status::FATAL,
                    };
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts(values, nvr) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let status = fmu.$trait_set(*vr, *value);
                        if status != Fmi1Status::OK {
                            return status;
                        }
                    }
                    Fmi1Status::OK
                }
            };
        }

        generate_get_set!(
            fmiGetInteger,
            fmiSetInteger,
            get_integer,
            set_integer,
            Fmi1Int
        );
        generate_get_set!(fmiGetReal, fmiSetReal, get_real, set_real, Fmi1Real);
        generate_get_set!(
            fmiGetBoolean,
            fmiSetBoolean,
            get_boolean,
            set_boolean,
            Fmi1Bool
        );
        generate_get_set!(fmiGetString, fmiSetString, get_string, set_string, Fmi1Str);

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiGetRealOutputDerivatives(
            fmu: *mut $t,
            vr: *const Fmi1Uint,
            nvr: usize,
            order: *const Fmi1Int,
            value: *mut Fmi1Real,
        ) -> Fmi1Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi1Status::FATAL,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Fmi1Status::FATAL;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts_mut(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.get_real_output_derivative(vrs[i], orders[i], &mut values[i]);
                if status != Fmi1Status::OK {
                    return status;
                }
            }
            Fmi1Status::OK
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiSetRealOutputDerivatives(
            fmu: *mut $t,
            vr: *const Fmi1Uint,
            nvr: usize,
            order: *const i32,
            value: *const f64,
        ) -> Fmi1Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi1Status::FATAL,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Fmi1Status::FATAL;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts(value, nvr) };
            for i in 0..vrs.len() {
                let status = fmu.set_real_output_derivative(vrs[i], orders[i], values[i]);
                if status != Fmi1Status::OK {
                    return status;
                }
            }
            Fmi1Status::OK
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmiDoStep(
            fmu: *mut $t,
            current_communication_point: Fmi1Real,
            communication_step_size: Fmi1Real,
            new_step: Fmi1Bool,
        ) -> Fmi1Status {
            let fmu = match unsafe { fmu.as_mut() } {
                Some(f) => f,
                None => return Fmi1Status::FATAL,
            };
            fmu.do_step(
                current_communication_point,
                communication_step_size,
                new_step,
            )
        }

        macro_rules! generate_get_status {
            ($get_fn:ident, $trait_get:ident, $t_val:ty) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    fmu: *mut $t,
                    status_kind: *const Fmi1StatusKind,
                    value: *mut $t_val,
                ) -> Fmi1Status {
                    let fmu = match unsafe { fmu.as_mut() } {
                        Some(f) => f,
                        None => return Fmi1Status::FATAL,
                    };
                    let status_kind = match unsafe { status_kind.as_ref() } {
                        Some(s) => s,
                        None => return Fmi1Status::FATAL,
                    };
                    let value = match unsafe { value.as_mut() } {
                        Some(v) => v,
                        None => return Fmi1Status::FATAL,
                    };
                    fmu.$trait_get(*status_kind, value)
                }
            };
        }

        generate_get_status!(fmiGetStatus, get_status, Fmi1Status);
        generate_get_status!(fmiGetRealStatus, get_real_status, Fmi1Real);
        generate_get_status!(fmiGetIntegerStatus, get_integer_status, Fmi1Int);
        generate_get_status!(fmiGetBooleanStatus, get_boolean_status, Fmi1Bool);
        generate_get_status!(fmiGetStringStatus, get_string_status, Fmi1Str);
    };
}

#[cfg(test)]
mod cargo_check {
    use super::*;
    // Usesd to get type checking on the macro.
    #[derive(Default)]
    pub struct Model {}
    impl Fmi1 for Model {
        fn instantiate_fmu(
            _instance_name: Fmi1Str,
            _guid: Fmi1Str,
            _fmu_location: Fmi1Str,
            _mime_type: Fmi1Str,
            _timeout: Fmi1Real,
            _visible: Fmi1Bool,
            _interactive: Fmi1Bool,
            _functions: *const Fmi1CallbackFunctions,
            _logging_on: Fmi1Bool,
        ) -> Self {
            Self::default()
        }
    }
    generate_fmi1_ffi!(Model);
}
