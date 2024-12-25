pub use cubature_sys::ffi::ErrorNorm;

use super::error::CubatureError;

pub struct TerminationInfo {
    pub max_eval: usize,
    pub req_abs_error: f64,
    pub req_rel_error: f64,
    pub norm: ErrorNorm,
}

impl TerminationInfo {
    pub fn new() -> TerminationInfoBuilder {
        TerminationInfoBuilder::default()
    }
}
#[derive(Default)]
pub struct TerminationInfoBuilder {
    max_eval: Option<usize>,
    req_abs_error: Option<f64>,
    req_rel_error: Option<f64>,
    norm: Option<cubature_sys::ffi::ErrorNorm>,
}

impl TerminationInfoBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn max_eval(mut self, max_eval: usize) -> Self {
        self.max_eval = Some(max_eval);
        self
    }

    pub fn req_abs_error(mut self, req_abs_error: f64) -> Self {
        self.req_abs_error = Some(req_abs_error);
        self
    }

    pub fn req_rel_error(mut self, req_rel_error: f64) -> Self {
        self.req_rel_error = Some(req_rel_error);
        self
    }

    pub fn norm(mut self, norm: cubature_sys::ffi::ErrorNorm) -> Self {
        self.norm = Some(norm);
        self
    }

    pub fn build(self) -> std::result::Result<TerminationInfo, CubatureError> {
        const DEFAULT_MAX_EVAL: usize = 100000;
        const DEFAULT_REQ_ABS_ERROR: f64 = 1.0e-8;
        const DEFAULT_REQ_REL_ERROR: f64 = 1.0e-8;

        let max_eval = self.max_eval.unwrap_or(DEFAULT_MAX_EVAL);
        let req_abs_error = self.req_abs_error.unwrap_or(DEFAULT_REQ_ABS_ERROR);
        let req_rel_error = self.req_rel_error.unwrap_or(DEFAULT_REQ_REL_ERROR);
        let norm = self.norm.unwrap_or(ErrorNorm::Individual);

        Ok(TerminationInfo {
            max_eval,
            req_abs_error,
            req_rel_error,
            norm,
        })
    }
}
