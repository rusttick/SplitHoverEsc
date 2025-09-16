#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    cfgr: Cfgr,
    cr: Cr,
    chsr: Chsr,
    cmpr: Cmpr,
    sr: Sr,
    ch0dr: Ch0dr,
    ch1dr: Ch1dr,
    ch2dr: Ch2dr,
    ch3dr: Ch3dr,
    ch4dr: Ch4dr,
    ch5dr: Ch5dr,
    ch6dr: Ch6dr,
    ch7dr: Ch7dr,
    ch8dr: Ch8dr,
    ch9dr: Ch9dr,
    ch10dr: Ch10dr,
    ch11dr: Ch11dr,
    _reserved18: [u8; 0x08],
    ch14dr: Ch14dr,
    ch15dr: Ch15dr,
    sta_ext: StaExt,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
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
    pub const fn ch0dr(&self) -> &Ch0dr {
        &self.ch0dr
    }
    #[doc = "0x1c - Channel 1 data register"]
    #[inline(always)]
    pub const fn ch1dr(&self) -> &Ch1dr {
        &self.ch1dr
    }
    #[doc = "0x20 - Channel 2 data register"]
    #[inline(always)]
    pub const fn ch2dr(&self) -> &Ch2dr {
        &self.ch2dr
    }
    #[doc = "0x24 - Channel 3 data register"]
    #[inline(always)]
    pub const fn ch3dr(&self) -> &Ch3dr {
        &self.ch3dr
    }
    #[doc = "0x28 - Channel 4 data register"]
    #[inline(always)]
    pub const fn ch4dr(&self) -> &Ch4dr {
        &self.ch4dr
    }
    #[doc = "0x2c - Channel 5 data register"]
    #[inline(always)]
    pub const fn ch5dr(&self) -> &Ch5dr {
        &self.ch5dr
    }
    #[doc = "0x30 - Channel 6 data register"]
    #[inline(always)]
    pub const fn ch6dr(&self) -> &Ch6dr {
        &self.ch6dr
    }
    #[doc = "0x34 - Channel 7 data register"]
    #[inline(always)]
    pub const fn ch7dr(&self) -> &Ch7dr {
        &self.ch7dr
    }
    #[doc = "0x38 - Channel 8 data register"]
    #[inline(always)]
    pub const fn ch8dr(&self) -> &Ch8dr {
        &self.ch8dr
    }
    #[doc = "0x3c - Channel 9 data register"]
    #[inline(always)]
    pub const fn ch9dr(&self) -> &Ch9dr {
        &self.ch9dr
    }
    #[doc = "0x40 - Channel 10 data register"]
    #[inline(always)]
    pub const fn ch10dr(&self) -> &Ch10dr {
        &self.ch10dr
    }
    #[doc = "0x44 - Channel 11 data register"]
    #[inline(always)]
    pub const fn ch11dr(&self) -> &Ch11dr {
        &self.ch11dr
    }
    #[doc = "0x50 - Channel 14 data register"]
    #[inline(always)]
    pub const fn ch14dr(&self) -> &Ch14dr {
        &self.ch14dr
    }
    #[doc = "0x54 - Channel 15 data register"]
    #[inline(always)]
    pub const fn ch15dr(&self) -> &Ch15dr {
        &self.ch15dr
    }
    #[doc = "0x58 - Extended status register"]
    #[inline(always)]
    pub const fn sta_ext(&self) -> &StaExt {
        &self.sta_ext
    }
}
#[doc = "DR (r) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data register"]
pub mod dr;
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
#[doc = "CH0DR (r) register accessor: Channel 0 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0dr`] module"]
#[doc(alias = "CH0DR")]
pub type Ch0dr = crate::Reg<ch0dr::Ch0drSpec>;
#[doc = "Channel 0 data register"]
pub mod ch0dr;
#[doc = "CH1DR (r) register accessor: Channel 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1dr`] module"]
#[doc(alias = "CH1DR")]
pub type Ch1dr = crate::Reg<ch1dr::Ch1drSpec>;
#[doc = "Channel 1 data register"]
pub mod ch1dr;
#[doc = "CH2DR (r) register accessor: Channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2dr`] module"]
#[doc(alias = "CH2DR")]
pub type Ch2dr = crate::Reg<ch2dr::Ch2drSpec>;
#[doc = "Channel 2 data register"]
pub mod ch2dr;
#[doc = "CH3DR (r) register accessor: Channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3dr`] module"]
#[doc(alias = "CH3DR")]
pub type Ch3dr = crate::Reg<ch3dr::Ch3drSpec>;
#[doc = "Channel 3 data register"]
pub mod ch3dr;
#[doc = "CH4DR (r) register accessor: Channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4dr`] module"]
#[doc(alias = "CH4DR")]
pub type Ch4dr = crate::Reg<ch4dr::Ch4drSpec>;
#[doc = "Channel 4 data register"]
pub mod ch4dr;
#[doc = "CH5DR (r) register accessor: Channel 5 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5dr`] module"]
#[doc(alias = "CH5DR")]
pub type Ch5dr = crate::Reg<ch5dr::Ch5drSpec>;
#[doc = "Channel 5 data register"]
pub mod ch5dr;
#[doc = "CH6DR (r) register accessor: Channel 6 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6dr`] module"]
#[doc(alias = "CH6DR")]
pub type Ch6dr = crate::Reg<ch6dr::Ch6drSpec>;
#[doc = "Channel 6 data register"]
pub mod ch6dr;
#[doc = "CH7DR (r) register accessor: Channel 7 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7dr`] module"]
#[doc(alias = "CH7DR")]
pub type Ch7dr = crate::Reg<ch7dr::Ch7drSpec>;
#[doc = "Channel 7 data register"]
pub mod ch7dr;
#[doc = "CH8DR (r) register accessor: Channel 8 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch8dr`] module"]
#[doc(alias = "CH8DR")]
pub type Ch8dr = crate::Reg<ch8dr::Ch8drSpec>;
#[doc = "Channel 8 data register"]
pub mod ch8dr;
#[doc = "CH9DR (r) register accessor: Channel 9 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch9dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch9dr`] module"]
#[doc(alias = "CH9DR")]
pub type Ch9dr = crate::Reg<ch9dr::Ch9drSpec>;
#[doc = "Channel 9 data register"]
pub mod ch9dr;
#[doc = "CH10DR (r) register accessor: Channel 10 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch10dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch10dr`] module"]
#[doc(alias = "CH10DR")]
pub type Ch10dr = crate::Reg<ch10dr::Ch10drSpec>;
#[doc = "Channel 10 data register"]
pub mod ch10dr;
#[doc = "CH11DR (r) register accessor: Channel 11 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch11dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch11dr`] module"]
#[doc(alias = "CH11DR")]
pub type Ch11dr = crate::Reg<ch11dr::Ch11drSpec>;
#[doc = "Channel 11 data register"]
pub mod ch11dr;
#[doc = "CH14DR (r) register accessor: Channel 14 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch14dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch14dr`] module"]
#[doc(alias = "CH14DR")]
pub type Ch14dr = crate::Reg<ch14dr::Ch14drSpec>;
#[doc = "Channel 14 data register"]
pub mod ch14dr;
#[doc = "CH15DR (r) register accessor: Channel 15 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch15dr`] module"]
#[doc(alias = "CH15DR")]
pub type Ch15dr = crate::Reg<ch15dr::Ch15drSpec>;
#[doc = "Channel 15 data register"]
pub mod ch15dr;
#[doc = "STA_EXT (r) register accessor: Extended status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta_ext::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta_ext`] module"]
#[doc(alias = "STA_EXT")]
pub type StaExt = crate::Reg<sta_ext::StaExtSpec>;
#[doc = "Extended status register"]
pub mod sta_ext;
