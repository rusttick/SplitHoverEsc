#[doc = "Advanced timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1 {
    ptr: *mut u8,
}
unsafe impl Send for Tim1 {}
unsafe impl Sync for Tim1 {}
impl Tim1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register 1"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(self) -> crate::common::Reg<regs::Ccmr1Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_output(self) -> crate::common::Reg<regs::Ccmr1Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr2_input(self) -> crate::common::Reg<regs::Ccmr2Input, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare mode register 2(output mode)"]
    #[inline(always)]
    pub const fn ccmr2_output(self) -> crate::common::Reg<regs::Ccmr2Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture/compare register 1"]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "capture/compare register 2"]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::common::Reg<regs::Ccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "capture/compare register 3"]
    #[inline(always)]
    pub const fn ccr3(self) -> crate::common::Reg<regs::Ccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "capture/compare register 4"]
    #[inline(always)]
    pub const fn ccr4(self) -> crate::common::Reg<regs::Ccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::Dmar, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "capture/compare mode register 3 (output mode)"]
    #[inline(always)]
    pub const fn ccmr3_output(self) -> crate::common::Reg<regs::Ccmr3Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "capture/compare register 5"]
    #[inline(always)]
    pub const fn ccr5(self) -> crate::common::Reg<regs::Ccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "PWM/DMA repeat enable register"]
    #[inline(always)]
    pub const fn pder(self) -> crate::common::Reg<regs::Pder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "pwm shift count CCR1 register"]
    #[inline(always)]
    pub const fn ccr1fall(self) -> crate::common::Reg<regs::Ccr1fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "pwm shift count CCR2 register"]
    #[inline(always)]
    pub const fn ccr2fall(self) -> crate::common::Reg<regs::Ccr2fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "pwm shift count CCR3 register"]
    #[inline(always)]
    pub const fn ccr3fall(self) -> crate::common::Reg<regs::Ccr3fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "pwm shift count CCR4 register"]
    #[inline(always)]
    pub const fn ccr4fall(self) -> crate::common::Reg<regs::Ccr4fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "pwm shift count CCR5 register"]
    #[inline(always)]
    pub const fn ccr5fall(self) -> crate::common::Reg<regs::Ccr5fall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
}
pub mod regs;
