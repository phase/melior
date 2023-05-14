//! Passes and pass managers.

pub mod r#async;
pub mod conversion;
pub mod gpu;
pub mod linalg;
mod manager;
mod operation_manager;
pub mod sparse_tensor;
pub mod transform;

pub use self::{manager::PassManager, operation_manager::OperationPassManager};
use mlir_sys::MlirPass;

/// A pass.
pub struct Pass {
    raw: MlirPass,
}

impl Pass {
    pub unsafe fn from_raw_fn(create_raw: unsafe extern "C" fn() -> MlirPass) -> Self {
        Self {
            raw: unsafe { create_raw() },
        }
    }

    pub unsafe fn to_raw(&self) -> MlirPass {
        self.raw
    }

    #[doc(hidden)]
    pub fn __private_from_raw_fn(create_raw: unsafe extern "C" fn() -> MlirPass) -> Self {
        unsafe { Self::from_raw_fn(create_raw) }
    }
}
