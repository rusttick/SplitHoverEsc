#[doc = "Debug support"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csm {
    ptr: *mut u8,
}
unsafe impl Send for Csm {}
unsafe impl Sync for Csm {}
impl Csm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Transmit register 1"]
    #[inline(always)]
    pub const fn txreg1(self) -> crate::common::Reg<regs::Txreg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Transmit register 2"]
    #[inline(always)]
    pub const fn txreg2(self) -> crate::common::Reg<regs::Txreg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Reveive register 1"]
    #[inline(always)]
    pub const fn rxreg1(self) -> crate::common::Reg<regs::Rxreg1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Reveive register 2"]
    #[inline(always)]
    pub const fn rxreg2(self) -> crate::common::Reg<regs::Rxreg2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control Register 1"]
    #[inline(always)]
    pub const fn ctl1(self) -> crate::common::Reg<regs::Ctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Control Register 2"]
    #[inline(always)]
    pub const fn ctl2(self) -> crate::common::Reg<regs::Ctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configure Register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn spbrg(self) -> crate::common::Reg<regs::Spbrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "bit count"]
    #[inline(always)]
    pub const fn bcnt(self) -> crate::common::Reg<regs::Bcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs;
