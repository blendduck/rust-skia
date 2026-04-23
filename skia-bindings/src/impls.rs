//! This file contains implementations for types that are re-exported in skia-safe.
//!
//! We could provide trait implementations in skia-safe, but then users of the library would have to
//! import the implementation type _and_ the trait.
//!
//! See also: <https://github.com/rust-lang/rfcs/issues/1880>

use crate::{
    SkAlphaType, SkBlendMode, SkBlendModeCoeff, SkPath, SkPathFillType, SkPathVerb, SkPath_Verb,
    SkYUVColorSpace,
};
use std::ffi::CStr;

impl SkBlendMode {
    pub fn as_coeff(self) -> Option<(SkBlendModeCoeff, SkBlendModeCoeff)> {
        let mut src = SkBlendModeCoeff::Zero;
        let mut dst = SkBlendModeCoeff::Zero;
        if unsafe { crate::SkBlendMode_AsCoeff(self, &mut src, &mut dst) } {
            Some((src, dst))
        } else {
            None
        }
    }

    pub fn name(self) -> &'static str {
        unsafe {
            let name_ptr = crate::SkBlendMode_Name(self);
            CStr::from_ptr(name_ptr).to_str().unwrap()
        }
    }
}

//
// m84 introduced two different variants of the Path verb types.
// One with Done and one without.
//

impl SkPathVerb {
    /// The maximum number of points an iterator will return for the verb.
    pub const MAX_POINTS: usize = SkPath_Verb::MAX_POINTS;
    /// The number of points an iterator will return for the verb.
    pub fn points(self) -> usize {
        SkPath_Verb::from(self).points()
    }
}

impl SkPath_Verb {
    /// The maximum number of points an iterator will return for the verb.
    pub const MAX_POINTS: usize = 4;
    /// The number of points an iterator will return for the verb.
    pub fn points(self) -> usize {
        match self {
            SkPath_Verb::Move => 1,
            SkPath_Verb::Line => 2,
            SkPath_Verb::Quad => 3,
            SkPath_Verb::Conic => 3,
            SkPath_Verb::Cubic => 4,
            SkPath_Verb::Close => 0,
            SkPath_Verb::Done => 0,
        }
    }
}

impl From<SkPathVerb> for SkPath_Verb {
    fn from(v: SkPathVerb) -> Self {
        match v {
            SkPathVerb::Move => SkPath_Verb::Move,
            SkPathVerb::Line => SkPath_Verb::Line,
            SkPathVerb::Quad => SkPath_Verb::Quad,
            SkPathVerb::Conic => SkPath_Verb::Conic,
            SkPathVerb::Cubic => SkPath_Verb::Cubic,
            SkPathVerb::Close => SkPath_Verb::Close,
        }
    }
}

impl SkPathFillType {
    pub fn is_even_odd(self) -> bool {
        (self as i32 & 1) != 0
    }

    pub fn is_inverse(self) -> bool {
        (self as i32 & 2) != 0
    }

    #[must_use]
    pub fn to_non_inverse(self) -> Self {
        use SkPathFillType::*;
        match self {
            Winding => self,
            EvenOdd => self,
            InverseWinding => Winding,
            InverseEvenOdd => EvenOdd,
        }
    }
}

impl SkPath {
    pub fn fFillType(&self) -> i32 {
        self.fFillType as i32
    }

    pub fn set_fFillType(&mut self, fill_type: i32) {
        self.fFillType = match fill_type {
            0 => SkPathFillType::Winding,
            1 => SkPathFillType::EvenOdd,
            2 => SkPathFillType::InverseWinding,
            3 => SkPathFillType::InverseEvenOdd,
            _ => self.fFillType,
        };
    }

    pub fn fIsVolatile(&self) -> i32 {
        self.fIsVolatile as i32
    }

    pub fn set_fIsVolatile(&mut self, is_volatile: i32) {
        self.fIsVolatile = is_volatile != 0;
    }

    pub unsafe fn countPoints(&self) -> i32 {
        self.points().fSize as i32
    }

    pub unsafe fn countVerbs(&self) -> i32 {
        self.verbs().fSize as i32
    }

    pub unsafe fn incReserve(
        &mut self,
        _extra_pt_count: ::core::ffi::c_int,
        _extra_verb_count: ::core::ffi::c_int,
        _extra_conic_count: ::core::ffi::c_int,
    ) {
    }

    pub unsafe fn isArc(&self, arc: *mut crate::SkArc) -> bool {
        crate::C_SkPath_isArc(self, arc)
    }

    pub unsafe fn rewind(&mut self) -> *mut SkPath {
        crate::SkPath_reset(self)
    }

    pub unsafe fn moveTo(&mut self, x: crate::SkScalar, y: crate::SkScalar) -> *mut SkPath {
        crate::C_SkPath_moveTo(self, x, y)
    }

    pub unsafe fn rMoveTo(&mut self, dx: crate::SkScalar, dy: crate::SkScalar) -> *mut SkPath {
        crate::C_SkPath_rMoveTo(self, dx, dy)
    }

    pub unsafe fn lineTo(&mut self, x: crate::SkScalar, y: crate::SkScalar) -> *mut SkPath {
        crate::C_SkPath_lineTo(self, x, y)
    }

    pub unsafe fn rLineTo(&mut self, dx: crate::SkScalar, dy: crate::SkScalar) -> *mut SkPath {
        crate::C_SkPath_rLineTo(self, dx, dy)
    }

    pub unsafe fn quadTo(
        &mut self,
        x1: crate::SkScalar,
        y1: crate::SkScalar,
        x2: crate::SkScalar,
        y2: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_quadTo(self, x1, y1, x2, y2)
    }

    pub unsafe fn rQuadTo(
        &mut self,
        dx1: crate::SkScalar,
        dy1: crate::SkScalar,
        dx2: crate::SkScalar,
        dy2: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_rQuadTo(self, dx1, dy1, dx2, dy2)
    }

    pub unsafe fn conicTo(
        &mut self,
        x1: crate::SkScalar,
        y1: crate::SkScalar,
        x2: crate::SkScalar,
        y2: crate::SkScalar,
        w: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_conicTo(self, x1, y1, x2, y2, w)
    }

    pub unsafe fn rConicTo(
        &mut self,
        dx1: crate::SkScalar,
        dy1: crate::SkScalar,
        dx2: crate::SkScalar,
        dy2: crate::SkScalar,
        w: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_rConicTo(self, dx1, dy1, dx2, dy2, w)
    }

    pub unsafe fn cubicTo(
        &mut self,
        x1: crate::SkScalar,
        y1: crate::SkScalar,
        x2: crate::SkScalar,
        y2: crate::SkScalar,
        x3: crate::SkScalar,
        y3: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_cubicTo(self, x1, y1, x2, y2, x3, y3)
    }

    pub unsafe fn rCubicTo(
        &mut self,
        dx1: crate::SkScalar,
        dy1: crate::SkScalar,
        dx2: crate::SkScalar,
        dy2: crate::SkScalar,
        dx3: crate::SkScalar,
        dy3: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_rCubicTo(self, dx1, dy1, dx2, dy2, dx3, dy3)
    }

    pub unsafe fn arcTo(
        &mut self,
        oval: *const crate::SkRect,
        start_angle_deg: crate::SkScalar,
        sweep_angle_deg: crate::SkScalar,
        force_move_to: bool,
    ) -> *mut SkPath {
        crate::C_SkPath_arcTo(self, oval, start_angle_deg, sweep_angle_deg, force_move_to)
    }

    pub unsafe fn arcTo1(
        &mut self,
        x1: crate::SkScalar,
        y1: crate::SkScalar,
        x2: crate::SkScalar,
        y2: crate::SkScalar,
        radius: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_arcTo1(self, x1, y1, x2, y2, radius)
    }

    pub unsafe fn arcTo2(
        &mut self,
        r: crate::SkPoint,
        x_axis_rotate: crate::SkScalar,
        large_arc: crate::SkPathBuilder_ArcSize,
        sweep: crate::SkPathDirection,
        xy: crate::SkPoint,
    ) -> *mut SkPath {
        crate::C_SkPath_arcTo2(self, r, x_axis_rotate, large_arc, sweep, xy)
    }

    pub unsafe fn rArcTo(
        &mut self,
        r: crate::SkPoint,
        x_axis_rotate: crate::SkScalar,
        large_arc: crate::SkPathBuilder_ArcSize,
        sweep: crate::SkPathDirection,
        xy: crate::SkPoint,
    ) -> *mut SkPath {
        crate::C_SkPath_rArcTo(self, r, x_axis_rotate, large_arc, sweep, xy)
    }

    pub unsafe fn close(&mut self) -> *mut SkPath {
        crate::C_SkPath_close(self)
    }

    pub unsafe fn addCircle(
        &mut self,
        x: crate::SkScalar,
        y: crate::SkScalar,
        radius: crate::SkScalar,
        dir: crate::SkPathDirection,
    ) -> *mut SkPath {
        crate::C_SkPath_addCircle(self, x, y, radius, dir)
    }

    pub unsafe fn addRect(
        &mut self,
        rect: *const crate::SkRect,
        dir: crate::SkPathDirection,
        start_index: ::core::ffi::c_uint,
    ) -> *mut SkPath {
        crate::C_SkPath_addRect(self, rect, dir, start_index)
    }

    pub unsafe fn addOval1(
        &mut self,
        rect: *const crate::SkRect,
        dir: crate::SkPathDirection,
        start_index: ::core::ffi::c_uint,
    ) -> *mut SkPath {
        crate::C_SkPath_addOval1(self, rect, dir, start_index)
    }

    pub unsafe fn addArc(
        &mut self,
        oval: *const crate::SkRect,
        start_angle_deg: crate::SkScalar,
        sweep_angle_deg: crate::SkScalar,
    ) -> *mut SkPath {
        crate::C_SkPath_addArc(self, oval, start_angle_deg, sweep_angle_deg)
    }

    pub unsafe fn addRoundRect(
        &mut self,
        rect: *const crate::SkRect,
        rx: crate::SkScalar,
        ry: crate::SkScalar,
        dir: crate::SkPathDirection,
    ) -> *mut SkPath {
        crate::C_SkPath_addRoundRect(self, rect, rx, ry, dir)
    }

    pub unsafe fn addRRect1(
        &mut self,
        rrect: *const crate::SkRRect,
        dir: crate::SkPathDirection,
        start_index: ::core::ffi::c_uint,
    ) -> *mut SkPath {
        crate::C_SkPath_addRRect1(self, rrect, dir, start_index)
    }

    pub unsafe fn addPoly(
        &mut self,
        pts: *const crate::SkPoint,
        count: ::core::ffi::c_int,
        close: bool,
    ) -> *mut SkPath {
        crate::C_SkPath_addPoly(self, pts, count, close)
    }

    pub unsafe fn addPath(
        &mut self,
        src: *const SkPath,
        dx: crate::SkScalar,
        dy: crate::SkScalar,
        mode: crate::SkPath_AddPathMode,
    ) -> *mut SkPath {
        crate::C_SkPath_addPathOffset(self, src, dx, dy, mode)
    }

    pub unsafe fn addPath1(
        &mut self,
        src: *const SkPath,
        matrix: *const crate::SkMatrix,
        mode: crate::SkPath_AddPathMode,
    ) -> *mut SkPath {
        crate::C_SkPath_addPath1(self, src, matrix, mode)
    }

    pub unsafe fn reverseAddPath(&mut self, src: *const SkPath) -> *mut SkPath {
        crate::C_SkPath_reverseAddPath(self, src)
    }

    pub unsafe fn offset(&self, dx: crate::SkScalar, dy: crate::SkScalar, dst: *mut SkPath) {
        crate::C_SkPath_offset(self, dx, dy, dst)
    }

    pub unsafe fn transform(
        &self,
        matrix: *const crate::SkMatrix,
        dst: *mut SkPath,
        _perspective_clip: crate::SkApplyPerspectiveClip,
    ) {
        crate::C_SkPath_transform(self, matrix, dst)
    }

    pub unsafe fn setLastPt(&mut self, x: crate::SkScalar, y: crate::SkScalar) -> *mut SkPath {
        crate::C_SkPath_setLastPt(self, x, y)
    }

    pub unsafe fn dumpArrays(&self, stream: *mut crate::SkWStream, dump_as_hex: bool) {
        crate::C_SkPath_dumpArrays(self, stream as *mut _, dump_as_hex)
    }

    pub unsafe fn readFromMemory(
        &mut self,
        buffer: *const ::core::ffi::c_void,
        length: usize,
    ) -> usize {
        crate::C_SkPath_readFromMemory(self, buffer, length)
    }
}

impl SkAlphaType {
    pub fn is_opaque(self) -> bool {
        self == SkAlphaType::Opaque
    }
}

impl SkYUVColorSpace {
    pub fn is_limited_range(self) -> bool {
        unsafe { crate::SkYUVColorSpaceIsLimitedRange(self) }
    }
}

#[cfg(feature = "gl")]
impl From<crate::GrGLenum> for crate::GrGLFormat {
    fn from(e: crate::GrGLenum) -> Self {
        unsafe { crate::C_GrGLFormatFromGLEnum(e) }
    }
}

#[cfg(feature = "gl")]
impl From<crate::GrGLFormat> for crate::GrGLenum {
    fn from(format: crate::GrGLFormat) -> Self {
        unsafe { crate::C_GrGLFormatToEnum(format) }
    }
}

#[cfg(feature = "vulkan")]
mod vulkan {
    impl PartialEq for crate::VkComponentMapping {
        fn eq(&self, other: &Self) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
        }
    }

    impl Eq for crate::VkComponentMapping {}
}

#[cfg(feature = "d3d")]
mod d3d {
    use std::marker::PhantomData;

    impl<T> Default for crate::gr_cp<T> {
        fn default() -> Self {
            Self {
                fObject: std::ptr::null_mut(),
                _phantom_0: PhantomData,
            }
        }
    }

    impl Default for crate::GrD3DTextureResourceInfo {
        fn default() -> Self {
            let mut instance = std::mem::MaybeUninit::uninit();
            unsafe {
                crate::C_GrD3DTextureResourceInfo_Construct(instance.as_mut_ptr());
                instance.assume_init()
            }
        }
    }
}
