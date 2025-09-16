#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_isr: DmaIsr,
    dma_ifcr: DmaIfcr,
    dma_ccr1: DmaCcr1,
    dma_cndtr1: DmaCndtr1,
    dma_cpar1: DmaCpar1,
    dma_cmar1: DmaCmar1,
    _reserved6: [u8; 0x04],
    dma_ccr2: DmaCcr2,
    dma_cndtr2: DmaCndtr2,
    dma_cpar2: DmaCpar2,
    dma_cmar2: DmaCmar2,
    _reserved10: [u8; 0x04],
    dma_ccr3: DmaCcr3,
    dma_cndtr3: DmaCndtr3,
    dma_cpar3: DmaCpar3,
    dma_cmar3: DmaCmar3,
    _reserved14: [u8; 0x04],
    dma_ccr4: DmaCcr4,
    dma_cndtr4: DmaCndtr4,
    dma_cpar4: DmaCpar4,
    dma_cmar4: DmaCmar4,
    _reserved18: [u8; 0x04],
    dma_ccr5: DmaCcr5,
    dma_cndtr5: DmaCndtr5,
    dma_cpar5: DmaCpar5,
    dma_cmar5: DmaCmar5,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    #[inline(always)]
    pub const fn dma_isr(&self) -> &DmaIsr {
        &self.dma_isr
    }
    #[doc = "0x04 - DMA interrupt flag clear reigster"]
    #[inline(always)]
    pub const fn dma_ifcr(&self) -> &DmaIfcr {
        &self.dma_ifcr
    }
    #[doc = "0x08 - DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr1(&self) -> &DmaCcr1 {
        &self.dma_ccr1
    }
    #[doc = "0x0c - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr1(&self) -> &DmaCndtr1 {
        &self.dma_cndtr1
    }
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar1(&self) -> &DmaCpar1 {
        &self.dma_cpar1
    }
    #[doc = "0x14 - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar1(&self) -> &DmaCmar1 {
        &self.dma_cmar1
    }
    #[doc = "0x1c - DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr2(&self) -> &DmaCcr2 {
        &self.dma_ccr2
    }
    #[doc = "0x20 - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr2(&self) -> &DmaCndtr2 {
        &self.dma_cndtr2
    }
    #[doc = "0x24 - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar2(&self) -> &DmaCpar2 {
        &self.dma_cpar2
    }
    #[doc = "0x28 - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar2(&self) -> &DmaCmar2 {
        &self.dma_cmar2
    }
    #[doc = "0x30 - DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr3(&self) -> &DmaCcr3 {
        &self.dma_ccr3
    }
    #[doc = "0x34 - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr3(&self) -> &DmaCndtr3 {
        &self.dma_cndtr3
    }
    #[doc = "0x38 - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar3(&self) -> &DmaCpar3 {
        &self.dma_cpar3
    }
    #[doc = "0x3c - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar3(&self) -> &DmaCmar3 {
        &self.dma_cmar3
    }
    #[doc = "0x44 - DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr4(&self) -> &DmaCcr4 {
        &self.dma_ccr4
    }
    #[doc = "0x48 - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr4(&self) -> &DmaCndtr4 {
        &self.dma_cndtr4
    }
    #[doc = "0x4c - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar4(&self) -> &DmaCpar4 {
        &self.dma_cpar4
    }
    #[doc = "0x50 - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar4(&self) -> &DmaCmar4 {
        &self.dma_cmar4
    }
    #[doc = "0x58 - DMA channel 1 configuration reigster"]
    #[inline(always)]
    pub const fn dma_ccr5(&self) -> &DmaCcr5 {
        &self.dma_ccr5
    }
    #[doc = "0x5c - DMA channel 1 number of data register"]
    #[inline(always)]
    pub const fn dma_cndtr5(&self) -> &DmaCndtr5 {
        &self.dma_cndtr5
    }
    #[doc = "0x60 - DMA channel 1 peripheral address register"]
    #[inline(always)]
    pub const fn dma_cpar5(&self) -> &DmaCpar5 {
        &self.dma_cpar5
    }
    #[doc = "0x64 - DMA channel 1 memory address register"]
    #[inline(always)]
    pub const fn dma_cmar5(&self) -> &DmaCmar5 {
        &self.dma_cmar5
    }
}
#[doc = "DMA_ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_isr`] module"]
#[doc(alias = "DMA_ISR")]
pub type DmaIsr = crate::Reg<dma_isr::DmaIsrSpec>;
#[doc = "DMA interrupt status register"]
pub mod dma_isr;
#[doc = "DMA_IFCR (w) register accessor: DMA interrupt flag clear reigster\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ifcr`] module"]
#[doc(alias = "DMA_IFCR")]
pub type DmaIfcr = crate::Reg<dma_ifcr::DmaIfcrSpec>;
#[doc = "DMA interrupt flag clear reigster"]
pub mod dma_ifcr;
#[doc = "DMA_CCR1 (rw) register accessor: DMA channel 1 configuration reigster\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_ccr1`] module"]
#[doc(alias = "DMA_CCR1")]
pub type DmaCcr1 = crate::Reg<dma_ccr1::DmaCcr1Spec>;
#[doc = "DMA channel 1 configuration reigster"]
pub mod dma_ccr1;
pub use DmaCcr1 as DmaCcr2;
pub use DmaCcr1 as DmaCcr3;
pub use DmaCcr1 as DmaCcr4;
pub use DmaCcr1 as DmaCcr5;
pub use dma_ccr1 as dma_ccr2;
pub use dma_ccr1 as dma_ccr3;
pub use dma_ccr1 as dma_ccr4;
pub use dma_ccr1 as dma_ccr5;
#[doc = "DMA_CNDTR1 (rw) register accessor: DMA channel 1 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cndtr1`] module"]
#[doc(alias = "DMA_CNDTR1")]
pub type DmaCndtr1 = crate::Reg<dma_cndtr1::DmaCndtr1Spec>;
#[doc = "DMA channel 1 number of data register"]
pub mod dma_cndtr1;
pub use DmaCndtr1 as DmaCndtr2;
pub use DmaCndtr1 as DmaCndtr3;
pub use DmaCndtr1 as DmaCndtr4;
pub use DmaCndtr1 as DmaCndtr5;
pub use dma_cndtr1 as dma_cndtr2;
pub use dma_cndtr1 as dma_cndtr3;
pub use dma_cndtr1 as dma_cndtr4;
pub use dma_cndtr1 as dma_cndtr5;
#[doc = "DMA_CPAR1 (rw) register accessor: DMA channel 1 peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cpar1`] module"]
#[doc(alias = "DMA_CPAR1")]
pub type DmaCpar1 = crate::Reg<dma_cpar1::DmaCpar1Spec>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod dma_cpar1;
pub use DmaCpar1 as DmaCpar2;
pub use DmaCpar1 as DmaCpar3;
pub use DmaCpar1 as DmaCpar4;
pub use DmaCpar1 as DmaCpar5;
pub use dma_cpar1 as dma_cpar2;
pub use dma_cpar1 as dma_cpar3;
pub use dma_cpar1 as dma_cpar4;
pub use dma_cpar1 as dma_cpar5;
#[doc = "DMA_CMAR1 (rw) register accessor: DMA channel 1 memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cmar1`] module"]
#[doc(alias = "DMA_CMAR1")]
pub type DmaCmar1 = crate::Reg<dma_cmar1::DmaCmar1Spec>;
#[doc = "DMA channel 1 memory address register"]
pub mod dma_cmar1;
pub use DmaCmar1 as DmaCmar2;
pub use DmaCmar1 as DmaCmar3;
pub use DmaCmar1 as DmaCmar4;
pub use DmaCmar1 as DmaCmar5;
pub use dma_cmar1 as dma_cmar2;
pub use dma_cmar1 as dma_cmar3;
pub use dma_cmar1 as dma_cmar4;
pub use dma_cmar1 as dma_cmar5;
