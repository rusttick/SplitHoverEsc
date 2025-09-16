#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    keyr: Keyr,
    optkeyr: Optkeyr,
    sr: Sr,
    cr: Cr,
    ar: Ar,
    _reserved6: [u8; 0x04],
    obr: Obr,
    wrpr: Wrpr,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x04 - Flash key"]
    #[inline(always)]
    pub const fn keyr(&self) -> &Keyr {
        &self.keyr
    }
    #[doc = "0x08 - Option byte key"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x0c - Flash status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Flash control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x14 - Flash address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &Ar {
        &self.ar
    }
    #[doc = "0x1c - Option byte register"]
    #[inline(always)]
    pub const fn obr(&self) -> &Obr {
        &self.obr
    }
    #[doc = "0x20 - Write protect register"]
    #[inline(always)]
    pub const fn wrpr(&self) -> &Wrpr {
        &self.wrpr
    }
}
#[doc = "ACR (rw) register accessor: Flash access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "KEYR (w) register accessor: Flash key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`] module"]
#[doc(alias = "KEYR")]
pub type Keyr = crate::Reg<keyr::KeyrSpec>;
#[doc = "Flash key"]
pub mod keyr;
#[doc = "OPTKEYR (w) register accessor: Option byte key\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = "Option byte key"]
pub mod optkeyr;
#[doc = "SR (rw) register accessor: Flash status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Flash status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Flash control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Flash control register"]
pub mod cr;
#[doc = "AR (w) register accessor: Flash address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`] module"]
#[doc(alias = "AR")]
pub type Ar = crate::Reg<ar::ArSpec>;
#[doc = "Flash address register"]
pub mod ar;
#[doc = "OBR (r) register accessor: Option byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`obr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@obr`] module"]
#[doc(alias = "OBR")]
pub type Obr = crate::Reg<obr::ObrSpec>;
#[doc = "Option byte register"]
pub mod obr;
#[doc = "WRPR (r) register accessor: Write protect register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrpr`] module"]
#[doc(alias = "WRPR")]
pub type Wrpr = crate::Reg<wrpr::WrprSpec>;
#[doc = "Write protect register"]
pub mod wrpr;
