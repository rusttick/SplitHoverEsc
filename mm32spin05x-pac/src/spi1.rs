#[doc = "Serial peripheral interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi1 {
    ptr: *mut u8,
}
unsafe impl Send for Spi1 {}
unsafe impl Sync for Spi1 {}
impl Spi1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TXREG"]
    #[inline(always)]
    pub const fn txreg(self) -> crate::common::Reg<regs::Txreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RXREG"]
    #[inline(always)]
    pub const fn rxreg(self) -> crate::common::Reg<regs::Rxreg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CSTAT"]
    #[inline(always)]
    pub const fn cstat(self) -> crate::common::Reg<regs::Cstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "INTSTAT"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "INTEN"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "INTCLR"]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GCTL"]
    #[inline(always)]
    pub const fn gctl(self) -> crate::common::Reg<regs::Gctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CCTL"]
    #[inline(always)]
    pub const fn cctl(self) -> crate::common::Reg<regs::Cctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPBRG"]
    #[inline(always)]
    pub const fn spbrg(self) -> crate::common::Reg<regs::Spbrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RXDNR"]
    #[inline(always)]
    pub const fn rxdnr(self) -> crate::common::Reg<regs::Rxdnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "NSSR"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "EXTCTL"]
    #[inline(always)]
    pub const fn extctl(self) -> crate::common::Reg<regs::Extctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs;
