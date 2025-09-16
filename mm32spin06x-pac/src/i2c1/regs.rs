#[doc = "Clear ACTIVITY Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Activ(pub u32);
impl Activ {
    #[doc = "Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
    #[must_use]
    #[inline(always)]
    pub const fn activ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore"]
    #[inline(always)]
    pub const fn set_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Activ {
    #[inline(always)]
    fn default() -> Activ {
        Activ(0)
    }
}
impl core::fmt::Debug for Activ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Activ")
            .field("activ", &self.activ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Activ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Activ {{ activ: {=bool:?} }}", self.activ())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "This bit controls whether the DW_apb_i2c master is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls whether the DW_apb_i2c master is enabled"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "These bits control at which speed the DW_apb_i2c operates"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "These bits control at which speed the DW_apb_i2c operates"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "When acting as a alsve"]
    #[must_use]
    #[inline(always)]
    pub const fn slave10(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When acting as a alsve"]
    #[inline(always)]
    pub const fn set_slave10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Address mode when acting as a master"]
    #[must_use]
    #[inline(always)]
    pub const fn master10(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Address mode when acting as a master"]
    #[inline(always)]
    pub const fn set_master10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Determines whether RESTART comdtions may be sent when acting as a master"]
    #[must_use]
    #[inline(always)]
    pub const fn repen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Determines whether RESTART comdtions may be sent when acting as a master"]
    #[inline(always)]
    pub const fn set_repen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit controls whether I2C has its slave diabled"]
    #[must_use]
    #[inline(always)]
    pub const fn disslave(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls whether I2C has its slave diabled"]
    #[inline(always)]
    pub const fn set_disslave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "STOP_DET_IFADDRESSED"]
    #[must_use]
    #[inline(always)]
    pub const fn stopint(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub const fn set_stopint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit controls the generation of the TX_EMPTY interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn empint(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls the generation of the TX_EMPTY interrupt"]
    #[inline(always)]
    pub const fn set_empint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Whether to generate a STOP signal after sending or receiving"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Whether to generate a STOP signal after sending or receiving"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Whether to generate a RESTART signal after sending or receiving"]
    #[must_use]
    #[inline(always)]
    pub const fn restart(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Whether to generate a RESTART signal after sending or receiving"]
    #[inline(always)]
    pub const fn set_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "when acting as a slave"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_tx_abrt_dis(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "when acting as a slave"]
    #[inline(always)]
    pub const fn set_slv_tx_abrt_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("master", &self.master())
            .field("speed", &self.speed())
            .field("slave10", &self.slave10())
            .field("master10", &self.master10())
            .field("repen", &self.repen())
            .field("disslave", &self.disslave())
            .field("stopint", &self.stopint())
            .field("empint", &self.empint())
            .field("stop", &self.stop())
            .field("restart", &self.restart())
            .field("slv_tx_abrt_dis", &self.slv_tx_abrt_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ master: {=bool:?}, speed: {=u8:?}, slave10: {=bool:?}, master10: {=bool:?}, repen: {=bool:?}, disslave: {=bool:?}, stopint: {=bool:?}, empint: {=bool:?}, stop: {=bool:?}, restart: {=bool:?}, slv_tx_abrt_dis: {=bool:?} }}",
            self.master(),
            self.speed(),
            self.slave10(),
            self.master10(),
            self.repen(),
            self.disslave(),
            self.stopint(),
            self.empint(),
            self.stop(),
            self.restart(),
            self.slv_tx_abrt_dis()
        )
    }
}
#[doc = "DMA Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma(pub u32);
impl Dma {
    #[doc = "Receive DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA enable"]
    #[inline(always)]
    pub const fn set_rxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA enable"]
    #[inline(always)]
    pub const fn set_txen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dma {
    #[inline(always)]
    fn default() -> Dma {
        Dma(0)
    }
}
impl core::fmt::Debug for Dma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma")
            .field("rxen", &self.rxen())
            .field("txen", &self.txen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma {{ rxen: {=bool:?}, txen: {=bool:?} }}",
            self.rxen(),
            self.txen()
        )
    }
}
#[doc = "Data Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc = "This register contains the data to be transimitted or received on the i2c bus."]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This register contains the data to be transimitted or received on the i2c bus."]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "This bit controls whether a read or a write is perormed"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls whether a read or a write is perormed"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr")
            .field("dat", &self.dat())
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr {{ dat: {=u8:?}, cmd: {=bool:?} }}",
            self.dat(),
            self.cmd()
        )
    }
}
#[doc = "Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enr(pub u32);
impl Enr {
    #[doc = "I2C mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C mode enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "I2C transfer abort"]
    #[must_use]
    #[inline(always)]
    pub const fn abort(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "I2C transfer abort"]
    #[inline(always)]
    pub const fn set_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Enr {
    #[inline(always)]
    fn default() -> Enr {
        Enr(0)
    }
}
impl core::fmt::Debug for Enr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enr")
            .field("enable", &self.enable())
            .field("abort", &self.abort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enr {{ enable: {=bool:?}, abort: {=bool:?} }}",
            self.enable(),
            self.abort()
        )
    }
}
#[doc = "SCL High Period Count for Fast Speed Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fshr(pub u32);
impl Fshr {
    #[doc = "This register sets the SCL clock high_period count for standard speed"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register sets the SCL clock high_period count for standard speed"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Fshr {
    #[inline(always)]
    fn default() -> Fshr {
        Fshr(0)
    }
}
impl core::fmt::Debug for Fshr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fshr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fshr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fshr {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "SCL Low Period Count for Fast Speed Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fslr(pub u32);
impl Fslr {
    #[doc = "This register sets the SCL clock low period count for standard speed"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Fslr {
    #[inline(always)]
    fn default() -> Fslr {
        Fslr(0)
    }
}
impl core::fmt::Debug for Fslr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fslr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fslr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fslr {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Clear GEN_CALL Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gc(pub u32);
impl Gc {
    #[doc = "Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the GEN_CALL interrupt(bit 11)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Gc {
    #[inline(always)]
    fn default() -> Gc {
        Gc(0)
    }
}
impl core::fmt::Debug for Gc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gc").field("gc", &self.gc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gc {{ gc: {=bool:?} }}", self.gc())
    }
}
#[doc = "ACK General Call Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "ACK general call"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ACK general call"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Gcr").field("gc", &self.gc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gcr {{ gc: {=bool:?} }}", self.gc())
    }
}
#[doc = "SDA Hold Time Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hold(pub u32);
impl Hold {
    #[doc = "Sets the required SDA hold time in units of ic_clk period"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_hold(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub const fn set_tx_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_hold(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period"]
    #[inline(always)]
    pub const fn set_rx_hold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Hold {
    #[inline(always)]
    fn default() -> Hold {
        Hold(0)
    }
}
impl core::fmt::Debug for Hold {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hold")
            .field("tx_hold", &self.tx_hold())
            .field("rx_hold", &self.rx_hold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hold {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hold {{ tx_hold: {=u16:?}, rx_hold: {=u8:?} }}",
            self.tx_hold(),
            self.rx_hold()
        )
    }
}
#[doc = "Clear All Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Read this register to clear the combined interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn icr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the combined interrupt"]
    #[inline(always)]
    pub const fn set_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        f.debug_struct("Icr").field("icr", &self.icr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icr {{ icr: {=bool:?} }}", self.icr())
    }
}
#[doc = "Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Specific bit description shield RAWISR"]
    #[must_use]
    #[inline(always)]
    pub const fn imr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Specific bit description shield RAWISR"]
    #[inline(always)]
    pub const fn set_imr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr").field("imr", &self.imr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imr {{ imr: {=u16:?} }}", self.imr())
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Specific bit description refer to RAWISR"]
    #[must_use]
    #[inline(always)]
    pub const fn isr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Specific bit description refer to RAWISR"]
    #[inline(always)]
    pub const fn set_isr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
        f.debug_struct("Isr").field("isr", &self.isr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isr {{ isr: {=u16:?} }}", self.isr())
    }
}
#[doc = "RAW Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rawisr(pub u32);
impl Rawisr {
    #[doc = "Receive buffer under"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_under(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer under"]
    #[inline(always)]
    pub const fn set_rx_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive buffer over"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_over(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer over"]
    #[inline(always)]
    pub const fn set_rx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive buffer not empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_full(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer not empty"]
    #[inline(always)]
    pub const fn set_rx_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit buffer over"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_over(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer over"]
    #[inline(always)]
    pub const fn set_tx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit buffer empty"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer empty"]
    #[inline(always)]
    pub const fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read request"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Read request"]
    #[inline(always)]
    pub const fn set_rd_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit abort"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_abrt(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit abort"]
    #[inline(always)]
    pub const fn set_tx_abrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit done"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit done"]
    #[inline(always)]
    pub const fn set_rx_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[must_use]
    #[inline(always)]
    pub const fn activ(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This bit captures DW_spb_i2c acticity and stays set until it is cleared"]
    #[inline(always)]
    pub const fn set_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop condition detection"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stop condition detection"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Start condition detection"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Start condition detection"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "General call"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "General call"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Rawisr {
    #[inline(always)]
    fn default() -> Rawisr {
        Rawisr(0)
    }
}
impl core::fmt::Debug for Rawisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rawisr")
            .field("rx_under", &self.rx_under())
            .field("rx_over", &self.rx_over())
            .field("rx_full", &self.rx_full())
            .field("tx_over", &self.tx_over())
            .field("tx_empty", &self.tx_empty())
            .field("rd_req", &self.rd_req())
            .field("tx_abrt", &self.tx_abrt())
            .field("rx_done", &self.rx_done())
            .field("activ", &self.activ())
            .field("stop", &self.stop())
            .field("start", &self.start())
            .field("gc", &self.gc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rawisr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rawisr {{ rx_under: {=bool:?}, rx_over: {=bool:?}, rx_full: {=bool:?}, tx_over: {=bool:?}, tx_empty: {=bool:?}, rd_req: {=bool:?}, tx_abrt: {=bool:?}, rx_done: {=bool:?}, activ: {=bool:?}, stop: {=bool:?}, start: {=bool:?}, gc: {=bool:?} }}",
            self.rx_under(),
            self.rx_over(),
            self.rx_full(),
            self.tx_over(),
            self.tx_empty(),
            self.rd_req(),
            self.tx_abrt(),
            self.rx_done(),
            self.activ(),
            self.stop(),
            self.start(),
            self.gc()
        )
    }
}
#[doc = "Clear RD_REQ Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdReq(pub u32);
impl RdReq {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 5)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_rd_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RdReq {
    #[inline(always)]
    fn default() -> RdReq {
        RdReq(0)
    }
}
impl core::fmt::Debug for RdReq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdReq")
            .field("rd_req", &self.rd_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdReq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdReq {{ rd_req: {=bool:?} }}", self.rd_req())
    }
}
#[doc = "Clear RX_DONE Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDone(pub u32);
impl RxDone {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 7)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_rx_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RxDone {
    #[inline(always)]
    fn default() -> RxDone {
        RxDone(0)
    }
}
impl core::fmt::Debug for RxDone {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxDone")
            .field("rx_done", &self.rx_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxDone {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxDone {{ rx_done: {=bool:?} }}", self.rx_done())
    }
}
#[doc = "Clear RX_OVER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxOver(pub u32);
impl RxOver {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_over(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 1)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_rx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RxOver {
    #[inline(always)]
    fn default() -> RxOver {
        RxOver(0)
    }
}
impl core::fmt::Debug for RxOver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxOver")
            .field("rx_over", &self.rx_over())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxOver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxOver {{ rx_over: {=bool:?} }}", self.rx_over())
    }
}
#[doc = "Clear RX_UNDER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxUnder(pub u32);
impl RxUnder {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_under(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 0)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_rx_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RxUnder {
    #[inline(always)]
    fn default() -> RxUnder {
        RxUnder(0)
    }
}
impl core::fmt::Debug for RxUnder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxUnder")
            .field("rx_under", &self.rx_under())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxUnder {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxUnder {{ rx_under: {=bool:?} }}", self.rx_under())
    }
}
#[doc = "Receive FIFO Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxflr(pub u32);
impl Rxflr {
    #[doc = "Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO level. Contains the number of valid data entires in the receive FIFO"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Rxflr {
    #[inline(always)]
    fn default() -> Rxflr {
        Rxflr(0)
    }
}
impl core::fmt::Debug for Rxflr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxflr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxflr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxflr {{ cnt: {=u8:?} }}", self.cnt())
    }
}
#[doc = "Receive FIFO Threshold Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxtlr(pub u32);
impl Rxtlr {
    #[doc = "Receive FIFO threshold level"]
    #[must_use]
    #[inline(always)]
    pub const fn tl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO threshold level"]
    #[inline(always)]
    pub const fn set_tl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxtlr {
    #[inline(always)]
    fn default() -> Rxtlr {
        Rxtlr(0)
    }
}
impl core::fmt::Debug for Rxtlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxtlr").field("tl", &self.tl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxtlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxtlr {{ tl: {=u8:?} }}", self.tl())
    }
}
#[doc = "Slave Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sar(pub u32);
impl Sar {
    #[doc = "The SAR holds the slave address when the i2c is operation as a slave"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "The SAR holds the slave address when the i2c is operation as a slave"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Sar {
    #[inline(always)]
    fn default() -> Sar {
        Sar(0)
    }
}
impl core::fmt::Debug for Sar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sar").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sar {{ addr: {=u16:?} }}", self.addr())
    }
}
#[doc = "SDA Setup Time Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setup(pub u32);
impl Setup {
    #[doc = "SDA setup"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SDA setup"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Setup {
    #[inline(always)]
    fn default() -> Setup {
        Setup(0)
    }
}
impl core::fmt::Debug for Setup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setup").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Setup {{ cnt: {=u8:?} }}", self.cnt())
    }
}
#[doc = "Slave Address Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvmask(pub u32);
impl Slvmask {
    #[doc = "Slave Address Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Slave Address Mask"]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Slvmask {
    #[inline(always)]
    fn default() -> Slvmask {
        Slvmask(0)
    }
}
impl core::fmt::Debug for Slvmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slvmask")
            .field("mask", &self.mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slvmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Slvmask {{ mask: {=u16:?} }}", self.mask())
    }
}
#[doc = "Receiver Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvrcvaddr(pub u32);
impl Slvrcvaddr {
    #[doc = "Slave Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Slave Address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Slvrcvaddr {
    #[inline(always)]
    fn default() -> Slvrcvaddr {
        Slvrcvaddr(0)
    }
}
impl core::fmt::Debug for Slvrcvaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slvrcvaddr")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slvrcvaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Slvrcvaddr {{ addr: {=u16:?} }}", self.addr())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "I2C activity status"]
    #[must_use]
    #[inline(always)]
    pub const fn activ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "I2C activity status"]
    #[inline(always)]
    pub const fn set_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO not full"]
    #[must_use]
    #[inline(always)]
    pub const fn tfnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full"]
    #[inline(always)]
    pub const fn set_tfnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO completely empty"]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO completely empty"]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO not empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rfne(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty"]
    #[inline(always)]
    pub const fn set_rfne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO completely full"]
    #[must_use]
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO completely full"]
    #[inline(always)]
    pub const fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master FSM activity status"]
    #[must_use]
    #[inline(always)]
    pub const fn mst_activ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Master FSM activity status"]
    #[inline(always)]
    pub const fn set_mst_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Slave FSM activity status"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_activ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Slave FSM activity status"]
    #[inline(always)]
    pub const fn set_slv_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("activ", &self.activ())
            .field("tfnf", &self.tfnf())
            .field("tfe", &self.tfe())
            .field("rfne", &self.rfne())
            .field("rff", &self.rff())
            .field("mst_activ", &self.mst_activ())
            .field("slv_activ", &self.slv_activ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ activ: {=bool:?}, tfnf: {=bool:?}, tfe: {=bool:?}, rfne: {=bool:?}, rff: {=bool:?}, mst_activ: {=bool:?}, slv_activ: {=bool:?} }}",
            self.activ(),
            self.tfnf(),
            self.tfe(),
            self.rfne(),
            self.rff(),
            self.mst_activ(),
            self.slv_activ()
        )
    }
}
#[doc = "SCL High Period Count for Std. Speed Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sshr(pub u32);
impl Sshr {
    #[doc = "This register sets the SCL clock high period count for standard speed"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register sets the SCL clock high period count for standard speed"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sshr {
    #[inline(always)]
    fn default() -> Sshr {
        Sshr(0)
    }
}
impl core::fmt::Debug for Sshr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sshr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sshr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sshr {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "SCL Low Period Count for Std. Speed Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sslr(pub u32);
impl Sslr {
    #[doc = "This register sets the SCL clock low period count for standard speed"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register sets the SCL clock low period count for standard speed"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sslr {
    #[inline(always)]
    fn default() -> Sslr {
        Sslr(0)
    }
}
impl core::fmt::Debug for Sslr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sslr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sslr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sslr {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Clear START_DET Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the START_DET interrupt(bit 10)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start")
            .field("start", &self.start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Start {{ start: {=bool:?} }}", self.start())
    }
}
#[doc = "Clear STOP_DET Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the STOP_DET interrupt(bit 9)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop").field("stop", &self.stop()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stop {{ stop: {=bool:?} }}", self.stop())
    }
}
#[doc = "Target Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar(pub u32);
impl Tar {
    #[doc = "This is the target address for any master transaction"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "This is the target address for any master transaction"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If bit 11(SPECIAL)is set to 1"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If bit 11(SPECIAL)is set to 1"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[must_use]
    #[inline(always)]
    pub const fn special(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates whether software performs a General Call or START BYTE conmmend"]
    #[inline(always)]
    pub const fn set_special(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Tar {
    #[inline(always)]
    fn default() -> Tar {
        Tar(0)
    }
}
impl core::fmt::Debug for Tar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tar")
            .field("addr", &self.addr())
            .field("gc", &self.gc())
            .field("special", &self.special())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tar {{ addr: {=u16:?}, gc: {=bool:?}, special: {=bool:?} }}",
            self.addr(),
            self.gc(),
            self.special()
        )
    }
}
#[doc = "Clear TX_ABRT Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxAbrt(pub u32);
impl TxAbrt {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_abrt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 6)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_tx_abrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TxAbrt {
    #[inline(always)]
    fn default() -> TxAbrt {
        TxAbrt(0)
    }
}
impl core::fmt::Debug for TxAbrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxAbrt")
            .field("tx_abrt", &self.tx_abrt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxAbrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxAbrt {{ tx_abrt: {=bool:?} }}", self.tx_abrt())
    }
}
#[doc = "Clear TX_OVER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOver(pub u32);
impl TxOver {
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_over(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt(bit 3)of the RAWISR register"]
    #[inline(always)]
    pub const fn set_tx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TxOver {
    #[inline(always)]
    fn default() -> TxOver {
        TxOver(0)
    }
}
impl core::fmt::Debug for TxOver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxOver")
            .field("tx_over", &self.tx_over())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxOver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxOver {{ tx_over: {=bool:?} }}", self.tx_over())
    }
}
#[doc = "Transmit FIFO Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txflr(pub u32);
impl Txflr {
    #[doc = "Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO level.Contains the number of valid data entires in the transmit FIFO"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Txflr {
    #[inline(always)]
    fn default() -> Txflr {
        Txflr(0)
    }
}
impl core::fmt::Debug for Txflr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txflr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txflr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txflr {{ cnt: {=u8:?} }}", self.cnt())
    }
}
#[doc = "Transmit FIFO Threshold Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txtlr(pub u32);
impl Txtlr {
    #[doc = "Transmit FIFO threshold level"]
    #[must_use]
    #[inline(always)]
    pub const fn tl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO threshold level"]
    #[inline(always)]
    pub const fn set_tl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txtlr {
    #[inline(always)]
    fn default() -> Txtlr {
        Txtlr(0)
    }
}
impl core::fmt::Debug for Txtlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txtlr").field("tl", &self.tl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txtlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txtlr {{ tl: {=u8:?} }}", self.tl())
    }
}
