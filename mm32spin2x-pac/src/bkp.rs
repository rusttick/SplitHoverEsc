#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    dr9: Dr9,
    dr10: Dr10,
    dr11: Dr11,
    dr12: Dr12,
    dr13: Dr13,
    dr14: Dr14,
    dr15: Dr15,
    dr16: Dr16,
    dr17: Dr17,
    dr18: Dr18,
    dr19: Dr19,
    dr20: Dr20,
}
impl RegisterBlock {
    #[doc = "0x04 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x08 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x0c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x10 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x14 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x18 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x1c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x20 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0x24 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr9(&self) -> &Dr9 {
        &self.dr9
    }
    #[doc = "0x28 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr10(&self) -> &Dr10 {
        &self.dr10
    }
    #[doc = "0x2c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr11(&self) -> &Dr11 {
        &self.dr11
    }
    #[doc = "0x30 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr12(&self) -> &Dr12 {
        &self.dr12
    }
    #[doc = "0x34 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr13(&self) -> &Dr13 {
        &self.dr13
    }
    #[doc = "0x38 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr14(&self) -> &Dr14 {
        &self.dr14
    }
    #[doc = "0x3c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr15(&self) -> &Dr15 {
        &self.dr15
    }
    #[doc = "0x40 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr16(&self) -> &Dr16 {
        &self.dr16
    }
    #[doc = "0x44 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr17(&self) -> &Dr17 {
        &self.dr17
    }
    #[doc = "0x48 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr18(&self) -> &Dr18 {
        &self.dr18
    }
    #[doc = "0x4c - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr19(&self) -> &Dr19 {
        &self.dr19
    }
    #[doc = "0x50 - Backup data register(BKP_DR)"]
    #[inline(always)]
    pub const fn dr20(&self) -> &Dr20 {
        &self.dr20
    }
}
#[doc = "DR1 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`] module"]
#[doc(alias = "DR9")]
pub type Dr9 = crate::Reg<dr9::Dr9Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`] module"]
#[doc(alias = "DR10")]
pub type Dr10 = crate::Reg<dr10::Dr10Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr10;
#[doc = "DR11 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`] module"]
#[doc(alias = "DR11")]
pub type Dr11 = crate::Reg<dr11::Dr11Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`] module"]
#[doc(alias = "DR12")]
pub type Dr12 = crate::Reg<dr12::Dr12Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`] module"]
#[doc(alias = "DR13")]
pub type Dr13 = crate::Reg<dr13::Dr13Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`] module"]
#[doc(alias = "DR14")]
pub type Dr14 = crate::Reg<dr14::Dr14Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`] module"]
#[doc(alias = "DR15")]
pub type Dr15 = crate::Reg<dr15::Dr15Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`] module"]
#[doc(alias = "DR16")]
pub type Dr16 = crate::Reg<dr16::Dr16Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr17`] module"]
#[doc(alias = "DR17")]
pub type Dr17 = crate::Reg<dr17::Dr17Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr18`] module"]
#[doc(alias = "DR18")]
pub type Dr18 = crate::Reg<dr18::Dr18Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr19`] module"]
#[doc(alias = "DR19")]
pub type Dr19 = crate::Reg<dr19::Dr19Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: Backup data register(BKP_DR)\n\nYou can [`read`](crate::Reg::read) this register and get [`dr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr20`] module"]
#[doc(alias = "DR20")]
pub type Dr20 = crate::Reg<dr20::Dr20Spec>;
#[doc = "Backup data register(BKP_DR)"]
pub mod dr20;
