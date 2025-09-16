#[doc = "Hardware divider"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div {
    ptr: *mut u8,
}
unsafe impl Send for Div {}
unsafe impl Sync for Div {}
impl Div {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DVDR"]
    #[inline(always)]
    pub const fn dvdr(self) -> crate::common::Reg<regs::Dvdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DVSR"]
    #[inline(always)]
    pub const fn dvsr(self) -> crate::common::Reg<regs::Dvsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "QUOTR"]
    #[inline(always)]
    pub const fn quotr(self) -> crate::common::Reg<regs::Quotr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RMDR"]
    #[inline(always)]
    pub const fn rmdr(self) -> crate::common::Reg<regs::Rmdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SR"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CR"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs;
