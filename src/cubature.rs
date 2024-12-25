use super::error::*;
use super::termination_info::*;

pub struct ResultInfo {
    pub num_evals: usize,
}

pub type Result = std::result::Result<ResultInfo, CubatureError>;

#[derive(Clone, Copy)]
pub enum Algorithm {
    HAdaptive,
    PAdaptive,
}

struct Cubature<Integrand: Fn(&[f64], &mut [f64]) -> bool> {
    integrand: Integrand,
    num_evals: usize,
}

impl<Integrand: Fn(&[f64], &mut [f64]) -> bool> Cubature<Integrand> {
    fn call_native(
        &mut self,
        algorithm: Algorithm,
        xmin: &[f64],
        xmax: &[f64],
        termination_info: &TerminationInfo,
        val: &mut [f64],
        err: &mut [f64],
    ) -> std::result::Result<(), CubatureError> {
        unsafe {
            let error_code = (match algorithm {
                Algorithm::HAdaptive => cubature_sys::hcubature,
                Algorithm::PAdaptive => cubature_sys::pcubature,
            })(
                val.len() as u32,
                Some(integrand_wrapper::<Integrand>),
                self as *mut Self as *mut _,
                xmin.len() as std::os::raw::c_uint,
                xmin.as_ptr(),
                xmax.as_ptr(),
                termination_info.max_eval,
                termination_info.req_abs_error,
                termination_info.req_rel_error,
                termination_info.norm,
                val.as_mut_ptr(),
                err.as_mut_ptr(),
            );

            match error_code {
                0 => Ok(()),
                _ => Err(error_code.into()),
            }
        }
    }
}

unsafe extern "C" fn integrand_wrapper<Integrand: Fn(&[f64], &mut [f64]) -> bool>(
    dimension: ::std::os::raw::c_uint,
    x_ptr: *const f64,
    data: *mut ::std::os::raw::c_void,
    codimension: ::std::os::raw::c_uint,
    function_value_ptr: *mut f64,
) -> ::std::os::raw::c_int {
    if data.is_null() {
        return CubatureError::Unknown as ::std::os::raw::c_int;
    }

    let cubature = std::ptr::NonNull::new(data as *mut Cubature<Integrand>)
        .unwrap()
        .as_mut();
    cubature.num_evals += 1;

    let x = std::slice::from_raw_parts(x_ptr, dimension as usize);
    let function_value = std::slice::from_raw_parts_mut(function_value_ptr, codimension as usize);
    assert!(!data.is_null());
    if (cubature.integrand)(x, function_value) {
        0
    } else {
        CubatureError::IntegrandEvalFailed as ::std::os::raw::c_int
    }
}

pub fn cubature<Integrand: Fn(&[f64], &mut [f64]) -> bool>(
    algorithm: Algorithm,
    integrand: Integrand,
    xmin: &[f64],
    xmax: &[f64],
    termination_info: &TerminationInfo,
    val: &mut [f64],
    err: &mut [f64],
) -> Result {
    if xmin.len() != xmax.len() {
        return Err(CubatureError::BoundsInconsistent);
    }

    let mut cubature = Cubature::<Integrand> {
        integrand,
        num_evals: 0,
    };

    cubature.call_native(algorithm, xmin, xmax, termination_info, val, err)?;

    Ok(ResultInfo {
        num_evals: cubature.num_evals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hcubature() {
        let xmin = [0.0, 0.0];
        let xmax = [1.5, 1.5];
        let termination_info = TerminationInfo::new().build().unwrap();
        let mut val: [f64; 3] = Default::default();
        let mut err: [f64; 3] = Default::default();

        let info = cubature(
            Algorithm::HAdaptive,
            |x, fval| {
                fval[0] = x[0] - x[0].floor();
                fval[1] = x[1] - x[1].floor();
                fval[2] = (x[0] - x[0].floor()) * (x[1] - x[1].floor());
                true
            },
            xmin.as_slice(),
            xmax.as_slice(),
            &termination_info,
            val.as_mut_slice(),
            err.as_mut_slice(),
        )
        .unwrap();

        assert!(info.num_evals < termination_info.max_eval);
        let expected_vals = [0.9375, 0.9375, 0.390625];
        for ((&val, &err), &expected_val) in val.iter().zip(err.iter()).zip(expected_vals.iter()) {
            assert!((val - expected_val).abs() < termination_info.req_abs_error);
            assert!(err < termination_info.req_abs_error);
        }
    }

    #[test]
    fn test_pcubature() {
        let xmin = [0.0, 0.0];
        let xmax = [1.0, 1.0];
        let termination_info = TerminationInfo::new().build().unwrap();
        let mut val: [f64; 2] = Default::default();
        let mut err: [f64; 2] = Default::default();

        let c = (1.0 + 10.0_f64.sqrt()) / 9.0;
        let info = cubature(
            Algorithm::PAdaptive,
            |x, fval| {
                fval[0] = x
                    .iter()
                    .map(|v| c / (c + 1.0) * ((c + 1.0) / (c + v)).powf(2.0))
                    .product();
                fval[1] = fval[0];
                true
            },
            xmin.as_slice(),
            xmax.as_slice(),
            &termination_info,
            val.as_mut_slice(),
            err.as_mut_slice(),
        )
        .unwrap();

        assert!(info.num_evals < termination_info.max_eval);
        for (&val, &err) in val.iter().zip(err.iter()) {
            assert!((val - 1.0).abs() < termination_info.req_abs_error);
            assert!(err < termination_info.req_abs_error);
        }
    }
}
