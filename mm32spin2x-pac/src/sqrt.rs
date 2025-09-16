#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sqr: Sqr,
    result: Result,
}
impl RegisterBlock {
    #[doc = "0x00 - SQR"]
    #[inline(always)]
    pub const fn sqr(&self) -> &Sqr {
        &self.sqr
    }
    #[doc = "0x04 - RESULT"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
}
#[doc = "SQR (rw) register accessor: SQR\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr`] module"]
#[doc(alias = "SQR")]
pub type Sqr = crate::Reg<sqr::SqrSpec>;
#[doc = "SQR"]
pub mod sqr;
#[doc = "RESULT (rw) register accessor: RESULT\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`] module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "RESULT"]
pub mod result;
