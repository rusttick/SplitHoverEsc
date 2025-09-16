#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    cfgr: Cfgr,
    cir: Cir,
    apb2rstr: Apb2rstr,
    apb1rstr: Apb1rstr,
    ahbenr: Ahbenr,
    apb2enr: Apb2enr,
    apb1enr: Apb1enr,
    bdcr: Bdcr,
    csr: Csr,
    ahbrstr: Ahbrstr,
    _reserved11: [u8; 0x14],
    config: Config,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - Clock Interrupt Register"]
    #[inline(always)]
    pub const fn cir(&self) -> &Cir {
        &self.cir
    }
    #[doc = "0x0c - Advanced Peripheral Bus 2 Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x10 - Advanced Peripheral Bus 1 Reset Register"]
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &Apb1rstr {
        &self.apb1rstr
    }
    #[doc = "0x14 - Advanced High Performance Bus Enable Register"]
    #[inline(always)]
    pub const fn ahbenr(&self) -> &Ahbenr {
        &self.ahbenr
    }
    #[doc = "0x18 - Advanced Peripheral Bus 2 Enable Register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0x1c - Advanced Peripheral Bus 1 Enable Register"]
    #[inline(always)]
    pub const fn apb1enr(&self) -> &Apb1enr {
        &self.apb1enr
    }
    #[doc = "0x20 - Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x24 - Control Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x28 - Advanced High Performance Bus Reset Register"]
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &Ahbrstr {
        &self.ahbrstr
    }
    #[doc = "0x40 - System Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "Configuration Register"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: Clock Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir`] module"]
#[doc(alias = "CIR")]
pub type Cir = crate::Reg<cir::CirSpec>;
#[doc = "Clock Interrupt Register"]
pub mod cir;
#[doc = "APB2RSTR (rw) register accessor: Advanced Peripheral Bus 2 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "Advanced Peripheral Bus 2 Reset Register"]
pub mod apb2rstr;
#[doc = "APB1RSTR (rw) register accessor: Advanced Peripheral Bus 1 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1rstr`] module"]
#[doc(alias = "APB1RSTR")]
pub type Apb1rstr = crate::Reg<apb1rstr::Apb1rstrSpec>;
#[doc = "Advanced Peripheral Bus 1 Reset Register"]
pub mod apb1rstr;
#[doc = "AHBENR (rw) register accessor: Advanced High Performance Bus Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbenr`] module"]
#[doc(alias = "AHBENR")]
pub type Ahbenr = crate::Reg<ahbenr::AhbenrSpec>;
#[doc = "Advanced High Performance Bus Enable Register"]
pub mod ahbenr;
#[doc = "APB2ENR (rw) register accessor: Advanced Peripheral Bus 2 Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "Advanced Peripheral Bus 2 Enable Register"]
pub mod apb2enr;
#[doc = "APB1ENR (rw) register accessor: Advanced Peripheral Bus 1 Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1enr`] module"]
#[doc(alias = "APB1ENR")]
pub type Apb1enr = crate::Reg<apb1enr::Apb1enrSpec>;
#[doc = "Advanced Peripheral Bus 1 Enable Register"]
pub mod apb1enr;
#[doc = "BDCR (rw) register accessor: Backup Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control Status Register"]
pub mod csr;
#[doc = "AHBRSTR (rw) register accessor: Advanced High Performance Bus Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbrstr`] module"]
#[doc(alias = "AHBRSTR")]
pub type Ahbrstr = crate::Reg<ahbrstr::AhbrstrSpec>;
#[doc = "Advanced High Performance Bus Reset Register"]
pub mod ahbrstr;
#[doc = "CONFIG (rw) register accessor: System Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "System Configuration Register"]
pub mod config;
