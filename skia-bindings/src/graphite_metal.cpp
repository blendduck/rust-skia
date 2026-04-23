#ifndef SK_METAL
    #define SK_METAL
#endif

#include "bindings.h"

#include "include/gpu/graphite/ContextOptions.h"
#include "include/gpu/graphite/mtl/MtlBackendContext.h"
#include "include/gpu/graphite/mtl/MtlGraphiteTypes_cpp.h"

extern "C" skgpu::graphite::MtlBackendContext* C_SkGraphiteMtlBackendContext_new(
        const void* device,
        const void* queue) {
    auto* context = new skgpu::graphite::MtlBackendContext();
    context->fDevice.retain((CFTypeRef)device);
    context->fQueue.retain((CFTypeRef)queue);
    return context;
}

extern "C" void C_SkGraphiteMtlBackendContext_delete(
        skgpu::graphite::MtlBackendContext* self) {
    delete self;
}

extern "C" skgpu::graphite::Context* C_SkGraphiteContext_MakeMetal(
        const skgpu::graphite::MtlBackendContext* context) {
    skgpu::graphite::ContextOptions options;
    return skgpu::graphite::ContextFactory::MakeMetal(*context, options).release();
}

extern "C" skgpu::graphite::BackendTexture* C_SkGraphiteBackendTextures_MakeMetal(
        int width,
        int height,
        const void* texture) {
    return new skgpu::graphite::BackendTexture(
            skgpu::graphite::BackendTextures::MakeMetal({width, height}, (CFTypeRef)texture));
}

extern "C" const void* C_SkGraphiteBackendTextures_GetMtlTexture(
        const skgpu::graphite::BackendTexture* texture) {
    return skgpu::graphite::BackendTextures::GetMtlTexture(*texture);
}
