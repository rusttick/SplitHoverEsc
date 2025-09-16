#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crl: Crl,
    crh: Crh,
    idr: Idr,
    odr: Odr,
    bsrr: Bsrr,
    brr: Brr,
    lckr: Lckr,
    dcr: Dcr,
    afrl: Afrl,
    afrh: Afrh,
}
impl RegisterBlock {
    #[doc = "0x00 - configuration low register"]
    #[inline(always)]
    pub const fn crl(&self) -> &Crl {
        &self.crl
    }
    #[doc = "0x04 - configuration high register"]
    #[inline(always)]
    pub const fn crh(&self) -> &Crh {
        &self.crh
    }
    #[doc = "0x08 - input data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x0c - output data register"]
    #[inline(always)]
    pub const fn odr(&self) -> &Odr {
        &self.odr
    }
    #[doc = "0x10 - bit set/reset register"]
    #[inline(always)]
    pub const fn bsrr(&self) -> &Bsrr {
        &self.bsrr
    }
    #[doc = "0x14 - bit reset register"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x18 - Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(&self) -> &Lckr {
        &self.lckr
    }
    #[doc = "0x1c - Port output open drain control register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x20 - Port Multiplexing Function Low Register"]
    #[inline(always)]
    pub const fn afrl(&self) -> &Afrl {
        &self.afrl
    }
    #[doc = "0x24 - Port Multiplexing Function High Register"]
    #[inline(always)]
    pub const fn afrh(&self) -> &Afrh {
        &self.afrh
    }
}
#[doc = "CRL (rw) register accessor: configuration low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crl`] module"]
#[doc(alias = "CRL")]
pub type Crl = crate::Reg<crl::CrlSpec>;
#[doc = "configuration low register"]
pub mod crl;
#[doc = "CRH (rw) register accessor: configuration high register\n\nYou can [`read`](crate::Reg::read) this register and get [`crh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crh`] module"]
#[doc(alias = "CRH")]
pub type Crh = crate::Reg<crh::CrhSpec>;
#[doc = "configuration high register"]
pub mod crh;
#[doc = "IDR (r) register accessor: input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odr`] module"]
#[doc(alias = "ODR")]
pub type Odr = crate::Reg<odr::OdrSpec>;
#[doc = "output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsrr`] module"]
#[doc(alias = "BSRR")]
pub type Bsrr = crate::Reg<bsrr::BsrrSpec>;
#[doc = "bit set/reset register"]
pub mod bsrr;
#[doc = "BRR (w) register accessor: bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "bit reset register"]
pub mod brr;
#[doc = "LCKR (rw) register accessor: Port configuration lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckr`] module"]
#[doc(alias = "LCKR")]
pub type Lckr = crate::Reg<lckr::LckrSpec>;
#[doc = "Port configuration lock register"]
pub mod lckr;
#[doc = "DCR (rw) register accessor: Port output open drain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "Port output open drain control register"]
pub mod dcr;
#[doc = "AFRL (rw) register accessor: Port Multiplexing Function Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrl`] module"]
#[doc(alias = "AFRL")]
pub type Afrl = crate::Reg<afrl::AfrlSpec>;
#[doc = "Port Multiplexing Function Low Register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: Port Multiplexing Function High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrh`] module"]
#[doc(alias = "AFRH")]
pub type Afrh = crate::Reg<afrh::AfrhSpec>;
#[doc = "Port Multiplexing Function High Register"]
pub mod afrh;
