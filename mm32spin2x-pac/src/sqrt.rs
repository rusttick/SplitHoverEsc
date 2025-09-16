#[doc = "Hardware Square"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sqrt {
    ptr: *mut u8,
}
unsafe impl Send for Sqrt {}
unsafe impl Sync for Sqrt {}
impl Sqrt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SQR"]
    #[inline(always)]
    pub const fn sqr(self) -> crate::common::Reg<regs::Sqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RESULT"]
    #[inline(always)]
    pub const fn result(self) -> crate::common::Reg<regs::Result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs;
