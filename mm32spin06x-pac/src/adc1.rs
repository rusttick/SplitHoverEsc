#[doc = "Analog to digital converter 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1 {
    ptr: *mut u8,
}
unsafe impl Send for Adc1 {}
unsafe impl Sync for Adc1 {}
impl Adc1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Configure register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel select register"]
    #[inline(always)]
    pub const fn chsr(self) -> crate::common::Reg<regs::Chsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Compare register"]
    #[inline(always)]
    pub const fn cmpr(self) -> crate::common::Reg<regs::Cmpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel 0 data register"]
    #[inline(always)]
    pub const fn dr0(self) -> crate::common::Reg<regs::Dr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel 1 data register"]
    #[inline(always)]
    pub const fn dr1(self) -> crate::common::Reg<regs::Dr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel 2 data register"]
    #[inline(always)]
    pub const fn dr2(self) -> crate::common::Reg<regs::Dr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel 3 data register"]
    #[inline(always)]
    pub const fn dr3(self) -> crate::common::Reg<regs::Dr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel 4 data register"]
    #[inline(always)]
    pub const fn dr4(self) -> crate::common::Reg<regs::Dr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel 5 data register"]
    #[inline(always)]
    pub const fn dr5(self) -> crate::common::Reg<regs::Dr5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel 6 data register"]
    #[inline(always)]
    pub const fn dr6(self) -> crate::common::Reg<regs::Dr6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel 7 data register"]
    #[inline(always)]
    pub const fn dr7(self) -> crate::common::Reg<regs::Dr7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel 8 data register"]
    #[inline(always)]
    pub const fn dr8(self) -> crate::common::Reg<regs::Dr8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel 9 data register"]
    #[inline(always)]
    pub const fn dr9(self) -> crate::common::Reg<regs::Dr9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Channel 14 data register"]
    #[inline(always)]
    pub const fn dr14(self) -> crate::common::Reg<regs::Dr14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Channel 15 data register"]
    #[inline(always)]
    pub const fn dr15(self) -> crate::common::Reg<regs::Dr15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Extended status register"]
    #[inline(always)]
    pub const fn sta_ext(self) -> crate::common::Reg<regs::StaExt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Arbitrary channel channel selection register 0"]
    #[inline(always)]
    pub const fn chany0(self) -> crate::common::Reg<regs::Chany0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Arbitrary channel channel selection register 1"]
    #[inline(always)]
    pub const fn chany1(self) -> crate::common::Reg<regs::Chany1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Arbitrary channel configuration register"]
    #[inline(always)]
    pub const fn any_cfg(self) -> crate::common::Reg<regs::AnyCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Arbitrary channel control register"]
    #[inline(always)]
    pub const fn any_cr(self) -> crate::common::Reg<regs::AnyCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
}
pub mod regs;
