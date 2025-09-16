#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    cfgr: Cfgr,
    cr: Cr,
    chsr: Chsr,
    cmpr: Cmpr,
    sr: Sr,
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    dr9: Dr9,
    _reserved16: [u8; 0x10],
    dr14: Dr14,
    dr15: Dr15,
    sta_ext: StaExt,
    chany0: Chany0,
    chany1: Chany1,
    any_cfg: AnyCfg,
    any_cr: AnyCr,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - Configure register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - Channel select register"]
    #[inline(always)]
    pub const fn chsr(&self) -> &Chsr {
        &self.chsr
    }
    #[doc = "0x10 - Compare register"]
    #[inline(always)]
    pub const fn cmpr(&self) -> &Cmpr {
        &self.cmpr
    }
    #[doc = "0x14 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x18 - Channel 0 data register"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x1c - Channel 1 data register"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x20 - Channel 2 data register"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x24 - Channel 3 data register"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x28 - Channel 4 data register"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x2c - Channel 5 data register"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x30 - Channel 6 data register"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x34 - Channel 7 data register"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x38 - Channel 8 data register"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0x3c - Channel 9 data register"]
    #[inline(always)]
    pub const fn dr9(&self) -> &Dr9 {
        &self.dr9
    }
    #[doc = "0x50 - Channel 14 data register"]
    #[inline(always)]
    pub const fn dr14(&self) -> &Dr14 {
        &self.dr14
    }
    #[doc = "0x54 - Channel 15 data register"]
    #[inline(always)]
    pub const fn dr15(&self) -> &Dr15 {
        &self.dr15
    }
    #[doc = "0x58 - Extended status register"]
    #[inline(always)]
    pub const fn sta_ext(&self) -> &StaExt {
        &self.sta_ext
    }
    #[doc = "0x5c - Arbitrary channel channel selection register 0"]
    #[inline(always)]
    pub const fn chany0(&self) -> &Chany0 {
        &self.chany0
    }
    #[doc = "0x60 - Arbitrary channel channel selection register 1"]
    #[inline(always)]
    pub const fn chany1(&self) -> &Chany1 {
        &self.chany1
    }
    #[doc = "0x64 - Arbitrary channel configuration register"]
    #[inline(always)]
    pub const fn any_cfg(&self) -> &AnyCfg {
        &self.any_cfg
    }
    #[doc = "0x68 - Arbitrary channel control register"]
    #[inline(always)]
    pub const fn any_cr(&self) -> &AnyCr {
        &self.any_cr
    }
}
#[doc = "DATA (r) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data register"]
pub mod data;
#[doc = "CFGR (rw) register accessor: Configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Configure register"]
pub mod cfgr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "CHSR (rw) register accessor: Channel select register\n\nYou can [`read`](crate::Reg::read) this register and get [`chsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsr`] module"]
#[doc(alias = "CHSR")]
pub type Chsr = crate::Reg<chsr::ChsrSpec>;
#[doc = "Channel select register"]
pub mod chsr;
#[doc = "CMPR (rw) register accessor: Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpr`] module"]
#[doc(alias = "CMPR")]
pub type Cmpr = crate::Reg<cmpr::CmprSpec>;
#[doc = "Compare register"]
pub mod cmpr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "DR0 (r) register accessor: Channel 0 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "Channel 0 data register"]
pub mod dr0;
#[doc = "DR1 (r) register accessor: Channel 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "Channel 1 data register"]
pub mod dr1;
#[doc = "DR2 (r) register accessor: Channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "Channel 2 data register"]
pub mod dr2;
#[doc = "DR3 (r) register accessor: Channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "Channel 3 data register"]
pub mod dr3;
#[doc = "DR4 (r) register accessor: Channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "Channel 4 data register"]
pub mod dr4;
#[doc = "DR5 (r) register accessor: Channel 5 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "Channel 5 data register"]
pub mod dr5;
#[doc = "DR6 (r) register accessor: Channel 6 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "Channel 6 data register"]
pub mod dr6;
#[doc = "DR7 (r) register accessor: Channel 7 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "Channel 7 data register"]
pub mod dr7;
#[doc = "DR8 (r) register accessor: Channel 8 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "Channel 8 data register"]
pub mod dr8;
#[doc = "DR9 (r) register accessor: Channel 9 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`] module"]
#[doc(alias = "DR9")]
pub type Dr9 = crate::Reg<dr9::Dr9Spec>;
#[doc = "Channel 9 data register"]
pub mod dr9;
#[doc = "DR14 (r) register accessor: Channel 14 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`] module"]
#[doc(alias = "DR14")]
pub type Dr14 = crate::Reg<dr14::Dr14Spec>;
#[doc = "Channel 14 data register"]
pub mod dr14;
#[doc = "DR15 (r) register accessor: Channel 15 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`] module"]
#[doc(alias = "DR15")]
pub type Dr15 = crate::Reg<dr15::Dr15Spec>;
#[doc = "Channel 15 data register"]
pub mod dr15;
#[doc = "STA_EXT (r) register accessor: Extended status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta_ext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta_ext`] module"]
#[doc(alias = "STA_EXT")]
pub type StaExt = crate::Reg<sta_ext::StaExtSpec>;
#[doc = "Extended status register"]
pub mod sta_ext;
#[doc = "CHANY0 (rw) register accessor: Arbitrary channel channel selection register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chany0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany0`] module"]
#[doc(alias = "CHANY0")]
pub type Chany0 = crate::Reg<chany0::Chany0Spec>;
#[doc = "Arbitrary channel channel selection register 0"]
pub mod chany0;
#[doc = "CHANY1 (rw) register accessor: Arbitrary channel channel selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chany1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany1`] module"]
#[doc(alias = "CHANY1")]
pub type Chany1 = crate::Reg<chany1::Chany1Spec>;
#[doc = "Arbitrary channel channel selection register 1"]
pub mod chany1;
#[doc = "ANY_CFG (rw) register accessor: Arbitrary channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cfg`] module"]
#[doc(alias = "ANY_CFG")]
pub type AnyCfg = crate::Reg<any_cfg::AnyCfgSpec>;
#[doc = "Arbitrary channel configuration register"]
pub mod any_cfg;
#[doc = "ANY_CR (rw) register accessor: Arbitrary channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cr`] module"]
#[doc(alias = "ANY_CR")]
pub type AnyCr = crate::Reg<any_cr::AnyCrSpec>;
#[doc = "Arbitrary channel control register"]
pub mod any_cr;
