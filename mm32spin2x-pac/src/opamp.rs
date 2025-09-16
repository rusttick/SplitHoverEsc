#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp_csr: OpampCsr,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP_CSR"]
    #[inline(always)]
    pub const fn opamp_csr(&self) -> &OpampCsr {
        &self.opamp_csr
    }
}
#[doc = "OPAMP_CSR (rw) register accessor: OPAMP_CSR\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp_csr`] module"]
#[doc(alias = "OPAMP_CSR")]
pub type OpampCsr = crate::Reg<opamp_csr::OpampCsrSpec>;
#[doc = "OPAMP_CSR"]
pub mod opamp_csr;
