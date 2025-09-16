#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ifcr: Ifcr,
    ccr1: Ccr1,
    cndtr1: Cndtr1,
    cpar1: Cpar1,
    cmar1: Cmar1,
    _reserved6: [u8; 0x04],
    ccr2: Ccr2,
    cndtr2: Cndtr2,
    cpar2: Cpar2,
    cmar2: Cmar2,
    _reserved10: [u8; 0x04],
    ccr3: Ccr3,
    cndtr3: Cndtr3,
    cpar3: Cpar3,
    cmar3: Cmar3,
    _reserved14: [u8; 0x04],
    ccr4: Ccr4,
    cndtr4: Cndtr4,
    cpar4: Cpar4,
    cmar4: Cmar4,
    _reserved18: [u8; 0x04],
    ccr5: Ccr5,
    cndtr5: Cndtr5,
    cpar5: Cpar5,
    cmar5: Cmar5,
}
impl RegisterBlock {
    #[doc = "0x00 - ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - IFCR"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x08 - CCR1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x0c - CNDTR1"]
    #[inline(always)]
    pub const fn cndtr1(&self) -> &Cndtr1 {
        &self.cndtr1
    }
    #[doc = "0x10 - CPAR1"]
    #[inline(always)]
    pub const fn cpar1(&self) -> &Cpar1 {
        &self.cpar1
    }
    #[doc = "0x14 - CMAR1"]
    #[inline(always)]
    pub const fn cmar1(&self) -> &Cmar1 {
        &self.cmar1
    }
    #[doc = "0x1c - CCR2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x20 - CNDTR2"]
    #[inline(always)]
    pub const fn cndtr2(&self) -> &Cndtr2 {
        &self.cndtr2
    }
    #[doc = "0x24 - CPAR2"]
    #[inline(always)]
    pub const fn cpar2(&self) -> &Cpar2 {
        &self.cpar2
    }
    #[doc = "0x28 - CMAR2"]
    #[inline(always)]
    pub const fn cmar2(&self) -> &Cmar2 {
        &self.cmar2
    }
    #[doc = "0x30 - CCR3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x34 - CNDTR3"]
    #[inline(always)]
    pub const fn cndtr3(&self) -> &Cndtr3 {
        &self.cndtr3
    }
    #[doc = "0x38 - CPAR3"]
    #[inline(always)]
    pub const fn cpar3(&self) -> &Cpar3 {
        &self.cpar3
    }
    #[doc = "0x3c - CMAR3"]
    #[inline(always)]
    pub const fn cmar3(&self) -> &Cmar3 {
        &self.cmar3
    }
    #[doc = "0x44 - CCR4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x48 - CNDTR4"]
    #[inline(always)]
    pub const fn cndtr4(&self) -> &Cndtr4 {
        &self.cndtr4
    }
    #[doc = "0x4c - CPAR4"]
    #[inline(always)]
    pub const fn cpar4(&self) -> &Cpar4 {
        &self.cpar4
    }
    #[doc = "0x50 - CMAR4"]
    #[inline(always)]
    pub const fn cmar4(&self) -> &Cmar4 {
        &self.cmar4
    }
    #[doc = "0x58 - CCR5"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    #[doc = "0x5c - CNDTR5"]
    #[inline(always)]
    pub const fn cndtr5(&self) -> &Cndtr5 {
        &self.cndtr5
    }
    #[doc = "0x60 - CPAR5"]
    #[inline(always)]
    pub const fn cpar5(&self) -> &Cpar5 {
        &self.cpar5
    }
    #[doc = "0x64 - CMAR5"]
    #[inline(always)]
    pub const fn cmar5(&self) -> &Cmar5 {
        &self.cmar5
    }
}
#[doc = "ISR (r) register accessor: ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ISR"]
pub mod isr;
#[doc = "IFCR (rw) register accessor: IFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "IFCR"]
pub mod ifcr;
#[doc = "CCR1 (rw) register accessor: CCR1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "CCR1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: CCR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "CCR2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: CCR3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "CCR3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: CCR4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "CCR4"]
pub mod ccr4;
#[doc = "CCR5 (rw) register accessor: CCR5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`] module"]
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::Ccr5Spec>;
#[doc = "CCR5"]
pub mod ccr5;
#[doc = "CNDTR1 (rw) register accessor: CNDTR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr1`] module"]
#[doc(alias = "CNDTR1")]
pub type Cndtr1 = crate::Reg<cndtr1::Cndtr1Spec>;
#[doc = "CNDTR1"]
pub mod cndtr1;
#[doc = "CNDTR2 (rw) register accessor: CNDTR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr2`] module"]
#[doc(alias = "CNDTR2")]
pub type Cndtr2 = crate::Reg<cndtr2::Cndtr2Spec>;
#[doc = "CNDTR2"]
pub mod cndtr2;
#[doc = "CNDTR3 (rw) register accessor: CNDTR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr3`] module"]
#[doc(alias = "CNDTR3")]
pub type Cndtr3 = crate::Reg<cndtr3::Cndtr3Spec>;
#[doc = "CNDTR3"]
pub mod cndtr3;
#[doc = "CNDTR4 (rw) register accessor: CNDTR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr4`] module"]
#[doc(alias = "CNDTR4")]
pub type Cndtr4 = crate::Reg<cndtr4::Cndtr4Spec>;
#[doc = "CNDTR4"]
pub mod cndtr4;
#[doc = "CNDTR5 (rw) register accessor: CNDTR5\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cndtr5`] module"]
#[doc(alias = "CNDTR5")]
pub type Cndtr5 = crate::Reg<cndtr5::Cndtr5Spec>;
#[doc = "CNDTR5"]
pub mod cndtr5;
#[doc = "CPAR1 (rw) register accessor: CPAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar1`] module"]
#[doc(alias = "CPAR1")]
pub type Cpar1 = crate::Reg<cpar1::Cpar1Spec>;
#[doc = "CPAR1"]
pub mod cpar1;
#[doc = "CPAR2 (rw) register accessor: CPAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar2`] module"]
#[doc(alias = "CPAR2")]
pub type Cpar2 = crate::Reg<cpar2::Cpar2Spec>;
#[doc = "CPAR2"]
pub mod cpar2;
#[doc = "CPAR3 (rw) register accessor: CPAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar3`] module"]
#[doc(alias = "CPAR3")]
pub type Cpar3 = crate::Reg<cpar3::Cpar3Spec>;
#[doc = "CPAR3"]
pub mod cpar3;
#[doc = "CPAR4 (rw) register accessor: CPAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar4`] module"]
#[doc(alias = "CPAR4")]
pub type Cpar4 = crate::Reg<cpar4::Cpar4Spec>;
#[doc = "CPAR4"]
pub mod cpar4;
#[doc = "CPAR5 (rw) register accessor: CPAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpar5`] module"]
#[doc(alias = "CPAR5")]
pub type Cpar5 = crate::Reg<cpar5::Cpar5Spec>;
#[doc = "CPAR5"]
pub mod cpar5;
#[doc = "CMAR1 (rw) register accessor: CMAR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar1`] module"]
#[doc(alias = "CMAR1")]
pub type Cmar1 = crate::Reg<cmar1::Cmar1Spec>;
#[doc = "CMAR1"]
pub mod cmar1;
#[doc = "CMAR2 (rw) register accessor: CMAR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar2`] module"]
#[doc(alias = "CMAR2")]
pub type Cmar2 = crate::Reg<cmar2::Cmar2Spec>;
#[doc = "CMAR2"]
pub mod cmar2;
#[doc = "CMAR3 (rw) register accessor: CMAR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar3`] module"]
#[doc(alias = "CMAR3")]
pub type Cmar3 = crate::Reg<cmar3::Cmar3Spec>;
#[doc = "CMAR3"]
pub mod cmar3;
#[doc = "CMAR4 (rw) register accessor: CMAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar4`] module"]
#[doc(alias = "CMAR4")]
pub type Cmar4 = crate::Reg<cmar4::Cmar4Spec>;
#[doc = "CMAR4"]
pub mod cmar4;
#[doc = "CMAR5 (rw) register accessor: CMAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmar5`] module"]
#[doc(alias = "CMAR5")]
pub type Cmar5 = crate::Reg<cmar5::Cmar5Spec>;
#[doc = "CMAR5"]
pub mod cmar5;
