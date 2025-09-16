#[doc = "bit count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcnt(pub u32);
impl Bcnt {
    #[doc = "send or receive bit count"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "send or receive bit count"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bcnt {
    #[inline(always)]
    fn default() -> Bcnt {
        Bcnt(0)
    }
}
impl core::fmt::Debug for Bcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bcnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bcnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Configure Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "start select"]
    #[must_use]
    #[inline(always)]
    pub const fn startsel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start select"]
    #[inline(always)]
    pub const fn set_startsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "stop select"]
    #[must_use]
    #[inline(always)]
    pub const fn stopsel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "stop select"]
    #[inline(always)]
    pub const fn set_stopsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "master or slave select"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "master or slave select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Vtx enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vtxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Vtx enable"]
    #[inline(always)]
    pub const fn set_vtxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Vtx select"]
    #[must_use]
    #[inline(always)]
    pub const fn vtxsel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Vtx select"]
    #[inline(always)]
    pub const fn set_vtxsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "max tx and rx bit"]
    #[must_use]
    #[inline(always)]
    pub const fn maxbit(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "max tx and rx bit"]
    #[inline(always)]
    pub const fn set_maxbit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("startsel", &self.startsel())
            .field("stopsel", &self.stopsel())
            .field("msel", &self.msel())
            .field("vtxen", &self.vtxen())
            .field("vtxsel", &self.vtxsel())
            .field("maxbit", &self.maxbit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ startsel: {=bool:?}, stopsel: {=bool:?}, msel: {=bool:?}, vtxen: {=bool:?}, vtxsel: {=bool:?}, maxbit: {=u16:?} }}",
            self.startsel(),
            self.stopsel(),
            self.msel(),
            self.vtxen(),
            self.vtxsel(),
            self.maxbit()
        )
    }
}
#[doc = "Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl1(pub u32);
impl Ctl1 {
    #[doc = "enable Control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable Control bit"]
    #[inline(always)]
    pub const fn set_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "idel state polority"]
    #[must_use]
    #[inline(always)]
    pub const fn idlep1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "idel state polority"]
    #[inline(always)]
    pub const fn set_idlep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "input signal select"]
    #[must_use]
    #[inline(always)]
    pub const fn insel1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "input signal select"]
    #[inline(always)]
    pub const fn set_insel1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable bit"]
    #[inline(always)]
    pub const fn set_dmaen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TXandRX data select control"]
    #[must_use]
    #[inline(always)]
    pub const fn txsel1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TXandRX data select control"]
    #[inline(always)]
    pub const fn set_txsel1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctl1 {
    #[inline(always)]
    fn default() -> Ctl1 {
        Ctl1(0)
    }
}
impl core::fmt::Debug for Ctl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl1")
            .field("en1", &self.en1())
            .field("idlep1", &self.idlep1())
            .field("insel1", &self.insel1())
            .field("dmaen1", &self.dmaen1())
            .field("txsel1", &self.txsel1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl1 {{ en1: {=bool:?}, idlep1: {=bool:?}, insel1: {=bool:?}, dmaen1: {=bool:?}, txsel1: {=bool:?} }}",
            self.en1(),
            self.idlep1(),
            self.insel1(),
            self.dmaen1(),
            self.txsel1()
        )
    }
}
#[doc = "Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl2(pub u32);
impl Ctl2 {
    #[doc = "enable Control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn en2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable Control bit"]
    #[inline(always)]
    pub const fn set_en2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "idel state polority"]
    #[must_use]
    #[inline(always)]
    pub const fn idlep2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "idel state polority"]
    #[inline(always)]
    pub const fn set_idlep2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "input signal select"]
    #[must_use]
    #[inline(always)]
    pub const fn insel2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "input signal select"]
    #[inline(always)]
    pub const fn set_insel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA enable bit"]
    #[inline(always)]
    pub const fn set_dmaen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TXandRX data select control"]
    #[must_use]
    #[inline(always)]
    pub const fn txsel2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TXandRX data select control"]
    #[inline(always)]
    pub const fn set_txsel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctl2 {
    #[inline(always)]
    fn default() -> Ctl2 {
        Ctl2(0)
    }
}
impl core::fmt::Debug for Ctl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl2")
            .field("en2", &self.en2())
            .field("idlep2", &self.idlep2())
            .field("insel2", &self.insel2())
            .field("dmaen2", &self.dmaen2())
            .field("txsel2", &self.txsel2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl2 {{ en2: {=bool:?}, idlep2: {=bool:?}, insel2: {=bool:?}, dmaen2: {=bool:?}, txsel2: {=bool:?} }}",
            self.en2(),
            self.idlep2(),
            self.insel2(),
            self.dmaen2(),
            self.txsel2()
        )
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Transmit FIFO available interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_ien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO available interrupt enable"]
    #[inline(always)]
    pub const fn set_tx_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive data available interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ien(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data available interrupt enable"]
    #[inline(always)]
    pub const fn set_rx_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txc_ien(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete interrupt enable"]
    #[inline(always)]
    pub const fn set_txc_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start Reveive data interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn start_ien(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start Reveive data interrupt enable"]
    #[inline(always)]
    pub const fn set_start_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Stop Reveive data interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_ien(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Reveive data interrupt enable"]
    #[inline(always)]
    pub const fn set_stop_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("tx_ien", &self.tx_ien())
            .field("rx_ien", &self.rx_ien())
            .field("txc_ien", &self.txc_ien())
            .field("start_ien", &self.start_ien())
            .field("stop_ien", &self.stop_ien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ tx_ien: {=bool:?}, rx_ien: {=bool:?}, txc_ien: {=bool:?}, start_ien: {=bool:?}, stop_ien: {=bool:?} }}",
            self.tx_ien(),
            self.rx_ien(),
            self.txc_ien(),
            self.start_ien(),
            self.stop_ien()
        )
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Transmit FIFO available interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_intf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO available interrupt flag"]
    #[inline(always)]
    pub const fn set_tx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive data available interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_intf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data available interrupt flag"]
    #[inline(always)]
    pub const fn set_rx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit complete interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn txc_intf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete interrupt flag"]
    #[inline(always)]
    pub const fn set_txc_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Start Reveive data interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn start_intf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Start Reveive data interrupt flag"]
    #[inline(always)]
    pub const fn set_start_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Stop Reveive data interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_intf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Reveive data interrupt flag"]
    #[inline(always)]
    pub const fn set_stop_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("tx_intf", &self.tx_intf())
            .field("rx_intf", &self.rx_intf())
            .field("txc_intf", &self.txc_intf())
            .field("start_intf", &self.start_intf())
            .field("stop_intf", &self.stop_intf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ tx_intf: {=bool:?}, rx_intf: {=bool:?}, txc_intf: {=bool:?}, start_intf: {=bool:?}, stop_intf: {=bool:?} }}",
            self.tx_intf(),
            self.rx_intf(),
            self.txc_intf(),
            self.start_intf(),
            self.stop_intf()
        )
    }
}
#[doc = "Reveive register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxreg1(pub u32);
impl Rxreg1 {
    #[doc = "Receive data"]
    #[must_use]
    #[inline(always)]
    pub const fn rxreg1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive data"]
    #[inline(always)]
    pub const fn set_rxreg1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxreg1 {
    #[inline(always)]
    fn default() -> Rxreg1 {
        Rxreg1(0)
    }
}
impl core::fmt::Debug for Rxreg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxreg1")
            .field("rxreg1", &self.rxreg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxreg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxreg1 {{ rxreg1: {=u32:?} }}", self.rxreg1())
    }
}
#[doc = "Reveive register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxreg2(pub u32);
impl Rxreg2 {
    #[doc = "Receive data"]
    #[must_use]
    #[inline(always)]
    pub const fn rxreg2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive data"]
    #[inline(always)]
    pub const fn set_rxreg2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxreg2 {
    #[inline(always)]
    fn default() -> Rxreg2 {
        Rxreg2(0)
    }
}
impl core::fmt::Debug for Rxreg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxreg2")
            .field("rxreg2", &self.rxreg2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxreg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxreg2 {{ rxreg2: {=u32:?} }}", self.rxreg2())
    }
}
#[doc = "Baud rate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbrg(pub u32);
impl Spbrg {
    #[doc = "baud rate control register for simple data"]
    #[must_use]
    #[inline(always)]
    pub const fn spbrg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "baud rate control register for simple data"]
    #[inline(always)]
    pub const fn set_spbrg(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Spbrg {
    #[inline(always)]
    fn default() -> Spbrg {
        Spbrg(0)
    }
}
impl core::fmt::Debug for Spbrg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spbrg")
            .field("spbrg", &self.spbrg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spbrg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Spbrg {{ spbrg: {=u16:?} }}", self.spbrg())
    }
}
#[doc = "Transmit register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txreg1(pub u32);
impl Txreg1 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn txreg1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_txreg1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txreg1 {
    #[inline(always)]
    fn default() -> Txreg1 {
        Txreg1(0)
    }
}
impl core::fmt::Debug for Txreg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txreg1")
            .field("txreg1", &self.txreg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txreg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txreg1 {{ txreg1: {=u32:?} }}", self.txreg1())
    }
}
#[doc = "Transmit register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txreg2(pub u32);
impl Txreg2 {
    #[doc = "Transfer data"]
    #[must_use]
    #[inline(always)]
    pub const fn txreg2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transfer data"]
    #[inline(always)]
    pub const fn set_txreg2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txreg2 {
    #[inline(always)]
    fn default() -> Txreg2 {
        Txreg2(0)
    }
}
impl core::fmt::Debug for Txreg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txreg2")
            .field("txreg2", &self.txreg2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txreg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txreg2 {{ txreg2: {=u32:?} }}", self.txreg2())
    }
}
