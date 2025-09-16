#[doc = "DMA1 controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1 {
    ptr: *mut u8,
}
unsafe impl Send for Dma1 {}
unsafe impl Sync for Dma1 {}
impl Dma1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ISR"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IFCR"]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CCR1"]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CNDTR1"]
    #[inline(always)]
    pub const fn cndtr1(self) -> crate::common::Reg<regs::Cndtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CPAR1"]
    #[inline(always)]
    pub const fn cpar1(self) -> crate::common::Reg<regs::Cpar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CMAR1"]
    #[inline(always)]
    pub const fn cmar1(self) -> crate::common::Reg<regs::Cmar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CCR2"]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::common::Reg<regs::Ccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CNDTR2"]
    #[inline(always)]
    pub const fn cndtr2(self) -> crate::common::Reg<regs::Cndtr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CPAR2"]
    #[inline(always)]
    pub const fn cpar2(self) -> crate::common::Reg<regs::Cpar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CMAR2"]
    #[inline(always)]
    pub const fn cmar2(self) -> crate::common::Reg<regs::Cmar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CCR3"]
    #[inline(always)]
    pub const fn ccr3(self) -> crate::common::Reg<regs::Ccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "CNDTR3"]
    #[inline(always)]
    pub const fn cndtr3(self) -> crate::common::Reg<regs::Cndtr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "CPAR3"]
    #[inline(always)]
    pub const fn cpar3(self) -> crate::common::Reg<regs::Cpar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "CMAR3"]
    #[inline(always)]
    pub const fn cmar3(self) -> crate::common::Reg<regs::Cmar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "CCR4"]
    #[inline(always)]
    pub const fn ccr4(self) -> crate::common::Reg<regs::Ccr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "CNDTR4"]
    #[inline(always)]
    pub const fn cndtr4(self) -> crate::common::Reg<regs::Cndtr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "CPAR4"]
    #[inline(always)]
    pub const fn cpar4(self) -> crate::common::Reg<regs::Cpar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "CMAR4"]
    #[inline(always)]
    pub const fn cmar4(self) -> crate::common::Reg<regs::Cmar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "CCR5"]
    #[inline(always)]
    pub const fn ccr5(self) -> crate::common::Reg<regs::Ccr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "CNDTR5"]
    #[inline(always)]
    pub const fn cndtr5(self) -> crate::common::Reg<regs::Cndtr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CPAR5"]
    #[inline(always)]
    pub const fn cpar5(self) -> crate::common::Reg<regs::Cpar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "CMAR5"]
    #[inline(always)]
    pub const fn cmar5(self) -> crate::common::Reg<regs::Cmar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
}
pub mod regs;
