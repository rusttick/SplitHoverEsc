#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uid1: Uid1,
    uid2: Uid2,
    uid3: Uid3,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    #[inline(always)]
    pub const fn uid1(&self) -> &Uid1 {
        &self.uid1
    }
    #[doc = "0x04 - Configuration register"]
    #[inline(always)]
    pub const fn uid2(&self) -> &Uid2 {
        &self.uid2
    }
    #[doc = "0x08 - Configuration register"]
    #[inline(always)]
    pub const fn uid3(&self) -> &Uid3 {
        &self.uid3
    }
}
#[doc = "UID1 (r) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid1`] module"]
#[doc(alias = "UID1")]
pub type Uid1 = crate::Reg<uid1::Uid1Spec>;
#[doc = "Configuration register"]
pub mod uid1;
#[doc = "UID2 (r) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid2`] module"]
#[doc(alias = "UID2")]
pub type Uid2 = crate::Reg<uid2::Uid2Spec>;
#[doc = "Configuration register"]
pub mod uid2;
#[doc = "UID3 (r) register accessor: Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uid3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uid3`] module"]
#[doc(alias = "UID3")]
pub type Uid3 = crate::Reg<uid3::Uid3Spec>;
#[doc = "Configuration register"]
pub mod uid3;
