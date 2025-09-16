#[doc = "General purpose I/O"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpioa {
    ptr: *mut u8,
}
unsafe impl Send for Gpioa {}
unsafe impl Sync for Gpioa {}
impl Gpioa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "configuration low register"]
    #[inline(always)]
    pub const fn crl(self) -> crate::common::Reg<regs::Crl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "configuration high register"]
    #[inline(always)]
    pub const fn crh(self) -> crate::common::Reg<regs::Crh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "input data register"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "output data register"]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "bit set/reset register"]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "bit reset register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Port Multiplexing Function Low Register"]
    #[inline(always)]
    pub const fn afrl(self) -> crate::common::Reg<regs::Afrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Port Multiplexing Function High Register"]
    #[inline(always)]
    pub const fn afrh(self) -> crate::common::Reg<regs::Afrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs;
