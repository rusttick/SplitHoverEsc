#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tdr: Tdr,
    rdr: Rdr,
    sr: Sr,
    isr: Isr,
    ier: Ier,
    icr: Icr,
    gcr: Gcr,
    ccr: Ccr,
    brr: Brr,
    rdnr: Rdnr,
    nssr: Nssr,
    ecr: Ecr,
}
impl RegisterBlock {
    #[doc = "0x00 - TDR"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x04 - RDR"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x08 - SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - ISR"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x10 - IER"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - ICR"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x18 - GCR"]
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    #[doc = "0x1c - CCR"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x20 - BRR"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x24 - RDNR"]
    #[inline(always)]
    pub const fn rdnr(&self) -> &Rdnr {
        &self.rdnr
    }
    #[doc = "0x28 - NSSR"]
    #[inline(always)]
    pub const fn nssr(&self) -> &Nssr {
        &self.nssr
    }
    #[doc = "0x2c - ECR"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
}
#[doc = "TDR (rw) register accessor: TDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "TDR"]
pub mod tdr;
#[doc = "RDR (r) register accessor: RDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "RDR"]
pub mod rdr;
#[doc = "SR (r) register accessor: SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "SR"]
pub mod sr;
#[doc = "ISR (rw) register accessor: ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ISR"]
pub mod isr;
#[doc = "IER (rw) register accessor: IER\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IER"]
pub mod ier;
#[doc = "ICR (rw) register accessor: ICR\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "ICR"]
pub mod icr;
#[doc = "GCR (rw) register accessor: GCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`] module"]
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
#[doc = "GCR"]
pub mod gcr;
#[doc = "CCR (rw) register accessor: CCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "CCR"]
pub mod ccr;
#[doc = "BRR (rw) register accessor: BRR\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "BRR"]
pub mod brr;
#[doc = "RDNR (rw) register accessor: RDNR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdnr`] module"]
#[doc(alias = "RDNR")]
pub type Rdnr = crate::Reg<rdnr::RdnrSpec>;
#[doc = "RDNR"]
pub mod rdnr;
#[doc = "NSSR (rw) register accessor: NSSR\n\nYou can [`read`](crate::Reg::read) this register and get [`nssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nssr`] module"]
#[doc(alias = "NSSR")]
pub type Nssr = crate::Reg<nssr::NssrSpec>;
#[doc = "NSSR"]
pub mod nssr;
#[doc = "ECR (rw) register accessor: ECR\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "ECR"]
pub mod ecr;
