#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp1_csr: Comp1Csr,
    _reserved1: [u8; 0x14],
    comp_crv: CompCrv,
    comp1_poll: Comp1Poll,
}
impl RegisterBlock {
    #[doc = "0x00 - COMP1_CSR"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &Comp1Csr {
        &self.comp1_csr
    }
    #[doc = "0x18 - COMP_CRV"]
    #[inline(always)]
    pub const fn comp_crv(&self) -> &CompCrv {
        &self.comp_crv
    }
    #[doc = "0x1c - COMP1_POLL"]
    #[inline(always)]
    pub const fn comp1_poll(&self) -> &Comp1Poll {
        &self.comp1_poll
    }
}
#[doc = "COMP1_CSR (rw) register accessor: COMP1_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`] module"]
#[doc(alias = "COMP1_CSR")]
pub type Comp1Csr = crate::Reg<comp1_csr::Comp1CsrSpec>;
#[doc = "COMP1_CSR"]
pub mod comp1_csr;
#[doc = "COMP_CRV (rw) register accessor: COMP_CRV\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_crv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_crv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_crv`] module"]
#[doc(alias = "COMP_CRV")]
pub type CompCrv = crate::Reg<comp_crv::CompCrvSpec>;
#[doc = "COMP_CRV"]
pub mod comp_crv;
#[doc = "COMP1_POLL (rw) register accessor: COMP1_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_poll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_poll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_poll`] module"]
#[doc(alias = "COMP1_POLL")]
pub type Comp1Poll = crate::Reg<comp1_poll::Comp1PollSpec>;
#[doc = "COMP1_POLL"]
pub mod comp1_poll;
