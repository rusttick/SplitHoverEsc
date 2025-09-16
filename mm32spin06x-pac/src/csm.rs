#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txreg1: Txreg1,
    txreg2: Txreg2,
    rxreg1: Rxreg1,
    rxreg2: Rxreg2,
    intstat: Intstat,
    inten: Inten,
    ctl1: Ctl1,
    ctl2: Ctl2,
    cfg: Cfg,
    spbrg: Spbrg,
    bcnt: Bcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit register 1"]
    #[inline(always)]
    pub const fn txreg1(&self) -> &Txreg1 {
        &self.txreg1
    }
    #[doc = "0x04 - Transmit register 2"]
    #[inline(always)]
    pub const fn txreg2(&self) -> &Txreg2 {
        &self.txreg2
    }
    #[doc = "0x08 - Reveive register 1"]
    #[inline(always)]
    pub const fn rxreg1(&self) -> &Rxreg1 {
        &self.rxreg1
    }
    #[doc = "0x0c - Reveive register 2"]
    #[inline(always)]
    pub const fn rxreg2(&self) -> &Rxreg2 {
        &self.rxreg2
    }
    #[doc = "0x10 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x14 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x18 - Control Register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x1c - Control Register 2"]
    #[inline(always)]
    pub const fn ctl2(&self) -> &Ctl2 {
        &self.ctl2
    }
    #[doc = "0x20 - Configure Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x24 - Baud rate register"]
    #[inline(always)]
    pub const fn spbrg(&self) -> &Spbrg {
        &self.spbrg
    }
    #[doc = "0x28 - bit count"]
    #[inline(always)]
    pub const fn bcnt(&self) -> &Bcnt {
        &self.bcnt
    }
}
#[doc = "TXREG1 (rw) register accessor: Transmit register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txreg1`] module"]
#[doc(alias = "TXREG1")]
pub type Txreg1 = crate::Reg<txreg1::Txreg1Spec>;
#[doc = "Transmit register 1"]
pub mod txreg1;
#[doc = "TXREG2 (rw) register accessor: Transmit register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`txreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txreg2`] module"]
#[doc(alias = "TXREG2")]
pub type Txreg2 = crate::Reg<txreg2::Txreg2Spec>;
#[doc = "Transmit register 2"]
pub mod txreg2;
#[doc = "RXREG1 (r) register accessor: Reveive register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxreg1`] module"]
#[doc(alias = "RXREG1")]
pub type Rxreg1 = crate::Reg<rxreg1::Rxreg1Spec>;
#[doc = "Reveive register 1"]
pub mod rxreg1;
#[doc = "RXREG2 (r) register accessor: Reveive register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rxreg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxreg2`] module"]
#[doc(alias = "RXREG2")]
pub type Rxreg2 = crate::Reg<rxreg2::Rxreg2Spec>;
#[doc = "Reveive register 2"]
pub mod rxreg2;
#[doc = "INTSTAT (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt Status Register"]
pub mod intstat;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "CTL1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control Register 1"]
pub mod ctl1;
#[doc = "CTL2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl2`] module"]
#[doc(alias = "CTL2")]
pub type Ctl2 = crate::Reg<ctl2::Ctl2Spec>;
#[doc = "Control Register 2"]
pub mod ctl2;
#[doc = "CFG (rw) register accessor: Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "SPBRG (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`spbrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spbrg`] module"]
#[doc(alias = "SPBRG")]
pub type Spbrg = crate::Reg<spbrg::SpbrgSpec>;
#[doc = "Baud rate register"]
pub mod spbrg;
#[doc = "BCNT (rw) register accessor: bit count\n\nYou can [`read`](crate::Reg::read) this register and get [`bcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcnt`] module"]
#[doc(alias = "BCNT")]
pub type Bcnt = crate::Reg<bcnt::BcntSpec>;
#[doc = "bit count"]
pub mod bcnt;
