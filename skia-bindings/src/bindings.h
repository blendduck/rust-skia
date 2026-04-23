#ifndef SKIA_BINDINGS_BINDINGS_H
#define SKIA_BINDINGS_BINDINGS_H

#include <vector>
#include <optional>
#include "include/core/SkRefCnt.h"
#include "include/core/SkFont.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkPoint.h"
#include "include/core/SkRect.h"
#include "include/core/SkString.h"
#include "include/pathops/SkPathOps.h"
#include "include/utils/SkParsePath.h"

template<typename T>
inline sk_sp<T> spFromConst(const T* pt) {
    return sk_sp<T>(const_cast<T*>(pt));
}

template<typename T>
inline sk_sp<T> sp(T* pt) {
    return sk_sp<T>(pt);
}

template<typename T>
inline std::optional<T> opt(const T* pt) {
    return pt ? std::optional<T>(*pt) : std::nullopt;
}

extern "C" struct TraitObject {
    void* data;
    void* vtable;
};

/// A VecSink is passed from Rust to C++ for receiving a slice of values.
template<typename T> struct VecSink {
    TraitObject fn_trait;
    void (*set_fn)(T *, size_t, TraitObject);

    void set(T* ptr, size_t len) {
        set_fn(ptr, len, fn_trait);
    }

    void set(std::vector<T>& v) {
        if (v.empty()) {
            set_fn(nullptr, 0, fn_trait);
        } else {
            set_fn(v.data(), v.size(), fn_trait);
        }
    }
};

template<typename T> struct Sink {
    TraitObject fn_trait;
    void (*set_fn)(const T *, TraitObject);

    void set(const T& value) {
        set_fn(&value, fn_trait);
    }
};

struct SkStrings {
    std::vector<SkString> strings;
};

extern "C" bool C_SkPathOp_Op(
        const SkPath* one, const SkPath* two, SkPathOp op, SkPath* result);
extern "C" bool C_SkPathOp_Simplify(const SkPath* path, SkPath* result);
extern "C" bool C_SkPathOp_TightBounds(const SkPath* path, SkRect* result);
extern "C" bool C_SkPathOp_AsWinding(const SkPath* path, SkPath* result);
extern "C" bool C_SkOpBuilder_resolve2(SkOpBuilder* self, SkPath* result);
extern "C" bool C_SkParsePath_FromSVGString(const char* str, SkPath* result);
extern "C" bool C_SkFont_getPath(const SkFont* self, SkGlyphID glyph_id, SkPath* path);
extern "C" bool C_SkPath_getLastPt2(const SkPath* self, SkPoint* point);
extern "C" bool C_SkPathBuilder_getLastPt2(const SkPathBuilder* self, SkPoint* point);
extern "C" size_t C_SkPath_readFromMemory(SkPath* self, const void* buffer, size_t length);
extern "C" void C_SkPath_dumpArrays(const SkPath* self, void* stream, bool dump_as_hex);
extern "C" bool C_SkMatrix_Rect2Rect(
        const SkRect* src, const SkRect* dst, SkMatrix::ScaleToFit stf, SkMatrix* result);
extern "C" bool C_SkMatrix_PolyToPoly(
        const SkPoint* src, const SkPoint* dst, int count, SkMatrix* result);
extern "C" void C_SkMatrix_mapXY(
        const SkMatrix* self, SkScalar x, SkScalar y, SkPoint* point);

#endif //SKIA_BINDINGS_BINDINGS_H
