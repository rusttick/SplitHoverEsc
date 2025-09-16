#[doc = "Analog to digital converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
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
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
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
    pub const fn ch0dr(self) -> crate::common::Reg<regs::Ch0dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel 1 data register"]
    #[inline(always)]
    pub const fn ch1dr(self) -> crate::common::Reg<regs::Ch1dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel 2 data register"]
    #[inline(always)]
    pub const fn ch2dr(self) -> crate::common::Reg<regs::Ch2dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel 3 data register"]
    #[inline(always)]
    pub const fn ch3dr(self) -> crate::common::Reg<regs::Ch3dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel 4 data register"]
    #[inline(always)]
    pub const fn ch4dr(self) -> crate::common::Reg<regs::Ch4dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel 5 data register"]
    #[inline(always)]
    pub const fn ch5dr(self) -> crate::common::Reg<regs::Ch5dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel 6 data register"]
    #[inline(always)]
    pub const fn ch6dr(self) -> crate::common::Reg<regs::Ch6dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel 7 data register"]
    #[inline(always)]
    pub const fn ch7dr(self) -> crate::common::Reg<regs::Ch7dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel 8 data register"]
    #[inline(always)]
    pub const fn ch8dr(self) -> crate::common::Reg<regs::Ch8dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel 9 data register"]
    #[inline(always)]
    pub const fn ch9dr(self) -> crate::common::Reg<regs::Ch9dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Channel 10 data register"]
    #[inline(always)]
    pub const fn ch10dr(self) -> crate::common::Reg<regs::Ch10dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Channel 11 data register"]
    #[inline(always)]
    pub const fn ch11dr(self) -> crate::common::Reg<regs::Ch11dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel 12 data register"]
    #[inline(always)]
    pub const fn ch12dr(self) -> crate::common::Reg<regs::Ch12dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Channel 14 data register"]
    #[inline(always)]
    pub const fn ch14dr(self) -> crate::common::Reg<regs::Ch14dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Channel 15 data register"]
    #[inline(always)]
    pub const fn ch15dr(self) -> crate::common::Reg<regs::Ch15dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Extended status register"]
    #[inline(always)]
    pub const fn sta_ext(self) -> crate::common::Reg<regs::StaExt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
pub mod regs;
