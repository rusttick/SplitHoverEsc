#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp4_csr: Comp4Csr,
    comp5_csr: Comp5Csr,
    comp1_csr: Comp1Csr,
    comp2_csr: Comp2Csr,
    _reserved4: [u8; 0x04],
    comp3_csr: Comp3Csr,
    comp_crv: CompCrv,
    comp4_poll: Comp4Poll,
    comp5_poll: Comp5Poll,
}
impl RegisterBlock {
    #[doc = "0x00 - COMP4_CSR"]
    #[inline(always)]
    pub const fn comp4_csr(&self) -> &Comp4Csr {
        &self.comp4_csr
    }
    #[doc = "0x04 - COMP5_CSR"]
    #[inline(always)]
    pub const fn comp5_csr(&self) -> &Comp5Csr {
        &self.comp5_csr
    }
    #[doc = "0x08 - COMP1_CSR"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &Comp1Csr {
        &self.comp1_csr
    }
    #[doc = "0x0c - COMP2_CSR"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &Comp2Csr {
        &self.comp2_csr
    }
    #[doc = "0x14 - COMP3_CSR"]
    #[inline(always)]
    pub const fn comp3_csr(&self) -> &Comp3Csr {
        &self.comp3_csr
    }
    #[doc = "0x18 - COMP_CRV"]
    #[inline(always)]
    pub const fn comp_crv(&self) -> &CompCrv {
        &self.comp_crv
    }
    #[doc = "0x1c - COMP4_POLL"]
    #[inline(always)]
    pub const fn comp4_poll(&self) -> &Comp4Poll {
        &self.comp4_poll
    }
    #[doc = "0x20 - COMP5_POLL"]
    #[inline(always)]
    pub const fn comp5_poll(&self) -> &Comp5Poll {
        &self.comp5_poll
    }
}
#[doc = "COMP4_CSR (rw) register accessor: COMP4_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp4_csr`] module"]
#[doc(alias = "COMP4_CSR")]
pub type Comp4Csr = crate::Reg<comp4_csr::Comp4CsrSpec>;
#[doc = "COMP4_CSR"]
pub mod comp4_csr;
#[doc = "COMP5_CSR (rw) register accessor: COMP5_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp5_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp5_csr`] module"]
#[doc(alias = "COMP5_CSR")]
pub type Comp5Csr = crate::Reg<comp5_csr::Comp5CsrSpec>;
#[doc = "COMP5_CSR"]
pub mod comp5_csr;
#[doc = "COMP1_CSR (rw) register accessor: COMP1_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`] module"]
#[doc(alias = "COMP1_CSR")]
pub type Comp1Csr = crate::Reg<comp1_csr::Comp1CsrSpec>;
#[doc = "COMP1_CSR"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: COMP2_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`] module"]
#[doc(alias = "COMP2_CSR")]
pub type Comp2Csr = crate::Reg<comp2_csr::Comp2CsrSpec>;
#[doc = "COMP2_CSR"]
pub mod comp2_csr;
#[doc = "COMP3_CSR (rw) register accessor: COMP3_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`comp3_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp3_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3_csr`] module"]
#[doc(alias = "COMP3_CSR")]
pub type Comp3Csr = crate::Reg<comp3_csr::Comp3CsrSpec>;
#[doc = "COMP3_CSR"]
pub mod comp3_csr;
#[doc = "COMP_CRV (rw) register accessor: COMP_CRV\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_crv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_crv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_crv`] module"]
#[doc(alias = "COMP_CRV")]
pub type CompCrv = crate::Reg<comp_crv::CompCrvSpec>;
#[doc = "COMP_CRV"]
pub mod comp_crv;
#[doc = "COMP4_POLL (rw) register accessor: COMP4_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp4_poll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp4_poll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp4_poll`] module"]
#[doc(alias = "COMP4_POLL")]
pub type Comp4Poll = crate::Reg<comp4_poll::Comp4PollSpec>;
#[doc = "COMP4_POLL"]
pub mod comp4_poll;
#[doc = "COMP5_POLL (rw) register accessor: COMP5_POLL\n\nYou can [`read`](crate::Reg::read) this register and get [`comp5_poll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp5_poll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp5_poll`] module"]
#[doc(alias = "COMP5_POLL")]
pub type Comp5Poll = crate::Reg<comp5_poll::Comp5PollSpec>;
#[doc = "COMP5_POLL"]
pub mod comp5_poll;
