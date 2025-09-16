#[doc = "CCTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctl(pub u32);
impl Cctl {
    #[doc = "Clock phase select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock phase select bit"]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clock polarity select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clock polarity select bit"]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LSI first enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lsbfe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LSI first enable bit"]
    #[inline(always)]
    pub const fn set_lsbfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SPI character length bit"]
    #[must_use]
    #[inline(always)]
    pub const fn spilen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SPI character length bit"]
    #[inline(always)]
    pub const fn set_spilen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive data edge select"]
    #[must_use]
    #[inline(always)]
    pub const fn rxedge(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data edge select"]
    #[inline(always)]
    pub const fn set_rxedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit data edge select"]
    #[must_use]
    #[inline(always)]
    pub const fn txedge(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit data edge select"]
    #[inline(always)]
    pub const fn set_txedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CPHA polarity select"]
    #[must_use]
    #[inline(always)]
    pub const fn cphasel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CPHA polarity select"]
    #[inline(always)]
    pub const fn set_cphasel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "High speed slave mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hispd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "High speed slave mode"]
    #[inline(always)]
    pub const fn set_hispd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cctl {
    #[inline(always)]
    fn default() -> Cctl {
        Cctl(0)
    }
}
impl core::fmt::Debug for Cctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cctl")
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("lsbfe", &self.lsbfe())
            .field("spilen", &self.spilen())
            .field("rxedge", &self.rxedge())
            .field("txedge", &self.txedge())
            .field("cphasel", &self.cphasel())
            .field("hispd", &self.hispd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cctl {{ cpha: {=bool:?}, cpol: {=bool:?}, lsbfe: {=bool:?}, spilen: {=bool:?}, rxedge: {=bool:?}, txedge: {=bool:?}, cphasel: {=bool:?}, hispd: {=bool:?} }}",
            self.cpha(),
            self.cpol(),
            self.lsbfe(),
            self.spilen(),
            self.rxedge(),
            self.txedge(),
            self.cphasel(),
            self.hispd()
        )
    }
}
#[doc = "CSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cstat(pub u32);
impl Cstat {
    #[doc = "Transmitter empty bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txept(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter empty bit"]
    #[inline(always)]
    pub const fn set_txept(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive available byte data message"]
    #[must_use]
    #[inline(always)]
    pub const fn rxavl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive available byte data message"]
    #[inline(always)]
    pub const fn set_rxavl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter FIFO full status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter FIFO full status bit"]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive available 4 byte data message"]
    #[must_use]
    #[inline(always)]
    pub const fn rxavl_4byte(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive available 4 byte data message"]
    #[inline(always)]
    pub const fn set_rxavl_4byte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO address"]
    #[must_use]
    #[inline(always)]
    pub const fn txfaddr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO address"]
    #[inline(always)]
    pub const fn set_txfaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Transmit FIFO address"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfaddr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO address"]
    #[inline(always)]
    pub const fn set_rxfaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Cstat {
    #[inline(always)]
    fn default() -> Cstat {
        Cstat(0)
    }
}
impl core::fmt::Debug for Cstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cstat")
            .field("txept", &self.txept())
            .field("rxavl", &self.rxavl())
            .field("txfull", &self.txfull())
            .field("rxavl_4byte", &self.rxavl_4byte())
            .field("txfaddr", &self.txfaddr())
            .field("rxfaddr", &self.rxfaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cstat {{ txept: {=bool:?}, rxavl: {=bool:?}, txfull: {=bool:?}, rxavl_4byte: {=bool:?}, txfaddr: {=u8:?}, rxfaddr: {=u8:?} }}",
            self.txept(),
            self.rxavl(),
            self.txfull(),
            self.rxavl_4byte(),
            self.txfaddr(),
            self.rxfaddr()
        )
    }
}
#[doc = "EXTCTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extctl(pub u32);
impl Extctl {
    #[doc = "Control SPI data length"]
    #[must_use]
    #[inline(always)]
    pub const fn extlen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Control SPI data length"]
    #[inline(always)]
    pub const fn set_extlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Extctl {
    #[inline(always)]
    fn default() -> Extctl {
        Extctl(0)
    }
}
impl core::fmt::Debug for Extctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Extctl")
            .field("extlen", &self.extlen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Extctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Extctl {{ extlen: {=u8:?} }}", self.extlen())
    }
}
#[doc = "GCTL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gctl(pub u32);
impl Gctl {
    #[doc = "SPI select bit"]
    #[must_use]
    #[inline(always)]
    pub const fn spien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPI select bit"]
    #[inline(always)]
    pub const fn set_spien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPI interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPI interrupt enable bit"]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master mode bit"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master mode bit"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit enable bit"]
    #[inline(always)]
    pub const fn set_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive enable bit"]
    #[inline(always)]
    pub const fn set_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX FIFO trigger level bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxtlf(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "RX FIFO trigger level bit"]
    #[inline(always)]
    pub const fn set_rxtlf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "TX FIFO trigger level bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txtlf(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "TX FIFO trigger level bit"]
    #[inline(always)]
    pub const fn set_txtlf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "DMA access mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA access mode enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "NSS select signal that from software and hardware"]
    #[must_use]
    #[inline(always)]
    pub const fn nss_sel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "NSS select signal that from software and hardware"]
    #[inline(always)]
    pub const fn set_nss_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Valid byte or double-word data select signal"]
    #[must_use]
    #[inline(always)]
    pub const fn dw8_32(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Valid byte or double-word data select signal"]
    #[inline(always)]
    pub const fn set_dw8_32(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NSS selection signal is automatically flipped"]
    #[must_use]
    #[inline(always)]
    pub const fn nsstog(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NSS selection signal is automatically flipped"]
    #[inline(always)]
    pub const fn set_nsstog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Gctl {
    #[inline(always)]
    fn default() -> Gctl {
        Gctl(0)
    }
}
impl core::fmt::Debug for Gctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gctl")
            .field("spien", &self.spien())
            .field("int_en", &self.int_en())
            .field("mode", &self.mode())
            .field("txen", &self.txen())
            .field("rxen", &self.rxen())
            .field("rxtlf", &self.rxtlf())
            .field("txtlf", &self.txtlf())
            .field("dmaen", &self.dmaen())
            .field("nss_sel", &self.nss_sel())
            .field("dw8_32", &self.dw8_32())
            .field("nsstog", &self.nsstog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gctl {{ spien: {=bool:?}, int_en: {=bool:?}, mode: {=bool:?}, txen: {=bool:?}, rxen: {=bool:?}, rxtlf: {=u8:?}, txtlf: {=u8:?}, dmaen: {=bool:?}, nss_sel: {=bool:?}, dw8_32: {=bool:?}, nsstog: {=bool:?} }}",
            self.spien(),
            self.int_en(),
            self.mode(),
            self.txen(),
            self.rxen(),
            self.rxtlf(),
            self.txtlf(),
            self.dmaen(),
            self.nss_sel(),
            self.dw8_32(),
            self.nsstog()
        )
    }
}
#[doc = "INTCLR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Transmitter FIFO empty interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_iclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter FIFO empty interrupt clear bit"]
    #[inline(always)]
    pub const fn set_tx_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_iclr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rx_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn underrun_iclr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter underrun interrupt clear bit(SPI slave mode only)"]
    #[inline(always)]
    pub const fn set_underrun_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerr_iclr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxoerr_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive completed interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmatch_iclr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive completed interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxmatch_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receiver buffer full interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull_iclr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver buffer full interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxfull_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter empty interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txept_iclr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter empty interrupt clear bit"]
    #[inline(always)]
    pub const fn set_txept_iclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Intclr {
    #[inline(always)]
    fn default() -> Intclr {
        Intclr(0)
    }
}
impl core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intclr")
            .field("tx_iclr", &self.tx_iclr())
            .field("rx_iclr", &self.rx_iclr())
            .field("underrun_iclr", &self.underrun_iclr())
            .field("rxoerr_iclr", &self.rxoerr_iclr())
            .field("rxmatch_iclr", &self.rxmatch_iclr())
            .field("rxfull_iclr", &self.rxfull_iclr())
            .field("txept_iclr", &self.txept_iclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intclr {{ tx_iclr: {=bool:?}, rx_iclr: {=bool:?}, underrun_iclr: {=bool:?}, rxoerr_iclr: {=bool:?}, rxmatch_iclr: {=bool:?}, rxfull_iclr: {=bool:?}, txept_iclr: {=bool:?} }}",
            self.tx_iclr(),
            self.rx_iclr(),
            self.underrun_iclr(),
            self.rxoerr_iclr(),
            self.rxmatch_iclr(),
            self.rxfull_iclr(),
            self.txept_iclr()
        )
    }
}
#[doc = "INTEN"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Transmit FIFO empty interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_ien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty interrupt enable bit"]
    #[inline(always)]
    pub const fn set_tx_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ien(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rx_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn underrun_ien(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter underrun interrupt enable bit(SPI slave mode only)"]
    #[inline(always)]
    pub const fn set_underrun_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerr_ien(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxoerr_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive data complete interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmatch_ien(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data complete interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxmatch_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO full interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull_ien(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxfull_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit empty interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txept_ien(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit empty interrupt enable bit"]
    #[inline(always)]
    pub const fn set_txept_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            .field("underrun_ien", &self.underrun_ien())
            .field("rxoerr_ien", &self.rxoerr_ien())
            .field("rxmatch_ien", &self.rxmatch_ien())
            .field("rxfull_ien", &self.rxfull_ien())
            .field("txept_ien", &self.txept_ien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ tx_ien: {=bool:?}, rx_ien: {=bool:?}, underrun_ien: {=bool:?}, rxoerr_ien: {=bool:?}, rxmatch_ien: {=bool:?}, rxfull_ien: {=bool:?}, txept_ien: {=bool:?} }}",
            self.tx_ien(),
            self.rx_ien(),
            self.underrun_ien(),
            self.rxoerr_ien(),
            self.rxmatch_ien(),
            self.rxfull_ien(),
            self.txept_ien()
        )
    }
}
#[doc = "INTSTAT"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Transmit FIFO avialable interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_intf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO avialable interrupt flag bit"]
    #[inline(always)]
    pub const fn set_tx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive data available interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_intf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data available interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI underrun interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn underrun_intf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SPI underrun interrupt flag bit"]
    #[inline(always)]
    pub const fn set_underrun_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive overrun error interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerr_intf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun error interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxoerr_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive data match the RXDNR number"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmatch_intf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive data match the RXDNR number"]
    #[inline(always)]
    pub const fn set_rxmatch_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX FIFO full interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfull_intf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO full interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxfull_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmitter empty interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txept_intf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter empty interrupt flag bit"]
    #[inline(always)]
    pub const fn set_txept_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            .field("underrun_intf", &self.underrun_intf())
            .field("rxoerr_intf", &self.rxoerr_intf())
            .field("rxmatch_intf", &self.rxmatch_intf())
            .field("rxfull_intf", &self.rxfull_intf())
            .field("txept_intf", &self.txept_intf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ tx_intf: {=bool:?}, rx_intf: {=bool:?}, underrun_intf: {=bool:?}, rxoerr_intf: {=bool:?}, rxmatch_intf: {=bool:?}, rxfull_intf: {=bool:?}, txept_intf: {=bool:?} }}",
            self.tx_intf(),
            self.rx_intf(),
            self.underrun_intf(),
            self.rxoerr_intf(),
            self.rxmatch_intf(),
            self.rxfull_intf(),
            self.txept_intf()
        )
    }
}
#[doc = "NSSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nssr(pub u32);
impl Nssr {
    #[doc = "Chip select output signal in Master mode"]
    #[must_use]
    #[inline(always)]
    pub const fn nss(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Chip select output signal in Master mode"]
    #[inline(always)]
    pub const fn set_nss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Nssr {
    #[inline(always)]
    fn default() -> Nssr {
        Nssr(0)
    }
}
impl core::fmt::Debug for Nssr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nssr").field("nss", &self.nss()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nssr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Nssr {{ nss: {=bool:?} }}", self.nss())
    }
}
#[doc = "RXDNR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdnr(pub u32);
impl Rxdnr {
    #[doc = "The register is used to hold a count of to be received bytes in next receive process"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdnr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The register is used to hold a count of to be received bytes in next receive process"]
    #[inline(always)]
    pub const fn set_rxdnr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Rxdnr {
    #[inline(always)]
    fn default() -> Rxdnr {
        Rxdnr(0)
    }
}
impl core::fmt::Debug for Rxdnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxdnr")
            .field("rxdnr", &self.rxdnr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxdnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxdnr {{ rxdnr: {=u16:?} }}", self.rxdnr())
    }
}
#[doc = "RXREG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxreg(pub u32);
impl Rxreg {
    #[doc = "Receive data register"]
    #[must_use]
    #[inline(always)]
    pub const fn rxreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive data register"]
    #[inline(always)]
    pub const fn set_rxreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxreg {
    #[inline(always)]
    fn default() -> Rxreg {
        Rxreg(0)
    }
}
impl core::fmt::Debug for Rxreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxreg")
            .field("rxreg", &self.rxreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxreg {{ rxreg: {=u32:?} }}", self.rxreg())
    }
}
#[doc = "SPBRG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbrg(pub u32);
impl Spbrg {
    #[doc = "SPI baud rate control register for baud rate"]
    #[must_use]
    #[inline(always)]
    pub const fn spbrg(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SPI baud rate control register for baud rate"]
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
#[doc = "TXREG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txreg(pub u32);
impl Txreg {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txreg {
    #[inline(always)]
    fn default() -> Txreg {
        Txreg(0)
    }
}
impl core::fmt::Debug for Txreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txreg")
            .field("txreg", &self.txreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txreg {{ txreg: {=u32:?} }}", self.txreg())
    }
}
