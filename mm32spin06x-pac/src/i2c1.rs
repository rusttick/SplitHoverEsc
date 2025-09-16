#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    tar: Tar,
    sar: Sar,
    _reserved3: [u8; 0x04],
    dr: Dr,
    sshr: Sshr,
    sslr: Sslr,
    fshr: Fshr,
    fslr: Fslr,
    _reserved8: [u8; 0x08],
    isr: Isr,
    imr: Imr,
    rawisr: Rawisr,
    rxtlr: Rxtlr,
    txtlr: Txtlr,
    icr: Icr,
    rx_under: RxUnder,
    rx_over: RxOver,
    tx_over: TxOver,
    rd_req: RdReq,
    tx_abrt: TxAbrt,
    rx_done: RxDone,
    activ: Activ,
    stop: Stop,
    start: Start,
    gc: Gc,
    enr: Enr,
    sr: Sr,
    txflr: Txflr,
    rxflr: Rxflr,
    hold: Hold,
    _reserved29: [u8; 0x08],
    dma: Dma,
    _reserved30: [u8; 0x08],
    setup: Setup,
    gcr: Gcr,
    _reserved32: [u8; 0x14],
    slvmask: Slvmask,
    slvrcvaddr: Slvrcvaddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Target Register"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x08 - Slave Address Register"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x10 - Data Command Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x14 - SCL High Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sshr(&self) -> &Sshr {
        &self.sshr
    }
    #[doc = "0x18 - SCL Low Period Count for Std. Speed Register"]
    #[inline(always)]
    pub const fn sslr(&self) -> &Sslr {
        &self.sslr
    }
    #[doc = "0x1c - SCL High Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fshr(&self) -> &Fshr {
        &self.fshr
    }
    #[doc = "0x20 - SCL Low Period Count for Fast Speed Register"]
    #[inline(always)]
    pub const fn fslr(&self) -> &Fslr {
        &self.fslr
    }
    #[doc = "0x2c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x30 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x34 - RAW Interrupt Status Register"]
    #[inline(always)]
    pub const fn rawisr(&self) -> &Rawisr {
        &self.rawisr
    }
    #[doc = "0x38 - Receive FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn rxtlr(&self) -> &Rxtlr {
        &self.rxtlr
    }
    #[doc = "0x3c - Transmit FIFO Threshold Level Register"]
    #[inline(always)]
    pub const fn txtlr(&self) -> &Txtlr {
        &self.txtlr
    }
    #[doc = "0x40 - Clear All Interrupt Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_under(&self) -> &RxUnder {
        &self.rx_under
    }
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn rx_over(&self) -> &RxOver {
        &self.rx_over
    }
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    #[inline(always)]
    pub const fn tx_over(&self) -> &TxOver {
        &self.tx_over
    }
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    #[inline(always)]
    pub const fn rd_req(&self) -> &RdReq {
        &self.rd_req
    }
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    #[inline(always)]
    pub const fn tx_abrt(&self) -> &TxAbrt {
        &self.tx_abrt
    }
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    #[inline(always)]
    pub const fn rx_done(&self) -> &RxDone {
        &self.rx_done
    }
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    #[inline(always)]
    pub const fn activ(&self) -> &Activ {
        &self.activ
    }
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x68 - Clear GEN_CALL Interrupt Register"]
    #[inline(always)]
    pub const fn gc(&self) -> &Gc {
        &self.gc
    }
    #[doc = "0x6c - Enable Register"]
    #[inline(always)]
    pub const fn enr(&self) -> &Enr {
        &self.enr
    }
    #[doc = "0x70 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x74 - Transmit FIFO Level Register"]
    #[inline(always)]
    pub const fn txflr(&self) -> &Txflr {
        &self.txflr
    }
    #[doc = "0x78 - Receive FIFO Level Register"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &Rxflr {
        &self.rxflr
    }
    #[doc = "0x7c - SDA Hold Time Register"]
    #[inline(always)]
    pub const fn hold(&self) -> &Hold {
        &self.hold
    }
    #[doc = "0x88 - DMA Control Register"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x94 - SDA Setup Time Register"]
    #[inline(always)]
    pub const fn setup(&self) -> &Setup {
        &self.setup
    }
    #[doc = "0x98 - ACK General Call Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    #[doc = "0xb0 - Slave Address Mask Register"]
    #[inline(always)]
    pub const fn slvmask(&self) -> &Slvmask {
        &self.slvmask
    }
    #[doc = "0xb4 - Receiver Address Register"]
    #[inline(always)]
    pub const fn slvrcvaddr(&self) -> &Slvrcvaddr {
        &self.slvrcvaddr
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "TAR (rw) register accessor: Target Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`] module"]
#[doc(alias = "TAR")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "Target Register"]
pub mod tar;
#[doc = "SAR (rw) register accessor: Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`] module"]
#[doc(alias = "SAR")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "DR (rw) register accessor: Data Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data Command Register"]
pub mod dr;
#[doc = "SSHR (rw) register accessor: SCL High Period Count for Std. Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sshr`] module"]
#[doc(alias = "SSHR")]
pub type Sshr = crate::Reg<sshr::SshrSpec>;
#[doc = "SCL High Period Count for Std. Speed Register"]
pub mod sshr;
#[doc = "SSLR (rw) register accessor: SCL Low Period Count for Std. Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sslr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sslr`] module"]
#[doc(alias = "SSLR")]
pub type Sslr = crate::Reg<sslr::SslrSpec>;
#[doc = "SCL Low Period Count for Std. Speed Register"]
pub mod sslr;
#[doc = "FSHR (rw) register accessor: SCL High Period Count for Fast Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fshr`] module"]
#[doc(alias = "FSHR")]
pub type Fshr = crate::Reg<fshr::FshrSpec>;
#[doc = "SCL High Period Count for Fast Speed Register"]
pub mod fshr;
#[doc = "FSLR (rw) register accessor: SCL Low Period Count for Fast Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fslr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fslr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fslr`] module"]
#[doc(alias = "FSLR")]
pub type Fslr = crate::Reg<fslr::FslrSpec>;
#[doc = "SCL Low Period Count for Fast Speed Register"]
pub mod fslr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "RAWISR (r) register accessor: RAW Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawisr`] module"]
#[doc(alias = "RAWISR")]
pub type Rawisr = crate::Reg<rawisr::RawisrSpec>;
#[doc = "RAW Interrupt Status Register"]
pub mod rawisr;
#[doc = "RXTLR (rw) register accessor: Receive FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxtlr`] module"]
#[doc(alias = "RXTLR")]
pub type Rxtlr = crate::Reg<rxtlr::RxtlrSpec>;
#[doc = "Receive FIFO Threshold Level Register"]
pub mod rxtlr;
#[doc = "TXTLR (rw) register accessor: Transmit FIFO Threshold Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txtlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txtlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txtlr`] module"]
#[doc(alias = "TXTLR")]
pub type Txtlr = crate::Reg<txtlr::TxtlrSpec>;
#[doc = "Transmit FIFO Threshold Level Register"]
pub mod txtlr;
#[doc = "ICR (r) register accessor: Clear All Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Clear All Interrupt Register"]
pub mod icr;
#[doc = "RX_UNDER (r) register accessor: Clear RX_UNDER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_under::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_under`] module"]
#[doc(alias = "RX_UNDER")]
pub type RxUnder = crate::Reg<rx_under::RxUnderSpec>;
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod rx_under;
#[doc = "RX_OVER (r) register accessor: Clear RX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_over`] module"]
#[doc(alias = "RX_OVER")]
pub type RxOver = crate::Reg<rx_over::RxOverSpec>;
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod rx_over;
#[doc = "TX_OVER (r) register accessor: Clear TX_OVER Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_over`] module"]
#[doc(alias = "TX_OVER")]
pub type TxOver = crate::Reg<tx_over::TxOverSpec>;
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod tx_over;
#[doc = "RD_REQ (r) register accessor: Clear RD_REQ Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_req::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_req`] module"]
#[doc(alias = "RD_REQ")]
pub type RdReq = crate::Reg<rd_req::RdReqSpec>;
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod rd_req;
#[doc = "TX_ABRT (r) register accessor: Clear TX_ABRT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abrt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_abrt`] module"]
#[doc(alias = "TX_ABRT")]
pub type TxAbrt = crate::Reg<tx_abrt::TxAbrtSpec>;
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod tx_abrt;
#[doc = "RX_DONE (r) register accessor: Clear RX_DONE Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_done::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_done`] module"]
#[doc(alias = "RX_DONE")]
pub type RxDone = crate::Reg<rx_done::RxDoneSpec>;
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod rx_done;
#[doc = "ACTIV (r) register accessor: Clear ACTIVITY Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`activ::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@activ`] module"]
#[doc(alias = "ACTIV")]
pub type Activ = crate::Reg<activ::ActivSpec>;
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod activ;
#[doc = "STOP (r) register accessor: Clear STOP_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`] module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod stop;
#[doc = "START (r) register accessor: Clear START_DET Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Clear START_DET Interrupt Register"]
pub mod start;
#[doc = "GC (r) register accessor: Clear GEN_CALL Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gc`] module"]
#[doc(alias = "GC")]
pub type Gc = crate::Reg<gc::GcSpec>;
#[doc = "Clear GEN_CALL Interrupt Register"]
pub mod gc;
#[doc = "ENR (rw) register accessor: Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enr`] module"]
#[doc(alias = "ENR")]
pub type Enr = crate::Reg<enr::EnrSpec>;
#[doc = "Enable Register"]
pub mod enr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "TXFLR (r) register accessor: Transmit FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txflr`] module"]
#[doc(alias = "TXFLR")]
pub type Txflr = crate::Reg<txflr::TxflrSpec>;
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "RXFLR (r) register accessor: Receive FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxflr`] module"]
#[doc(alias = "RXFLR")]
pub type Rxflr = crate::Reg<rxflr::RxflrSpec>;
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "HOLD (rw) register accessor: SDA Hold Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hold`] module"]
#[doc(alias = "HOLD")]
pub type Hold = crate::Reg<hold::HoldSpec>;
#[doc = "SDA Hold Time Register"]
pub mod hold;
#[doc = "DMA (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`] module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Control Register"]
pub mod dma;
#[doc = "SETUP (rw) register accessor: SDA Setup Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup`] module"]
#[doc(alias = "SETUP")]
pub type Setup = crate::Reg<setup::SetupSpec>;
#[doc = "SDA Setup Time Register"]
pub mod setup;
#[doc = "GCR (rw) register accessor: ACK General Call Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`] module"]
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
#[doc = "ACK General Call Register"]
pub mod gcr;
#[doc = "SLVMASK (rw) register accessor: Slave Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slvmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvmask`] module"]
#[doc(alias = "SLVMASK")]
pub type Slvmask = crate::Reg<slvmask::SlvmaskSpec>;
#[doc = "Slave Address Mask Register"]
pub mod slvmask;
#[doc = "SLVRCVADDR (r) register accessor: Receiver Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slvrcvaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slvrcvaddr`] module"]
#[doc(alias = "SLVRCVADDR")]
pub type Slvrcvaddr = crate::Reg<slvrcvaddr::SlvrcvaddrSpec>;
#[doc = "Receiver Address Register"]
pub mod slvrcvaddr;
