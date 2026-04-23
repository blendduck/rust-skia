#include "bindings.h"

#include "include/core/SkColorSpace.h"
#include "include/core/SkSize.h"
#include "include/core/SkSurface.h"
#include "include/gpu/graphite/BackendTexture.h"
#include "include/gpu/graphite/Context.h"
#include "include/gpu/graphite/Recorder.h"
#include "include/gpu/graphite/Recording.h"
#include "include/gpu/graphite/Surface.h"

extern "C" skgpu::graphite::Recorder* C_SkGraphiteContext_makeRecorder(
        skgpu::graphite::Context* self) {
    return self->makeRecorder().release();
}

extern "C" int C_SkGraphiteContext_insertRecording(
        skgpu::graphite::Context* self,
        skgpu::graphite::Recording* recording) {
    skgpu::graphite::InsertRecordingInfo info;
    info.fRecording = recording;
    return static_cast<int>(static_cast<skgpu::graphite::InsertStatus::V>(
            self->insertRecording(info)));
}

extern "C" bool C_SkGraphiteContext_submit(
        skgpu::graphite::Context* self,
        bool sync_to_cpu) {
    return self->submit(sync_to_cpu ? skgpu::graphite::SyncToCpu::kYes
                                    : skgpu::graphite::SyncToCpu::kNo);
}

extern "C" void C_SkGraphiteContext_delete(skgpu::graphite::Context* self) {
    delete self;
}

extern "C" skgpu::graphite::Recording* C_SkGraphiteRecorder_snap(
        skgpu::graphite::Recorder* self) {
    return self->snap().release();
}

extern "C" void C_SkGraphiteRecorder_delete(skgpu::graphite::Recorder* self) {
    delete self;
}

extern "C" void C_SkGraphiteRecording_delete(skgpu::graphite::Recording* self) {
    delete self;
}

extern "C" skgpu::graphite::BackendTexture* C_SkGraphiteBackendTexture_new() {
    return new skgpu::graphite::BackendTexture();
}

extern "C" skgpu::graphite::BackendTexture* C_SkGraphiteBackendTexture_Clone(
        const skgpu::graphite::BackendTexture* self) {
    return new skgpu::graphite::BackendTexture(*self);
}

extern "C" void C_SkGraphiteBackendTexture_delete(
        const skgpu::graphite::BackendTexture* self) {
    delete const_cast<skgpu::graphite::BackendTexture*>(self);
}

extern "C" bool C_SkGraphiteBackendTexture_isValid(
        const skgpu::graphite::BackendTexture* self) {
    return self->isValid();
}

extern "C" skgpu::BackendApi C_SkGraphiteBackendTexture_backend(
        const skgpu::graphite::BackendTexture* self) {
    return self->backend();
}

extern "C" void C_SkGraphiteBackendTexture_dimensions(
        const skgpu::graphite::BackendTexture* self,
        SkISize* dimensions) {
    *dimensions = self->dimensions();
}

extern "C" SkSurface* C_SkSurfaces_WrapGraphiteBackendTexture(
        skgpu::graphite::Recorder* recorder,
        const skgpu::graphite::BackendTexture* backend_texture,
        SkColorSpace* color_space,
        const SkSurfaceProps* surface_props) {
    return SkSurfaces::WrapBackendTexture(
                   recorder,
                   *backend_texture,
                   sp(color_space),
                   surface_props)
            .release();
}
