#[doc = "PWM Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control PWM output status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Auto phase mask register"]
    #[inline(always)]
    pub const fn apmskr(self) -> crate::common::Reg<regs::Apmskr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Auto phase mask dalay register"]
    #[inline(always)]
    pub const fn apmdlr(self) -> crate::common::Reg<regs::Apmdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
