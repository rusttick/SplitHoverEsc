#[doc = "USB address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u16);
impl Addr {
    #[doc = "USB address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
impl core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addr").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Addr {{ addr: {=u8:?} }}", self.addr())
    }
}
#[doc = "DMA1 ADDR0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamAddr0(pub u16);
impl DamAddr0 {
    #[doc = "USB DAM address0\\[7:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_addr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB DAM address0\\[7:0\\]"]
    #[inline(always)]
    pub const fn set_dma_addr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DamAddr0 {
    #[inline(always)]
    fn default() -> DamAddr0 {
        DamAddr0(0)
    }
}
impl core::fmt::Debug for DamAddr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DamAddr0")
            .field("dma_addr0", &self.dma_addr0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DamAddr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DamAddr0 {{ dma_addr0: {=u8:?} }}", self.dma_addr0())
    }
}
#[doc = "DMA1 ADDR1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamAddr1(pub u16);
impl DamAddr1 {
    #[doc = "USB DAM address1\\[15:8\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_addr1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB DAM address1\\[15:8\\]"]
    #[inline(always)]
    pub const fn set_dma_addr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DamAddr1 {
    #[inline(always)]
    fn default() -> DamAddr1 {
        DamAddr1(0)
    }
}
impl core::fmt::Debug for DamAddr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DamAddr1")
            .field("dma_addr1", &self.dma_addr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DamAddr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DamAddr1 {{ dma_addr1: {=u8:?} }}", self.dma_addr1())
    }
}
#[doc = "DMA1 ADDR2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamAddr2(pub u16);
impl DamAddr2 {
    #[doc = "USB DAM address2\\[23:16\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_addr2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB DAM address2\\[23:16\\]"]
    #[inline(always)]
    pub const fn set_dma_addr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DamAddr2 {
    #[inline(always)]
    fn default() -> DamAddr2 {
        DamAddr2(0)
    }
}
impl core::fmt::Debug for DamAddr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DamAddr2")
            .field("dma_addr2", &self.dma_addr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DamAddr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DamAddr2 {{ dma_addr2: {=u8:?} }}", self.dma_addr2())
    }
}
#[doc = "DMA1 ADDR3 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DamAddr3(pub u16);
impl DamAddr3 {
    #[doc = "USB DAM address3\\[31:24\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_addr3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB DAM address3\\[31:24\\]"]
    #[inline(always)]
    pub const fn set_dma_addr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DamAddr3 {
    #[inline(always)]
    fn default() -> DamAddr3 {
        DamAddr3(0)
    }
}
impl core::fmt::Debug for DamAddr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DamAddr3")
            .field("dma_addr3", &self.dma_addr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DamAddr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DamAddr3 {{ dma_addr3: {=u8:?} }}", self.dma_addr3())
    }
}
#[doc = "DMA NUMH register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaNumh(pub u16);
impl DmaNumh {
    #[doc = "EP2 DMA number 8-15"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_numh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP2 DMA number 8-15"]
    #[inline(always)]
    pub const fn set_dma1_numh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DmaNumh {
    #[inline(always)]
    fn default() -> DmaNumh {
        DmaNumh(0)
    }
}
impl core::fmt::Debug for DmaNumh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaNumh")
            .field("dma1_numh", &self.dma1_numh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaNumh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaNumh {{ dma1_numh: {=u8:?} }}", self.dma1_numh())
    }
}
#[doc = "DMA NUML register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaNuml(pub u16);
impl DmaNuml {
    #[doc = "EP2 DMA number 0-7"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_numl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP2 DMA number 0-7"]
    #[inline(always)]
    pub const fn set_dma_numl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for DmaNuml {
    #[inline(always)]
    fn default() -> DmaNuml {
        DmaNuml(0)
    }
}
impl core::fmt::Debug for DmaNuml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaNuml")
            .field("dma_numl", &self.dma_numl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaNuml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaNuml {{ dma_numl: {=u8:?} }}", self.dma_numl())
    }
}
#[doc = "EP0 AVAIL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Avail(pub u16);
impl Ep0Avail {
    #[doc = "EP0 FIFO available data number"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0avil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP0 FIFO available data number"]
    #[inline(always)]
    pub const fn set_ep0avil(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep0Avail {
    #[inline(always)]
    fn default() -> Ep0Avail {
        Ep0Avail(0)
    }
}
impl core::fmt::Debug for Ep0Avail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep0Avail")
            .field("ep0avil", &self.ep0avil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep0Avail {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep0Avail {{ ep0avil: {=u8:?} }}", self.ep0avil())
    }
}
#[doc = "EP0 CTRL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Ctrl(pub u16);
impl Ep0Ctrl {
    #[doc = "EP0 transfer counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trancount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EP0 transfer counter"]
    #[inline(always)]
    pub const fn set_trancount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "EP0 transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tranen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 transfer enable"]
    #[inline(always)]
    pub const fn set_tranen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep0Ctrl {
    #[inline(always)]
    fn default() -> Ep0Ctrl {
        Ep0Ctrl(0)
    }
}
impl core::fmt::Debug for Ep0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep0Ctrl")
            .field("trancount", &self.trancount())
            .field("tranen", &self.tranen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep0Ctrl {{ trancount: {=u8:?}, tranen: {=bool:?} }}",
            self.trancount(),
            self.tranen()
        )
    }
}
#[doc = "EP0 FIFO register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0Fifo(pub u16);
impl Ep0Fifo {
    #[doc = "EP0 FIFO port"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0_fifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP0 FIFO port"]
    #[inline(always)]
    pub const fn set_ep0_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep0Fifo {
    #[inline(always)]
    fn default() -> Ep0Fifo {
        Ep0Fifo(0)
    }
}
impl core::fmt::Debug for Ep0Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep0Fifo")
            .field("ep0_fifo", &self.ep0_fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep0Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep0Fifo {{ ep0_fifo: {=u8:?} }}", self.ep0_fifo())
    }
}
#[doc = "EP0 interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0IntEn(pub u16);
impl Ep0IntEn {
    #[doc = "SETUP packet interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn setupie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP packet interrupt enable"]
    #[inline(always)]
    pub const fn set_setupie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Status stage finished interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn endie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished interrupt enable"]
    #[inline(always)]
    pub const fn set_endie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_in_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_out_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep0IntEn {
    #[inline(always)]
    fn default() -> Ep0IntEn {
        Ep0IntEn(0)
    }
}
impl core::fmt::Debug for Ep0IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep0IntEn")
            .field("setupie", &self.setupie())
            .field("endie", &self.endie())
            .field("in_nackie", &self.in_nackie())
            .field("in_ackie", &self.in_ackie())
            .field("in_stallie", &self.in_stallie())
            .field("out_nackie", &self.out_nackie())
            .field("out_ackie", &self.out_ackie())
            .field("out_stallie", &self.out_stallie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep0IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep0IntEn {{ setupie: {=bool:?}, endie: {=bool:?}, in_nackie: {=bool:?}, in_ackie: {=bool:?}, in_stallie: {=bool:?}, out_nackie: {=bool:?}, out_ackie: {=bool:?}, out_stallie: {=bool:?} }}",
            self.setupie(),
            self.endie(),
            self.in_nackie(),
            self.in_ackie(),
            self.in_stallie(),
            self.out_nackie(),
            self.out_ackie(),
            self.out_stallie()
        )
    }
}
#[doc = "EP0 interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep0IntState(pub u16);
impl Ep0IntState {
    #[doc = "SETUP packet received"]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP packet received"]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Status stage finished"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK received"]
    #[inline(always)]
    pub const fn set_in_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK received"]
    #[inline(always)]
    pub const fn set_in_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL received"]
    #[inline(always)]
    pub const fn set_in_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK received"]
    #[inline(always)]
    pub const fn set_out_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK received"]
    #[inline(always)]
    pub const fn set_out_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL received"]
    #[inline(always)]
    pub const fn set_out_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep0IntState {
    #[inline(always)]
    fn default() -> Ep0IntState {
        Ep0IntState(0)
    }
}
impl core::fmt::Debug for Ep0IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep0IntState")
            .field("setup", &self.setup())
            .field("end", &self.end())
            .field("in_nackf", &self.in_nackf())
            .field("in_ackf", &self.in_ackf())
            .field("in_stallf", &self.in_stallf())
            .field("out_nackf", &self.out_nackf())
            .field("out_ackf", &self.out_ackf())
            .field("out_stallf", &self.out_stallf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep0IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep0IntState {{ setup: {=bool:?}, end: {=bool:?}, in_nackf: {=bool:?}, in_ackf: {=bool:?}, in_stallf: {=bool:?}, out_nackf: {=bool:?}, out_ackf: {=bool:?}, out_stallf: {=bool:?} }}",
            self.setup(),
            self.end(),
            self.in_nackf(),
            self.in_ackf(),
            self.in_stallf(),
            self.out_nackf(),
            self.out_ackf(),
            self.out_stallf()
        )
    }
}
#[doc = "EP1 AVAIL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1Avail(pub u16);
impl Ep1Avail {
    #[doc = "EP1 FIFO available data number"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1avil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP1 FIFO available data number"]
    #[inline(always)]
    pub const fn set_ep1avil(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep1Avail {
    #[inline(always)]
    fn default() -> Ep1Avail {
        Ep1Avail(0)
    }
}
impl core::fmt::Debug for Ep1Avail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep1Avail")
            .field("ep1avil", &self.ep1avil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep1Avail {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep1Avail {{ ep1avil: {=u8:?} }}", self.ep1avil())
    }
}
#[doc = "EP1 CTRL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1Ctrl(pub u16);
impl Ep1Ctrl {
    #[doc = "EP1 transfer counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trancount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EP1 transfer counter"]
    #[inline(always)]
    pub const fn set_trancount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "EP1 transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tranen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 transfer enable"]
    #[inline(always)]
    pub const fn set_tranen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep1Ctrl {
    #[inline(always)]
    fn default() -> Ep1Ctrl {
        Ep1Ctrl(0)
    }
}
impl core::fmt::Debug for Ep1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep1Ctrl")
            .field("trancount", &self.trancount())
            .field("tranen", &self.tranen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep1Ctrl {{ trancount: {=u8:?}, tranen: {=bool:?} }}",
            self.trancount(),
            self.tranen()
        )
    }
}
#[doc = "EP1 FIFO register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1Fifo(pub u16);
impl Ep1Fifo {
    #[doc = "EP1 FIFO port"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1_fifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP1 FIFO port"]
    #[inline(always)]
    pub const fn set_ep1_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep1Fifo {
    #[inline(always)]
    fn default() -> Ep1Fifo {
        Ep1Fifo(0)
    }
}
impl core::fmt::Debug for Ep1Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep1Fifo")
            .field("ep1_fifo", &self.ep1_fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep1Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep1Fifo {{ ep1_fifo: {=u8:?} }}", self.ep1_fifo())
    }
}
#[doc = "EP1 interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1IntEn(pub u16);
impl Ep1IntEn {
    #[doc = "Status stage finished interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn endie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished interrupt enable"]
    #[inline(always)]
    pub const fn set_endie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_in_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_out_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep1IntEn {
    #[inline(always)]
    fn default() -> Ep1IntEn {
        Ep1IntEn(0)
    }
}
impl core::fmt::Debug for Ep1IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep1IntEn")
            .field("endie", &self.endie())
            .field("in_nackie", &self.in_nackie())
            .field("in_ackie", &self.in_ackie())
            .field("in_stallie", &self.in_stallie())
            .field("out_nackie", &self.out_nackie())
            .field("out_ackie", &self.out_ackie())
            .field("out_stallie", &self.out_stallie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep1IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep1IntEn {{ endie: {=bool:?}, in_nackie: {=bool:?}, in_ackie: {=bool:?}, in_stallie: {=bool:?}, out_nackie: {=bool:?}, out_ackie: {=bool:?}, out_stallie: {=bool:?} }}",
            self.endie(),
            self.in_nackie(),
            self.in_ackie(),
            self.in_stallie(),
            self.out_nackie(),
            self.out_ackie(),
            self.out_stallie()
        )
    }
}
#[doc = "EP1 interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep1IntState(pub u16);
impl Ep1IntState {
    #[doc = "Status stage finished"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nacf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK received"]
    #[inline(always)]
    pub const fn set_in_nacf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK received"]
    #[inline(always)]
    pub const fn set_in_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL received"]
    #[inline(always)]
    pub const fn set_in_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK received"]
    #[inline(always)]
    pub const fn set_out_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK received"]
    #[inline(always)]
    pub const fn set_out_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL received"]
    #[inline(always)]
    pub const fn set_out_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep1IntState {
    #[inline(always)]
    fn default() -> Ep1IntState {
        Ep1IntState(0)
    }
}
impl core::fmt::Debug for Ep1IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep1IntState")
            .field("end", &self.end())
            .field("in_nacf", &self.in_nacf())
            .field("in_ackf", &self.in_ackf())
            .field("in_stallf", &self.in_stallf())
            .field("out_nackf", &self.out_nackf())
            .field("out_ackf", &self.out_ackf())
            .field("out_stallf", &self.out_stallf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep1IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep1IntState {{ end: {=bool:?}, in_nacf: {=bool:?}, in_ackf: {=bool:?}, in_stallf: {=bool:?}, out_nackf: {=bool:?}, out_ackf: {=bool:?}, out_stallf: {=bool:?} }}",
            self.end(),
            self.in_nacf(),
            self.in_ackf(),
            self.in_stallf(),
            self.out_nackf(),
            self.out_ackf(),
            self.out_stallf()
        )
    }
}
#[doc = "EP2 AVAIL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2Avail(pub u16);
impl Ep2Avail {
    #[doc = "EP2 FIFO available data number"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2avil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP2 FIFO available data number"]
    #[inline(always)]
    pub const fn set_ep2avil(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep2Avail {
    #[inline(always)]
    fn default() -> Ep2Avail {
        Ep2Avail(0)
    }
}
impl core::fmt::Debug for Ep2Avail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep2Avail")
            .field("ep2avil", &self.ep2avil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep2Avail {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep2Avail {{ ep2avil: {=u8:?} }}", self.ep2avil())
    }
}
#[doc = "EP2 CTRL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2Ctrl(pub u16);
impl Ep2Ctrl {
    #[doc = "EP2 transfer counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trancount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EP2 transfer counter"]
    #[inline(always)]
    pub const fn set_trancount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "EP2 transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tranen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 transfer enable"]
    #[inline(always)]
    pub const fn set_tranen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep2Ctrl {
    #[inline(always)]
    fn default() -> Ep2Ctrl {
        Ep2Ctrl(0)
    }
}
impl core::fmt::Debug for Ep2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep2Ctrl")
            .field("trancount", &self.trancount())
            .field("tranen", &self.tranen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep2Ctrl {{ trancount: {=u8:?}, tranen: {=bool:?} }}",
            self.trancount(),
            self.tranen()
        )
    }
}
#[doc = "EP2 FIFO register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2Fifo(pub u16);
impl Ep2Fifo {
    #[doc = "EP2 FIFO port"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2_fifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP2 FIFO port"]
    #[inline(always)]
    pub const fn set_ep2_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep2Fifo {
    #[inline(always)]
    fn default() -> Ep2Fifo {
        Ep2Fifo(0)
    }
}
impl core::fmt::Debug for Ep2Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep2Fifo")
            .field("ep2_fifo", &self.ep2_fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep2Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep2Fifo {{ ep2_fifo: {=u8:?} }}", self.ep2_fifo())
    }
}
#[doc = "EP2 interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2IntEn(pub u16);
impl Ep2IntEn {
    #[doc = "Status stage finished interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn endie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished interrupt enable"]
    #[inline(always)]
    pub const fn set_endie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_in_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_out_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep2IntEn {
    #[inline(always)]
    fn default() -> Ep2IntEn {
        Ep2IntEn(0)
    }
}
impl core::fmt::Debug for Ep2IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep2IntEn")
            .field("endie", &self.endie())
            .field("in_nackie", &self.in_nackie())
            .field("in_ackie", &self.in_ackie())
            .field("in_stallie", &self.in_stallie())
            .field("out_nackie", &self.out_nackie())
            .field("out_ackie", &self.out_ackie())
            .field("out_stallie", &self.out_stallie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep2IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep2IntEn {{ endie: {=bool:?}, in_nackie: {=bool:?}, in_ackie: {=bool:?}, in_stallie: {=bool:?}, out_nackie: {=bool:?}, out_ackie: {=bool:?}, out_stallie: {=bool:?} }}",
            self.endie(),
            self.in_nackie(),
            self.in_ackie(),
            self.in_stallie(),
            self.out_nackie(),
            self.out_ackie(),
            self.out_stallie()
        )
    }
}
#[doc = "EP2 interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep2IntState(pub u16);
impl Ep2IntState {
    #[doc = "Status stage finished"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nacf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK received"]
    #[inline(always)]
    pub const fn set_in_nacf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK received"]
    #[inline(always)]
    pub const fn set_in_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL received"]
    #[inline(always)]
    pub const fn set_in_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK received"]
    #[inline(always)]
    pub const fn set_out_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK received"]
    #[inline(always)]
    pub const fn set_out_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL received"]
    #[inline(always)]
    pub const fn set_out_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep2IntState {
    #[inline(always)]
    fn default() -> Ep2IntState {
        Ep2IntState(0)
    }
}
impl core::fmt::Debug for Ep2IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep2IntState")
            .field("end", &self.end())
            .field("in_nacf", &self.in_nacf())
            .field("in_ackf", &self.in_ackf())
            .field("in_stallf", &self.in_stallf())
            .field("out_nackf", &self.out_nackf())
            .field("out_ackf", &self.out_ackf())
            .field("out_stallf", &self.out_stallf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep2IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep2IntState {{ end: {=bool:?}, in_nacf: {=bool:?}, in_ackf: {=bool:?}, in_stallf: {=bool:?}, out_nackf: {=bool:?}, out_ackf: {=bool:?}, out_stallf: {=bool:?} }}",
            self.end(),
            self.in_nacf(),
            self.in_ackf(),
            self.in_stallf(),
            self.out_nackf(),
            self.out_ackf(),
            self.out_stallf()
        )
    }
}
#[doc = "EP3 AVAIL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3Avail(pub u16);
impl Ep3Avail {
    #[doc = "EP3 FIFO available data number"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3avil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP3 FIFO available data number"]
    #[inline(always)]
    pub const fn set_ep3avil(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep3Avail {
    #[inline(always)]
    fn default() -> Ep3Avail {
        Ep3Avail(0)
    }
}
impl core::fmt::Debug for Ep3Avail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep3Avail")
            .field("ep3avil", &self.ep3avil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep3Avail {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep3Avail {{ ep3avil: {=u8:?} }}", self.ep3avil())
    }
}
#[doc = "EP3 CTRL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3Ctrl(pub u16);
impl Ep3Ctrl {
    #[doc = "EP3 transfer counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trancount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EP3 transfer counter"]
    #[inline(always)]
    pub const fn set_trancount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "EP3 transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tranen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 transfer enable"]
    #[inline(always)]
    pub const fn set_tranen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep3Ctrl {
    #[inline(always)]
    fn default() -> Ep3Ctrl {
        Ep3Ctrl(0)
    }
}
impl core::fmt::Debug for Ep3Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep3Ctrl")
            .field("trancount", &self.trancount())
            .field("tranen", &self.tranen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep3Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep3Ctrl {{ trancount: {=u8:?}, tranen: {=bool:?} }}",
            self.trancount(),
            self.tranen()
        )
    }
}
#[doc = "EP3 FIFO register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3Fifo(pub u16);
impl Ep3Fifo {
    #[doc = "EP3 FIFO port"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3_fifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP3 FIFO port"]
    #[inline(always)]
    pub const fn set_ep3_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep3Fifo {
    #[inline(always)]
    fn default() -> Ep3Fifo {
        Ep3Fifo(0)
    }
}
impl core::fmt::Debug for Ep3Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep3Fifo")
            .field("ep3_fifo", &self.ep3_fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep3Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep3Fifo {{ ep3_fifo: {=u8:?} }}", self.ep3_fifo())
    }
}
#[doc = "EP3 interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3IntEn(pub u16);
impl Ep3IntEn {
    #[doc = "Status stage finished interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn endie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished interrupt enable"]
    #[inline(always)]
    pub const fn set_endie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_in_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_out_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep3IntEn {
    #[inline(always)]
    fn default() -> Ep3IntEn {
        Ep3IntEn(0)
    }
}
impl core::fmt::Debug for Ep3IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep3IntEn")
            .field("endie", &self.endie())
            .field("in_nackie", &self.in_nackie())
            .field("in_ackie", &self.in_ackie())
            .field("in_stallie", &self.in_stallie())
            .field("out_nackie", &self.out_nackie())
            .field("out_ackie", &self.out_ackie())
            .field("out_stallie", &self.out_stallie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep3IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep3IntEn {{ endie: {=bool:?}, in_nackie: {=bool:?}, in_ackie: {=bool:?}, in_stallie: {=bool:?}, out_nackie: {=bool:?}, out_ackie: {=bool:?}, out_stallie: {=bool:?} }}",
            self.endie(),
            self.in_nackie(),
            self.in_ackie(),
            self.in_stallie(),
            self.out_nackie(),
            self.out_ackie(),
            self.out_stallie()
        )
    }
}
#[doc = "EP3 interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep3IntState(pub u16);
impl Ep3IntState {
    #[doc = "Status stage finished"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nacf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK received"]
    #[inline(always)]
    pub const fn set_in_nacf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK received"]
    #[inline(always)]
    pub const fn set_in_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL received"]
    #[inline(always)]
    pub const fn set_in_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK received"]
    #[inline(always)]
    pub const fn set_out_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK received"]
    #[inline(always)]
    pub const fn set_out_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL received"]
    #[inline(always)]
    pub const fn set_out_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep3IntState {
    #[inline(always)]
    fn default() -> Ep3IntState {
        Ep3IntState(0)
    }
}
impl core::fmt::Debug for Ep3IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep3IntState")
            .field("end", &self.end())
            .field("in_nacf", &self.in_nacf())
            .field("in_ackf", &self.in_ackf())
            .field("in_stallf", &self.in_stallf())
            .field("out_nackf", &self.out_nackf())
            .field("out_ackf", &self.out_ackf())
            .field("out_stallf", &self.out_stallf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep3IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep3IntState {{ end: {=bool:?}, in_nacf: {=bool:?}, in_ackf: {=bool:?}, in_stallf: {=bool:?}, out_nackf: {=bool:?}, out_ackf: {=bool:?}, out_stallf: {=bool:?} }}",
            self.end(),
            self.in_nacf(),
            self.in_ackf(),
            self.in_stallf(),
            self.out_nackf(),
            self.out_ackf(),
            self.out_stallf()
        )
    }
}
#[doc = "EP4 AVAIL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4Avail(pub u16);
impl Ep4Avail {
    #[doc = "EP4 FIFO available data number"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4avil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP4 FIFO available data number"]
    #[inline(always)]
    pub const fn set_ep4avil(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep4Avail {
    #[inline(always)]
    fn default() -> Ep4Avail {
        Ep4Avail(0)
    }
}
impl core::fmt::Debug for Ep4Avail {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep4Avail")
            .field("ep4avil", &self.ep4avil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep4Avail {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep4Avail {{ ep4avil: {=u8:?} }}", self.ep4avil())
    }
}
#[doc = "EP4 CTRL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4Ctrl(pub u16);
impl Ep4Ctrl {
    #[doc = "EP4 transfer counter"]
    #[must_use]
    #[inline(always)]
    pub const fn trancount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "EP4 transfer counter"]
    #[inline(always)]
    pub const fn set_trancount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
    }
    #[doc = "EP4 transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tranen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 transfer enable"]
    #[inline(always)]
    pub const fn set_tranen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep4Ctrl {
    #[inline(always)]
    fn default() -> Ep4Ctrl {
        Ep4Ctrl(0)
    }
}
impl core::fmt::Debug for Ep4Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep4Ctrl")
            .field("trancount", &self.trancount())
            .field("tranen", &self.tranen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep4Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep4Ctrl {{ trancount: {=u8:?}, tranen: {=bool:?} }}",
            self.trancount(),
            self.tranen()
        )
    }
}
#[doc = "EP4 FIFO register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4Fifo(pub u16);
impl Ep4Fifo {
    #[doc = "EP4 FIFO port"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4_fifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "EP4 FIFO port"]
    #[inline(always)]
    pub const fn set_ep4_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Ep4Fifo {
    #[inline(always)]
    fn default() -> Ep4Fifo {
        Ep4Fifo(0)
    }
}
impl core::fmt::Debug for Ep4Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep4Fifo")
            .field("ep4_fifo", &self.ep4_fifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep4Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ep4Fifo {{ ep4_fifo: {=u8:?} }}", self.ep4_fifo())
    }
}
#[doc = "EP4 interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4IntEn(pub u16);
impl Ep4IntEn {
    #[doc = "Status stage finished interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn endie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished interrupt enable"]
    #[inline(always)]
    pub const fn set_endie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nackie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_in_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_in_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_nackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK interrupt enable"]
    #[inline(always)]
    pub const fn set_out_ackie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL interrupt enable"]
    #[inline(always)]
    pub const fn set_out_stallie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep4IntEn {
    #[inline(always)]
    fn default() -> Ep4IntEn {
        Ep4IntEn(0)
    }
}
impl core::fmt::Debug for Ep4IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep4IntEn")
            .field("endie", &self.endie())
            .field("in_nackie", &self.in_nackie())
            .field("in_ackie", &self.in_ackie())
            .field("in_stallie", &self.in_stallie())
            .field("out_nackie", &self.out_nackie())
            .field("out_ackie", &self.out_ackie())
            .field("out_stallie", &self.out_stallie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep4IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep4IntEn {{ endie: {=bool:?}, in_nackie: {=bool:?}, in_ackie: {=bool:?}, in_stallie: {=bool:?}, out_nackie: {=bool:?}, out_ackie: {=bool:?}, out_stallie: {=bool:?} }}",
            self.endie(),
            self.in_nackie(),
            self.in_ackie(),
            self.in_stallie(),
            self.out_nackie(),
            self.out_ackie(),
            self.out_stallie()
        )
    }
}
#[doc = "EP4 interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ep4IntState(pub u16);
impl Ep4IntState {
    #[doc = "Status stage finished"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status stage finished"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "IN-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_nacf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IN-NACK received"]
    #[inline(always)]
    pub const fn set_in_nacf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "IN-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_ackf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IN-ACK received"]
    #[inline(always)]
    pub const fn set_in_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "IN-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn in_stallf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IN-STALL received"]
    #[inline(always)]
    pub const fn set_in_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "OUT-NACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_nackf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-NACK received"]
    #[inline(always)]
    pub const fn set_out_nackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "OUT-ACK received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_ackf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-ACK received"]
    #[inline(always)]
    pub const fn set_out_ackf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "OUT-STALL received"]
    #[must_use]
    #[inline(always)]
    pub const fn out_stallf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "OUT-STALL received"]
    #[inline(always)]
    pub const fn set_out_stallf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Ep4IntState {
    #[inline(always)]
    fn default() -> Ep4IntState {
        Ep4IntState(0)
    }
}
impl core::fmt::Debug for Ep4IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ep4IntState")
            .field("end", &self.end())
            .field("in_nacf", &self.in_nacf())
            .field("in_ackf", &self.in_ackf())
            .field("in_stallf", &self.in_stallf())
            .field("out_nackf", &self.out_nackf())
            .field("out_ackf", &self.out_ackf())
            .field("out_stallf", &self.out_stallf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ep4IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ep4IntState {{ end: {=bool:?}, in_nacf: {=bool:?}, in_ackf: {=bool:?}, in_stallf: {=bool:?}, out_nackf: {=bool:?}, out_ackf: {=bool:?}, out_stallf: {=bool:?} }}",
            self.end(),
            self.in_nacf(),
            self.in_ackf(),
            self.in_stallf(),
            self.out_nackf(),
            self.out_ackf(),
            self.out_stallf()
        )
    }
}
#[doc = "EP DMA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpDma(pub u16);
impl EpDma {
    #[doc = "EP0 DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 DMA enable"]
    #[inline(always)]
    pub const fn set_dma0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "EP1 DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 DMA enable"]
    #[inline(always)]
    pub const fn set_dma1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "EP2 DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 DMA enable"]
    #[inline(always)]
    pub const fn set_dma2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "EP3 DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 DMA enable"]
    #[inline(always)]
    pub const fn set_dma3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP4 DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma4en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 DMA enable"]
    #[inline(always)]
    pub const fn set_dma4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for EpDma {
    #[inline(always)]
    fn default() -> EpDma {
        EpDma(0)
    }
}
impl core::fmt::Debug for EpDma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpDma")
            .field("dma0en", &self.dma0en())
            .field("dma1en", &self.dma1en())
            .field("dma2en", &self.dma2en())
            .field("dma3en", &self.dma3en())
            .field("dma4en", &self.dma4en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpDma {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpDma {{ dma0en: {=bool:?}, dma1en: {=bool:?}, dma2en: {=bool:?}, dma3en: {=bool:?}, dma4en: {=bool:?} }}",
            self.dma0en(),
            self.dma1en(),
            self.dma2en(),
            self.dma3en(),
            self.dma4en()
        )
    }
}
#[doc = "DMA End-point direction register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpDmaDir(pub u16);
impl EpDmaDir {
    #[doc = "Point 1 Dma Dir"]
    #[must_use]
    #[inline(always)]
    pub const fn dam_dir1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Point 1 Dma Dir"]
    #[inline(always)]
    pub const fn set_dam_dir1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Point 2 Dma Dir"]
    #[must_use]
    #[inline(always)]
    pub const fn dam_dir2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Point 2 Dma Dir"]
    #[inline(always)]
    pub const fn set_dam_dir2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Point 3 Dma Dir"]
    #[must_use]
    #[inline(always)]
    pub const fn dam_dir3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Point 3 Dma Dir"]
    #[inline(always)]
    pub const fn set_dam_dir3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Point 4 Dma Dir"]
    #[must_use]
    #[inline(always)]
    pub const fn dam_dir4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Point 4 Dma Dir"]
    #[inline(always)]
    pub const fn set_dam_dir4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for EpDmaDir {
    #[inline(always)]
    fn default() -> EpDmaDir {
        EpDmaDir(0)
    }
}
impl core::fmt::Debug for EpDmaDir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpDmaDir")
            .field("dam_dir1", &self.dam_dir1())
            .field("dam_dir2", &self.dam_dir2())
            .field("dam_dir3", &self.dam_dir3())
            .field("dam_dir4", &self.dam_dir4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpDmaDir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpDmaDir {{ dam_dir1: {=bool:?}, dam_dir2: {=bool:?}, dam_dir3: {=bool:?}, dam_dir4: {=bool:?} }}",
            self.dam_dir1(),
            self.dam_dir2(),
            self.dam_dir3(),
            self.dam_dir4()
        )
    }
}
#[doc = "EP Enable End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpEn(pub u16);
impl EpEn {
    #[doc = "Enable End Point 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End Point 0"]
    #[inline(always)]
    pub const fn set_ep0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable End Point 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End Point 1"]
    #[inline(always)]
    pub const fn set_ep1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable End Point 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End Point 2"]
    #[inline(always)]
    pub const fn set_ep2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Enable End Point 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End Point 3"]
    #[inline(always)]
    pub const fn set_ep3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable End Point 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End Point 4"]
    #[inline(always)]
    pub const fn set_ep4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for EpEn {
    #[inline(always)]
    fn default() -> EpEn {
        EpEn(0)
    }
}
impl core::fmt::Debug for EpEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpEn")
            .field("ep0en", &self.ep0en())
            .field("ep1en", &self.ep1en())
            .field("ep2en", &self.ep2en())
            .field("ep3en", &self.ep3en())
            .field("ep4en", &self.ep4en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpEn {{ ep0en: {=bool:?}, ep1en: {=bool:?}, ep2en: {=bool:?}, ep3en: {=bool:?}, ep4en: {=bool:?} }}",
            self.ep0en(),
            self.ep1en(),
            self.ep2en(),
            self.ep3en(),
            self.ep4en()
        )
    }
}
#[doc = "EP HALT register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpHalt(pub u16);
impl EpHalt {
    #[doc = "EP0 halt"]
    #[must_use]
    #[inline(always)]
    pub const fn halt0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 halt"]
    #[inline(always)]
    pub const fn set_halt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "EP1 halt"]
    #[must_use]
    #[inline(always)]
    pub const fn halt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 halt"]
    #[inline(always)]
    pub const fn set_halt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "EP2 halt"]
    #[must_use]
    #[inline(always)]
    pub const fn halt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 halt"]
    #[inline(always)]
    pub const fn set_halt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "EP3 halt"]
    #[must_use]
    #[inline(always)]
    pub const fn halt3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 halt"]
    #[inline(always)]
    pub const fn set_halt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP4 halt"]
    #[must_use]
    #[inline(always)]
    pub const fn halt4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 halt"]
    #[inline(always)]
    pub const fn set_halt4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for EpHalt {
    #[inline(always)]
    fn default() -> EpHalt {
        EpHalt(0)
    }
}
impl core::fmt::Debug for EpHalt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpHalt")
            .field("halt0", &self.halt0())
            .field("halt1", &self.halt1())
            .field("halt2", &self.halt2())
            .field("halt3", &self.halt3())
            .field("halt4", &self.halt4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpHalt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpHalt {{ halt0: {=bool:?}, halt1: {=bool:?}, halt2: {=bool:?}, halt3: {=bool:?}, halt4: {=bool:?} }}",
            self.halt0(),
            self.halt1(),
            self.halt2(),
            self.halt3(),
            self.halt4()
        )
    }
}
#[doc = "End-point index register 1_2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpIndex12(pub u16);
impl EpIndex12 {
    #[doc = "Point 1 index"]
    #[must_use]
    #[inline(always)]
    pub const fn index1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Point 1 index"]
    #[inline(always)]
    pub const fn set_index1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Point 2 index"]
    #[must_use]
    #[inline(always)]
    pub const fn index2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Point 2 index"]
    #[inline(always)]
    pub const fn set_index2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
}
impl Default for EpIndex12 {
    #[inline(always)]
    fn default() -> EpIndex12 {
        EpIndex12(0)
    }
}
impl core::fmt::Debug for EpIndex12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpIndex12")
            .field("index1", &self.index1())
            .field("index2", &self.index2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpIndex12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpIndex12 {{ index1: {=u8:?}, index2: {=u8:?} }}",
            self.index1(),
            self.index2()
        )
    }
}
#[doc = "End-point index register 3_4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpIndex34(pub u16);
impl EpIndex34 {
    #[doc = "Point 3 index"]
    #[must_use]
    #[inline(always)]
    pub const fn index3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Point 3 index"]
    #[inline(always)]
    pub const fn set_index3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Point 4 index"]
    #[must_use]
    #[inline(always)]
    pub const fn index4(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Point 4 index"]
    #[inline(always)]
    pub const fn set_index4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
}
impl Default for EpIndex34 {
    #[inline(always)]
    fn default() -> EpIndex34 {
        EpIndex34(0)
    }
}
impl core::fmt::Debug for EpIndex34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpIndex34")
            .field("index3", &self.index3())
            .field("index4", &self.index4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpIndex34 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpIndex34 {{ index3: {=u8:?}, index4: {=u8:?} }}",
            self.index3(),
            self.index4()
        )
    }
}
#[doc = "EP interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpIntEn(pub u16);
impl EpIntEn {
    #[doc = "EP0 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 interrupt enable"]
    #[inline(always)]
    pub const fn set_ep0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "EP1 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 interrupt enable"]
    #[inline(always)]
    pub const fn set_ep1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "EP2 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 interrupt enable"]
    #[inline(always)]
    pub const fn set_ep2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "EP3 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 interrupt enable"]
    #[inline(always)]
    pub const fn set_ep3ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP4 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 interrupt enable"]
    #[inline(always)]
    pub const fn set_ep4ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for EpIntEn {
    #[inline(always)]
    fn default() -> EpIntEn {
        EpIntEn(0)
    }
}
impl core::fmt::Debug for EpIntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpIntEn")
            .field("ep0ie", &self.ep0ie())
            .field("ep1ie", &self.ep1ie())
            .field("ep2ie", &self.ep2ie())
            .field("ep3ie", &self.ep3ie())
            .field("ep4ie", &self.ep4ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpIntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpIntEn {{ ep0ie: {=bool:?}, ep1ie: {=bool:?}, ep2ie: {=bool:?}, ep3ie: {=bool:?}, ep4ie: {=bool:?} }}",
            self.ep0ie(),
            self.ep1ie(),
            self.ep2ie(),
            self.ep3ie(),
            self.ep4ie()
        )
    }
}
#[doc = "EP interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpIntState(pub u16);
impl EpIntState {
    #[doc = "EP0 interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EP0 interrupt received"]
    #[inline(always)]
    pub const fn set_ep0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "EP1 interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "EP1 interrupt received"]
    #[inline(always)]
    pub const fn set_ep1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "EP2 interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "EP2 interrupt received"]
    #[inline(always)]
    pub const fn set_ep2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "EP3 interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3f(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "EP3 interrupt received"]
    #[inline(always)]
    pub const fn set_ep3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP4 interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4f(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP4 interrupt received"]
    #[inline(always)]
    pub const fn set_ep4f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for EpIntState {
    #[inline(always)]
    fn default() -> EpIntState {
        EpIntState(0)
    }
}
impl core::fmt::Debug for EpIntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpIntState")
            .field("ep0f", &self.ep0f())
            .field("ep1f", &self.ep1f())
            .field("ep2f", &self.ep2f())
            .field("ep3f", &self.ep3f())
            .field("ep4f", &self.ep4f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpIntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpIntState {{ ep0f: {=bool:?}, ep1f: {=bool:?}, ep2f: {=bool:?}, ep3f: {=bool:?}, ep4f: {=bool:?} }}",
            self.ep0f(),
            self.ep1f(),
            self.ep2f(),
            self.ep3f(),
            self.ep4f()
        )
    }
}
#[doc = "EP MEM register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpMem(pub u16);
impl EpMem {
    #[doc = "EP memory"]
    #[must_use]
    #[inline(always)]
    pub const fn ep_mem(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "EP memory"]
    #[inline(always)]
    pub const fn set_ep_mem(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
}
impl Default for EpMem {
    #[inline(always)]
    fn default() -> EpMem {
        EpMem(0)
    }
}
impl core::fmt::Debug for EpMem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpMem")
            .field("ep_mem", &self.ep_mem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpMem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EpMem {{ ep_mem: {=u8:?} }}", self.ep_mem())
    }
}
#[doc = "Endpoint type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpType(pub u16);
impl EpType {
    #[doc = "Point 1 type"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1_type(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Point 1 type"]
    #[inline(always)]
    pub const fn set_ep1_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Point 2 type"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2_type(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Point 2 type"]
    #[inline(always)]
    pub const fn set_ep2_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Point 3 type"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3_type(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Point 3 type"]
    #[inline(always)]
    pub const fn set_ep3_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Point 4 type"]
    #[must_use]
    #[inline(always)]
    pub const fn ep4_type(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Point 4 type"]
    #[inline(always)]
    pub const fn set_ep4_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for EpType {
    #[inline(always)]
    fn default() -> EpType {
        EpType(0)
    }
}
impl core::fmt::Debug for EpType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpType")
            .field("ep1_type", &self.ep1_type())
            .field("ep2_type", &self.ep2_type())
            .field("ep3_type", &self.ep3_type())
            .field("ep4_type", &self.ep4_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EpType {{ ep1_type: {=bool:?}, ep2_type: {=bool:?}, ep3_type: {=bool:?}, ep4_type: {=bool:?} }}",
            self.ep1_type(),
            self.ep2_type(),
            self.ep3_type(),
            self.ep4_type()
        )
    }
}
#[doc = "interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEn(pub u16);
impl IntEn {
    #[doc = "BUS reset interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rstie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BUS reset interrupt enable"]
    #[inline(always)]
    pub const fn set_rstie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "BUS suspend interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suspendie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BUS suspend interrupt enable"]
    #[inline(always)]
    pub const fn set_suspendie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "BUS resume interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn resumie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BUS resume interrupt enable"]
    #[inline(always)]
    pub const fn set_resumie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "SOF interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sofie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SOF interrupt enable"]
    #[inline(always)]
    pub const fn set_sofie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP interrupt enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn intmask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask"]
    #[inline(always)]
    pub const fn set_intmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for IntEn {
    #[inline(always)]
    fn default() -> IntEn {
        IntEn(0)
    }
}
impl core::fmt::Debug for IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEn")
            .field("rstie", &self.rstie())
            .field("suspendie", &self.suspendie())
            .field("resumie", &self.resumie())
            .field("sofie", &self.sofie())
            .field("epie", &self.epie())
            .field("intmask", &self.intmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntEn {{ rstie: {=bool:?}, suspendie: {=bool:?}, resumie: {=bool:?}, sofie: {=bool:?}, epie: {=bool:?}, intmask: {=bool:?} }}",
            self.rstie(),
            self.suspendie(),
            self.resumie(),
            self.sofie(),
            self.epie(),
            self.intmask()
        )
    }
}
#[doc = "interrupt state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntState(pub u16);
impl IntState {
    #[doc = "BUS reset received"]
    #[must_use]
    #[inline(always)]
    pub const fn rstf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BUS reset received"]
    #[inline(always)]
    pub const fn set_rstf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "BUS suspend received"]
    #[must_use]
    #[inline(always)]
    pub const fn suspendf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BUS suspend received"]
    #[inline(always)]
    pub const fn set_suspendf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "BUS resume received"]
    #[must_use]
    #[inline(always)]
    pub const fn resumf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BUS resume received"]
    #[inline(always)]
    pub const fn set_resumf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "BUS received"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BUS received"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "EP interrupt received"]
    #[must_use]
    #[inline(always)]
    pub const fn epintf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "EP interrupt received"]
    #[inline(always)]
    pub const fn set_epintf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for IntState {
    #[inline(always)]
    fn default() -> IntState {
        IntState(0)
    }
}
impl core::fmt::Debug for IntState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntState")
            .field("rstf", &self.rstf())
            .field("suspendf", &self.suspendf())
            .field("resumf", &self.resumf())
            .field("soff", &self.soff())
            .field("epintf", &self.epintf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntState {{ rstf: {=bool:?}, suspendf: {=bool:?}, resumf: {=bool:?}, soff: {=bool:?}, epintf: {=bool:?} }}",
            self.rstf(),
            self.suspendf(),
            self.resumf(),
            self.soff(),
            self.epintf()
        )
    }
}
#[doc = "PACKET SIZEH register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PacketSizeh(pub u32);
impl PacketSizeh {
    #[doc = "USB Max Packet Size"]
    #[must_use]
    #[inline(always)]
    pub const fn size1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB Max Packet Size"]
    #[inline(always)]
    pub const fn set_size1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PacketSizeh {
    #[inline(always)]
    fn default() -> PacketSizeh {
        PacketSizeh(0)
    }
}
impl core::fmt::Debug for PacketSizeh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PacketSizeh")
            .field("size1", &self.size1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PacketSizeh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PacketSizeh {{ size1: {=u8:?} }}", self.size1())
    }
}
#[doc = "PACKET SIZEL register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PacketSizel(pub u16);
impl PacketSizel {
    #[doc = "USB Max Packet Size"]
    #[must_use]
    #[inline(always)]
    pub const fn size0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "USB Max Packet Size"]
    #[inline(always)]
    pub const fn set_size0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for PacketSizel {
    #[inline(always)]
    fn default() -> PacketSizel {
        PacketSizel(0)
    }
}
impl core::fmt::Debug for PacketSizel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PacketSizel")
            .field("size0", &self.size0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PacketSizel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PacketSizel {{ size0: {=u8:?} }}", self.size0())
    }
}
#[doc = "Power control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u16);
impl Power {
    #[doc = "BUS suspend enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn suspen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BUS suspend enable bit"]
    #[inline(always)]
    pub const fn set_suspen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "suspend"]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable controller wake up from suspend state"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable controller wake up from suspend state"]
    #[inline(always)]
    pub const fn set_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
impl core::fmt::Debug for Power {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Power")
            .field("suspen", &self.suspen())
            .field("susp", &self.susp())
            .field("wkup", &self.wkup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Power {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Power {{ suspen: {=bool:?}, susp: {=bool:?}, wkup: {=bool:?} }}",
            self.suspen(),
            self.susp(),
            self.wkup()
        )
    }
}
#[doc = "Setup Date 0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup0(pub u16);
impl Setup0 {
    #[doc = "Setup Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 0"]
    #[inline(always)]
    pub const fn set_setupd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup0 {
    #[inline(always)]
    fn default() -> Setup0 {
        Setup0(0)
    }
}
impl core::fmt::Debug for Setup0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup0")
            .field("setupd0", &self.setupd0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup0 {{ setupd0: {=u8:?} }}", self.setupd0())
    }
}
#[doc = "Setup Date 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup1(pub u16);
impl Setup1 {
    #[doc = "Setup Data 1"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 1"]
    #[inline(always)]
    pub const fn set_setupd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup1 {
    #[inline(always)]
    fn default() -> Setup1 {
        Setup1(0)
    }
}
impl core::fmt::Debug for Setup1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup1")
            .field("setupd1", &self.setupd1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup1 {{ setupd1: {=u8:?} }}", self.setupd1())
    }
}
#[doc = "Setup Date 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup2(pub u16);
impl Setup2 {
    #[doc = "Setup Data 2"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 2"]
    #[inline(always)]
    pub const fn set_setupd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup2 {
    #[inline(always)]
    fn default() -> Setup2 {
        Setup2(0)
    }
}
impl core::fmt::Debug for Setup2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup2")
            .field("setupd2", &self.setupd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup2 {{ setupd2: {=u8:?} }}", self.setupd2())
    }
}
#[doc = "Setup Date 3 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup3(pub u16);
impl Setup3 {
    #[doc = "Setup Data 3"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 3"]
    #[inline(always)]
    pub const fn set_setupd3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup3 {
    #[inline(always)]
    fn default() -> Setup3 {
        Setup3(0)
    }
}
impl core::fmt::Debug for Setup3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup3")
            .field("setupd3", &self.setupd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup3 {{ setupd3: {=u8:?} }}", self.setupd3())
    }
}
#[doc = "Setup Date 4 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup4(pub u16);
impl Setup4 {
    #[doc = "Setup Data 4"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 4"]
    #[inline(always)]
    pub const fn set_setupd4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup4 {
    #[inline(always)]
    fn default() -> Setup4 {
        Setup4(0)
    }
}
impl core::fmt::Debug for Setup4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup4")
            .field("setupd4", &self.setupd4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup4 {{ setupd4: {=u8:?} }}", self.setupd4())
    }
}
#[doc = "Setup Date 5 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup5(pub u16);
impl Setup5 {
    #[doc = "Setup Data 5"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 5"]
    #[inline(always)]
    pub const fn set_setupd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup5 {
    #[inline(always)]
    fn default() -> Setup5 {
        Setup5(0)
    }
}
impl core::fmt::Debug for Setup5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup5")
            .field("setupd5", &self.setupd5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup5 {{ setupd5: {=u8:?} }}", self.setupd5())
    }
}
#[doc = "Setup Date 6 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup6(pub u16);
impl Setup6 {
    #[doc = "Setup Data 6"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 6"]
    #[inline(always)]
    pub const fn set_setupd6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup6 {
    #[inline(always)]
    fn default() -> Setup6 {
        Setup6(0)
    }
}
impl core::fmt::Debug for Setup6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup6")
            .field("setupd6", &self.setupd6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup6 {{ setupd6: {=u8:?} }}", self.setupd6())
    }
}
#[doc = "Setup Date 7 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup7(pub u16);
impl Setup7 {
    #[doc = "Setup Data 7"]
    #[must_use]
    #[inline(always)]
    pub const fn setupd7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Data 7"]
    #[inline(always)]
    pub const fn set_setupd7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Setup7 {
    #[inline(always)]
    fn default() -> Setup7 {
        Setup7(0)
    }
}
impl core::fmt::Debug for Setup7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup7")
            .field("setupd7", &self.setupd7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup7 {{ setupd7: {=u8:?} }}", self.setupd7())
    }
}
#[doc = "TOG CTRL1_4 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TogCtrl14(pub u16);
impl TogCtrl14 {
    #[doc = "Set End Point 1 Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 1 Toggle"]
    #[inline(always)]
    pub const fn set_dtog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Set End Point 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 1 enable"]
    #[inline(always)]
    pub const fn set_dtog1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Set End Point 2 Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 2 Toggle"]
    #[inline(always)]
    pub const fn set_dtog2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Set End Point 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog2en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 2 enable"]
    #[inline(always)]
    pub const fn set_dtog2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Set End Point 3 Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog3(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 3 Toggle"]
    #[inline(always)]
    pub const fn set_dtog3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Set End Point 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog3en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 3 enable"]
    #[inline(always)]
    pub const fn set_dtog3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Set End Point 4 Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 4 Toggle"]
    #[inline(always)]
    pub const fn set_dtog4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Set End Point 4 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtog4en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set End Point 4 enable"]
    #[inline(always)]
    pub const fn set_dtog4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for TogCtrl14 {
    #[inline(always)]
    fn default() -> TogCtrl14 {
        TogCtrl14(0)
    }
}
impl core::fmt::Debug for TogCtrl14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TogCtrl14")
            .field("dtog1", &self.dtog1())
            .field("dtog1en", &self.dtog1en())
            .field("dtog2", &self.dtog2())
            .field("dtog2en", &self.dtog2en())
            .field("dtog3", &self.dtog3())
            .field("dtog3en", &self.dtog3en())
            .field("dtog4", &self.dtog4())
            .field("dtog4en", &self.dtog4en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TogCtrl14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TogCtrl14 {{ dtog1: {=bool:?}, dtog1en: {=bool:?}, dtog2: {=bool:?}, dtog2en: {=bool:?}, dtog3: {=bool:?}, dtog3en: {=bool:?}, dtog4: {=bool:?}, dtog4en: {=bool:?} }}",
            self.dtog1(),
            self.dtog1en(),
            self.dtog2(),
            self.dtog2en(),
            self.dtog3(),
            self.dtog3en(),
            self.dtog4(),
            self.dtog4en()
        )
    }
}
#[doc = "TOG STAT1_4 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TogStat14(pub u16);
impl TogStat14 {
    #[doc = "End Point 1 Toggle IN State"]
    #[must_use]
    #[inline(always)]
    pub const fn in_tog1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 1 Toggle IN State"]
    #[inline(always)]
    pub const fn set_in_tog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "End Point 1 Toggle ON State"]
    #[must_use]
    #[inline(always)]
    pub const fn out_tog1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 1 Toggle ON State"]
    #[inline(always)]
    pub const fn set_out_tog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "End Point 2 Toggle IN State"]
    #[must_use]
    #[inline(always)]
    pub const fn in_tog2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 2 Toggle IN State"]
    #[inline(always)]
    pub const fn set_in_tog2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "End Point 2 Toggle ON State"]
    #[must_use]
    #[inline(always)]
    pub const fn out_tog2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 2 Toggle ON State"]
    #[inline(always)]
    pub const fn set_out_tog2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "End Point 3 Toggle IN State"]
    #[must_use]
    #[inline(always)]
    pub const fn in_tog3(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 3 Toggle IN State"]
    #[inline(always)]
    pub const fn set_in_tog3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "End Point 3 Toggle ON State"]
    #[must_use]
    #[inline(always)]
    pub const fn out_tog3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 3 Toggle ON State"]
    #[inline(always)]
    pub const fn set_out_tog3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "End Point 4 Toggle IN State"]
    #[must_use]
    #[inline(always)]
    pub const fn in_tog4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 4 Toggle IN State"]
    #[inline(always)]
    pub const fn set_in_tog4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "End Point 4 Toggle ON State"]
    #[must_use]
    #[inline(always)]
    pub const fn out_tog4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End Point 4 Toggle ON State"]
    #[inline(always)]
    pub const fn set_out_tog4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for TogStat14 {
    #[inline(always)]
    fn default() -> TogStat14 {
        TogStat14(0)
    }
}
impl core::fmt::Debug for TogStat14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TogStat14")
            .field("in_tog1", &self.in_tog1())
            .field("out_tog1", &self.out_tog1())
            .field("in_tog2", &self.in_tog2())
            .field("out_tog2", &self.out_tog2())
            .field("in_tog3", &self.in_tog3())
            .field("out_tog3", &self.out_tog3())
            .field("in_tog4", &self.in_tog4())
            .field("out_tog4", &self.out_tog4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TogStat14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TogStat14 {{ in_tog1: {=bool:?}, out_tog1: {=bool:?}, in_tog2: {=bool:?}, out_tog2: {=bool:?}, in_tog3: {=bool:?}, out_tog3: {=bool:?}, in_tog4: {=bool:?}, out_tog4: {=bool:?} }}",
            self.in_tog1(),
            self.out_tog1(),
            self.in_tog2(),
            self.out_tog2(),
            self.in_tog3(),
            self.out_tog3(),
            self.in_tog4(),
            self.out_tog4()
        )
    }
}
#[doc = "USB_TOP"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Top(pub u16);
impl Top {
    #[doc = "SPEED"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPEED"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "USB connection"]
    #[must_use]
    #[inline(always)]
    pub const fn connect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB connection"]
    #[inline(always)]
    pub const fn set_connect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Reset EP and FIFO in USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reset EP and FIFO in USB controller"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "USB suspend state"]
    #[must_use]
    #[inline(always)]
    pub const fn suspend(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB suspend state"]
    #[inline(always)]
    pub const fn set_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Current USB DP/DM line state"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_dmstate(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Current USB DP/DM line state"]
    #[inline(always)]
    pub const fn set_dp_dmstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u16) & 0x03) << 5usize);
    }
    #[doc = "USB bus is active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB bus is active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Top {
    #[inline(always)]
    fn default() -> Top {
        Top(0)
    }
}
impl core::fmt::Debug for Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Top")
            .field("speed", &self.speed())
            .field("connect", &self.connect())
            .field("reset", &self.reset())
            .field("suspend", &self.suspend())
            .field("dp_dmstate", &self.dp_dmstate())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Top {{ speed: {=bool:?}, connect: {=bool:?}, reset: {=bool:?}, suspend: {=bool:?}, dp_dmstate: {=u8:?}, active: {=bool:?} }}",
            self.speed(),
            self.connect(),
            self.reset(),
            self.suspend(),
            self.dp_dmstate(),
            self.active()
        )
    }
}
#[doc = "USB AHB DMA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbAhbDma(pub u16);
impl UsbAhbDma {
    #[doc = "Channel 0 burst set"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0_bs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Channel 0 burst set"]
    #[inline(always)]
    pub const fn set_ch0_bs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
    }
    #[doc = "Channel 1 burst set"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1_bs(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Channel 1 burst set"]
    #[inline(always)]
    pub const fn set_ch1_bs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u16) & 0x03) << 2usize);
    }
    #[doc = "Channel 2 burst set"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2_bs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Channel 2 burst set"]
    #[inline(always)]
    pub const fn set_ch2_bs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u16) & 0x03) << 4usize);
    }
    #[doc = "Channel 3 burst set"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3_bs(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Channel 3 burst set"]
    #[inline(always)]
    pub const fn set_ch3_bs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
    }
}
impl Default for UsbAhbDma {
    #[inline(always)]
    fn default() -> UsbAhbDma {
        UsbAhbDma(0)
    }
}
impl core::fmt::Debug for UsbAhbDma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbAhbDma")
            .field("ch0_bs", &self.ch0_bs())
            .field("ch1_bs", &self.ch1_bs())
            .field("ch2_bs", &self.ch2_bs())
            .field("ch3_bs", &self.ch3_bs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbAhbDma {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbAhbDma {{ ch0_bs: {=u8:?}, ch1_bs: {=u8:?}, ch2_bs: {=u8:?}, ch3_bs: {=u8:?} }}",
            self.ch0_bs(),
            self.ch1_bs(),
            self.ch2_bs(),
            self.ch3_bs()
        )
    }
}
#[doc = "USB AHB RST register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbAhbRst(pub u16);
impl UsbAhbRst {
    #[doc = "Endpoint 0 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ep0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint 0 reset"]
    #[inline(always)]
    pub const fn set_ep0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Endpoint 1 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ep1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint 1 reset"]
    #[inline(always)]
    pub const fn set_ep1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Endpoint 2 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ep2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint 2 reset"]
    #[inline(always)]
    pub const fn set_ep2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Endpoint 3 reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ep3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint 3 reset"]
    #[inline(always)]
    pub const fn set_ep3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for UsbAhbRst {
    #[inline(always)]
    fn default() -> UsbAhbRst {
        UsbAhbRst(0)
    }
}
impl core::fmt::Debug for UsbAhbRst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbAhbRst")
            .field("ep0", &self.ep0())
            .field("ep1", &self.ep1())
            .field("ep2", &self.ep2())
            .field("ep3", &self.ep3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbAhbRst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbAhbRst {{ ep0: {=bool:?}, ep1: {=bool:?}, ep2: {=bool:?}, ep3: {=bool:?} }}",
            self.ep0(),
            self.ep1(),
            self.ep2(),
            self.ep3()
        )
    }
}
