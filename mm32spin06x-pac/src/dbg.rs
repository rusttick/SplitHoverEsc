#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idcode: Idcode,
    cr: Cr,
}
impl RegisterBlock {
    #[doc = "0x00 - ID CODE"]
    #[inline(always)]
    pub const fn idcode(&self) -> &Idcode {
        &self.idcode
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
}
#[doc = "IDCODE (r) register accessor: ID CODE\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idcode`] module"]
#[doc(alias = "IDCODE")]
pub type Idcode = crate::Reg<idcode::IdcodeSpec>;
#[doc = "ID CODE"]
pub mod idcode;
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
