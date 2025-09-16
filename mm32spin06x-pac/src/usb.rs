#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    top: Top,
    _reserved1: [u8; 0x02],
    int_state: IntState,
    _reserved2: [u8; 0x02],
    ep_int_state: EpIntState,
    _reserved3: [u8; 0x02],
    ep0_int_state: Ep0IntState,
    _reserved4: [u8; 0x02],
    int_en: IntEn,
    _reserved5: [u8; 0x02],
    ep_int_en: EpIntEn,
    _reserved6: [u8; 0x02],
    ep0_int_en: Ep0IntEn,
    _reserved7: [u8; 0x06],
    ep1_int_state: Ep1IntState,
    _reserved8: [u8; 0x02],
    ep2_int_state: Ep2IntState,
    _reserved9: [u8; 0x02],
    ep3_int_state: Ep3IntState,
    _reserved10: [u8; 0x02],
    ep4_int_state: Ep4IntState,
    _reserved11: [u8; 0x12],
    ep1_int_en: Ep1IntEn,
    _reserved12: [u8; 0x02],
    ep2_int_en: Ep2IntEn,
    _reserved13: [u8; 0x02],
    ep3_int_en: Ep3IntEn,
    _reserved14: [u8; 0x02],
    ep4_int_en: Ep4IntEn,
    _reserved15: [u8; 0x12],
    addr: Addr,
    _reserved16: [u8; 0x02],
    ep_en: EpEn,
    _reserved17: [u8; 0x02],
    ep_dma_dir: EpDmaDir,
    _reserved18: [u8; 0x02],
    ep_type: EpType,
    _reserved19: [u8; 0x02],
    ep_index1_2: EpIndex1_2,
    _reserved20: [u8; 0x02],
    ep_index3_4: EpIndex3_4,
    _reserved21: [u8; 0x02],
    tog_ctrl1_4: TogCtrl1_4,
    _reserved22: [u8; 0x02],
    tog_stat1_4: TogStat1_4,
    _reserved23: [u8; 0x02],
    setup0: Setup0,
    _reserved24: [u8; 0x02],
    setup1: Setup1,
    _reserved25: [u8; 0x02],
    setup2: Setup2,
    _reserved26: [u8; 0x02],
    setup3: Setup3,
    _reserved27: [u8; 0x02],
    setup4: Setup4,
    _reserved28: [u8; 0x02],
    setup5: Setup5,
    _reserved29: [u8; 0x02],
    setup6: Setup6,
    _reserved30: [u8; 0x02],
    setup7: Setup7,
    _reserved31: [u8; 0x02],
    packet_sizel: PacketSizel,
    _reserved32: [u8; 0x02],
    packet_sizeh: PacketSizeh,
    _reserved33: [u8; 0x58],
    ep0_avail: Ep0Avail,
    _reserved34: [u8; 0x02],
    ep1_avail: Ep1Avail,
    _reserved35: [u8; 0x02],
    ep2_avail: Ep2Avail,
    _reserved36: [u8; 0x02],
    ep3_avail: Ep3Avail,
    _reserved37: [u8; 0x02],
    ep4_avail: Ep4Avail,
    _reserved38: [u8; 0x0e],
    dam_addr0: DamAddr0,
    _reserved39: [u8; 0x02],
    dam_addr1: DamAddr1,
    _reserved40: [u8; 0x02],
    dam_addr2: DamAddr2,
    _reserved41: [u8; 0x02],
    dam_addr3: DamAddr3,
    _reserved42: [u8; 0x02],
    dma_numl: DmaNuml,
    _reserved43: [u8; 0x02],
    dma_numh: DmaNumh,
    _reserved44: [u8; 0x0a],
    ep0_ctrl: Ep0Ctrl,
    _reserved45: [u8; 0x02],
    ep1_ctrl: Ep1Ctrl,
    _reserved46: [u8; 0x02],
    ep2_ctrl: Ep2Ctrl,
    _reserved47: [u8; 0x02],
    ep3_ctrl: Ep3Ctrl,
    _reserved48: [u8; 0x02],
    ep4_ctrl: Ep4Ctrl,
    _reserved49: [u8; 0x0e],
    ep0_fifo: Ep0Fifo,
    _reserved50: [u8; 0x02],
    ep1_fifo: Ep1Fifo,
    _reserved51: [u8; 0x02],
    ep2_fifo: Ep2Fifo,
    _reserved52: [u8; 0x02],
    ep3_fifo: Ep3Fifo,
    _reserved53: [u8; 0x02],
    ep4_fifo: Ep4Fifo,
    _reserved54: [u8; 0x0e],
    ep_mem: EpMem,
    _reserved55: [u8; 0x02],
    ep_dma: EpDma,
    _reserved56: [u8; 0x02],
    ep_halt: EpHalt,
    _reserved57: [u8; 0x36],
    power: Power,
    _reserved58: [u8; 0x02],
    usb_ahb_dma: UsbAhbDma,
    _reserved59: [u8; 0x02],
    usb_ahb_rst: UsbAhbRst,
}
impl RegisterBlock {
    #[doc = "0x00 - USB_TOP"]
    #[inline(always)]
    pub const fn top(&self) -> &Top {
        &self.top
    }
    #[doc = "0x04 - interrupt state register"]
    #[inline(always)]
    pub const fn int_state(&self) -> &IntState {
        &self.int_state
    }
    #[doc = "0x08 - EP interrupt state register"]
    #[inline(always)]
    pub const fn ep_int_state(&self) -> &EpIntState {
        &self.ep_int_state
    }
    #[doc = "0x0c - EP0 interrupt state register"]
    #[inline(always)]
    pub const fn ep0_int_state(&self) -> &Ep0IntState {
        &self.ep0_int_state
    }
    #[doc = "0x10 - interrupt enable register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x14 - EP interrupt enable register"]
    #[inline(always)]
    pub const fn ep_int_en(&self) -> &EpIntEn {
        &self.ep_int_en
    }
    #[doc = "0x18 - EP0 interrupt enable register"]
    #[inline(always)]
    pub const fn ep0_int_en(&self) -> &Ep0IntEn {
        &self.ep0_int_en
    }
    #[doc = "0x20 - EP1 interrupt state register"]
    #[inline(always)]
    pub const fn ep1_int_state(&self) -> &Ep1IntState {
        &self.ep1_int_state
    }
    #[doc = "0x24 - EP2 interrupt state register"]
    #[inline(always)]
    pub const fn ep2_int_state(&self) -> &Ep2IntState {
        &self.ep2_int_state
    }
    #[doc = "0x28 - EP3 interrupt state register"]
    #[inline(always)]
    pub const fn ep3_int_state(&self) -> &Ep3IntState {
        &self.ep3_int_state
    }
    #[doc = "0x2c - EP4 interrupt state register"]
    #[inline(always)]
    pub const fn ep4_int_state(&self) -> &Ep4IntState {
        &self.ep4_int_state
    }
    #[doc = "0x40 - EP1 interrupt enable register"]
    #[inline(always)]
    pub const fn ep1_int_en(&self) -> &Ep1IntEn {
        &self.ep1_int_en
    }
    #[doc = "0x44 - EP2 interrupt enable register"]
    #[inline(always)]
    pub const fn ep2_int_en(&self) -> &Ep2IntEn {
        &self.ep2_int_en
    }
    #[doc = "0x48 - EP3 interrupt enable register"]
    #[inline(always)]
    pub const fn ep3_int_en(&self) -> &Ep3IntEn {
        &self.ep3_int_en
    }
    #[doc = "0x4c - EP4 interrupt enable register"]
    #[inline(always)]
    pub const fn ep4_int_en(&self) -> &Ep4IntEn {
        &self.ep4_int_en
    }
    #[doc = "0x60 - USB address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x64 - EP Enable End"]
    #[inline(always)]
    pub const fn ep_en(&self) -> &EpEn {
        &self.ep_en
    }
    #[doc = "0x68 - DMA End-point direction register"]
    #[inline(always)]
    pub const fn ep_dma_dir(&self) -> &EpDmaDir {
        &self.ep_dma_dir
    }
    #[doc = "0x6c - Endpoint type register"]
    #[inline(always)]
    pub const fn ep_type(&self) -> &EpType {
        &self.ep_type
    }
    #[doc = "0x70 - End-point index register 1_2"]
    #[inline(always)]
    pub const fn ep_index1_2(&self) -> &EpIndex1_2 {
        &self.ep_index1_2
    }
    #[doc = "0x74 - End-point index register 3_4"]
    #[inline(always)]
    pub const fn ep_index3_4(&self) -> &EpIndex3_4 {
        &self.ep_index3_4
    }
    #[doc = "0x78 - TOG CTRL1_4 register"]
    #[inline(always)]
    pub const fn tog_ctrl1_4(&self) -> &TogCtrl1_4 {
        &self.tog_ctrl1_4
    }
    #[doc = "0x7c - TOG STAT1_4 register"]
    #[inline(always)]
    pub const fn tog_stat1_4(&self) -> &TogStat1_4 {
        &self.tog_stat1_4
    }
    #[doc = "0x80 - Setup Date 0 register"]
    #[inline(always)]
    pub const fn setup0(&self) -> &Setup0 {
        &self.setup0
    }
    #[doc = "0x84 - Setup Date 1 register"]
    #[inline(always)]
    pub const fn setup1(&self) -> &Setup1 {
        &self.setup1
    }
    #[doc = "0x88 - Setup Date 2 register"]
    #[inline(always)]
    pub const fn setup2(&self) -> &Setup2 {
        &self.setup2
    }
    #[doc = "0x8c - Setup Date 3 register"]
    #[inline(always)]
    pub const fn setup3(&self) -> &Setup3 {
        &self.setup3
    }
    #[doc = "0x90 - Setup Date 4 register"]
    #[inline(always)]
    pub const fn setup4(&self) -> &Setup4 {
        &self.setup4
    }
    #[doc = "0x94 - Setup Date 5 register"]
    #[inline(always)]
    pub const fn setup5(&self) -> &Setup5 {
        &self.setup5
    }
    #[doc = "0x98 - Setup Date 6 register"]
    #[inline(always)]
    pub const fn setup6(&self) -> &Setup6 {
        &self.setup6
    }
    #[doc = "0x9c - Setup Date 7 register"]
    #[inline(always)]
    pub const fn setup7(&self) -> &Setup7 {
        &self.setup7
    }
    #[doc = "0xa0 - PACKET SIZEL register"]
    #[inline(always)]
    pub const fn packet_sizel(&self) -> &PacketSizel {
        &self.packet_sizel
    }
    #[doc = "0xa4 - PACKET SIZEH register"]
    #[inline(always)]
    pub const fn packet_sizeh(&self) -> &PacketSizeh {
        &self.packet_sizeh
    }
    #[doc = "0x100 - EP0 AVAIL register"]
    #[inline(always)]
    pub const fn ep0_avail(&self) -> &Ep0Avail {
        &self.ep0_avail
    }
    #[doc = "0x104 - EP1 AVAIL register"]
    #[inline(always)]
    pub const fn ep1_avail(&self) -> &Ep1Avail {
        &self.ep1_avail
    }
    #[doc = "0x108 - EP2 AVAIL register"]
    #[inline(always)]
    pub const fn ep2_avail(&self) -> &Ep2Avail {
        &self.ep2_avail
    }
    #[doc = "0x10c - EP3 AVAIL register"]
    #[inline(always)]
    pub const fn ep3_avail(&self) -> &Ep3Avail {
        &self.ep3_avail
    }
    #[doc = "0x110 - EP4 AVAIL register"]
    #[inline(always)]
    pub const fn ep4_avail(&self) -> &Ep4Avail {
        &self.ep4_avail
    }
    #[doc = "0x120 - DMA1 ADDR0 register"]
    #[inline(always)]
    pub const fn dam_addr0(&self) -> &DamAddr0 {
        &self.dam_addr0
    }
    #[doc = "0x124 - DMA1 ADDR1 register"]
    #[inline(always)]
    pub const fn dam_addr1(&self) -> &DamAddr1 {
        &self.dam_addr1
    }
    #[doc = "0x128 - DMA1 ADDR2 register"]
    #[inline(always)]
    pub const fn dam_addr2(&self) -> &DamAddr2 {
        &self.dam_addr2
    }
    #[doc = "0x12c - DMA1 ADDR3 register"]
    #[inline(always)]
    pub const fn dam_addr3(&self) -> &DamAddr3 {
        &self.dam_addr3
    }
    #[doc = "0x130 - DMA NUML register"]
    #[inline(always)]
    pub const fn dma_numl(&self) -> &DmaNuml {
        &self.dma_numl
    }
    #[doc = "0x134 - DMA NUMH register"]
    #[inline(always)]
    pub const fn dma_numh(&self) -> &DmaNumh {
        &self.dma_numh
    }
    #[doc = "0x140 - EP0 CTRL register"]
    #[inline(always)]
    pub const fn ep0_ctrl(&self) -> &Ep0Ctrl {
        &self.ep0_ctrl
    }
    #[doc = "0x144 - EP1 CTRL register"]
    #[inline(always)]
    pub const fn ep1_ctrl(&self) -> &Ep1Ctrl {
        &self.ep1_ctrl
    }
    #[doc = "0x148 - EP2 CTRL register"]
    #[inline(always)]
    pub const fn ep2_ctrl(&self) -> &Ep2Ctrl {
        &self.ep2_ctrl
    }
    #[doc = "0x14c - EP3 CTRL register"]
    #[inline(always)]
    pub const fn ep3_ctrl(&self) -> &Ep3Ctrl {
        &self.ep3_ctrl
    }
    #[doc = "0x150 - EP4 CTRL register"]
    #[inline(always)]
    pub const fn ep4_ctrl(&self) -> &Ep4Ctrl {
        &self.ep4_ctrl
    }
    #[doc = "0x160 - EP0 FIFO register"]
    #[inline(always)]
    pub const fn ep0_fifo(&self) -> &Ep0Fifo {
        &self.ep0_fifo
    }
    #[doc = "0x164 - EP1 FIFO register"]
    #[inline(always)]
    pub const fn ep1_fifo(&self) -> &Ep1Fifo {
        &self.ep1_fifo
    }
    #[doc = "0x168 - EP2 FIFO register"]
    #[inline(always)]
    pub const fn ep2_fifo(&self) -> &Ep2Fifo {
        &self.ep2_fifo
    }
    #[doc = "0x16c - EP3 FIFO register"]
    #[inline(always)]
    pub const fn ep3_fifo(&self) -> &Ep3Fifo {
        &self.ep3_fifo
    }
    #[doc = "0x170 - EP4 FIFO register"]
    #[inline(always)]
    pub const fn ep4_fifo(&self) -> &Ep4Fifo {
        &self.ep4_fifo
    }
    #[doc = "0x180 - EP MEM register"]
    #[inline(always)]
    pub const fn ep_mem(&self) -> &EpMem {
        &self.ep_mem
    }
    #[doc = "0x184 - EP DMA register"]
    #[inline(always)]
    pub const fn ep_dma(&self) -> &EpDma {
        &self.ep_dma
    }
    #[doc = "0x188 - EP HALT register"]
    #[inline(always)]
    pub const fn ep_halt(&self) -> &EpHalt {
        &self.ep_halt
    }
    #[doc = "0x1c0 - Power control register"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x1c4 - USB AHB DMA register"]
    #[inline(always)]
    pub const fn usb_ahb_dma(&self) -> &UsbAhbDma {
        &self.usb_ahb_dma
    }
    #[doc = "0x1c8 - USB AHB RST register"]
    #[inline(always)]
    pub const fn usb_ahb_rst(&self) -> &UsbAhbRst {
        &self.usb_ahb_rst
    }
}
#[doc = "TOP (rw) register accessor: USB_TOP\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top`] module"]
#[doc(alias = "TOP")]
pub type Top = crate::Reg<top::TopSpec>;
#[doc = "USB_TOP"]
pub mod top;
#[doc = "INT_STATE (rw) register accessor: interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_state`] module"]
#[doc(alias = "INT_STATE")]
pub type IntState = crate::Reg<int_state::IntStateSpec>;
#[doc = "interrupt state register"]
pub mod int_state;
#[doc = "EP_INT_STATE (r) register accessor: EP interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_int_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_int_state`] module"]
#[doc(alias = "EP_INT_STATE")]
pub type EpIntState = crate::Reg<ep_int_state::EpIntStateSpec>;
#[doc = "EP interrupt state register"]
pub mod ep_int_state;
#[doc = "EP0_INT_STATE (rw) register accessor: EP0 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_int_state`] module"]
#[doc(alias = "EP0_INT_STATE")]
pub type Ep0IntState = crate::Reg<ep0_int_state::Ep0IntStateSpec>;
#[doc = "EP0 interrupt state register"]
pub mod ep0_int_state;
#[doc = "INT_EN (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "interrupt enable register"]
pub mod int_en;
#[doc = "EP_INT_EN (rw) register accessor: EP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_int_en`] module"]
#[doc(alias = "EP_INT_EN")]
pub type EpIntEn = crate::Reg<ep_int_en::EpIntEnSpec>;
#[doc = "EP interrupt enable register"]
pub mod ep_int_en;
#[doc = "EP0_INT_EN (rw) register accessor: EP0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_int_en`] module"]
#[doc(alias = "EP0_INT_EN")]
pub type Ep0IntEn = crate::Reg<ep0_int_en::Ep0IntEnSpec>;
#[doc = "EP0 interrupt enable register"]
pub mod ep0_int_en;
#[doc = "EP1_INT_STATE (rw) register accessor: EP1 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1_int_state`] module"]
#[doc(alias = "EP1_INT_STATE")]
pub type Ep1IntState = crate::Reg<ep1_int_state::Ep1IntStateSpec>;
#[doc = "EP1 interrupt state register"]
pub mod ep1_int_state;
#[doc = "EP2_INT_STATE (rw) register accessor: EP2 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2_int_state`] module"]
#[doc(alias = "EP2_INT_STATE")]
pub type Ep2IntState = crate::Reg<ep2_int_state::Ep2IntStateSpec>;
#[doc = "EP2 interrupt state register"]
pub mod ep2_int_state;
#[doc = "EP3_INT_STATE (rw) register accessor: EP3 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3_int_state`] module"]
#[doc(alias = "EP3_INT_STATE")]
pub type Ep3IntState = crate::Reg<ep3_int_state::Ep3IntStateSpec>;
#[doc = "EP3 interrupt state register"]
pub mod ep3_int_state;
#[doc = "EP4_INT_STATE (rw) register accessor: EP4 interrupt state register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_int_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_int_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4_int_state`] module"]
#[doc(alias = "EP4_INT_STATE")]
pub type Ep4IntState = crate::Reg<ep4_int_state::Ep4IntStateSpec>;
#[doc = "EP4 interrupt state register"]
pub mod ep4_int_state;
#[doc = "EP1_INT_EN (rw) register accessor: EP1 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1_int_en`] module"]
#[doc(alias = "EP1_INT_EN")]
pub type Ep1IntEn = crate::Reg<ep1_int_en::Ep1IntEnSpec>;
#[doc = "EP1 interrupt enable register"]
pub mod ep1_int_en;
#[doc = "EP2_INT_EN (rw) register accessor: EP2 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2_int_en`] module"]
#[doc(alias = "EP2_INT_EN")]
pub type Ep2IntEn = crate::Reg<ep2_int_en::Ep2IntEnSpec>;
#[doc = "EP2 interrupt enable register"]
pub mod ep2_int_en;
#[doc = "EP3_INT_EN (rw) register accessor: EP3 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3_int_en`] module"]
#[doc(alias = "EP3_INT_EN")]
pub type Ep3IntEn = crate::Reg<ep3_int_en::Ep3IntEnSpec>;
#[doc = "EP3 interrupt enable register"]
pub mod ep3_int_en;
#[doc = "EP4_INT_EN (rw) register accessor: EP4 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4_int_en`] module"]
#[doc(alias = "EP4_INT_EN")]
pub type Ep4IntEn = crate::Reg<ep4_int_en::Ep4IntEnSpec>;
#[doc = "EP4 interrupt enable register"]
pub mod ep4_int_en;
#[doc = "ADDR (rw) register accessor: USB address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "USB address"]
pub mod addr;
#[doc = "EP_EN (rw) register accessor: EP Enable End\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_en`] module"]
#[doc(alias = "EP_EN")]
pub type EpEn = crate::Reg<ep_en::EpEnSpec>;
#[doc = "EP Enable End"]
pub mod ep_en;
#[doc = "EP_DMA_DIR (rw) register accessor: DMA End-point direction register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_dma_dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_dma_dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_dma_dir`] module"]
#[doc(alias = "EP_DMA_DIR")]
pub type EpDmaDir = crate::Reg<ep_dma_dir::EpDmaDirSpec>;
#[doc = "DMA End-point direction register"]
pub mod ep_dma_dir;
#[doc = "EP_TYPE (rw) register accessor: Endpoint type register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_type`] module"]
#[doc(alias = "EP_TYPE")]
pub type EpType = crate::Reg<ep_type::EpTypeSpec>;
#[doc = "Endpoint type register"]
pub mod ep_type;
#[doc = "EP_INDEX1_2 (rw) register accessor: End-point index register 1_2\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_index1_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_index1_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_index1_2`] module"]
#[doc(alias = "EP_INDEX1_2")]
pub type EpIndex1_2 = crate::Reg<ep_index1_2::EpIndex1_2Spec>;
#[doc = "End-point index register 1_2"]
pub mod ep_index1_2;
#[doc = "EP_INDEX3_4 (rw) register accessor: End-point index register 3_4\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_index3_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_index3_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_index3_4`] module"]
#[doc(alias = "EP_INDEX3_4")]
pub type EpIndex3_4 = crate::Reg<ep_index3_4::EpIndex3_4Spec>;
#[doc = "End-point index register 3_4"]
pub mod ep_index3_4;
#[doc = "TOG_CTRL1_4 (rw) register accessor: TOG CTRL1_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tog_ctrl1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tog_ctrl1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tog_ctrl1_4`] module"]
#[doc(alias = "TOG_CTRL1_4")]
pub type TogCtrl1_4 = crate::Reg<tog_ctrl1_4::TogCtrl1_4Spec>;
#[doc = "TOG CTRL1_4 register"]
pub mod tog_ctrl1_4;
#[doc = "TOG_STAT1_4 (rw) register accessor: TOG STAT1_4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`tog_stat1_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tog_stat1_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tog_stat1_4`] module"]
#[doc(alias = "TOG_STAT1_4")]
pub type TogStat1_4 = crate::Reg<tog_stat1_4::TogStat1_4Spec>;
#[doc = "TOG STAT1_4 register"]
pub mod tog_stat1_4;
#[doc = "SETUP0 (r) register accessor: Setup Date 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup0`] module"]
#[doc(alias = "SETUP0")]
pub type Setup0 = crate::Reg<setup0::Setup0Spec>;
#[doc = "Setup Date 0 register"]
pub mod setup0;
#[doc = "SETUP1 (r) register accessor: Setup Date 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup1`] module"]
#[doc(alias = "SETUP1")]
pub type Setup1 = crate::Reg<setup1::Setup1Spec>;
#[doc = "Setup Date 1 register"]
pub mod setup1;
#[doc = "SETUP2 (r) register accessor: Setup Date 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup2`] module"]
#[doc(alias = "SETUP2")]
pub type Setup2 = crate::Reg<setup2::Setup2Spec>;
#[doc = "Setup Date 2 register"]
pub mod setup2;
#[doc = "SETUP3 (r) register accessor: Setup Date 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup3`] module"]
#[doc(alias = "SETUP3")]
pub type Setup3 = crate::Reg<setup3::Setup3Spec>;
#[doc = "Setup Date 3 register"]
pub mod setup3;
#[doc = "SETUP4 (r) register accessor: Setup Date 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup4`] module"]
#[doc(alias = "SETUP4")]
pub type Setup4 = crate::Reg<setup4::Setup4Spec>;
#[doc = "Setup Date 4 register"]
pub mod setup4;
#[doc = "SETUP5 (r) register accessor: Setup Date 5 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup5`] module"]
#[doc(alias = "SETUP5")]
pub type Setup5 = crate::Reg<setup5::Setup5Spec>;
#[doc = "Setup Date 5 register"]
pub mod setup5;
#[doc = "SETUP6 (r) register accessor: Setup Date 6 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup6`] module"]
#[doc(alias = "SETUP6")]
pub type Setup6 = crate::Reg<setup6::Setup6Spec>;
#[doc = "Setup Date 6 register"]
pub mod setup6;
#[doc = "SETUP7 (r) register accessor: Setup Date 7 register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup7`] module"]
#[doc(alias = "SETUP7")]
pub type Setup7 = crate::Reg<setup7::Setup7Spec>;
#[doc = "Setup Date 7 register"]
pub mod setup7;
#[doc = "PACKET_SIZEL (rw) register accessor: PACKET SIZEL register\n\nYou can [`read`](crate::Reg::read) this register and get [`packet_sizel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packet_sizel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packet_sizel`] module"]
#[doc(alias = "PACKET_SIZEL")]
pub type PacketSizel = crate::Reg<packet_sizel::PacketSizelSpec>;
#[doc = "PACKET SIZEL register"]
pub mod packet_sizel;
#[doc = "PACKET_SIZEH (rw) register accessor: PACKET SIZEH register\n\nYou can [`read`](crate::Reg::read) this register and get [`packet_sizeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packet_sizeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packet_sizeh`] module"]
#[doc(alias = "PACKET_SIZEH")]
pub type PacketSizeh = crate::Reg<packet_sizeh::PacketSizehSpec>;
#[doc = "PACKET SIZEH register"]
pub mod packet_sizeh;
#[doc = "EP0_AVAIL (r) register accessor: EP0 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_avail::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_avail`] module"]
#[doc(alias = "EP0_AVAIL")]
pub type Ep0Avail = crate::Reg<ep0_avail::Ep0AvailSpec>;
#[doc = "EP0 AVAIL register"]
pub mod ep0_avail;
#[doc = "EP1_AVAIL (r) register accessor: EP1 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_avail::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1_avail`] module"]
#[doc(alias = "EP1_AVAIL")]
pub type Ep1Avail = crate::Reg<ep1_avail::Ep1AvailSpec>;
#[doc = "EP1 AVAIL register"]
pub mod ep1_avail;
#[doc = "EP2_AVAIL (r) register accessor: EP2 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_avail::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2_avail`] module"]
#[doc(alias = "EP2_AVAIL")]
pub type Ep2Avail = crate::Reg<ep2_avail::Ep2AvailSpec>;
#[doc = "EP2 AVAIL register"]
pub mod ep2_avail;
#[doc = "EP3_AVAIL (r) register accessor: EP3 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_avail::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3_avail`] module"]
#[doc(alias = "EP3_AVAIL")]
pub type Ep3Avail = crate::Reg<ep3_avail::Ep3AvailSpec>;
#[doc = "EP3 AVAIL register"]
pub mod ep3_avail;
#[doc = "EP4_AVAIL (r) register accessor: EP4 AVAIL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_avail::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4_avail`] module"]
#[doc(alias = "EP4_AVAIL")]
pub type Ep4Avail = crate::Reg<ep4_avail::Ep4AvailSpec>;
#[doc = "EP4 AVAIL register"]
pub mod ep4_avail;
#[doc = "DAM_ADDR0 (r) register accessor: DMA1 ADDR0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam_addr0`] module"]
#[doc(alias = "DAM_ADDR0")]
pub type DamAddr0 = crate::Reg<dam_addr0::DamAddr0Spec>;
#[doc = "DMA1 ADDR0 register"]
pub mod dam_addr0;
#[doc = "DAM_ADDR1 (r) register accessor: DMA1 ADDR1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam_addr1`] module"]
#[doc(alias = "DAM_ADDR1")]
pub type DamAddr1 = crate::Reg<dam_addr1::DamAddr1Spec>;
#[doc = "DMA1 ADDR1 register"]
pub mod dam_addr1;
#[doc = "DAM_ADDR2 (r) register accessor: DMA1 ADDR2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam_addr2`] module"]
#[doc(alias = "DAM_ADDR2")]
pub type DamAddr2 = crate::Reg<dam_addr2::DamAddr2Spec>;
#[doc = "DMA1 ADDR2 register"]
pub mod dam_addr2;
#[doc = "DAM_ADDR3 (r) register accessor: DMA1 ADDR3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dam_addr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dam_addr3`] module"]
#[doc(alias = "DAM_ADDR3")]
pub type DamAddr3 = crate::Reg<dam_addr3::DamAddr3Spec>;
#[doc = "DMA1 ADDR3 register"]
pub mod dam_addr3;
#[doc = "DMA_NUML (rw) register accessor: DMA NUML register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_numl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_numl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_numl`] module"]
#[doc(alias = "DMA_NUML")]
pub type DmaNuml = crate::Reg<dma_numl::DmaNumlSpec>;
#[doc = "DMA NUML register"]
pub mod dma_numl;
#[doc = "DMA_NUMH (rw) register accessor: DMA NUMH register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_numh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_numh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_numh`] module"]
#[doc(alias = "DMA_NUMH")]
pub type DmaNumh = crate::Reg<dma_numh::DmaNumhSpec>;
#[doc = "DMA NUMH register"]
pub mod dma_numh;
#[doc = "EP0_CTRL (rw) register accessor: EP0 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_ctrl`] module"]
#[doc(alias = "EP0_CTRL")]
pub type Ep0Ctrl = crate::Reg<ep0_ctrl::Ep0CtrlSpec>;
#[doc = "EP0 CTRL register"]
pub mod ep0_ctrl;
#[doc = "EP1_CTRL (rw) register accessor: EP1 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1_ctrl`] module"]
#[doc(alias = "EP1_CTRL")]
pub type Ep1Ctrl = crate::Reg<ep1_ctrl::Ep1CtrlSpec>;
#[doc = "EP1 CTRL register"]
pub mod ep1_ctrl;
#[doc = "EP2_CTRL (rw) register accessor: EP2 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2_ctrl`] module"]
#[doc(alias = "EP2_CTRL")]
pub type Ep2Ctrl = crate::Reg<ep2_ctrl::Ep2CtrlSpec>;
#[doc = "EP2 CTRL register"]
pub mod ep2_ctrl;
#[doc = "EP3_CTRL (rw) register accessor: EP3 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3_ctrl`] module"]
#[doc(alias = "EP3_CTRL")]
pub type Ep3Ctrl = crate::Reg<ep3_ctrl::Ep3CtrlSpec>;
#[doc = "EP3 CTRL register"]
pub mod ep3_ctrl;
#[doc = "EP4_CTRL (rw) register accessor: EP4 CTRL register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4_ctrl`] module"]
#[doc(alias = "EP4_CTRL")]
pub type Ep4Ctrl = crate::Reg<ep4_ctrl::Ep4CtrlSpec>;
#[doc = "EP4 CTRL register"]
pub mod ep4_ctrl;
#[doc = "EP0_FIFO (rw) register accessor: EP0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_fifo`] module"]
#[doc(alias = "EP0_FIFO")]
pub type Ep0Fifo = crate::Reg<ep0_fifo::Ep0FifoSpec>;
#[doc = "EP0 FIFO register"]
pub mod ep0_fifo;
#[doc = "EP1_FIFO (rw) register accessor: EP1 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep1_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep1_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1_fifo`] module"]
#[doc(alias = "EP1_FIFO")]
pub type Ep1Fifo = crate::Reg<ep1_fifo::Ep1FifoSpec>;
#[doc = "EP1 FIFO register"]
pub mod ep1_fifo;
#[doc = "EP2_FIFO (rw) register accessor: EP2 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep2_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep2_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2_fifo`] module"]
#[doc(alias = "EP2_FIFO")]
pub type Ep2Fifo = crate::Reg<ep2_fifo::Ep2FifoSpec>;
#[doc = "EP2 FIFO register"]
pub mod ep2_fifo;
#[doc = "EP3_FIFO (rw) register accessor: EP3 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep3_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep3_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep3_fifo`] module"]
#[doc(alias = "EP3_FIFO")]
pub type Ep3Fifo = crate::Reg<ep3_fifo::Ep3FifoSpec>;
#[doc = "EP3 FIFO register"]
pub mod ep3_fifo;
#[doc = "EP4_FIFO (rw) register accessor: EP4 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep4_fifo`] module"]
#[doc(alias = "EP4_FIFO")]
pub type Ep4Fifo = crate::Reg<ep4_fifo::Ep4FifoSpec>;
#[doc = "EP4 FIFO register"]
pub mod ep4_fifo;
#[doc = "EP_MEM (rw) register accessor: EP MEM register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_mem`] module"]
#[doc(alias = "EP_MEM")]
pub type EpMem = crate::Reg<ep_mem::EpMemSpec>;
#[doc = "EP MEM register"]
pub mod ep_mem;
#[doc = "EP_DMA (rw) register accessor: EP DMA register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_dma`] module"]
#[doc(alias = "EP_DMA")]
pub type EpDma = crate::Reg<ep_dma::EpDmaSpec>;
#[doc = "EP DMA register"]
pub mod ep_dma;
#[doc = "EP_HALT (rw) register accessor: EP HALT register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_halt`] module"]
#[doc(alias = "EP_HALT")]
pub type EpHalt = crate::Reg<ep_halt::EpHaltSpec>;
#[doc = "EP HALT register"]
pub mod ep_halt;
#[doc = "POWER (rw) register accessor: Power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Power control register"]
pub mod power;
#[doc = "USB_AHB_DMA (rw) register accessor: USB AHB DMA register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ahb_dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ahb_dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ahb_dma`] module"]
#[doc(alias = "USB_AHB_DMA")]
pub type UsbAhbDma = crate::Reg<usb_ahb_dma::UsbAhbDmaSpec>;
#[doc = "USB AHB DMA register"]
pub mod usb_ahb_dma;
#[doc = "USB_AHB_RST (rw) register accessor: USB AHB RST register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ahb_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ahb_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ahb_rst`] module"]
#[doc(alias = "USB_AHB_RST")]
pub type UsbAhbRst = crate::Reg<usb_ahb_rst::UsbAhbRstSpec>;
#[doc = "USB AHB RST register"]
pub mod usb_ahb_rst;
