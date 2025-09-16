#[doc = "Backup registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkp {
    ptr: *mut u8,
}
unsafe impl Send for Bkp {}
unsafe impl Sync for Bkp {}
impl Bkp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RTC clock calibration register"]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Backup control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "BKP control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr1(self) -> crate::common::Reg<regs::Dr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr2(self) -> crate::common::Reg<regs::Dr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr3(self) -> crate::common::Reg<regs::Dr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr4(self) -> crate::common::Reg<regs::Dr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr5(self) -> crate::common::Reg<regs::Dr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr6(self) -> crate::common::Reg<regs::Dr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr7(self) -> crate::common::Reg<regs::Dr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr8(self) -> crate::common::Reg<regs::Dr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr9(self) -> crate::common::Reg<regs::Dr9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr10(self) -> crate::common::Reg<regs::Dr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr11(self) -> crate::common::Reg<regs::Dr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr12(self) -> crate::common::Reg<regs::Dr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr13(self) -> crate::common::Reg<regs::Dr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr14(self) -> crate::common::Reg<regs::Dr14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr15(self) -> crate::common::Reg<regs::Dr15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr16(self) -> crate::common::Reg<regs::Dr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr17(self) -> crate::common::Reg<regs::Dr17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr18(self) -> crate::common::Reg<regs::Dr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr19(self) -> crate::common::Reg<regs::Dr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr20(self) -> crate::common::Reg<regs::Dr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
