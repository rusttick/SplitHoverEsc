#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtc_crh: RtcCrh,
    _reserved1: [u8; 0x02],
    rtc_crl: RtcCrl,
    _reserved2: [u8; 0x02],
    rtc_prlh: RtcPrlh,
    _reserved3: [u8; 0x02],
    rtc_prll: RtcPrll,
    _reserved4: [u8; 0x02],
    rtc_divh: RtcDivh,
    _reserved5: [u8; 0x02],
    rtc_divl: RtcDivl,
    _reserved6: [u8; 0x02],
    rtc_cnth: RtcCnth,
    _reserved7: [u8; 0x02],
    rtc_cntl: RtcCntl,
    _reserved8: [u8; 0x02],
    rtc_alrh: RtcAlrh,
    _reserved9: [u8; 0x02],
    rtc_alrl: RtcAlrl,
    _reserved10: [u8; 0x02],
    rtc_msrh: RtcMsrh,
    _reserved11: [u8; 0x02],
    rtc_msrl: RtcMsrl,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC configuration high register"]
    #[inline(always)]
    pub const fn rtc_crh(&self) -> &RtcCrh {
        &self.rtc_crh
    }
    #[doc = "0x04 - RTC configuration low register"]
    #[inline(always)]
    pub const fn rtc_crl(&self) -> &RtcCrl {
        &self.rtc_crl
    }
    #[doc = "0x08 - RTC Prescaler load high register"]
    #[inline(always)]
    pub const fn rtc_prlh(&self) -> &RtcPrlh {
        &self.rtc_prlh
    }
    #[doc = "0x0c - RTC Prescaler load low register"]
    #[inline(always)]
    pub const fn rtc_prll(&self) -> &RtcPrll {
        &self.rtc_prll
    }
    #[doc = "0x10 - RTC prescaler divider factor high register"]
    #[inline(always)]
    pub const fn rtc_divh(&self) -> &RtcDivh {
        &self.rtc_divh
    }
    #[doc = "0x14 - RTC prescaler divider factor low register"]
    #[inline(always)]
    pub const fn rtc_divl(&self) -> &RtcDivl {
        &self.rtc_divl
    }
    #[doc = "0x18 - RTC counter high register"]
    #[inline(always)]
    pub const fn rtc_cnth(&self) -> &RtcCnth {
        &self.rtc_cnth
    }
    #[doc = "0x1c - RTC counter low register"]
    #[inline(always)]
    pub const fn rtc_cntl(&self) -> &RtcCntl {
        &self.rtc_cntl
    }
    #[doc = "0x20 - RTC alarm high register"]
    #[inline(always)]
    pub const fn rtc_alrh(&self) -> &RtcAlrh {
        &self.rtc_alrh
    }
    #[doc = "0x24 - RTC alarm low register"]
    #[inline(always)]
    pub const fn rtc_alrl(&self) -> &RtcAlrl {
        &self.rtc_alrl
    }
    #[doc = "0x28 - RTC millisecond alarm high register"]
    #[inline(always)]
    pub const fn rtc_msrh(&self) -> &RtcMsrh {
        &self.rtc_msrh
    }
    #[doc = "0x2c - RTC millisecond alarm low register"]
    #[inline(always)]
    pub const fn rtc_msrl(&self) -> &RtcMsrl {
        &self.rtc_msrl
    }
}
#[doc = "RTC_CRH (rw) register accessor: RTC configuration high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_crh`] module"]
#[doc(alias = "RTC_CRH")]
pub type RtcCrh = crate::Reg<rtc_crh::RtcCrhSpec>;
#[doc = "RTC configuration high register"]
pub mod rtc_crh;
#[doc = "RTC_CRL (rw) register accessor: RTC configuration low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_crl`] module"]
#[doc(alias = "RTC_CRL")]
pub type RtcCrl = crate::Reg<rtc_crl::RtcCrlSpec>;
#[doc = "RTC configuration low register"]
pub mod rtc_crl;
#[doc = "RTC_PRLH (w) register accessor: RTC Prescaler load high register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prlh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prlh`] module"]
#[doc(alias = "RTC_PRLH")]
pub type RtcPrlh = crate::Reg<rtc_prlh::RtcPrlhSpec>;
#[doc = "RTC Prescaler load high register"]
pub mod rtc_prlh;
#[doc = "RTC_PRLL (w) register accessor: RTC Prescaler load low register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prll::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prll`] module"]
#[doc(alias = "RTC_PRLL")]
pub type RtcPrll = crate::Reg<rtc_prll::RtcPrllSpec>;
#[doc = "RTC Prescaler load low register"]
pub mod rtc_prll;
#[doc = "RTC_DIVH (r) register accessor: RTC prescaler divider factor high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_divh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_divh`] module"]
#[doc(alias = "RTC_DIVH")]
pub type RtcDivh = crate::Reg<rtc_divh::RtcDivhSpec>;
#[doc = "RTC prescaler divider factor high register"]
pub mod rtc_divh;
#[doc = "RTC_DIVL (r) register accessor: RTC prescaler divider factor low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_divl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_divl`] module"]
#[doc(alias = "RTC_DIVL")]
pub type RtcDivl = crate::Reg<rtc_divl::RtcDivlSpec>;
#[doc = "RTC prescaler divider factor low register"]
pub mod rtc_divl;
#[doc = "RTC_CNTH (rw) register accessor: RTC counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cnth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cnth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cnth`] module"]
#[doc(alias = "RTC_CNTH")]
pub type RtcCnth = crate::Reg<rtc_cnth::RtcCnthSpec>;
#[doc = "RTC counter high register"]
pub mod rtc_cnth;
#[doc = "RTC_CNTL (rw) register accessor: RTC counter low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cntl`] module"]
#[doc(alias = "RTC_CNTL")]
pub type RtcCntl = crate::Reg<rtc_cntl::RtcCntlSpec>;
#[doc = "RTC counter low register"]
pub mod rtc_cntl;
#[doc = "RTC_ALRH (rw) register accessor: RTC alarm high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrh`] module"]
#[doc(alias = "RTC_ALRH")]
pub type RtcAlrh = crate::Reg<rtc_alrh::RtcAlrhSpec>;
#[doc = "RTC alarm high register"]
pub mod rtc_alrh;
#[doc = "RTC_ALRL (rw) register accessor: RTC alarm low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrl`] module"]
#[doc(alias = "RTC_ALRL")]
pub type RtcAlrl = crate::Reg<rtc_alrl::RtcAlrlSpec>;
#[doc = "RTC alarm low register"]
pub mod rtc_alrl;
#[doc = "RTC_MSRH (rw) register accessor: RTC millisecond alarm high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_msrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_msrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_msrh`] module"]
#[doc(alias = "RTC_MSRH")]
pub type RtcMsrh = crate::Reg<rtc_msrh::RtcMsrhSpec>;
#[doc = "RTC millisecond alarm high register"]
pub mod rtc_msrh;
#[doc = "RTC_MSRL (rw) register accessor: RTC millisecond alarm low register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_msrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_msrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_msrl`] module"]
#[doc(alias = "RTC_MSRL")]
pub type RtcMsrl = crate::Reg<rtc_msrl::RtcMsrlSpec>;
#[doc = "RTC millisecond alarm low register"]
pub mod rtc_msrl;
