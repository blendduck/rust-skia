use std::fmt;

use skia_bindings as sb;

use crate::{prelude::*, ISize};

#[cfg(feature = "metal")]
pub mod mtl;

pub use sb::skgpu_BackendApi as BackendApi;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum InsertStatus {
    Success = 0,
    InvalidRecording = 1,
    PromiseImageInstantiationFailed = 2,
    AddCommandsFailed = 3,
    AsyncShaderCompilesFailed = 4,
    OutOfOrderRecording = 5,
}

impl InsertStatus {
    fn from_native(value: i32) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::InvalidRecording,
            2 => Self::PromiseImageInstantiationFailed,
            3 => Self::AddCommandsFailed,
            4 => Self::AsyncShaderCompilesFailed,
            5 => Self::OutOfOrderRecording,
            _ => panic!("unknown Graphite insert status: {value}"),
        }
    }
}

pub mod surfaces {
    use skia_bindings as sb;

    use super::{BackendTexture, Recorder};
    use crate::{prelude::*, ColorSpace, Surface, SurfaceProps};

    pub fn wrap_backend_texture(
        recorder: &mut Recorder,
        backend_texture: &BackendTexture,
        color_space: impl Into<Option<ColorSpace>>,
        surface_props: Option<&SurfaceProps>,
    ) -> Option<Surface> {
        Surface::from_ptr(unsafe {
            sb::C_SkSurfaces_WrapGraphiteBackendTexture(
                recorder.native_mut(),
                backend_texture.native(),
                color_space.into().into_ptr_or_null(),
                surface_props.native_ptr_or_null(),
            )
        })
    }
}

pub type Context = RefHandle<sb::skgpu_graphite_Context>;

impl NativeDrop for sb::skgpu_graphite_Context {
    fn drop(&mut self) {
        unsafe { sb::C_SkGraphiteContext_delete(self) }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("graphite::Context").finish()
    }
}

impl Context {
    pub fn make_recorder(&mut self) -> Option<Recorder> {
        Recorder::from_ptr(unsafe { sb::C_SkGraphiteContext_makeRecorder(self.native_mut()) })
    }

    pub fn insert_recording(&mut self, mut recording: Recording) -> InsertStatus {
        InsertStatus::from_native(unsafe {
            sb::C_SkGraphiteContext_insertRecording(self.native_mut(), recording.native_mut())
        })
    }

    pub fn submit(&mut self, sync_to_cpu: bool) -> bool {
        unsafe { sb::C_SkGraphiteContext_submit(self.native_mut(), sync_to_cpu) }
    }
}

pub type Recorder = RefHandle<sb::skgpu_graphite_Recorder>;

impl NativeDrop for sb::skgpu_graphite_Recorder {
    fn drop(&mut self) {
        unsafe { sb::C_SkGraphiteRecorder_delete(self) }
    }
}

impl fmt::Debug for Recorder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("graphite::Recorder").finish()
    }
}

impl Recorder {
    pub fn snap(&mut self) -> Option<Recording> {
        Recording::from_ptr(unsafe { sb::C_SkGraphiteRecorder_snap(self.native_mut()) })
    }
}

pub type Recording = RefHandle<sb::skgpu_graphite_Recording>;

impl NativeDrop for sb::skgpu_graphite_Recording {
    fn drop(&mut self) {
        unsafe { sb::C_SkGraphiteRecording_delete(self) }
    }
}

impl fmt::Debug for Recording {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("graphite::Recording").finish()
    }
}

pub type BackendTexture = RefHandle<sb::skgpu_graphite_BackendTexture>;

impl NativeDrop for sb::skgpu_graphite_BackendTexture {
    fn drop(&mut self) {
        unsafe { sb::C_SkGraphiteBackendTexture_delete(self) }
    }
}

impl Clone for BackendTexture {
    fn clone(&self) -> Self {
        Self::from_ptr(unsafe { sb::C_SkGraphiteBackendTexture_Clone(self.native()) }).unwrap()
    }
}

impl fmt::Debug for BackendTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("graphite::BackendTexture")
            .field("backend", &self.backend())
            .field("dimensions", &self.dimensions())
            .field("is_valid", &self.is_valid())
            .finish()
    }
}

impl BackendTexture {
    pub fn new_invalid() -> Self {
        Self::from_ptr(unsafe { sb::C_SkGraphiteBackendTexture_new() }).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sb::C_SkGraphiteBackendTexture_isValid(self.native()) }
    }

    pub fn backend(&self) -> BackendApi {
        unsafe { sb::C_SkGraphiteBackendTexture_backend(self.native()) }
    }

    pub fn dimensions(&self) -> ISize {
        let mut size = unsafe { std::mem::zeroed() };
        unsafe { sb::C_SkGraphiteBackendTexture_dimensions(self.native(), &mut size) };
        ISize::from_native_c(size)
    }
}
