#[doc = "Direct Momory Accessuart"]
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
    #[doc = "DMA interrupt status register"]
    #[inline(always)]
    pub const fn dma_isr(self) -> crate::common::Reg<regs::DmaIsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA interrupt flag clear reigster"]
    #[inline(always)]
    pub const fn dma_ifcr(self) -> crate::common::Reg<regs::DmaIfcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr1(self) -> crate::common::Reg<regs::DmaCcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr1(self) -> crate::common::Reg<regs::DmaCndtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar1(self) -> crate::common::Reg<regs::DmaCpar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar1(self) -> crate::common::Reg<regs::DmaCmar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs;
