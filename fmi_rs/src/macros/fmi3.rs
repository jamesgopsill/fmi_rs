use std::ffi::{CStr, c_char};

/// Represents the status returned by FMI functions.
#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum Status {
    /// All well.
    Ok = 0,
    /// Things are not quite right, but the computation can continue.
    Warning = 1,
    /// The FMU decided to skip this step.
    Discard = 2,
    /// An error occurred that can be recovered from.
    Error = 3,
    /// A global error occurred; the simulation cannot continue.
    Fatal = 4,
}

impl TryFrom<i32> for Status {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::Warning),
            2 => Ok(Self::Discard),
            3 => Ok(Self::Error),
            4 => Ok(Self::Fatal),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum DependencyKind {
    Independent = 0,
    Constant = 1,
    Fixed = 2,
    Tunable = 3,
    Discrete = 4,
    Dependent = 5,
}

impl TryFrom<i32> for DependencyKind {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(Self::Independent),
            1 => Ok(Self::Constant),
            2 => Ok(Self::Fixed),
            3 => Ok(Self::Tunable),
            4 => Ok(Self::Discrete),
            5 => Ok(Self::Dependent),
            _ => Err(Status::Fatal),
        }
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq)]
pub enum IntervalQualifier {
    IntervalNotYetKnown = 0,
    IntervalUnchanged = 1,
    IntervalChanged = 2,
}

impl TryFrom<i32> for IntervalQualifier {
    type Error = Status;
    fn try_from(value: i32) -> Result<Self, Status> {
        match value {
            0 => Ok(Self::IntervalNotYetKnown),
            1 => Ok(Self::IntervalUnchanged),
            2 => Ok(Self::IntervalChanged),
            _ => Err(Status::Fatal),
        }
    }
}

pub trait ToStr {
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
macro_rules! generate_fmi3_ffi {
    ($t: ty) => {
        use std::ffi::{CStr, c_char, c_double, c_int, c_uint, c_void};
        use std::iter::zip;
        use std::slice::{from_raw_parts, from_raw_parts_mut};
        use $crate::fmi3::*;
        use $crate::macros::fmi3::ToStr;

        // -- TRAIT BOUND CHECK --
        const _: () = {
            const fn assert_impl<T: $crate::fmi3::FMI3>() {}
            assert_impl::<$t>();
        };

        // -- INQUIRE VERSION --
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetVersion() -> *const c_char {
            c"3.0".as_ptr() as *const c_char
        }

        // -- INSTANTIATION & TERMINATION --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateModelExchange(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: c_int,
            logging_on: c_int,
            instance_environment: fmi3InstanceEnvironment,
            log_message: *const extern "C" fn(
                instance_environment: fmi3InstanceEnvironment,
                status: fmi3Status,
                category: *const c_char,
                message: *const c_char,
            ) -> *mut c_void,
        ) -> fmi3Instance {
            let Ok(instance_name) = instance_name.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(instantiation_token) = instantiation_token.to_str() else {
                return std::ptr::null_mut();
            };
            let Ok(resource_path) = resource_path.to_str() else {
                return std::ptr::null_mut();
            };
            let instance = <$t>::instantiate_model_exchange(
                instance_name,
                instantiation_token,
                resource_path,
                fmi3_bool_to_bool(visible),
                fmi3_bool_to_bool(logging_on),
                instance_environment,
                log_message,
            );
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        /*

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateCoSimulation(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: fmi3Boolean,
            logging_on: fmi3Boolean,
            event_mode_used: fmi3Boolean,
            early_return_allowed: fmi3Boolean,
            required_intermediate_variables: *const fmi3ValueReference,
            n_required_intermediate_variables: usize,
            instance_environment: fmi3InstanceEnvironment,
            log_message: fmi3LogMessageCallback,
            intermediate_update: fmi3IntermediateUpdateCallback,
        ) -> fmi3Instance {
            let Ok(instance_name) = str_from_c_char(instance_name) else {
                return std::ptr::null_mut();
            };
            let Ok(instantiation_token) = str_from_c_char(instantiation_token) else {
                return std::ptr::null_mut();
            };
            let Ok(resource_path) = str_from_c_char(resource_path) else {
                return std::ptr::null_mut();
            };

            let rivs = if n_required_intermediate_variables > 0 {
                from_raw_parts(
                    required_intermediate_variables,
                    n_required_intermediate_variables,
                )
            } else {
                &[]
            };

            let instance = <$t>::instantiate_co_simulation(
                instance_name,
                instantiation_token,
                resource_path,
                fmi3_bool_to_bool(visible),
                fmi3_bool_to_bool(logging_on),
                fmi3_bool_to_bool(event_mode_used),
                fmi3_bool_to_bool(early_return_allowed),
                rivs,
                instance_environment,
                log_message,
                intermediate_update,
            );
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3InstantiateScheduledExecution(
            instance_name: *const c_char,
            instantiation_token: *const c_char,
            resource_path: *const c_char,
            visible: fmi3Boolean,
            logging_on: fmi3Boolean,
            instance_environment: fmi3InstanceEnvironment,
            log_message: fmi3LogMessageCallback,
            lock_preemption: fmi3LockPreemptionCallback,
            unlock_preemption: fmi3UnlockPreemptionCallback,
        ) -> fmi3Instance {
            let Ok(instance_name) = str_from_c_char(instance_name) else {
                return std::ptr::null_mut();
            };
            let Ok(instantiation_token) = str_from_c_char(instantiation_token) else {
                return std::ptr::null_mut();
            };
            let Ok(resource_path) = str_from_c_char(resource_path) else {
                return std::ptr::null_mut();
            };

            let instance = <$t>::instantiate_scheduled_execution(
                instance_name,
                instantiation_token,
                resource_path,
                fmi3_bool_to_bool(visible),
                fmi3_bool_to_bool(logging_on),
                instance_environment,
                log_message,
                lock_preemption,
                unlock_preemption,
            );
            Box::into_raw(Box::new(instance)) as *mut c_void
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3FreeInstance(instance: fmi3Instance) {
            if !instance.is_null() {
                let _ = unsafe { Box::from_raw(instance as *mut $t) };
            }
        }

        // -- LOGGING & DEBUG --
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetDebugLogging(
            instance: fmi3Instance,
            logging_on: fmi3Boolean,
            n_categories: usize,
            categories: *const *const c_char,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let mut cats = Vec::with_capacity(n_categories);
            let cat_ptrs = from_raw_parts(categories, n_categories);
            for &p in cat_ptrs {
                match str_from_c_char(p) {
                    Ok(s) => cats.push(s),
                    Err(_) => return FMI3Status::Fatal,
                }
            }
            model.set_debug_logging(fmi3_bool_to_bool(logging_on), &cats)
        }

        // -- ENTER/EXIT MODES --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterConfigurationMode(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.enter_configuration_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3ExitConfigurationMode(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.exit_configuration_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterInitializationMode(
            instance: fmi3Instance,
            tolerance_defined: fmi3Boolean,
            tolerance: f64,
            start_time: f64,
            stop_time_defined: fmi3Boolean,
            stop_time: f64,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.enter_initialization_mode(
                fmi3_bool_to_bool(tolerance_defined),
                tolerance,
                start_time,
                fmi3_bool_to_bool(stop_time_defined),
                stop_time,
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3ExitInitializationMode(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.exit_initialization_mode()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterEventMode(
            instance: fmi3Instance,
            step_event: fmi3Boolean,
            state_event: fmi3Boolean,
            roots_found: *const i32,
            n_event_indicators: usize,
            time_event: fmi3Boolean,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let roots = if n_event_indicators > 0 {
                from_raw_parts(roots_found, n_event_indicators)
            } else {
                &[]
            };
            model.enter_event_mode(
                fmi3_bool_to_bool(step_event),
                fmi3_bool_to_bool(state_event),
                roots,
                fmi3_bool_to_bool(time_event),
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3Terminate(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.terminate()
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3Reset(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.reset()
        }

        // -- VARIABLE GETTERS & SETTERS --
        // This pattern repeats for all FMI3 types

        macro_rules! generate_get_set {
            ($get_fn:ident, $set_fn:ident, $t_ffi:ty, $trait_get:ident, $trait_set:ident) => {
                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $get_fn(
                    instance: fmi3Instance,
                    vrs: *const fmi3ValueReference,
                    nvrs: usize,
                    values: *mut $t_ffi,
                    nvals: usize,
                ) -> fmi3Status {
                    let model = match unsafe { <$t>::from_ptr(instance) } {
                        Ok(m) => m,
                        Err(e) => return e,
                    };
                    let v_refs = from_raw_parts(vrs, nvrs);
                    let v_vals = from_raw_parts_mut(values, nvals);
                    model.$trait_get(v_refs, v_vals)
                }

                #[unsafe(no_mangle)]
                pub unsafe extern "C" fn $set_fn(
                    instance: fmi3Instance,
                    vrs: *const fmi3ValueReference,
                    nvrs: usize,
                    values: *const $t_ffi,
                    nvals: usize,
                ) -> fmi3Status {
                    let model = match unsafe { <$t>::from_ptr(instance) } {
                        Ok(m) => m,
                        Err(e) => return e,
                    };
                    let v_refs = from_raw_parts(vrs, nvrs);
                    let v_vals = from_raw_parts(values, nvals);
                    model.$trait_set(v_refs, v_vals)
                }
            };
        }

        generate_get_set!(
            fmi3GetFloat32,
            fmi3SetFloat32,
            f32,
            get_float32,
            set_float32
        );
        generate_get_set!(
            fmi3GetFloat64,
            fmi3SetFloat64,
            f64,
            get_float64,
            set_float64
        );
        generate_get_set!(fmi3GetInt8, fmi3SetInt8, i8, get_int8, set_int8);
        generate_get_set!(fmi3GetInt16, fmi3SetInt16, i16, get_int16, set_int16);
        generate_get_set!(fmi3GetInt32, fmi3SetInt32, i32, get_int32, set_int32);
        generate_get_set!(fmi3GetInt64, fmi3SetInt64, i64, get_int64, set_int64);
        generate_get_set!(fmi3GetUInt8, fmi3SetUInt8, u8, get_uint8, set_uint8);
        generate_get_set!(fmi3GetUInt16, fmi3SetUInt16, u16, get_uint16, set_uint16);
        generate_get_set!(fmi3GetUInt32, fmi3SetUInt32, u32, get_uint32, set_uint32);
        generate_get_set!(fmi3GetUInt64, fmi3SetUInt64, u64, get_uint64, set_uint64);

        // Boolean handles conversion from i32 to bool
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetBoolean(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *mut i32,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let mut b_vals = vec![false; nvals];
            let status = model.get_boolean(from_raw_parts(vrs, nvrs), &mut b_vals);
            if status == FMI3Status::OK {
                let out = from_raw_parts_mut(values, nvals);
                for i in 0..nvals {
                    out[i] = bool_to_fmi3_bool(b_vals[i]);
                }
            }
            status
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetBoolean(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *const i32,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let in_vals = from_raw_parts(values, nvals);
            let b_vals: Vec<bool> = in_vals.iter().map(|&v| fmi3_bool_to_bool(v)).collect();
            model.set_boolean(from_raw_parts(vrs, nvrs), &b_vals)
        }

        // String and Binary require custom handling due to pointers
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetString(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *mut *const c_char,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_string(from_raw_parts(vrs, nvrs), from_raw_parts_mut(values, nvals))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetString(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *const *const c_char,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let ptrs = from_raw_parts(values, nvals);
            let mut strs = Vec::with_capacity(nvals);
            for &p in ptrs {
                match str_from_c_char(p) {
                    Ok(s) => strs.push(s),
                    Err(_) => return FMI3Status::Fatal,
                }
            }
            model.set_string(from_raw_parts(vrs, nvrs), &strs)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetBinary(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            sizes: *mut usize,
            values: *mut *const u8,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_binary(
                from_raw_parts(vrs, nvrs),
                from_raw_parts_mut(sizes, nvals),
                from_raw_parts_mut(values, nvals),
            )
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetBinary(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            sizes: *const usize,
            values: *const *const u8,
            nvals: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.set_binary(
                from_raw_parts(vrs, nvrs),
                from_raw_parts(sizes, nvals),
                from_raw_parts(values, nvals),
            )
        }

        // -- CLOCKS --
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetClock(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *mut fmi3Clock,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_clock(from_raw_parts(vrs, nvrs), from_raw_parts_mut(values, nvrs))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetClock(
            instance: fmi3Instance,
            vrs: *const u32,
            nvrs: usize,
            values: *const fmi3Clock,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.set_clock(from_raw_parts(vrs, nvrs), from_raw_parts(values, nvrs))
        }

        // -- STATE MANAGEMENT --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetFMUState(
            instance: fmi3Instance,
            state: *mut fmi3FMUState,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_fmu_state(state)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetFMUState(
            instance: fmi3Instance,
            state: fmi3FMUState,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.set_fmu_state(state)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3FreeFMUState(
            instance: fmi3Instance,
            state: *mut fmi3FMUState,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.free_fmu_state(state)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SerializedFMUStateSize(
            instance: fmi3Instance,
            state: fmi3FMUState,
            size: *mut usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.serialized_fmu_state_size(state, size)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SerializeFMUState(
            instance: fmi3Instance,
            state: fmi3FMUState,
            serialized_state: *mut u8,
            size: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.serialize_fmu_state(state, from_raw_parts_mut(serialized_state, size))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3DeSerializeFMUState(
            instance: fmi3Instance,
            serialized_state: *const u8,
            size: usize,
            state: *mut fmi3FMUState,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.deserialize_fmu_state(from_raw_parts(serialized_state, size), state)
        }

        // -- MATH & DERIVATIVES --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetDirectionalDerivative(
            instance: fmi3Instance,
            v_unknown: *const u32,
            n_unknown: usize,
            v_known: *const u32,
            n_known: usize,
            dv_known: *const f64,
            n_dv_known: usize,
            dv_unknown: *mut f64,
            n_dv_unknown: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_directional_derivative(
                from_raw_parts(v_unknown, n_unknown),
                from_raw_parts(v_known, n_known),
                from_raw_parts(dv_known, n_dv_known),
                from_raw_parts_mut(dv_unknown, n_dv_unknown),
            )
        }

        // -- CO-SIMULATION / STEPPING --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3DoStep(
            instance: fmi3Instance,
            current_communication_point: f64,
            communication_step_size: f64,
            no_set_fmu_state_prior: fmi3Boolean,
            event_encountered: *mut fmi3Boolean,
            terminate: *mut fmi3Boolean,
            early_return: *mut fmi3Boolean,
            last_successful_time: *mut f64,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };

            let mut ev = false;
            let mut term = false;
            let mut er = false;
            let status = model.do_step(
                current_communication_point,
                communication_step_size,
                fmi3_bool_to_bool(no_set_fmu_state_prior),
                &mut ev,
                &mut term,
                &mut er,
                unsafe { &mut *last_successful_time },
            );

            unsafe {
                if !event_encountered.is_null() {
                    *event_encountered = bool_to_fmi3_bool(ev);
                }
                if !terminate.is_null() {
                    *terminate = bool_to_fmi3_bool(term);
                }
                if !early_return.is_null() {
                    *early_return = bool_to_fmi3_bool(er);
                }
            }
            status
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3EnterStepMode(instance: fmi3Instance) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.enter_step_mode()
        }

        // -- MODEL EXCHANGE SPECIFIC --

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetTime(instance: fmi3Instance, time: f64) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.set_time(time)
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3SetContinuousStates(
            instance: fmi3Instance,
            x: *const f64,
            nx: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.set_continuous_states(from_raw_parts(x, nx))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetDerivatives(
            instance: fmi3Instance,
            dx: *mut f64,
            nx: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_derivatives(from_raw_parts_mut(dx, nx))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetEventIndicators(
            instance: fmi3Instance,
            ei: *mut f64,
            ni: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_event_indicators(from_raw_parts_mut(ei, ni))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3GetContinuousStates(
            instance: fmi3Instance,
            x: *mut f64,
            nx: usize,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            model.get_continuous_states(from_raw_parts_mut(x, nx))
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn fmi3NewDiscreteStates(
            instance: fmi3Instance,
            discrete_states_need_update: *mut fmi3Boolean,
            terminate_simulation: *mut fmi3Boolean,
            nominals_of_continuous_states_changed: *mut fmi3Boolean,
            values_of_continuous_states_changed: *mut fmi3Boolean,
            next_event_time_defined: *mut fmi3Boolean,
            next_event_time: *mut f64,
        ) -> fmi3Status {
            let model = match unsafe { <$t>::from_ptr(instance) } {
                Ok(m) => m,
                Err(e) => return e,
            };
            let mut dsnu = false;
            let mut ts = false;
            let mut nocc = false;
            let mut vocc = false;
            let mut netd = false;
            let status = model.new_discrete_states(
                &mut dsnu,
                &mut ts,
                &mut nocc,
                &mut vocc,
                &mut netd,
                unsafe { &mut *next_event_time },
            );
            unsafe {
                *discrete_states_need_update = bool_to_fmi3_bool(dsnu);
                *terminate_simulation = bool_to_fmi3_bool(ts);
                *nominals_of_continuous_states_changed = bool_to_fmi3_bool(nocc);
                *values_of_continuous_states_changed = bool_to_fmi3_bool(vocc);
                *next_event_time_defined = bool_to_fmi3_bool(netd);
            }
            status
        }
        */
    };
}
