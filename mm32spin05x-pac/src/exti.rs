#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr: Cfgr,
    _reserved1: [u8; 0x04],
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    cr4: Cr4,
    _reserved5: [u8; 0x03e8],
    imr: Imr,
    emr: Emr,
    rtsr: Rtsr,
    ftsr: Ftsr,
    swier: Swier,
    pr: Pr,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - External interrupt configuration register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x0c - External interrupt configuration register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x10 - External interrupt configuration register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x14 - External interrupt configuration register 4"]
    #[inline(always)]
    pub const fn cr4(&self) -> &Cr4 {
        &self.cr4
    }
    #[doc = "0x400 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x404 - Event mask register"]
    #[inline(always)]
    pub const fn emr(&self) -> &Emr {
        &self.emr
    }
    #[doc = "0x408 - Rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(&self) -> &Rtsr {
        &self.rtsr
    }
    #[doc = "0x40c - Falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(&self) -> &Ftsr {
        &self.ftsr
    }
    #[doc = "0x410 - Software interrupt event register"]
    #[inline(always)]
    pub const fn swier(&self) -> &Swier {
        &self.swier
    }
    #[doc = "0x414 - Pending register"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
}
#[doc = "CFGR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Configuration register"]
pub mod cfgr;
#[doc = "CR1 (rw) register accessor: External interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "External interrupt configuration register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: External interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "External interrupt configuration register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: External interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "External interrupt configuration register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: External interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`] module"]
#[doc(alias = "CR4")]
pub type Cr4 = crate::Reg<cr4::Cr4Spec>;
#[doc = "External interrupt configuration register 4"]
pub mod cr4;
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "EMR (rw) register accessor: Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emr`] module"]
#[doc(alias = "EMR")]
pub type Emr = crate::Reg<emr::EmrSpec>;
#[doc = "Event mask register"]
pub mod emr;
#[doc = "RTSR (rw) register accessor: Rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr`] module"]
#[doc(alias = "RTSR")]
pub type Rtsr = crate::Reg<rtsr::RtsrSpec>;
#[doc = "Rising trigger selection register"]
pub mod rtsr;
#[doc = "FTSR (rw) register accessor: Falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr`] module"]
#[doc(alias = "FTSR")]
pub type Ftsr = crate::Reg<ftsr::FtsrSpec>;
#[doc = "Falling trigger selection register"]
pub mod ftsr;
#[doc = "SWIER (rw) register accessor: Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier`] module"]
#[doc(alias = "SWIER")]
pub type Swier = crate::Reg<swier::SwierSpec>;
#[doc = "Software interrupt event register"]
pub mod swier;
#[doc = "PR (rw) register accessor: Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Pending register"]
pub mod pr;
