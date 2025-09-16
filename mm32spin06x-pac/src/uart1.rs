#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tdr: Tdr,
    rdr: Rdr,
    csr: Csr,
    isr: Isr,
    ier: Ier,
    icr: Icr,
    gcr: Gcr,
    ccr: Ccr,
    brr: Brr,
    fra: Fra,
    rxaddr: Rxaddr,
    rxmask: Rxmask,
    scr: Scr,
    idlr: Idlr,
    abrcr: Abrcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit data register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x04 - Receive data register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x08 - Current status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x0c - Interrupt status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x10 - Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x18 - Global control register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    #[doc = "0x1c - common control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x20 - Baud rate register"]
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    #[doc = "0x24 - Fractional baud rate register"]
    #[inline(always)]
    pub const fn fra(&self) -> &Fra {
        &self.fra
    }
    #[doc = "0x28 - Receive Address Register"]
    #[inline(always)]
    pub const fn rxaddr(&self) -> &Rxaddr {
        &self.rxaddr
    }
    #[doc = "0x2c - Receive Mask Registe"]
    #[inline(always)]
    pub const fn rxmask(&self) -> &Rxmask {
        &self.rxmask
    }
    #[doc = "0x30 - Slave Control Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x34 - Data length register"]
    #[inline(always)]
    pub const fn idlr(&self) -> &Idlr {
        &self.idlr
    }
    #[doc = "0x38 - Automatic Baud Rate Register"]
    #[inline(always)]
    pub const fn abrcr(&self) -> &Abrcr {
        &self.abrcr
    }
}
#[doc = "TDR (rw) register accessor: Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transmit data register"]
pub mod tdr;
#[doc = "RDR (r) register accessor: Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`] module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "Receive data register"]
pub mod rdr;
#[doc = "CSR (r) register accessor: Current status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Current status register"]
pub mod csr;
#[doc = "ISR (r) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt enable register"]
pub mod ier;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "GCR (rw) register accessor: Global control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`] module"]
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "CCR (rw) register accessor: common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "common control register"]
pub mod ccr;
#[doc = "BRR (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`] module"]
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
#[doc = "Baud rate register"]
pub mod brr;
#[doc = "FRA (rw) register accessor: Fractional baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`fra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fra`] module"]
#[doc(alias = "FRA")]
pub type Fra = crate::Reg<fra::FraSpec>;
#[doc = "Fractional baud rate register"]
pub mod fra;
#[doc = "RXADDR (rw) register accessor: Receive Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxaddr`] module"]
#[doc(alias = "RXADDR")]
pub type Rxaddr = crate::Reg<rxaddr::RxaddrSpec>;
#[doc = "Receive Address Register"]
pub mod rxaddr;
#[doc = "RXMASK (rw) register accessor: Receive Mask Registe\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmask`] module"]
#[doc(alias = "RXMASK")]
pub type Rxmask = crate::Reg<rxmask::RxmaskSpec>;
#[doc = "Receive Mask Registe"]
pub mod rxmask;
#[doc = "SCR (rw) register accessor: Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Slave Control Register"]
pub mod scr;
#[doc = "IDLR (rw) register accessor: Data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`idlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idlr`] module"]
#[doc(alias = "IDLR")]
pub type Idlr = crate::Reg<idlr::IdlrSpec>;
#[doc = "Data length register"]
pub mod idlr;
#[doc = "ABRCR (rw) register accessor: Automatic Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`abrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abrcr`] module"]
#[doc(alias = "ABRCR")]
pub type Abrcr = crate::Reg<abrcr::AbrcrSpec>;
#[doc = "Automatic Baud Rate Register"]
pub mod abrcr;
