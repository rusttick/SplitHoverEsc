#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    dier: Dier,
    sr: Sr,
    egr: Egr,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ccer: Ccer,
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
    rcr: Rcr,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    bdtr: Bdtr,
    dcr: Dcr,
    dmar: Dmar,
    _reserved20: [u8; 0x04],
    ccmr3_output: Ccmr3Output,
    ccr5: Ccr5,
    pder: Pder,
    ccr1fall: Ccr1fall,
    ccr2fall: Ccr2fall,
    ccr3fall: Ccr3fall,
    ccr4fall: Ccr4fall,
    ccr5fall: Ccr5fall,
}
impl RegisterBlock {
    #[doc = "0x00 - control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - slave mode control register 1"]
    #[inline(always)]
    pub const fn smcr(&self) -> &Smcr {
        &self.smcr
    }
    #[doc = "0x0c - DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(&self) -> &Dier {
        &self.dier
    }
    #[doc = "0x10 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x14 - event generation register"]
    #[inline(always)]
    pub const fn egr(&self) -> &Egr {
        &self.egr
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &Ccmr1Input {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &Ccmr1Output {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &Ccmr2Input {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2(output mode)"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &Ccmr2Output {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(&self) -> &Ccer {
        &self.ccer
    }
    #[doc = "0x24 - counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x28 - prescaler"]
    #[inline(always)]
    pub const fn psc(&self) -> &Psc {
        &self.psc
    }
    #[doc = "0x2c - auto-reload register"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x30 - repetition counter register"]
    #[inline(always)]
    pub const fn rcr(&self) -> &Rcr {
        &self.rcr
    }
    #[doc = "0x34 - capture/compare register 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x38 - capture/compare register 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x3c - capture/compare register 3"]
    #[inline(always)]
    pub const fn ccr3(&self) -> &Ccr3 {
        &self.ccr3
    }
    #[doc = "0x40 - capture/compare register 4"]
    #[inline(always)]
    pub const fn ccr4(&self) -> &Ccr4 {
        &self.ccr4
    }
    #[doc = "0x44 - break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(&self) -> &Bdtr {
        &self.bdtr
    }
    #[doc = "0x48 - DMA control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x4c - DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(&self) -> &Dmar {
        &self.dmar
    }
    #[doc = "0x54 - capture/compare mode register 3 (output mode)"]
    #[inline(always)]
    pub const fn ccmr3_output(&self) -> &Ccmr3Output {
        &self.ccmr3_output
    }
    #[doc = "0x58 - capture/compare register 5"]
    #[inline(always)]
    pub const fn ccr5(&self) -> &Ccr5 {
        &self.ccr5
    }
    #[doc = "0x5c - PWM/DMA repeat enable register"]
    #[inline(always)]
    pub const fn pder(&self) -> &Pder {
        &self.pder
    }
    #[doc = "0x60 - pwm shift count CCR1 register"]
    #[inline(always)]
    pub const fn ccr1fall(&self) -> &Ccr1fall {
        &self.ccr1fall
    }
    #[doc = "0x64 - pwm shift count CCR2 register"]
    #[inline(always)]
    pub const fn ccr2fall(&self) -> &Ccr2fall {
        &self.ccr2fall
    }
    #[doc = "0x68 - pwm shift count CCR3 register"]
    #[inline(always)]
    pub const fn ccr3fall(&self) -> &Ccr3fall {
        &self.ccr3fall
    }
    #[doc = "0x6c - pwm shift count CCR4 register"]
    #[inline(always)]
    pub const fn ccr4fall(&self) -> &Ccr4fall {
        &self.ccr4fall
    }
    #[doc = "0x70 - pwm shift count CCR5 register"]
    #[inline(always)]
    pub const fn ccr5fall(&self) -> &Ccr5fall {
        &self.ccr5fall
    }
}
#[doc = "CR1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: slave mode control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`] module"]
#[doc(alias = "SMCR")]
pub type Smcr = crate::Reg<smcr::SmcrSpec>;
#[doc = "slave mode control register 1"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dier`] module"]
#[doc(alias = "DIER")]
pub type Dier = crate::Reg<dier::DierSpec>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@egr`] module"]
#[doc(alias = "EGR")]
pub type Egr = crate::Reg<egr::EgrSpec>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output (rw) register accessor: capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_output`] module"]
#[doc(alias = "CCMR1_Output")]
pub type Ccmr1Output = crate::Reg<ccmr1_output::Ccmr1OutputSpec>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: capture/compare mode register 1 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr1_input`] module"]
#[doc(alias = "CCMR1_Input")]
pub type Ccmr1Input = crate::Reg<ccmr1_input::Ccmr1InputSpec>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output (rw) register accessor: capture/compare mode register 2(output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_output`] module"]
#[doc(alias = "CCMR2_Output")]
pub type Ccmr2Output = crate::Reg<ccmr2_output::Ccmr2OutputSpec>;
#[doc = "capture/compare mode register 2(output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input (rw) register accessor: capture/compare mode register 2 (input mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr2_input`] module"]
#[doc(alias = "CCMR2_Input")]
pub type Ccmr2Input = crate::Reg<ccmr2_input::Ccmr2InputSpec>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccmr2_input;
#[doc = "CCER (rw) register accessor: capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccer`] module"]
#[doc(alias = "CCER")]
pub type Ccer = crate::Reg<ccer::CcerSpec>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psc`] module"]
#[doc(alias = "PSC")]
pub type Psc = crate::Reg<psc::PscSpec>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: repetition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr`] module"]
#[doc(alias = "RCR")]
pub type Rcr = crate::Reg<rcr::RcrSpec>;
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "CCR1 (rw) register accessor: capture/compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: capture/compare register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: capture/compare register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3`] module"]
#[doc(alias = "CCR3")]
pub type Ccr3 = crate::Reg<ccr3::Ccr3Spec>;
#[doc = "capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: capture/compare register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4`] module"]
#[doc(alias = "CCR4")]
pub type Ccr4 = crate::Reg<ccr4::Ccr4Spec>;
#[doc = "capture/compare register 4"]
pub mod ccr4;
#[doc = "BDTR (rw) register accessor: break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtr`] module"]
#[doc(alias = "BDTR")]
pub type Bdtr = crate::Reg<bdtr::BdtrSpec>;
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DCR (w) register accessor: DMA control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMAR (w) register accessor: DMA address for full transfer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmar`] module"]
#[doc(alias = "DMAR")]
pub type Dmar = crate::Reg<dmar::DmarSpec>;
#[doc = "DMA address for full transfer"]
pub mod dmar;
#[doc = "CCMR3_Output (rw) register accessor: capture/compare mode register 3 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccmr3_output`] module"]
#[doc(alias = "CCMR3_Output")]
pub type Ccmr3Output = crate::Reg<ccmr3_output::Ccmr3OutputSpec>;
#[doc = "capture/compare mode register 3 (output mode)"]
pub mod ccmr3_output;
#[doc = "CCR5 (rw) register accessor: capture/compare register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5`] module"]
#[doc(alias = "CCR5")]
pub type Ccr5 = crate::Reg<ccr5::Ccr5Spec>;
#[doc = "capture/compare register 5"]
pub mod ccr5;
#[doc = "PDER (rw) register accessor: PWM/DMA repeat enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pder::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pder::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pder`] module"]
#[doc(alias = "PDER")]
pub type Pder = crate::Reg<pder::PderSpec>;
#[doc = "PWM/DMA repeat enable register"]
pub mod pder;
#[doc = "CCR1FALL (rw) register accessor: pwm shift count CCR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1fall`] module"]
#[doc(alias = "CCR1FALL")]
pub type Ccr1fall = crate::Reg<ccr1fall::Ccr1fallSpec>;
#[doc = "pwm shift count CCR1 register"]
pub mod ccr1fall;
#[doc = "CCR2FALL (rw) register accessor: pwm shift count CCR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2fall`] module"]
#[doc(alias = "CCR2FALL")]
pub type Ccr2fall = crate::Reg<ccr2fall::Ccr2fallSpec>;
#[doc = "pwm shift count CCR2 register"]
pub mod ccr2fall;
#[doc = "CCR3FALL (rw) register accessor: pwm shift count CCR3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr3fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr3fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr3fall`] module"]
#[doc(alias = "CCR3FALL")]
pub type Ccr3fall = crate::Reg<ccr3fall::Ccr3fallSpec>;
#[doc = "pwm shift count CCR3 register"]
pub mod ccr3fall;
#[doc = "CCR4FALL (rw) register accessor: pwm shift count CCR4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr4fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr4fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr4fall`] module"]
#[doc(alias = "CCR4FALL")]
pub type Ccr4fall = crate::Reg<ccr4fall::Ccr4fallSpec>;
#[doc = "pwm shift count CCR4 register"]
pub mod ccr4fall;
#[doc = "CCR5FALL (rw) register accessor: pwm shift count CCR5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5fall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5fall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr5fall`] module"]
#[doc(alias = "CCR5FALL")]
pub type Ccr5fall = crate::Reg<ccr5fall::Ccr5fallSpec>;
#[doc = "pwm shift count CCR5 register"]
pub mod ccr5fall;
