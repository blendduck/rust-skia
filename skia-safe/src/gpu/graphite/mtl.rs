use std::fmt;

use skia_bindings as sb;

use super::BackendTexture;
use crate::prelude::*;

pub type BackendContext = RefHandle<sb::skgpu_graphite_MtlBackendContext>;

impl NativeDrop for sb::skgpu_graphite_MtlBackendContext {
    fn drop(&mut self) {
        unsafe { sb::C_SkGraphiteMtlBackendContext_delete(self) }
    }
}

impl fmt::Debug for BackendContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("graphite::mtl::BackendContext").finish()
    }
}

impl BackendContext {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(device: sb::GrMTLHandle, queue: sb::GrMTLHandle) -> Self {
        Self::from_ptr(sb::C_SkGraphiteMtlBackendContext_new(device, queue)).unwrap()
    }
}

pub mod contexts {
    use skia_bindings as sb;

    use super::BackendContext;
    use crate::gpu::graphite::Context;
    use crate::prelude::*;

    pub fn make_metal(backend: &BackendContext) -> Option<Context> {
        Context::from_ptr(unsafe { sb::C_SkGraphiteContext_MakeMetal(backend.native()) })
    }
}

pub mod backend_textures {
    use skia_bindings as sb;

    use super::BackendTexture;
    use crate::{prelude::*, ISize};

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn make_metal(size: impl Into<ISize>, texture: sb::GrMTLHandle) -> BackendTexture {
        let size = size.into();
        BackendTexture::from_ptr(sb::C_SkGraphiteBackendTextures_MakeMetal(
            size.width,
            size.height,
            texture,
        ))
        .unwrap()
    }

    pub fn get_mtl_texture(texture: &BackendTexture) -> sb::GrMTLHandle {
        unsafe { sb::C_SkGraphiteBackendTextures_GetMtlTexture(texture.native()) as _ }
    }
}
