
use ffi::cuda::*;
use ffi::vector_types::*;
use error::*;

use std::ptr::null_mut;
use std::os::raw::*;

use super::module::*;

/// Handler of CUDA Kernel function
///
/// This keep a reference to loaded module `'m`
#[derive(Debug)]
pub struct Kernel<'m> {
    pub(crate) func: CUfunction,
    pub(crate) _m: &'m Module,
}

impl<'m> Kernel<'m> {
    /// Call CUDA kernel using `cuLaunchKernel`
    pub unsafe fn launch(
        &mut self,
        args: *mut *mut c_void,
        grid: Grid,
        block: Block,
    ) -> Result<()> {
        cuLaunchKernel(
            self.func,
            grid.x,
            grid.y,
            grid.z,
            block.x,
            block.y,
            block.z,
            0, // FIXME: no shared memory
            null_mut(), // use default stream
            args,
            null_mut(), // no extra
        ).check()
    }
}

/// Get type-erased pointer
pub fn void_cast<T>(r: &T) -> *mut c_void {
    &*r as *const T as *mut c_void
}

/// Size of Block in CUDA thread hierarchy
/// http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#programming-model
#[derive(Debug, Clone, Copy, NewType)]
pub struct Block(dim3);

impl Block {
    /// one-dimensional
    pub fn x(x: u32) -> Self {
        Block(dim3 { x: x, y: 1, z: 1 })
    }

    /// two-dimensional
    pub fn xy(x: u32, y: u32) -> Self {
        Block(dim3 { x: x, y: y, z: 1 })
    }

    /// three-dimensional
    pub fn xyz(x: u32, y: u32, z: u32) -> Self {
        Block(dim3 { x: x, y: y, z: z })
    }
}

/// Size of Grid in CUDA thread hierarchy
/// http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#programming-model
#[derive(Debug, Clone, Copy, NewType)]
pub struct Grid(dim3);

impl Grid {
    /// one-dimensional
    pub fn x(x: u32) -> Self {
        Grid(dim3 { x: x, y: 1, z: 1 })
    }

    /// two-dimensional
    pub fn xy(x: u32, y: u32) -> Self {
        Grid(dim3 { x: x, y: y, z: 1 })
    }

    /// three-dimensional
    pub fn xyz(x: u32, y: u32, z: u32) -> Self {
        Grid(dim3 { x: x, y: y, z: z })
    }
}
