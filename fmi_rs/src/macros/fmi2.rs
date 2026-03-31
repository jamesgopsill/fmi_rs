use std::ffi::{CStr, c_char};

use crate::fmi2::Status;

pub trait ToStr {
    /// # Safety
    fn to_str<'a>(&self) -> Result<&'a str, Status>;
}

impl ToStr for *const c_char {
    fn to_str<'a>(&self) -> Result<&'a str, Status> {
        if self.is_null() {
            return Err(Status::Fatal);
        }
        let c_str = unsafe { CStr::from_ptr(*self) };
        match c_str.to_str() {
            Ok(str) => Ok(str),
            Err(_) => Err(Status::Fatal),
        }
    }
}

#[macro_export]
macro_rules! generate_fmi2_ffi {
    ($t: ty) => {
        use std::ffi::{CStr, c_char, c_double, c_int, c_uint, c_void};
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};
        use $crate::fmi2::*;
        use $crate::macros::fmi2::ToStr;

        // -- TRAIT BOUND CHECK --
        const _: () = {
            const fn assert_impl<T: $crate::fmi2::FMI2>() {}
            assert_impl::<$t>();
        };

        // -- HELPERS --
        fn c_int_to_bool(ptr: c_int) -> Result<bool, Status> {
            match ptr {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(Status::Fatal),
            }
        }

        // -- THE MACRO --
        /// # Safety
        /// I have manually checked I do not define these names anywhere else and
        /// the macros is called once (could there be a compile time check?)
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetTypesPlatform() -> *const c_char {
            c"default".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetVersion() -> *const c_char {
            c"2.0".as_ptr() as *const c_char
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Instantiate(
            instance_name: *const c_char,
            fmu_type: c_int,
            guid: *const c_char,
            resource_location: *const c_char,
            functions: *const CallbackFunctions,
            visible: c_int,
            logging_on: c_int,
        ) -> *mut c_void {
            // TODO: check structure of functions.
            if functions.is_null()
                || instance_name.is_null()
                || guid.is_null()
                || resource_location.is_null()
            {
                return std::ptr::null_mut();
            }
            let Ok(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(guid) = guid.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(resource_location) = resource_location.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(fmu_type) = Kind::try_from(fmu_type) else {
                return std::ptr::null_mut();
            };
            let Ok(visible) = c_int_to_bool(visible) else {
                return std::ptr::null_mut();
            };
            let Ok(logging_on) = c_int_to_bool(logging_on) else {
                return std::ptr::null_mut();
            };
            let instance = <$t>::instantiate(
                instance_name,
                fmu_type,
                guid,
                resource_location,
                functions,
                visible,
                logging_on,
            );
            let instance = Box::new(instance);
            Box::into_raw(instance) as *mut c_void
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2DoStep(
            c: *mut c_void,
            current_communication_point: c_double,
            communication_step_size: c_double,
            no_set_fmu_state_prior_to_current_point: c_int,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let n = match c_int_to_bool(no_set_fmu_state_prior_to_current_point) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.do_step(current_communication_point, communication_step_size, n)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2FreeInstance(component: *mut c_void) {
            if !component.is_null() {
                let _ = unsafe { Box::from_raw(component as *mut $t) };
            }
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetupExperiment(
            c: *mut c_void,
            tolerance_defined: c_int,
            tolerance: c_double,
            start_time: c_double,
            stop_time_defined: c_int,
            stop_time: c_double,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let tolerance_defined = match c_int_to_bool(tolerance_defined) {
                Ok(n) => n,
                Err(e) => return e,
            };
            let stop_time_defined = match c_int_to_bool(stop_time_defined) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.setup_experiment(
                tolerance_defined,
                tolerance,
                start_time,
                stop_time_defined,
                stop_time,
            )
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterInitializationMode(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.enter_initialization_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2ExitInitializationMode(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.exit_initialization_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Terminate(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.terminate()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2Reset(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.reset()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetDebugLogging(
            c: *mut c_void,
            logging_on: c_int,
            n_categories: usize,
            categories: *const *const c_char,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if categories.is_null() {
                return Status::Fatal;
            }
            let categories = unsafe { from_raw_parts(categories, n_categories) };
            let mut cats: Vec<String> = Vec::new();
            for c in categories {
                if c.is_null() {
                    return Status::Fatal;
                }
                let cs = unsafe { CStr::from_ptr(*c) };
                match cs.to_str() {
                    Ok(s) => cats.push(s.to_owned()),
                    Err(_) => return Status::Fatal,
                }
            }
            let logging_on = match c_int_to_bool(logging_on) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.set_debug_logging(logging_on, cats)
        }

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $trait_get:ident, $trait_set:ident,$t_c:ty, $t_rust:ty,  $to_rust:expr) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    c: *mut c_void,
                    vrs: *const c_uint,
                    nvr: usize,
                    values: *mut $t_c,
                ) -> Status {
                    let model = match unsafe { <$t>::from_ptr(c) } {
                        Ok(m) => m,
                        Err(e) => return e,
                    };
                    if vrs.is_null() || values.is_null() {
                        return Status::Fatal;
                    }
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts_mut(values, nvr) };
                    for (vr, value) in zip(vrs, values) {
                        let status = model.$trait_get(*vr, value);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    c: *mut c_void,
                    vrs: *const c_uint,
                    nvr: usize,
                    values: *const $t_c,
                ) -> Status {
                    let model = match unsafe { <$t>::from_ptr(c) } {
                        Ok(m) => m,
                        Err(e) => return e,
                    };
                    let vrs = unsafe { from_raw_parts(vrs, nvr) };
                    let values = unsafe { from_raw_parts(values, nvr) };
                    for (vr, value) in std::iter::zip(vrs, values) {
                        let rv: $t_rust = $to_rust(value);
                        let status = model.$trait_set(*vr, rv);
                        if status != Status::Ok {
                            return status;
                        }
                    }
                    Status::Ok
                }
            };
        }


        generate_get_set!(
            fmi2GetInteger,
            fmi2SetInteger,
            get_integer,
            set_integer,
            c_int,
            i32,
            |v: *const c_int| unsafe { *v }
        );
        generate_get_set!(
            fmi2GetReal,
            fmi2SetReal,
            get_real,
            set_real,
            c_double,
            f64,
            |v: *const c_double| unsafe { *v }
        );
        generate_get_set!(
            fmi2GetBoolean,
            fmi2SetBoolean,
            get_boolean,
            set_boolean,
            c_int,
            bool,
            // OPTION: Put error when it is not 1 or 0?
            |v: *const c_int| unsafe { *v != 0 }
        );
        generate_get_set!(
            fmi2GetString,
            fmi2SetString,
            get_string,
            set_string,
            c_char,
            &str,
            |v: *const c_char| unsafe { CStr::from_ptr(v).to_str().unwrap_or("") }
        );

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetStatus(
            c: *mut c_void,
            status_kind: c_int,
            value: *mut Status,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.get_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetRealStatus(
            c: *mut c_void,
            status_kind: c_int,
            value: *mut c_double,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.get_real_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetIntegerStatus(
            c: *mut c_void,
            status_kind: c_int,
            value: *mut c_int,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.get_integer_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetBooleanStatus(
            c: *mut c_void,
            status_kind: c_int,
            value: *mut c_int,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.get_boolean_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetStringStatus(
            c: *mut c_void,
            status_kind: c_int,
            value: *mut c_char,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            let status_kind = match StatusKind::try_from(status_kind) {
                Ok(n) => n,
                Err(e) => return e,
            };
            model.get_string_status(status_kind, value)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2CancelStep(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.cancel_step()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetRealInputDerivatives(
            c: *mut c_void,
            vr: *const c_uint,
            nvr: usize,
            order: *const c_int,
            value: *const c_double,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { std::slice::from_raw_parts(vr, nvr) };
            let orders = unsafe { std::slice::from_raw_parts(order, nvr) };
            let values = unsafe { std::slice::from_raw_parts(value, nvr) };
            for i in 0..vrs.len() {
                let status = model.set_real_input_derivative(vrs[i], orders[i], values[i]);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetRealOutputDerivatives(
            c: *mut c_void,
            vr: *const c_uint,
            nvr: usize,
            order: *const c_int,
            value: *mut c_double,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if vr.is_null() || order.is_null() || value.is_null() {
                return Status::Fatal;
            }
            let vrs = unsafe { from_raw_parts(vr, nvr) };
            let orders = unsafe { from_raw_parts(order, nvr) };
            let values = unsafe { from_raw_parts_mut(value, nvr) };
            for i in 0..vrs.len() {
                let status = model.get_real_output_derivative(vrs[i], &orders[i], &mut values[i]);
                if status != Status::Ok {
                    return status;
                }
            }
            Status::Ok
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SerializedFMUstateSize(
            c: *mut c_void,
            state: *mut c_void,
            size: *mut usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.serialized_fmu_state_size(state, size)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SerializeFMUstate(
            c: *mut c_void,
            state: *mut c_void,
            serialized_state: *mut u8, // fmi2Byte is u8
            size: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if state.is_null() || serialized_state.is_null() {
                return Status::Fatal;
            }
            let buffer = unsafe { from_raw_parts_mut(serialized_state, size) };
            model.serialize_fmu_state(state, buffer)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2DeSerializeFMUstate(
            c: *mut c_void,
            serialized_state: *const u8, // fmi2Byte is u8
            size: usize,
            state: *mut *mut c_void,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if state.is_null() || serialized_state.is_null() {
                return Status::Fatal;
            }
            let buffer = unsafe { std::slice::from_raw_parts(serialized_state, size) };
            model.deserialized_fmu_state(buffer, size, state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetFMUstate(
            c: *mut c_void,
            state: *mut *mut c_void,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            model.get_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetFMUstate(c: *mut c_void, state: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            model.set_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2FreeFMUstate(
            c: *mut c_void,
            state: *mut c_void,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if state.is_null() {
                return Status::Fatal;
            }
            model.free_fmu_state(state)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetDirectionalDerivative(
            c: *mut c_void,
            v_unknown_ptr: *const c_uint,
            n_unknown: usize,
            v_known_ptr: *const c_uint,
            n_known: usize,
            dv_known_ptr: *const c_double,
            dv_unknown_mut_ptr: *mut c_double,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if v_unknown_ptr.is_null()
                || v_known_ptr.is_null()
                || dv_known_ptr.is_null()
                || dv_unknown_mut_ptr.is_null()
            {
                return Status::Fatal;
            }
            let v_unknown = if n_unknown > 0 {
                unsafe { from_raw_parts(v_unknown_ptr, n_unknown) }
            } else {
                &[]
            };

            let dv_unknown = if n_unknown > 0 {
                unsafe { from_raw_parts_mut(dv_unknown_mut_ptr, n_unknown) }
            } else {
                &mut []
            };

            let v_known = if n_known > 0 {
                unsafe { from_raw_parts(v_known_ptr, n_known) }
            } else {
                &[]
            };

            let dv_known = if n_known > 0 {
                unsafe { from_raw_parts(dv_known_ptr, n_known) }
            } else {
                &[]
            };

            model.get_directional_derivative(v_known, v_unknown, dv_known, dv_unknown)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterEventMode(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.enter_event_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2NewDiscreteStates(
            c: *mut c_void,
            info: *mut EventInfo,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.new_discrete_states(info)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2EnterContinuousTimeMode(c: *mut c_void) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.enter_continuous_time_mode()
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2CompletedIntegratorStep(
            c: *mut c_void,
            no_prior: c_int,
            enter_event: *mut c_int,
            term: *mut c_int,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if enter_event.is_null() || term.is_null() {
                return Status::Fatal;
            }
            model.completed_integrator_step(no_prior, enter_event, term)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetTime(c: *mut c_void, time: c_double) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            model.set_time(time)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2SetContinuousStates(
            c: *mut c_void,
            x: *const c_double,
            nx: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if x.is_null() {
                return Status::Fatal;
            }
            let x = if nx > 0 {
                unsafe { from_raw_parts(x, nx) }
            } else {
                &[]
            };

            model.set_continuous_states(x)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetDerivatives(
            c: *mut c_void,
            dx: *mut c_double,
            nx: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if dx.is_null() {
                return Status::Fatal;
            }
            let dx = if nx > 0 {
                unsafe { from_raw_parts_mut(dx, nx) }
            } else {
                &mut []
            };

            model.get_derivatives(dx)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetEventIndicators(
            c: *mut c_void,
            ei: *mut c_double,
            ni: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if ei.is_null() {
                return Status::Fatal;
            }
            let ei = if ni > 0 {
                unsafe { from_raw_parts_mut(ei, ni) }
            } else {
                &mut []
            };

            model.get_event_indicators(ei)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetContinuousStates(
            c: *mut c_void,
            x: *mut c_double,
            nx: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if x.is_null() {
                return Status::Fatal;
            }
            let x = if nx > 0 {
                unsafe { from_raw_parts_mut(x, nx) }
            } else {
                &mut []
            };

            model.get_continuous_states(x)
        }

        /// # Safety
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi2GetNominalsOfContinuousStates(
            c: *mut c_void,
            x: *mut c_double,
            nx: usize,
        ) -> Status {
            let model = match unsafe { <$t>::from_ptr(c) } {
                Ok(model) => model,
                Err(e) => return e,
            };
            if x.is_null() {
                return Status::Fatal;
            }
            let x = if nx > 0 {
                unsafe { from_raw_parts_mut(x, nx) }
            } else {
                &mut []
            };

            model.get_nominals_of_continuous_states(x)
        }
    };
}

#[cfg(test)]
mod cargo_check {
    // Usesd to get type checking on the macro.
    use crate::fmi2::{CallbackFunctions, FMI2, Kind};
    #[derive(Default)]
    struct Counter {
        count: f64,
    }
    impl FMI2 for Counter {
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
            communication_step_size: f64,
            _no_set_fmu_state_prior_to_current_point: bool,
        ) -> Status {
            self.count += communication_step_size;
            Status::Ok
        }

        fn get_real(&mut self, vr: u32, value: &mut f64) -> Status {
            match vr {
                0 => *value = self.count,
                _ => return Status::Error,
            }
            Status::Ok
        }
    }
    generate_fmi2_ffi!(Counter);
}
