#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    comp1_csr: Comp1Csr,
    comp2_csr: Comp2Csr,
    _reserved2: [u8; 0x10],
    comp_crv: CompCrv,
    comp1_poll: Comp1Poll,
    comp2_poll: Comp2Poll,
}
impl RegisterBlock {
    #[doc = "0x00 - COMP1 Control State Register"]
    #[inline(always)]
    pub const fn comp1_csr(&self) -> &Comp1Csr {
        &self.comp1_csr
    }
    #[doc = "0x04 - COMP2 Control State Register"]
    #[inline(always)]
    pub const fn comp2_csr(&self) -> &Comp2Csr {
        &self.comp2_csr
    }
    #[doc = "0x18 - COMP Extern Reference Voltage"]
    #[inline(always)]
    pub const fn comp_crv(&self) -> &CompCrv {
        &self.comp_crv
    }
    #[doc = "0x1c - COMP1 Polling Output Register"]
    #[inline(always)]
    pub const fn comp1_poll(&self) -> &Comp1Poll {
        &self.comp1_poll
    }
    #[doc = "0x20 - COMP2 Polling Output Register"]
    #[inline(always)]
    pub const fn comp2_poll(&self) -> &Comp2Poll {
        &self.comp2_poll
    }
}
#[doc = "COMP1_CSR (rw) register accessor: COMP1 Control State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_csr`] module"]
#[doc(alias = "COMP1_CSR")]
pub type Comp1Csr = crate::Reg<comp1_csr::Comp1CsrSpec>;
#[doc = "COMP1 Control State Register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: COMP2 Control State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_csr`] module"]
#[doc(alias = "COMP2_CSR")]
pub type Comp2Csr = crate::Reg<comp2_csr::Comp2CsrSpec>;
#[doc = "COMP2 Control State Register"]
pub mod comp2_csr;
#[doc = "COMP_CRV (rw) register accessor: COMP Extern Reference Voltage\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_crv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_crv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_crv`] module"]
#[doc(alias = "COMP_CRV")]
pub type CompCrv = crate::Reg<comp_crv::CompCrvSpec>;
#[doc = "COMP Extern Reference Voltage"]
pub mod comp_crv;
#[doc = "COMP1_POLL (rw) register accessor: COMP1 Polling Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_poll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_poll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_poll`] module"]
#[doc(alias = "COMP1_POLL")]
pub type Comp1Poll = crate::Reg<comp1_poll::Comp1PollSpec>;
#[doc = "COMP1 Polling Output Register"]
pub mod comp1_poll;
#[doc = "COMP2_POLL (rw) register accessor: COMP2 Polling Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_poll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_poll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_poll`] module"]
#[doc(alias = "COMP2_POLL")]
pub type Comp2Poll = crate::Reg<comp2_poll::Comp2PollSpec>;
#[doc = "COMP2 Polling Output Register"]
pub mod comp2_poll;
