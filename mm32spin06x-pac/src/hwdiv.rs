#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dvdr: Dvdr,
    dvsr: Dvsr,
    quotr: Quotr,
    rmdr: Rmdr,
    sr: Sr,
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x00 - Dividend register"]
    #[inline(always)]
    pub const fn dvdr(&self) -> &Dvdr {
        &self.dvdr
    }
    #[doc = "0x04 - Divisor register"]
    #[inline(always)]
    pub const fn dvsr(&self) -> &Dvsr {
        &self.dvsr
    }
    #[doc = "0x08 - Quotient register"]
    #[inline(always)]
    pub const fn quotr(&self) -> &Quotr {
        &self.quotr
    }
    #[doc = "0x0c - Remainder register"]
    #[inline(always)]
    pub const fn rmdr(&self) -> &Rmdr {
        &self.rmdr
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "DVDR (rw) register accessor: Dividend register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvdr`] module"]
#[doc(alias = "DVDR")]
pub type Dvdr = crate::Reg<dvdr::DvdrSpec>;
#[doc = "Dividend register"]
pub mod dvdr;
#[doc = "DVSR (rw) register accessor: Divisor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvsr`] module"]
#[doc(alias = "DVSR")]
pub type Dvsr = crate::Reg<dvsr::DvsrSpec>;
#[doc = "Divisor register"]
pub mod dvsr;
#[doc = "QUOTR (r) register accessor: Quotient register\n\nYou can [`read`](crate::Reg::read) this register and get [`quotr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quotr`] module"]
#[doc(alias = "QUOTR")]
pub type Quotr = crate::Reg<quotr::QuotrSpec>;
#[doc = "Quotient register"]
pub mod quotr;
#[doc = "RMDR (r) register accessor: Remainder register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmdr`] module"]
#[doc(alias = "RMDR")]
pub type Rmdr = crate::Reg<rmdr::RmdrSpec>;
#[doc = "Remainder register"]
pub mod rmdr;
#[doc = "SR (r) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "control register"]
pub mod cr;
