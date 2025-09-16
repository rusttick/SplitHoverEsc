#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    apmskr: Apmskr,
    apmdlr: Apmdlr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control PWM output status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Auto phase mask register"]
    #[inline(always)]
    pub const fn apmskr(&self) -> &Apmskr {
        &self.apmskr
    }
    #[doc = "0x08 - Auto phase mask dalay register"]
    #[inline(always)]
    pub const fn apmdlr(&self) -> &Apmdlr {
        &self.apmdlr
    }
}
#[doc = "CSR (rw) register accessor: Control PWM output status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control PWM output status register"]
pub mod csr;
#[doc = "APMSKR (rw) register accessor: Auto phase mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`apmskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apmskr`] module"]
#[doc(alias = "APMSKR")]
pub type Apmskr = crate::Reg<apmskr::ApmskrSpec>;
#[doc = "Auto phase mask register"]
pub mod apmskr;
#[doc = "APMDLR (rw) register accessor: Auto phase mask dalay register\n\nYou can [`read`](crate::Reg::read) this register and get [`apmdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apmdlr`] module"]
#[doc(alias = "APMDLR")]
pub type Apmdlr = crate::Reg<apmdlr::ApmdlrSpec>;
#[doc = "Auto phase mask dalay register"]
pub mod apmdlr;
