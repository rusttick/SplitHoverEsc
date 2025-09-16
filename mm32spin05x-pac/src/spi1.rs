#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txreg: Txreg,
    rxreg: Rxreg,
    cstat: Cstat,
    intstat: Intstat,
    inten: Inten,
    intclr: Intclr,
    gctl: Gctl,
    cctl: Cctl,
    spbrg: Spbrg,
    rxdnr: Rxdnr,
    nssr: Nssr,
    extctl: Extctl,
}
impl RegisterBlock {
    #[doc = "0x00 - TXREG"]
    #[inline(always)]
    pub const fn txreg(&self) -> &Txreg {
        &self.txreg
    }
    #[doc = "0x04 - RXREG"]
    #[inline(always)]
    pub const fn rxreg(&self) -> &Rxreg {
        &self.rxreg
    }
    #[doc = "0x08 - CSTAT"]
    #[inline(always)]
    pub const fn cstat(&self) -> &Cstat {
        &self.cstat
    }
    #[doc = "0x0c - INTSTAT"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x10 - INTEN"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x14 - INTCLR"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x18 - GCTL"]
    #[inline(always)]
    pub const fn gctl(&self) -> &Gctl {
        &self.gctl
    }
    #[doc = "0x1c - CCTL"]
    #[inline(always)]
    pub const fn cctl(&self) -> &Cctl {
        &self.cctl
    }
    #[doc = "0x20 - SPBRG"]
    #[inline(always)]
    pub const fn spbrg(&self) -> &Spbrg {
        &self.spbrg
    }
    #[doc = "0x24 - RXDNR"]
    #[inline(always)]
    pub const fn rxdnr(&self) -> &Rxdnr {
        &self.rxdnr
    }
    #[doc = "0x28 - NSSR"]
    #[inline(always)]
    pub const fn nssr(&self) -> &Nssr {
        &self.nssr
    }
    #[doc = "0x2c - EXTCTL"]
    #[inline(always)]
    pub const fn extctl(&self) -> &Extctl {
        &self.extctl
    }
}
#[doc = "TXREG (rw) register accessor: TXREG\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txreg`] module"]
#[doc(alias = "TXREG")]
pub type Txreg = crate::Reg<txreg::TxregSpec>;
#[doc = "TXREG"]
pub mod txreg;
#[doc = "RXREG (r) register accessor: RXREG\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxreg`] module"]
#[doc(alias = "RXREG")]
pub type Rxreg = crate::Reg<rxreg::RxregSpec>;
#[doc = "RXREG"]
pub mod rxreg;
#[doc = "CSTAT (r) register accessor: CSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`cstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstat`] module"]
#[doc(alias = "CSTAT")]
pub type Cstat = crate::Reg<cstat::CstatSpec>;
#[doc = "CSTAT"]
pub mod cstat;
#[doc = "INTSTAT (rw) register accessor: INTSTAT\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "INTSTAT"]
pub mod intstat;
#[doc = "INTEN (rw) register accessor: INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "INTEN"]
pub mod inten;
#[doc = "INTCLR (rw) register accessor: INTCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`] module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "INTCLR"]
pub mod intclr;
#[doc = "GCTL (rw) register accessor: GCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctl`] module"]
#[doc(alias = "GCTL")]
pub type Gctl = crate::Reg<gctl::GctlSpec>;
#[doc = "GCTL"]
pub mod gctl;
#[doc = "CCTL (rw) register accessor: CCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`cctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cctl`] module"]
#[doc(alias = "CCTL")]
pub type Cctl = crate::Reg<cctl::CctlSpec>;
#[doc = "CCTL"]
pub mod cctl;
#[doc = "SPBRG (rw) register accessor: SPBRG\n\nYou can [`read`](crate::Reg::read) this register and get [`spbrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spbrg`] module"]
#[doc(alias = "SPBRG")]
pub type Spbrg = crate::Reg<spbrg::SpbrgSpec>;
#[doc = "SPBRG"]
pub mod spbrg;
#[doc = "RXDNR (rw) register accessor: RXDNR\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdnr`] module"]
#[doc(alias = "RXDNR")]
pub type Rxdnr = crate::Reg<rxdnr::RxdnrSpec>;
#[doc = "RXDNR"]
pub mod rxdnr;
#[doc = "NSSR (rw) register accessor: NSSR\n\nYou can [`read`](crate::Reg::read) this register and get [`nssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nssr`] module"]
#[doc(alias = "NSSR")]
pub type Nssr = crate::Reg<nssr::NssrSpec>;
#[doc = "NSSR"]
pub mod nssr;
#[doc = "EXTCTL (rw) register accessor: EXTCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`extctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extctl`] module"]
#[doc(alias = "EXTCTL")]
pub type Extctl = crate::Reg<extctl::ExtctlSpec>;
#[doc = "EXTCTL"]
pub mod extctl;
