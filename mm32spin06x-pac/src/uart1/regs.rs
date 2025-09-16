#[doc = "Automatic Baud Rate Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abrcr(pub u32);
impl Abrcr {
    #[doc = "Automatic baud rate enable"]
    #[must_use]
    #[inline(always)]
    pub const fn abren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate enable"]
    #[inline(always)]
    pub const fn set_abren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Automatic baud rate detection length"]
    #[must_use]
    #[inline(always)]
    pub const fn abr_bitcnt(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Automatic baud rate detection length"]
    #[inline(always)]
    pub const fn set_abr_bitcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Auto baud rate previous edge selection"]
    #[must_use]
    #[inline(always)]
    pub const fn former_edge(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate previous edge selection"]
    #[inline(always)]
    pub const fn set_former_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatic baud rate after edge selection"]
    #[must_use]
    #[inline(always)]
    pub const fn later_edge(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate after edge selection"]
    #[inline(always)]
    pub const fn set_later_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Abrcr {
    #[inline(always)]
    fn default() -> Abrcr {
        Abrcr(0)
    }
}
impl core::fmt::Debug for Abrcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Abrcr")
            .field("abren", &self.abren())
            .field("abr_bitcnt", &self.abr_bitcnt())
            .field("former_edge", &self.former_edge())
            .field("later_edge", &self.later_edge())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abrcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Abrcr {{ abren: {=bool:?}, abr_bitcnt: {=u8:?}, former_edge: {=bool:?}, later_edge: {=bool:?} }}",
            self.abren(),
            self.abr_bitcnt(),
            self.former_edge(),
            self.later_edge()
        )
    }
}
#[doc = "Baud rate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr(pub u32);
impl Brr {
    #[doc = "Mantissa part of UARTDIV"]
    #[must_use]
    #[inline(always)]
    pub const fn div_mantissa(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Mantissa part of UARTDIV"]
    #[inline(always)]
    pub const fn set_div_mantissa(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        Brr(0)
    }
}
impl core::fmt::Debug for Brr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Brr")
            .field("div_mantissa", &self.div_mantissa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Brr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Brr {{ div_mantissa: {=u16:?} }}", self.div_mantissa())
    }
}
#[doc = "common control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Parity enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Parity enable bit"]
    #[inline(always)]
    pub const fn set_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity selection bit"]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop bit 0 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn spb0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop bit 0 selection"]
    #[inline(always)]
    pub const fn set_spb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UART transmit frame break"]
    #[must_use]
    #[inline(always)]
    pub const fn brk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UART transmit frame break"]
    #[inline(always)]
    pub const fn set_brk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UART width bit"]
    #[must_use]
    #[inline(always)]
    pub const fn char(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "UART width bit"]
    #[inline(always)]
    pub const fn set_char(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Stop bit 1 selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn spb1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stop bit 1 selection bit"]
    #[inline(always)]
    pub const fn set_spb1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Synchronous frame receive"]
    #[must_use]
    #[inline(always)]
    pub const fn b8rxd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous frame receive"]
    #[inline(always)]
    pub const fn set_b8rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Synchronous frame transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn b8txd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous frame transmit"]
    #[inline(always)]
    pub const fn set_b8txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Synchronous frame polarity control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn b8pol(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous frame polarity control bit"]
    #[inline(always)]
    pub const fn set_b8pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Synchronous frame auto toggle bit"]
    #[must_use]
    #[inline(always)]
    pub const fn b8tog(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous frame auto toggle bit"]
    #[inline(always)]
    pub const fn set_b8tog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Synchronous frame enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn b8en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous frame enable bit"]
    #[inline(always)]
    pub const fn set_b8en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive wake up method"]
    #[must_use]
    #[inline(always)]
    pub const fn rwu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive wake up method"]
    #[inline(always)]
    pub const fn set_rwu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Wake up method"]
    #[must_use]
    #[inline(always)]
    pub const fn wake(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Wake up method"]
    #[inline(always)]
    pub const fn set_wake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UART LIN enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lin(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UART LIN enable bit"]
    #[inline(always)]
    pub const fn set_lin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("pen", &self.pen())
            .field("psel", &self.psel())
            .field("spb0", &self.spb0())
            .field("brk", &self.brk())
            .field("char", &self.char())
            .field("spb1", &self.spb1())
            .field("b8rxd", &self.b8rxd())
            .field("b8txd", &self.b8txd())
            .field("b8pol", &self.b8pol())
            .field("b8tog", &self.b8tog())
            .field("b8en", &self.b8en())
            .field("rwu", &self.rwu())
            .field("wake", &self.wake())
            .field("lin", &self.lin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ pen: {=bool:?}, psel: {=bool:?}, spb0: {=bool:?}, brk: {=bool:?}, char: {=u8:?}, spb1: {=bool:?}, b8rxd: {=bool:?}, b8txd: {=bool:?}, b8pol: {=bool:?}, b8tog: {=bool:?}, b8en: {=bool:?}, rwu: {=bool:?}, wake: {=bool:?}, lin: {=bool:?} }}",
            self.pen(),
            self.psel(),
            self.spb0(),
            self.brk(),
            self.char(),
            self.spb1(),
            self.b8rxd(),
            self.b8txd(),
            self.b8pol(),
            self.b8tog(),
            self.b8en(),
            self.rwu(),
            self.wake(),
            self.lin()
        )
    }
}
#[doc = "Current status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Transmit complete flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete flag bit"]
    #[inline(always)]
    pub const fn set_txc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive valid data flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxavl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive valid data flag bit"]
    #[inline(always)]
    pub const fn set_rxavl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit buffer full flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer full flag bit"]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit buffer empty flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txbuf_empty(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty flag bit"]
    #[inline(always)]
    pub const fn set_txbuf_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("txc", &self.txc())
            .field("rxavl", &self.rxavl())
            .field("txfull", &self.txfull())
            .field("txbuf_empty", &self.txbuf_empty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csr {{ txc: {=bool:?}, rxavl: {=bool:?}, txfull: {=bool:?}, txbuf_empty: {=bool:?} }}",
            self.txc(),
            self.rxavl(),
            self.txfull(),
            self.txbuf_empty()
        )
    }
}
#[doc = "Fractional baud rate register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fra(pub u32);
impl Fra {
    #[doc = "Fractional part of UARTDIV"]
    #[must_use]
    #[inline(always)]
    pub const fn div_fraction(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional part of UARTDIV"]
    #[inline(always)]
    pub const fn set_div_fraction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Fra {
    #[inline(always)]
    fn default() -> Fra {
        Fra(0)
    }
}
impl core::fmt::Debug for Fra {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fra")
            .field("div_fraction", &self.div_fraction())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fra {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fra {{ div_fraction: {=u8:?} }}", self.div_fraction())
    }
}
#[doc = "Global control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "UART mode selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn uarten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART mode selection bit"]
    #[inline(always)]
    pub const fn set_uarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA mode selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dmamode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA mode selection bit"]
    #[inline(always)]
    pub const fn set_dmamode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Automatic flow control enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn autoflowen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic flow control enable bit"]
    #[inline(always)]
    pub const fn set_autoflowen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable receive"]
    #[must_use]
    #[inline(always)]
    pub const fn rxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable receive"]
    #[inline(always)]
    pub const fn set_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn txen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable transmit"]
    #[inline(always)]
    pub const fn set_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Select bit8"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_b8(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Select bit8"]
    #[inline(always)]
    pub const fn set_sel_b8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "change swap"]
    #[must_use]
    #[inline(always)]
    pub const fn swap(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "change swap"]
    #[inline(always)]
    pub const fn set_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Toggle RX"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_tog(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Toggle RX"]
    #[inline(always)]
    pub const fn set_rx_tog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Toggle TX"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_tog(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Toggle TX"]
    #[inline(always)]
    pub const fn set_tx_tog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        Gcr(0)
    }
}
impl core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr")
            .field("uarten", &self.uarten())
            .field("dmamode", &self.dmamode())
            .field("autoflowen", &self.autoflowen())
            .field("rxen", &self.rxen())
            .field("txen", &self.txen())
            .field("sel_b8", &self.sel_b8())
            .field("swap", &self.swap())
            .field("rx_tog", &self.rx_tog())
            .field("tx_tog", &self.tx_tog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcr {{ uarten: {=bool:?}, dmamode: {=bool:?}, autoflowen: {=bool:?}, rxen: {=bool:?}, txen: {=bool:?}, sel_b8: {=bool:?}, swap: {=bool:?}, rx_tog: {=bool:?}, tx_tog: {=bool:?} }}",
            self.uarten(),
            self.dmamode(),
            self.autoflowen(),
            self.rxen(),
            self.txen(),
            self.sel_b8(),
            self.swap(),
            self.rx_tog(),
            self.tx_tog()
        )
    }
}
#[doc = "Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Transmit buffer empty interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txiclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty interrupt clear bit"]
    #[inline(always)]
    pub const fn set_txiclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxiclr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxiclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit complete interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txc_clr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete interrupt clear bit"]
    #[inline(always)]
    pub const fn set_txc_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive overflow error interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerrclr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overflow error interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxoerrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parity error interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxperrclr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxperrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame error interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxferrclr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame error interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxferrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive frame break interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxbrkclr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive frame break interrupt clear bit"]
    #[inline(always)]
    pub const fn set_rxbrkclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Break Frame Interrupt clear Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txbrk_clr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Break Frame Interrupt clear Bit"]
    #[inline(always)]
    pub const fn set_txbrk_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Bit 8 Interrupt clear Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb8_clr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub const fn set_rxb8_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Bit 8 Interrupt clear Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxidlclr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Interrupt clear Bit"]
    #[inline(always)]
    pub const fn set_rxidlclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Auto baud rate end interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn abrend_clr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate end interrupt clear bit"]
    #[inline(always)]
    pub const fn set_abrend_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Auto baud rate error interrupt clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn abrerr_clr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Auto baud rate error interrupt clear bit"]
    #[inline(always)]
    pub const fn set_abrerr_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("txiclr", &self.txiclr())
            .field("rxiclr", &self.rxiclr())
            .field("txc_clr", &self.txc_clr())
            .field("rxoerrclr", &self.rxoerrclr())
            .field("rxperrclr", &self.rxperrclr())
            .field("rxferrclr", &self.rxferrclr())
            .field("rxbrkclr", &self.rxbrkclr())
            .field("txbrk_clr", &self.txbrk_clr())
            .field("rxb8_clr", &self.rxb8_clr())
            .field("rxidlclr", &self.rxidlclr())
            .field("abrend_clr", &self.abrend_clr())
            .field("abrerr_clr", &self.abrerr_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ txiclr: {=bool:?}, rxiclr: {=bool:?}, txc_clr: {=bool:?}, rxoerrclr: {=bool:?}, rxperrclr: {=bool:?}, rxferrclr: {=bool:?}, rxbrkclr: {=bool:?}, txbrk_clr: {=bool:?}, rxb8_clr: {=bool:?}, rxidlclr: {=bool:?}, abrend_clr: {=bool:?}, abrerr_clr: {=bool:?} }}",
            self.txiclr(),
            self.rxiclr(),
            self.txc_clr(),
            self.rxoerrclr(),
            self.rxperrclr(),
            self.rxferrclr(),
            self.rxbrkclr(),
            self.txbrk_clr(),
            self.rxb8_clr(),
            self.rxidlclr(),
            self.abrend_clr(),
            self.abrerr_clr()
        )
    }
}
#[doc = "Data length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idlr(pub u32);
impl Idlr {
    #[doc = "Idle data length register"]
    #[must_use]
    #[inline(always)]
    pub const fn idlr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Idle data length register"]
    #[inline(always)]
    pub const fn set_idlr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Idlr {
    #[inline(always)]
    fn default() -> Idlr {
        Idlr(0)
    }
}
impl core::fmt::Debug for Idlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idlr").field("idlr", &self.idlr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idlr {{ idlr: {=u16:?} }}", self.idlr())
    }
}
#[doc = "Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Transmit buffer empty interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty interrupt enable bit"]
    #[inline(always)]
    pub const fn set_txien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive buffer interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxien(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit complete interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txc_ien(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit complete interrupt enable bit"]
    #[inline(always)]
    pub const fn set_txc_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive overflow error interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerren(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overflow error interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxoerren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parity error interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxperren(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxperren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame error interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxferren(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame error interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxferren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive frame break interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxbrken(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive frame break interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxbrken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Break Frame Interrupt Enable Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txbrk_ien(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Break Frame Interrupt Enable Bit"]
    #[inline(always)]
    pub const fn set_txbrk_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Bit 8 Interrupt Enable Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb8_ien(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub const fn set_rxb8_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive frame idle interrupt enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxidlen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive frame idle interrupt enable bit"]
    #[inline(always)]
    pub const fn set_rxidlen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Automatic baud rate end interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn abrend_ien(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate end interrupt enable"]
    #[inline(always)]
    pub const fn set_abrend_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Automatic baud rate error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn abrerr_ien(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate error interrupt enable"]
    #[inline(always)]
    pub const fn set_abrerr_ien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("txien", &self.txien())
            .field("rxien", &self.rxien())
            .field("txc_ien", &self.txc_ien())
            .field("rxoerren", &self.rxoerren())
            .field("rxperren", &self.rxperren())
            .field("rxferren", &self.rxferren())
            .field("rxbrken", &self.rxbrken())
            .field("txbrk_ien", &self.txbrk_ien())
            .field("rxb8_ien", &self.rxb8_ien())
            .field("rxidlen", &self.rxidlen())
            .field("abrend_ien", &self.abrend_ien())
            .field("abrerr_ien", &self.abrerr_ien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ txien: {=bool:?}, rxien: {=bool:?}, txc_ien: {=bool:?}, rxoerren: {=bool:?}, rxperren: {=bool:?}, rxferren: {=bool:?}, rxbrken: {=bool:?}, txbrk_ien: {=bool:?}, rxb8_ien: {=bool:?}, rxidlen: {=bool:?}, abrend_ien: {=bool:?}, abrerr_ien: {=bool:?} }}",
            self.txien(),
            self.rxien(),
            self.txc_ien(),
            self.rxoerren(),
            self.rxperren(),
            self.rxferren(),
            self.rxbrken(),
            self.txbrk_ien(),
            self.rxb8_ien(),
            self.rxidlen(),
            self.abrend_ien(),
            self.abrerr_ien()
        )
    }
}
#[doc = "Interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Transmit buffer empty interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_intf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty interrupt flag bit"]
    #[inline(always)]
    pub const fn set_tx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive valid data interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_intf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive valid data interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rx_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "UART Transmit Complete Interrupt Flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txc_intf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "UART Transmit Complete Interrupt Flag bit"]
    #[inline(always)]
    pub const fn set_txc_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive overflow error interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoerr_intf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overflow error interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxoerr_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parity error interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxperr_intf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxperr_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame error interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxferr_intf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame error interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxferr_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive frame break interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxbrk_intf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive frame break interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxbrk_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit Break Frame Interrupt Flag Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn txbrk_intf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Break Frame Interrupt Flag Bit"]
    #[inline(always)]
    pub const fn set_txbrk_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Bit 8 Interrupt Flag Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxb8_intf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Interrupt Flag Bit"]
    #[inline(always)]
    pub const fn set_rxb8_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive frame idle interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxidle_intf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive frame idle interrupt flag bit"]
    #[inline(always)]
    pub const fn set_rxidle_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Automatic baud rate end interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn abrend_intf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate end interrupt flag bit"]
    #[inline(always)]
    pub const fn set_abrend_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Automatic baud rate error interrupt flag bit"]
    #[must_use]
    #[inline(always)]
    pub const fn abrerr_intf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic baud rate error interrupt flag bit"]
    #[inline(always)]
    pub const fn set_abrerr_intf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("tx_intf", &self.tx_intf())
            .field("rx_intf", &self.rx_intf())
            .field("txc_intf", &self.txc_intf())
            .field("rxoerr_intf", &self.rxoerr_intf())
            .field("rxperr_intf", &self.rxperr_intf())
            .field("rxferr_intf", &self.rxferr_intf())
            .field("rxbrk_intf", &self.rxbrk_intf())
            .field("txbrk_intf", &self.txbrk_intf())
            .field("rxb8_intf", &self.rxb8_intf())
            .field("rxidle_intf", &self.rxidle_intf())
            .field("abrend_intf", &self.abrend_intf())
            .field("abrerr_intf", &self.abrerr_intf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ tx_intf: {=bool:?}, rx_intf: {=bool:?}, txc_intf: {=bool:?}, rxoerr_intf: {=bool:?}, rxperr_intf: {=bool:?}, rxferr_intf: {=bool:?}, rxbrk_intf: {=bool:?}, txbrk_intf: {=bool:?}, rxb8_intf: {=bool:?}, rxidle_intf: {=bool:?}, abrend_intf: {=bool:?}, abrerr_intf: {=bool:?} }}",
            self.tx_intf(),
            self.rx_intf(),
            self.txc_intf(),
            self.rxoerr_intf(),
            self.rxperr_intf(),
            self.rxferr_intf(),
            self.rxbrk_intf(),
            self.txbrk_intf(),
            self.rxb8_intf(),
            self.rxidle_intf(),
            self.abrend_intf(),
            self.abrerr_intf()
        )
    }
}
#[doc = "Receive data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Receive data register"]
    #[must_use]
    #[inline(always)]
    pub const fn rxreg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive data register"]
    #[inline(always)]
    pub const fn set_rxreg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
impl core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdr").field("rxreg", &self.rxreg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ rxreg: {=u8:?} }}", self.rxreg())
    }
}
#[doc = "Receive Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxaddr(pub u32);
impl Rxaddr {
    #[doc = "Synchronous frame match address"]
    #[must_use]
    #[inline(always)]
    pub const fn rxaddr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronous frame match address"]
    #[inline(always)]
    pub const fn set_rxaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxaddr {
    #[inline(always)]
    fn default() -> Rxaddr {
        Rxaddr(0)
    }
}
impl core::fmt::Debug for Rxaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxaddr")
            .field("rxaddr", &self.rxaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxaddr {{ rxaddr: {=u8:?} }}", self.rxaddr())
    }
}
#[doc = "Receive Mask Registe"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmask(pub u32);
impl Rxmask {
    #[doc = "Synchronous frame match address mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synchronous frame match address mask"]
    #[inline(always)]
    pub const fn set_rxmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxmask {
    #[inline(always)]
    fn default() -> Rxmask {
        Rxmask(0)
    }
}
impl core::fmt::Debug for Rxmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxmask")
            .field("rxmask", &self.rxmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxmask {{ rxmask: {=u8:?} }}", self.rxmask())
    }
}
#[doc = "Slave Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "ISO7816 enable control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn scen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ISO7816 enable control bit"]
    #[inline(always)]
    pub const fn set_scen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ISO7816 check auto-response bit"]
    #[must_use]
    #[inline(always)]
    pub const fn scaen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ISO7816 check auto-response bit"]
    #[inline(always)]
    pub const fn set_scaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master receive frame answer bit"]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master receive frame answer bit"]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ISO7816 protection counter bit"]
    #[must_use]
    #[inline(always)]
    pub const fn scfcnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "ISO7816 protection counter bit"]
    #[inline(always)]
    pub const fn set_scfcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Single-wire half-duplex mode selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn hdsel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Single-wire half-duplex mode selection bit"]
    #[inline(always)]
    pub const fn set_hdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("scen", &self.scen())
            .field("scaen", &self.scaen())
            .field("nack", &self.nack())
            .field("scfcnt", &self.scfcnt())
            .field("hdsel", &self.hdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ scen: {=bool:?}, scaen: {=bool:?}, nack: {=bool:?}, scfcnt: {=u8:?}, hdsel: {=bool:?} }}",
            self.scen(),
            self.scaen(),
            self.nack(),
            self.scfcnt(),
            self.hdsel()
        )
    }
}
#[doc = "Transmit data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txreg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txreg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}
impl core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr").field("txreg", &self.txreg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ txreg: {=u8:?} }}", self.txreg())
    }
}
