#[doc = "Real time clock"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC configuration high register"]
    #[inline(always)]
    pub const fn rtc_crh(self) -> crate::common::Reg<regs::RtcCrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RTC configuration low register"]
    #[inline(always)]
    pub const fn rtc_crl(self) -> crate::common::Reg<regs::RtcCrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RTC Prescaler load high register"]
    #[inline(always)]
    pub const fn rtc_prlh(self) -> crate::common::Reg<regs::RtcPrlh, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RTC Prescaler load low register"]
    #[inline(always)]
    pub const fn rtc_prll(self) -> crate::common::Reg<regs::RtcPrll, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RTC prescaler divider factor high register"]
    #[inline(always)]
    pub const fn rtc_divh(self) -> crate::common::Reg<regs::RtcDivh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RTC prescaler divider factor low register"]
    #[inline(always)]
    pub const fn rtc_divl(self) -> crate::common::Reg<regs::RtcDivl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RTC counter high register"]
    #[inline(always)]
    pub const fn rtc_cnth(self) -> crate::common::Reg<regs::RtcCnth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RTC counter low register"]
    #[inline(always)]
    pub const fn rtc_cntl(self) -> crate::common::Reg<regs::RtcCntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RTC alarm high register"]
    #[inline(always)]
    pub const fn rtc_alrh(self) -> crate::common::Reg<regs::RtcAlrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RTC alarm low register"]
    #[inline(always)]
    pub const fn rtc_alrl(self) -> crate::common::Reg<regs::RtcAlrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RTC millisecond alarm high register"]
    #[inline(always)]
    pub const fn rtc_msrh(self) -> crate::common::Reg<regs::RtcMsrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RTC millisecond alarm low register"]
    #[inline(always)]
    pub const fn rtc_msrl(self) -> crate::common::Reg<regs::RtcMsrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs;
