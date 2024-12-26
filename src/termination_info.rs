pub use cubature_sys::ffi::ErrorNorm;

use super::error::CubatureError;

pub struct TerminationInfo {
    pub max_eval: usize,
    pub req_abs_error: f64,
    pub req_rel_error: f64,
    pub norm: ErrorNorm,
}

impl TerminationInfo {
    pub fn new_builder() -> TerminationInfoBuilder {
        TerminationInfoBuilder::default()
    }
}
pub struct TerminationInfoBuilder {
    max_eval: usize,
    req_abs_error: f64,
    req_rel_error: f64,
    norm: ErrorNorm,
}

impl Default for TerminationInfoBuilder {
    fn default() -> Self {
        Self {
            max_eval: 100_000,
            req_abs_error: 1.0e-6,
            req_rel_error: 1.0e-6,
            norm: ErrorNorm::Individual,
        }
    }
}

impl TerminationInfoBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn max_eval(mut self, max_eval: usize) -> Self {
        self.max_eval = max_eval;
        self
    }

    pub fn req_abs_error(mut self, req_abs_error: f64) -> Self {
        self.req_abs_error = req_abs_error;
        self
    }

    pub fn req_rel_error(mut self, req_rel_error: f64) -> Self {
        self.req_rel_error = req_rel_error;
        self
    }

    pub fn norm(mut self, norm: cubature_sys::ffi::ErrorNorm) -> Self {
        self.norm = norm;
        self
    }

    pub fn build(self) -> std::result::Result<TerminationInfo, CubatureError> {
        Ok(TerminationInfo {
            max_eval: self.max_eval,
            req_abs_error: self.req_abs_error,
            req_rel_error: self.req_rel_error,
            norm: self.norm,
        })
    }
}
