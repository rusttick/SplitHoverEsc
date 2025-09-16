#[doc = "Filter Mode register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afm0(pub u32);
impl Afm0 {
    #[doc = "Acceptance filter mode"]
    #[must_use]
    #[inline(always)]
    pub const fn afm_7_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Acceptance filter mode"]
    #[inline(always)]
    pub const fn set_afm_7_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Afm0 {
    #[inline(always)]
    fn default() -> Afm0 {
        Afm0(0)
    }
}
impl core::fmt::Debug for Afm0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afm0")
            .field("afm_7_1", &self.afm_7_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afm0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Afm0 {{ afm_7_1: {=u8:?} }}", self.afm_7_1())
    }
}
#[doc = "Filter Mode register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afm1(pub u32);
impl Afm1 {
    #[doc = "Acceptance filter mode"]
    #[must_use]
    #[inline(always)]
    pub const fn afm_15_8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance filter mode"]
    #[inline(always)]
    pub const fn set_afm_15_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Afm1 {
    #[inline(always)]
    fn default() -> Afm1 {
        Afm1(0)
    }
}
impl core::fmt::Debug for Afm1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afm1")
            .field("afm_15_8", &self.afm_15_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afm1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Afm1 {{ afm_15_8: {=u8:?} }}", self.afm_15_8())
    }
}
#[doc = "Filter Mode register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afm2(pub u32);
impl Afm2 {
    #[doc = "Acceptance filter mode"]
    #[must_use]
    #[inline(always)]
    pub const fn afm_19_16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Acceptance filter mode"]
    #[inline(always)]
    pub const fn set_afm_19_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Afm2 {
    #[inline(always)]
    fn default() -> Afm2 {
        Afm2(0)
    }
}
impl core::fmt::Debug for Afm2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Afm2")
            .field("afm_19_16", &self.afm_19_16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Afm2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Afm2 {{ afm_19_16: {=u8:?} }}", self.afm_19_16())
    }
}
#[doc = "Peli Arbitration Lost Capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlcP(pub u32);
impl AlcP {
    #[doc = "Bit number"]
    #[must_use]
    #[inline(always)]
    pub const fn bitno(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Bit number"]
    #[inline(always)]
    pub const fn set_bitno(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for AlcP {
    #[inline(always)]
    fn default() -> AlcP {
        AlcP(0)
    }
}
impl core::fmt::Debug for AlcP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AlcP")
            .field("bitno", &self.bitno())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AlcP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AlcP {{ bitno: {=u8:?} }}", self.bitno())
    }
}
#[doc = "Bus Timing register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btr0(pub u32);
impl Btr0 {
    #[doc = "Baud rate prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn brp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Baud rate prescaler"]
    #[inline(always)]
    pub const fn set_brp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Synchronization jump width"]
    #[must_use]
    #[inline(always)]
    pub const fn sjw(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Synchronization jump width"]
    #[inline(always)]
    pub const fn set_sjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Btr0 {
    #[inline(always)]
    fn default() -> Btr0 {
        Btr0(0)
    }
}
impl core::fmt::Debug for Btr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btr0")
            .field("brp", &self.brp())
            .field("sjw", &self.sjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btr0 {{ brp: {=u8:?}, sjw: {=u8:?} }}",
            self.brp(),
            self.sjw()
        )
    }
}
#[doc = "Bus Timing register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btr1(pub u32);
impl Btr1 {
    #[doc = "Time segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn tseg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Time segment 1"]
    #[inline(always)]
    pub const fn set_tseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn tseg2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Time segment 2"]
    #[inline(always)]
    pub const fn set_tseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn sam(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sampling"]
    #[inline(always)]
    pub const fn set_sam(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Btr1 {
    #[inline(always)]
    fn default() -> Btr1 {
        Btr1(0)
    }
}
impl core::fmt::Debug for Btr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Btr1")
            .field("tseg1", &self.tseg1())
            .field("tseg2", &self.tseg2())
            .field("sam", &self.sam())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Btr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Btr1 {{ tseg1: {=u8:?}, tseg2: {=u8:?}, sam: {=bool:?} }}",
            self.tseg1(),
            self.tseg2(),
            self.sam()
        )
    }
}
#[doc = "Clock Divider register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "CAN mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CAN mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr").field("mode", &self.mode()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdr {{ mode: {=bool:?} }}", self.mode())
    }
}
#[doc = "Basic Command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmrB(pub u32);
impl CmrB {
    #[doc = "Transmission request"]
    #[must_use]
    #[inline(always)]
    pub const fn tr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission request"]
    #[inline(always)]
    pub const fn set_tr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Abort transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn at(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Abort transmission"]
    #[inline(always)]
    pub const fn set_at(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Release receive buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rrb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Release receive buffer"]
    #[inline(always)]
    pub const fn set_rrb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear data overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn cdo(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear data overrun"]
    #[inline(always)]
    pub const fn set_cdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Empty Rxbuffer"]
    #[must_use]
    #[inline(always)]
    pub const fn erb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Empty Rxbuffer"]
    #[inline(always)]
    pub const fn set_erb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CmrB {
    #[inline(always)]
    fn default() -> CmrB {
        CmrB(0)
    }
}
impl core::fmt::Debug for CmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmrB")
            .field("tr", &self.tr())
            .field("at", &self.at())
            .field("rrb", &self.rrb())
            .field("cdo", &self.cdo())
            .field("erb", &self.erb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmrB {{ tr: {=bool:?}, at: {=bool:?}, rrb: {=bool:?}, cdo: {=bool:?}, erb: {=bool:?} }}",
            self.tr(),
            self.at(),
            self.rrb(),
            self.cdo(),
            self.erb()
        )
    }
}
#[doc = "Peli Command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmrP(pub u32);
impl CmrP {
    #[doc = "Transmission request"]
    #[must_use]
    #[inline(always)]
    pub const fn tr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission request"]
    #[inline(always)]
    pub const fn set_tr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Abort transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn at(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Abort transmission"]
    #[inline(always)]
    pub const fn set_at(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Release receive buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rrb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Release receive buffer"]
    #[inline(always)]
    pub const fn set_rrb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear data overrun"]
    #[must_use]
    #[inline(always)]
    pub const fn cdo(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear data overrun"]
    #[inline(always)]
    pub const fn set_cdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Self reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Self reset request"]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Empty Rxbuffer"]
    #[must_use]
    #[inline(always)]
    pub const fn erb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Empty Rxbuffer"]
    #[inline(always)]
    pub const fn set_erb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CmrP {
    #[inline(always)]
    fn default() -> CmrP {
        CmrP(0)
    }
}
impl core::fmt::Debug for CmrP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmrP")
            .field("tr", &self.tr())
            .field("at", &self.at())
            .field("rrb", &self.rrb())
            .field("cdo", &self.cdo())
            .field("srr", &self.srr())
            .field("erb", &self.erb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmrP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmrP {{ tr: {=bool:?}, at: {=bool:?}, rrb: {=bool:?}, cdo: {=bool:?}, srr: {=bool:?}, erb: {=bool:?} }}",
            self.tr(),
            self.at(),
            self.rrb(),
            self.cdo(),
            self.srr(),
            self.erb()
        )
    }
}
#[doc = "Basic control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrB(pub u32);
impl CrB {
    #[doc = "Reset request"]
    #[must_use]
    #[inline(always)]
    pub const fn rr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reset request"]
    #[inline(always)]
    pub const fn set_rr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn set_eie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Overflow interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub const fn set_oie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for CrB {
    #[inline(always)]
    fn default() -> CrB {
        CrB(0)
    }
}
impl core::fmt::Debug for CrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CrB")
            .field("rr", &self.rr())
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("eie", &self.eie())
            .field("oie", &self.oie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CrB {{ rr: {=bool:?}, rie: {=bool:?}, tie: {=bool:?}, eie: {=bool:?}, oie: {=bool:?} }}",
            self.rr(),
            self.rie(),
            self.tie(),
            self.eie(),
            self.oie()
        )
    }
}
#[doc = "Peli Error Code Capture register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccP(pub u32);
impl EccP {
    #[doc = "SEG"]
    #[must_use]
    #[inline(always)]
    pub const fn seg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SEG"]
    #[inline(always)]
    pub const fn set_seg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Error code"]
    #[must_use]
    #[inline(always)]
    pub const fn errc(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Error code"]
    #[inline(always)]
    pub const fn set_errc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for EccP {
    #[inline(always)]
    fn default() -> EccP {
        EccP(0)
    }
}
impl core::fmt::Debug for EccP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EccP")
            .field("seg", &self.seg())
            .field("dir", &self.dir())
            .field("errc", &self.errc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EccP {{ seg: {=u8:?}, dir: {=bool:?}, errc: {=u8:?} }}",
            self.seg(),
            self.dir(),
            self.errc()
        )
    }
}
#[doc = "Peli Error Warning Limit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EwlrP(pub u32);
impl EwlrP {
    #[doc = "Programmable error warning limit"]
    #[must_use]
    #[inline(always)]
    pub const fn ewl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable error warning limit"]
    #[inline(always)]
    pub const fn set_ewl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for EwlrP {
    #[inline(always)]
    fn default() -> EwlrP {
        EwlrP(0)
    }
}
impl core::fmt::Debug for EwlrP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EwlrP").field("ewl", &self.ewl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EwlrP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EwlrP {{ ewl: {=u8:?} }}", self.ewl())
    }
}
#[doc = "Filter Group Enable Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fga0(pub u32);
impl Fga0 {
    #[doc = "Filter group enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fga_7_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Filter group enable"]
    #[inline(always)]
    pub const fn set_fga_7_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fga0 {
    #[inline(always)]
    fn default() -> Fga0 {
        Fga0(0)
    }
}
impl core::fmt::Debug for Fga0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fga0")
            .field("fga_7_0", &self.fga_7_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fga0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fga0 {{ fga_7_0: {=u8:?} }}", self.fga_7_0())
    }
}
#[doc = "Filter Group Enable Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fga1(pub u32);
impl Fga1 {
    #[doc = "Filter group enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fga_15_8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Filter group enable"]
    #[inline(always)]
    pub const fn set_fga_15_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fga1 {
    #[inline(always)]
    fn default() -> Fga1 {
        Fga1(0)
    }
}
impl core::fmt::Debug for Fga1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fga1")
            .field("fga_15_8", &self.fga_15_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fga1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fga1 {{ fga_15_8: {=u8:?} }}", self.fga_15_8())
    }
}
#[doc = "Filter Group Enable Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fga2(pub u32);
impl Fga2 {
    #[doc = "Filter group enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fga_19_16(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Filter group enable"]
    #[inline(always)]
    pub const fn set_fga_19_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Fga2 {
    #[inline(always)]
    fn default() -> Fga2 {
        Fga2(0)
    }
}
impl core::fmt::Debug for Fga2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fga2")
            .field("fga_19_16", &self.fga_19_16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fga2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fga2 {{ fga_19_16: {=u8:?} }}", self.fga_19_16())
    }
}
#[doc = "Peli Acceptance Code register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Acr0P(pub u32);
impl Group0Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Acr0P {
    #[inline(always)]
    fn default() -> Group0Acr0P {
        Group0Acr0P(0)
    }
}
impl core::fmt::Debug for Group0Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Acr1P(pub u32);
impl Group0Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Acr1P {
    #[inline(always)]
    fn default() -> Group0Acr1P {
        Group0Acr1P(0)
    }
}
impl core::fmt::Debug for Group0Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Acr2P(pub u32);
impl Group0Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Acr2P {
    #[inline(always)]
    fn default() -> Group0Acr2P {
        Group0Acr2P(0)
    }
}
impl core::fmt::Debug for Group0Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Acr3P(pub u32);
impl Group0Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Acr3P {
    #[inline(always)]
    fn default() -> Group0Acr3P {
        Group0Acr3P(0)
    }
}
impl core::fmt::Debug for Group0Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0AcrB(pub u32);
impl Group0AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0AcrB {
    #[inline(always)]
    fn default() -> Group0AcrB {
        Group0AcrB(0)
    }
}
impl core::fmt::Debug for Group0AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Amr0P(pub u32);
impl Group0Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Amr0P {
    #[inline(always)]
    fn default() -> Group0Amr0P {
        Group0Amr0P(0)
    }
}
impl core::fmt::Debug for Group0Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Amr1P(pub u32);
impl Group0Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Amr1P {
    #[inline(always)]
    fn default() -> Group0Amr1P {
        Group0Amr1P(0)
    }
}
impl core::fmt::Debug for Group0Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Amr2P(pub u32);
impl Group0Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Amr2P {
    #[inline(always)]
    fn default() -> Group0Amr2P {
        Group0Amr2P(0)
    }
}
impl core::fmt::Debug for Group0Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0Amr3P(pub u32);
impl Group0Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0Amr3P {
    #[inline(always)]
    fn default() -> Group0Amr3P {
        Group0Amr3P(0)
    }
}
impl core::fmt::Debug for Group0Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0AmrB(pub u32);
impl Group0AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group0AmrB {
    #[inline(always)]
    fn default() -> Group0AmrB {
        Group0AmrB(0)
    }
}
impl core::fmt::Debug for Group0AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group0AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group0AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group0AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Acr0P(pub u32);
impl Group10Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Acr0P {
    #[inline(always)]
    fn default() -> Group10Acr0P {
        Group10Acr0P(0)
    }
}
impl core::fmt::Debug for Group10Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Acr1P(pub u32);
impl Group10Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Acr1P {
    #[inline(always)]
    fn default() -> Group10Acr1P {
        Group10Acr1P(0)
    }
}
impl core::fmt::Debug for Group10Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Acr2P(pub u32);
impl Group10Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Acr2P {
    #[inline(always)]
    fn default() -> Group10Acr2P {
        Group10Acr2P(0)
    }
}
impl core::fmt::Debug for Group10Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Acr3P(pub u32);
impl Group10Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Acr3P {
    #[inline(always)]
    fn default() -> Group10Acr3P {
        Group10Acr3P(0)
    }
}
impl core::fmt::Debug for Group10Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10AcrB(pub u32);
impl Group10AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10AcrB {
    #[inline(always)]
    fn default() -> Group10AcrB {
        Group10AcrB(0)
    }
}
impl core::fmt::Debug for Group10AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Amr0P(pub u32);
impl Group10Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Amr0P {
    #[inline(always)]
    fn default() -> Group10Amr0P {
        Group10Amr0P(0)
    }
}
impl core::fmt::Debug for Group10Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Amr1P(pub u32);
impl Group10Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Amr1P {
    #[inline(always)]
    fn default() -> Group10Amr1P {
        Group10Amr1P(0)
    }
}
impl core::fmt::Debug for Group10Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Amr2P(pub u32);
impl Group10Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Amr2P {
    #[inline(always)]
    fn default() -> Group10Amr2P {
        Group10Amr2P(0)
    }
}
impl core::fmt::Debug for Group10Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10Amr3P(pub u32);
impl Group10Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10Amr3P {
    #[inline(always)]
    fn default() -> Group10Amr3P {
        Group10Amr3P(0)
    }
}
impl core::fmt::Debug for Group10Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group10AmrB(pub u32);
impl Group10AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group10AmrB {
    #[inline(always)]
    fn default() -> Group10AmrB {
        Group10AmrB(0)
    }
}
impl core::fmt::Debug for Group10AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group10AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group10AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group10AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Acr0P(pub u32);
impl Group11Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Acr0P {
    #[inline(always)]
    fn default() -> Group11Acr0P {
        Group11Acr0P(0)
    }
}
impl core::fmt::Debug for Group11Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Acr1P(pub u32);
impl Group11Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Acr1P {
    #[inline(always)]
    fn default() -> Group11Acr1P {
        Group11Acr1P(0)
    }
}
impl core::fmt::Debug for Group11Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Acr2P(pub u32);
impl Group11Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Acr2P {
    #[inline(always)]
    fn default() -> Group11Acr2P {
        Group11Acr2P(0)
    }
}
impl core::fmt::Debug for Group11Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Acr3P(pub u32);
impl Group11Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Acr3P {
    #[inline(always)]
    fn default() -> Group11Acr3P {
        Group11Acr3P(0)
    }
}
impl core::fmt::Debug for Group11Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11AcrB(pub u32);
impl Group11AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11AcrB {
    #[inline(always)]
    fn default() -> Group11AcrB {
        Group11AcrB(0)
    }
}
impl core::fmt::Debug for Group11AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Amr0P(pub u32);
impl Group11Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Amr0P {
    #[inline(always)]
    fn default() -> Group11Amr0P {
        Group11Amr0P(0)
    }
}
impl core::fmt::Debug for Group11Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Amr1P(pub u32);
impl Group11Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Amr1P {
    #[inline(always)]
    fn default() -> Group11Amr1P {
        Group11Amr1P(0)
    }
}
impl core::fmt::Debug for Group11Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Amr2P(pub u32);
impl Group11Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Amr2P {
    #[inline(always)]
    fn default() -> Group11Amr2P {
        Group11Amr2P(0)
    }
}
impl core::fmt::Debug for Group11Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11Amr3P(pub u32);
impl Group11Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11Amr3P {
    #[inline(always)]
    fn default() -> Group11Amr3P {
        Group11Amr3P(0)
    }
}
impl core::fmt::Debug for Group11Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group11AmrB(pub u32);
impl Group11AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group11AmrB {
    #[inline(always)]
    fn default() -> Group11AmrB {
        Group11AmrB(0)
    }
}
impl core::fmt::Debug for Group11AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group11AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group11AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group11AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Acr0P(pub u32);
impl Group12Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Acr0P {
    #[inline(always)]
    fn default() -> Group12Acr0P {
        Group12Acr0P(0)
    }
}
impl core::fmt::Debug for Group12Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Acr1P(pub u32);
impl Group12Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Acr1P {
    #[inline(always)]
    fn default() -> Group12Acr1P {
        Group12Acr1P(0)
    }
}
impl core::fmt::Debug for Group12Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Acr2P(pub u32);
impl Group12Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Acr2P {
    #[inline(always)]
    fn default() -> Group12Acr2P {
        Group12Acr2P(0)
    }
}
impl core::fmt::Debug for Group12Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Acr3P(pub u32);
impl Group12Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Acr3P {
    #[inline(always)]
    fn default() -> Group12Acr3P {
        Group12Acr3P(0)
    }
}
impl core::fmt::Debug for Group12Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12AcrB(pub u32);
impl Group12AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12AcrB {
    #[inline(always)]
    fn default() -> Group12AcrB {
        Group12AcrB(0)
    }
}
impl core::fmt::Debug for Group12AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Amr0P(pub u32);
impl Group12Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Amr0P {
    #[inline(always)]
    fn default() -> Group12Amr0P {
        Group12Amr0P(0)
    }
}
impl core::fmt::Debug for Group12Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Amr1P(pub u32);
impl Group12Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Amr1P {
    #[inline(always)]
    fn default() -> Group12Amr1P {
        Group12Amr1P(0)
    }
}
impl core::fmt::Debug for Group12Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Amr2P(pub u32);
impl Group12Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Amr2P {
    #[inline(always)]
    fn default() -> Group12Amr2P {
        Group12Amr2P(0)
    }
}
impl core::fmt::Debug for Group12Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12Amr3P(pub u32);
impl Group12Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12Amr3P {
    #[inline(always)]
    fn default() -> Group12Amr3P {
        Group12Amr3P(0)
    }
}
impl core::fmt::Debug for Group12Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group12AmrB(pub u32);
impl Group12AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group12AmrB {
    #[inline(always)]
    fn default() -> Group12AmrB {
        Group12AmrB(0)
    }
}
impl core::fmt::Debug for Group12AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group12AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group12AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group12AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Acr0P(pub u32);
impl Group13Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Acr0P {
    #[inline(always)]
    fn default() -> Group13Acr0P {
        Group13Acr0P(0)
    }
}
impl core::fmt::Debug for Group13Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Acr1P(pub u32);
impl Group13Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Acr1P {
    #[inline(always)]
    fn default() -> Group13Acr1P {
        Group13Acr1P(0)
    }
}
impl core::fmt::Debug for Group13Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Acr2P(pub u32);
impl Group13Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Acr2P {
    #[inline(always)]
    fn default() -> Group13Acr2P {
        Group13Acr2P(0)
    }
}
impl core::fmt::Debug for Group13Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Acr3P(pub u32);
impl Group13Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Acr3P {
    #[inline(always)]
    fn default() -> Group13Acr3P {
        Group13Acr3P(0)
    }
}
impl core::fmt::Debug for Group13Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13AcrB(pub u32);
impl Group13AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13AcrB {
    #[inline(always)]
    fn default() -> Group13AcrB {
        Group13AcrB(0)
    }
}
impl core::fmt::Debug for Group13AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Amr0P(pub u32);
impl Group13Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Amr0P {
    #[inline(always)]
    fn default() -> Group13Amr0P {
        Group13Amr0P(0)
    }
}
impl core::fmt::Debug for Group13Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Amr1P(pub u32);
impl Group13Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Amr1P {
    #[inline(always)]
    fn default() -> Group13Amr1P {
        Group13Amr1P(0)
    }
}
impl core::fmt::Debug for Group13Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Amr2P(pub u32);
impl Group13Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Amr2P {
    #[inline(always)]
    fn default() -> Group13Amr2P {
        Group13Amr2P(0)
    }
}
impl core::fmt::Debug for Group13Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13Amr3P(pub u32);
impl Group13Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13Amr3P {
    #[inline(always)]
    fn default() -> Group13Amr3P {
        Group13Amr3P(0)
    }
}
impl core::fmt::Debug for Group13Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group13AmrB(pub u32);
impl Group13AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group13AmrB {
    #[inline(always)]
    fn default() -> Group13AmrB {
        Group13AmrB(0)
    }
}
impl core::fmt::Debug for Group13AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group13AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group13AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group13AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Acr0P(pub u32);
impl Group14Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Acr0P {
    #[inline(always)]
    fn default() -> Group14Acr0P {
        Group14Acr0P(0)
    }
}
impl core::fmt::Debug for Group14Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Acr1P(pub u32);
impl Group14Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Acr1P {
    #[inline(always)]
    fn default() -> Group14Acr1P {
        Group14Acr1P(0)
    }
}
impl core::fmt::Debug for Group14Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Acr2P(pub u32);
impl Group14Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Acr2P {
    #[inline(always)]
    fn default() -> Group14Acr2P {
        Group14Acr2P(0)
    }
}
impl core::fmt::Debug for Group14Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Acr3P(pub u32);
impl Group14Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Acr3P {
    #[inline(always)]
    fn default() -> Group14Acr3P {
        Group14Acr3P(0)
    }
}
impl core::fmt::Debug for Group14Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14AcrB(pub u32);
impl Group14AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14AcrB {
    #[inline(always)]
    fn default() -> Group14AcrB {
        Group14AcrB(0)
    }
}
impl core::fmt::Debug for Group14AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Amr0P(pub u32);
impl Group14Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Amr0P {
    #[inline(always)]
    fn default() -> Group14Amr0P {
        Group14Amr0P(0)
    }
}
impl core::fmt::Debug for Group14Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Amr1P(pub u32);
impl Group14Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Amr1P {
    #[inline(always)]
    fn default() -> Group14Amr1P {
        Group14Amr1P(0)
    }
}
impl core::fmt::Debug for Group14Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Amr2P(pub u32);
impl Group14Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Amr2P {
    #[inline(always)]
    fn default() -> Group14Amr2P {
        Group14Amr2P(0)
    }
}
impl core::fmt::Debug for Group14Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14Amr3P(pub u32);
impl Group14Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14Amr3P {
    #[inline(always)]
    fn default() -> Group14Amr3P {
        Group14Amr3P(0)
    }
}
impl core::fmt::Debug for Group14Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group14AmrB(pub u32);
impl Group14AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group14AmrB {
    #[inline(always)]
    fn default() -> Group14AmrB {
        Group14AmrB(0)
    }
}
impl core::fmt::Debug for Group14AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group14AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group14AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group14AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Acr0P(pub u32);
impl Group15Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Acr0P {
    #[inline(always)]
    fn default() -> Group15Acr0P {
        Group15Acr0P(0)
    }
}
impl core::fmt::Debug for Group15Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Acr1P(pub u32);
impl Group15Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Acr1P {
    #[inline(always)]
    fn default() -> Group15Acr1P {
        Group15Acr1P(0)
    }
}
impl core::fmt::Debug for Group15Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Acr2P(pub u32);
impl Group15Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Acr2P {
    #[inline(always)]
    fn default() -> Group15Acr2P {
        Group15Acr2P(0)
    }
}
impl core::fmt::Debug for Group15Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Acr3P(pub u32);
impl Group15Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Acr3P {
    #[inline(always)]
    fn default() -> Group15Acr3P {
        Group15Acr3P(0)
    }
}
impl core::fmt::Debug for Group15Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15AcrB(pub u32);
impl Group15AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15AcrB {
    #[inline(always)]
    fn default() -> Group15AcrB {
        Group15AcrB(0)
    }
}
impl core::fmt::Debug for Group15AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Amr0P(pub u32);
impl Group15Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Amr0P {
    #[inline(always)]
    fn default() -> Group15Amr0P {
        Group15Amr0P(0)
    }
}
impl core::fmt::Debug for Group15Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Amr1P(pub u32);
impl Group15Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Amr1P {
    #[inline(always)]
    fn default() -> Group15Amr1P {
        Group15Amr1P(0)
    }
}
impl core::fmt::Debug for Group15Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Amr2P(pub u32);
impl Group15Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Amr2P {
    #[inline(always)]
    fn default() -> Group15Amr2P {
        Group15Amr2P(0)
    }
}
impl core::fmt::Debug for Group15Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15Amr3P(pub u32);
impl Group15Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15Amr3P {
    #[inline(always)]
    fn default() -> Group15Amr3P {
        Group15Amr3P(0)
    }
}
impl core::fmt::Debug for Group15Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group15AmrB(pub u32);
impl Group15AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group15AmrB {
    #[inline(always)]
    fn default() -> Group15AmrB {
        Group15AmrB(0)
    }
}
impl core::fmt::Debug for Group15AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group15AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group15AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group15AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Acr0P(pub u32);
impl Group16Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Acr0P {
    #[inline(always)]
    fn default() -> Group16Acr0P {
        Group16Acr0P(0)
    }
}
impl core::fmt::Debug for Group16Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Acr1P(pub u32);
impl Group16Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Acr1P {
    #[inline(always)]
    fn default() -> Group16Acr1P {
        Group16Acr1P(0)
    }
}
impl core::fmt::Debug for Group16Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Acr2P(pub u32);
impl Group16Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Acr2P {
    #[inline(always)]
    fn default() -> Group16Acr2P {
        Group16Acr2P(0)
    }
}
impl core::fmt::Debug for Group16Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Acr3P(pub u32);
impl Group16Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Acr3P {
    #[inline(always)]
    fn default() -> Group16Acr3P {
        Group16Acr3P(0)
    }
}
impl core::fmt::Debug for Group16Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16AcrB(pub u32);
impl Group16AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16AcrB {
    #[inline(always)]
    fn default() -> Group16AcrB {
        Group16AcrB(0)
    }
}
impl core::fmt::Debug for Group16AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Amr0P(pub u32);
impl Group16Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Amr0P {
    #[inline(always)]
    fn default() -> Group16Amr0P {
        Group16Amr0P(0)
    }
}
impl core::fmt::Debug for Group16Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Amr1P(pub u32);
impl Group16Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Amr1P {
    #[inline(always)]
    fn default() -> Group16Amr1P {
        Group16Amr1P(0)
    }
}
impl core::fmt::Debug for Group16Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Amr2P(pub u32);
impl Group16Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Amr2P {
    #[inline(always)]
    fn default() -> Group16Amr2P {
        Group16Amr2P(0)
    }
}
impl core::fmt::Debug for Group16Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16Amr3P(pub u32);
impl Group16Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16Amr3P {
    #[inline(always)]
    fn default() -> Group16Amr3P {
        Group16Amr3P(0)
    }
}
impl core::fmt::Debug for Group16Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group16AmrB(pub u32);
impl Group16AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group16AmrB {
    #[inline(always)]
    fn default() -> Group16AmrB {
        Group16AmrB(0)
    }
}
impl core::fmt::Debug for Group16AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group16AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group16AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group16AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Acr0P(pub u32);
impl Group17Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Acr0P {
    #[inline(always)]
    fn default() -> Group17Acr0P {
        Group17Acr0P(0)
    }
}
impl core::fmt::Debug for Group17Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Acr1P(pub u32);
impl Group17Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Acr1P {
    #[inline(always)]
    fn default() -> Group17Acr1P {
        Group17Acr1P(0)
    }
}
impl core::fmt::Debug for Group17Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Acr2P(pub u32);
impl Group17Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Acr2P {
    #[inline(always)]
    fn default() -> Group17Acr2P {
        Group17Acr2P(0)
    }
}
impl core::fmt::Debug for Group17Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Acr3P(pub u32);
impl Group17Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Acr3P {
    #[inline(always)]
    fn default() -> Group17Acr3P {
        Group17Acr3P(0)
    }
}
impl core::fmt::Debug for Group17Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17AcrB(pub u32);
impl Group17AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17AcrB {
    #[inline(always)]
    fn default() -> Group17AcrB {
        Group17AcrB(0)
    }
}
impl core::fmt::Debug for Group17AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Amr0P(pub u32);
impl Group17Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Amr0P {
    #[inline(always)]
    fn default() -> Group17Amr0P {
        Group17Amr0P(0)
    }
}
impl core::fmt::Debug for Group17Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Amr1P(pub u32);
impl Group17Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Amr1P {
    #[inline(always)]
    fn default() -> Group17Amr1P {
        Group17Amr1P(0)
    }
}
impl core::fmt::Debug for Group17Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Amr2P(pub u32);
impl Group17Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Amr2P {
    #[inline(always)]
    fn default() -> Group17Amr2P {
        Group17Amr2P(0)
    }
}
impl core::fmt::Debug for Group17Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17Amr3P(pub u32);
impl Group17Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17Amr3P {
    #[inline(always)]
    fn default() -> Group17Amr3P {
        Group17Amr3P(0)
    }
}
impl core::fmt::Debug for Group17Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group17AmrB(pub u32);
impl Group17AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group17AmrB {
    #[inline(always)]
    fn default() -> Group17AmrB {
        Group17AmrB(0)
    }
}
impl core::fmt::Debug for Group17AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group17AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group17AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group17AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Acr0P(pub u32);
impl Group18Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Acr0P {
    #[inline(always)]
    fn default() -> Group18Acr0P {
        Group18Acr0P(0)
    }
}
impl core::fmt::Debug for Group18Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Acr1P(pub u32);
impl Group18Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Acr1P {
    #[inline(always)]
    fn default() -> Group18Acr1P {
        Group18Acr1P(0)
    }
}
impl core::fmt::Debug for Group18Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Acr2P(pub u32);
impl Group18Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Acr2P {
    #[inline(always)]
    fn default() -> Group18Acr2P {
        Group18Acr2P(0)
    }
}
impl core::fmt::Debug for Group18Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Acr3P(pub u32);
impl Group18Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Acr3P {
    #[inline(always)]
    fn default() -> Group18Acr3P {
        Group18Acr3P(0)
    }
}
impl core::fmt::Debug for Group18Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18AcrB(pub u32);
impl Group18AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18AcrB {
    #[inline(always)]
    fn default() -> Group18AcrB {
        Group18AcrB(0)
    }
}
impl core::fmt::Debug for Group18AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Amr0P(pub u32);
impl Group18Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Amr0P {
    #[inline(always)]
    fn default() -> Group18Amr0P {
        Group18Amr0P(0)
    }
}
impl core::fmt::Debug for Group18Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Amr1P(pub u32);
impl Group18Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Amr1P {
    #[inline(always)]
    fn default() -> Group18Amr1P {
        Group18Amr1P(0)
    }
}
impl core::fmt::Debug for Group18Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Amr2P(pub u32);
impl Group18Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Amr2P {
    #[inline(always)]
    fn default() -> Group18Amr2P {
        Group18Amr2P(0)
    }
}
impl core::fmt::Debug for Group18Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18Amr3P(pub u32);
impl Group18Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18Amr3P {
    #[inline(always)]
    fn default() -> Group18Amr3P {
        Group18Amr3P(0)
    }
}
impl core::fmt::Debug for Group18Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group18AmrB(pub u32);
impl Group18AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group18AmrB {
    #[inline(always)]
    fn default() -> Group18AmrB {
        Group18AmrB(0)
    }
}
impl core::fmt::Debug for Group18AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group18AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group18AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group18AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Acr0P(pub u32);
impl Group19Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Acr0P {
    #[inline(always)]
    fn default() -> Group19Acr0P {
        Group19Acr0P(0)
    }
}
impl core::fmt::Debug for Group19Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Acr1P(pub u32);
impl Group19Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Acr1P {
    #[inline(always)]
    fn default() -> Group19Acr1P {
        Group19Acr1P(0)
    }
}
impl core::fmt::Debug for Group19Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Acr2P(pub u32);
impl Group19Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Acr2P {
    #[inline(always)]
    fn default() -> Group19Acr2P {
        Group19Acr2P(0)
    }
}
impl core::fmt::Debug for Group19Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Acr3P(pub u32);
impl Group19Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Acr3P {
    #[inline(always)]
    fn default() -> Group19Acr3P {
        Group19Acr3P(0)
    }
}
impl core::fmt::Debug for Group19Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19AcrB(pub u32);
impl Group19AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19AcrB {
    #[inline(always)]
    fn default() -> Group19AcrB {
        Group19AcrB(0)
    }
}
impl core::fmt::Debug for Group19AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Amr0P(pub u32);
impl Group19Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Amr0P {
    #[inline(always)]
    fn default() -> Group19Amr0P {
        Group19Amr0P(0)
    }
}
impl core::fmt::Debug for Group19Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Amr1P(pub u32);
impl Group19Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Amr1P {
    #[inline(always)]
    fn default() -> Group19Amr1P {
        Group19Amr1P(0)
    }
}
impl core::fmt::Debug for Group19Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Amr2P(pub u32);
impl Group19Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Amr2P {
    #[inline(always)]
    fn default() -> Group19Amr2P {
        Group19Amr2P(0)
    }
}
impl core::fmt::Debug for Group19Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19Amr3P(pub u32);
impl Group19Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19Amr3P {
    #[inline(always)]
    fn default() -> Group19Amr3P {
        Group19Amr3P(0)
    }
}
impl core::fmt::Debug for Group19Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group19AmrB(pub u32);
impl Group19AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group19AmrB {
    #[inline(always)]
    fn default() -> Group19AmrB {
        Group19AmrB(0)
    }
}
impl core::fmt::Debug for Group19AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group19AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group19AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group19AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Acr0P(pub u32);
impl Group1Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Acr0P {
    #[inline(always)]
    fn default() -> Group1Acr0P {
        Group1Acr0P(0)
    }
}
impl core::fmt::Debug for Group1Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Acr1P(pub u32);
impl Group1Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Acr1P {
    #[inline(always)]
    fn default() -> Group1Acr1P {
        Group1Acr1P(0)
    }
}
impl core::fmt::Debug for Group1Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Acr2P(pub u32);
impl Group1Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Acr2P {
    #[inline(always)]
    fn default() -> Group1Acr2P {
        Group1Acr2P(0)
    }
}
impl core::fmt::Debug for Group1Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Acr3P(pub u32);
impl Group1Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Acr3P {
    #[inline(always)]
    fn default() -> Group1Acr3P {
        Group1Acr3P(0)
    }
}
impl core::fmt::Debug for Group1Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1AcrB(pub u32);
impl Group1AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1AcrB {
    #[inline(always)]
    fn default() -> Group1AcrB {
        Group1AcrB(0)
    }
}
impl core::fmt::Debug for Group1AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Amr0P(pub u32);
impl Group1Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Amr0P {
    #[inline(always)]
    fn default() -> Group1Amr0P {
        Group1Amr0P(0)
    }
}
impl core::fmt::Debug for Group1Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Amr1P(pub u32);
impl Group1Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Amr1P {
    #[inline(always)]
    fn default() -> Group1Amr1P {
        Group1Amr1P(0)
    }
}
impl core::fmt::Debug for Group1Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Amr2P(pub u32);
impl Group1Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Amr2P {
    #[inline(always)]
    fn default() -> Group1Amr2P {
        Group1Amr2P(0)
    }
}
impl core::fmt::Debug for Group1Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1Amr3P(pub u32);
impl Group1Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1Amr3P {
    #[inline(always)]
    fn default() -> Group1Amr3P {
        Group1Amr3P(0)
    }
}
impl core::fmt::Debug for Group1Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group1AmrB(pub u32);
impl Group1AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group1AmrB {
    #[inline(always)]
    fn default() -> Group1AmrB {
        Group1AmrB(0)
    }
}
impl core::fmt::Debug for Group1AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group1AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group1AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group1AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Acr0P(pub u32);
impl Group2Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Acr0P {
    #[inline(always)]
    fn default() -> Group2Acr0P {
        Group2Acr0P(0)
    }
}
impl core::fmt::Debug for Group2Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Acr1P(pub u32);
impl Group2Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Acr1P {
    #[inline(always)]
    fn default() -> Group2Acr1P {
        Group2Acr1P(0)
    }
}
impl core::fmt::Debug for Group2Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Acr2P(pub u32);
impl Group2Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Acr2P {
    #[inline(always)]
    fn default() -> Group2Acr2P {
        Group2Acr2P(0)
    }
}
impl core::fmt::Debug for Group2Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Acr3P(pub u32);
impl Group2Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Acr3P {
    #[inline(always)]
    fn default() -> Group2Acr3P {
        Group2Acr3P(0)
    }
}
impl core::fmt::Debug for Group2Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2AcrB(pub u32);
impl Group2AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2AcrB {
    #[inline(always)]
    fn default() -> Group2AcrB {
        Group2AcrB(0)
    }
}
impl core::fmt::Debug for Group2AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Amr0P(pub u32);
impl Group2Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Amr0P {
    #[inline(always)]
    fn default() -> Group2Amr0P {
        Group2Amr0P(0)
    }
}
impl core::fmt::Debug for Group2Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Amr1P(pub u32);
impl Group2Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Amr1P {
    #[inline(always)]
    fn default() -> Group2Amr1P {
        Group2Amr1P(0)
    }
}
impl core::fmt::Debug for Group2Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Amr2P(pub u32);
impl Group2Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Amr2P {
    #[inline(always)]
    fn default() -> Group2Amr2P {
        Group2Amr2P(0)
    }
}
impl core::fmt::Debug for Group2Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2Amr3P(pub u32);
impl Group2Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2Amr3P {
    #[inline(always)]
    fn default() -> Group2Amr3P {
        Group2Amr3P(0)
    }
}
impl core::fmt::Debug for Group2Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group2AmrB(pub u32);
impl Group2AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group2AmrB {
    #[inline(always)]
    fn default() -> Group2AmrB {
        Group2AmrB(0)
    }
}
impl core::fmt::Debug for Group2AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group2AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group2AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group2AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Acr0P(pub u32);
impl Group3Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Acr0P {
    #[inline(always)]
    fn default() -> Group3Acr0P {
        Group3Acr0P(0)
    }
}
impl core::fmt::Debug for Group3Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Acr1P(pub u32);
impl Group3Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Acr1P {
    #[inline(always)]
    fn default() -> Group3Acr1P {
        Group3Acr1P(0)
    }
}
impl core::fmt::Debug for Group3Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Acr2P(pub u32);
impl Group3Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Acr2P {
    #[inline(always)]
    fn default() -> Group3Acr2P {
        Group3Acr2P(0)
    }
}
impl core::fmt::Debug for Group3Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Acr3P(pub u32);
impl Group3Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Acr3P {
    #[inline(always)]
    fn default() -> Group3Acr3P {
        Group3Acr3P(0)
    }
}
impl core::fmt::Debug for Group3Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3AcrB(pub u32);
impl Group3AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3AcrB {
    #[inline(always)]
    fn default() -> Group3AcrB {
        Group3AcrB(0)
    }
}
impl core::fmt::Debug for Group3AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Amr0P(pub u32);
impl Group3Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Amr0P {
    #[inline(always)]
    fn default() -> Group3Amr0P {
        Group3Amr0P(0)
    }
}
impl core::fmt::Debug for Group3Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Amr1P(pub u32);
impl Group3Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Amr1P {
    #[inline(always)]
    fn default() -> Group3Amr1P {
        Group3Amr1P(0)
    }
}
impl core::fmt::Debug for Group3Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Amr2P(pub u32);
impl Group3Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Amr2P {
    #[inline(always)]
    fn default() -> Group3Amr2P {
        Group3Amr2P(0)
    }
}
impl core::fmt::Debug for Group3Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3Amr3P(pub u32);
impl Group3Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3Amr3P {
    #[inline(always)]
    fn default() -> Group3Amr3P {
        Group3Amr3P(0)
    }
}
impl core::fmt::Debug for Group3Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group3AmrB(pub u32);
impl Group3AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group3AmrB {
    #[inline(always)]
    fn default() -> Group3AmrB {
        Group3AmrB(0)
    }
}
impl core::fmt::Debug for Group3AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group3AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group3AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group3AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Acr0P(pub u32);
impl Group4Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Acr0P {
    #[inline(always)]
    fn default() -> Group4Acr0P {
        Group4Acr0P(0)
    }
}
impl core::fmt::Debug for Group4Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Acr1P(pub u32);
impl Group4Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Acr1P {
    #[inline(always)]
    fn default() -> Group4Acr1P {
        Group4Acr1P(0)
    }
}
impl core::fmt::Debug for Group4Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Acr2P(pub u32);
impl Group4Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Acr2P {
    #[inline(always)]
    fn default() -> Group4Acr2P {
        Group4Acr2P(0)
    }
}
impl core::fmt::Debug for Group4Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Acr3P(pub u32);
impl Group4Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Acr3P {
    #[inline(always)]
    fn default() -> Group4Acr3P {
        Group4Acr3P(0)
    }
}
impl core::fmt::Debug for Group4Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4AcrB(pub u32);
impl Group4AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4AcrB {
    #[inline(always)]
    fn default() -> Group4AcrB {
        Group4AcrB(0)
    }
}
impl core::fmt::Debug for Group4AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Amr0P(pub u32);
impl Group4Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Amr0P {
    #[inline(always)]
    fn default() -> Group4Amr0P {
        Group4Amr0P(0)
    }
}
impl core::fmt::Debug for Group4Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Amr1P(pub u32);
impl Group4Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Amr1P {
    #[inline(always)]
    fn default() -> Group4Amr1P {
        Group4Amr1P(0)
    }
}
impl core::fmt::Debug for Group4Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Amr2P(pub u32);
impl Group4Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Amr2P {
    #[inline(always)]
    fn default() -> Group4Amr2P {
        Group4Amr2P(0)
    }
}
impl core::fmt::Debug for Group4Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4Amr3P(pub u32);
impl Group4Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4Amr3P {
    #[inline(always)]
    fn default() -> Group4Amr3P {
        Group4Amr3P(0)
    }
}
impl core::fmt::Debug for Group4Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group4AmrB(pub u32);
impl Group4AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group4AmrB {
    #[inline(always)]
    fn default() -> Group4AmrB {
        Group4AmrB(0)
    }
}
impl core::fmt::Debug for Group4AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group4AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group4AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group4AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Acr0P(pub u32);
impl Group5Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Acr0P {
    #[inline(always)]
    fn default() -> Group5Acr0P {
        Group5Acr0P(0)
    }
}
impl core::fmt::Debug for Group5Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Acr1P(pub u32);
impl Group5Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Acr1P {
    #[inline(always)]
    fn default() -> Group5Acr1P {
        Group5Acr1P(0)
    }
}
impl core::fmt::Debug for Group5Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Acr2P(pub u32);
impl Group5Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Acr2P {
    #[inline(always)]
    fn default() -> Group5Acr2P {
        Group5Acr2P(0)
    }
}
impl core::fmt::Debug for Group5Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Acr3P(pub u32);
impl Group5Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Acr3P {
    #[inline(always)]
    fn default() -> Group5Acr3P {
        Group5Acr3P(0)
    }
}
impl core::fmt::Debug for Group5Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5AcrB(pub u32);
impl Group5AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5AcrB {
    #[inline(always)]
    fn default() -> Group5AcrB {
        Group5AcrB(0)
    }
}
impl core::fmt::Debug for Group5AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Amr0P(pub u32);
impl Group5Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Amr0P {
    #[inline(always)]
    fn default() -> Group5Amr0P {
        Group5Amr0P(0)
    }
}
impl core::fmt::Debug for Group5Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Amr1P(pub u32);
impl Group5Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Amr1P {
    #[inline(always)]
    fn default() -> Group5Amr1P {
        Group5Amr1P(0)
    }
}
impl core::fmt::Debug for Group5Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Amr2P(pub u32);
impl Group5Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Amr2P {
    #[inline(always)]
    fn default() -> Group5Amr2P {
        Group5Amr2P(0)
    }
}
impl core::fmt::Debug for Group5Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5Amr3P(pub u32);
impl Group5Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5Amr3P {
    #[inline(always)]
    fn default() -> Group5Amr3P {
        Group5Amr3P(0)
    }
}
impl core::fmt::Debug for Group5Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group5AmrB(pub u32);
impl Group5AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group5AmrB {
    #[inline(always)]
    fn default() -> Group5AmrB {
        Group5AmrB(0)
    }
}
impl core::fmt::Debug for Group5AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group5AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group5AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group5AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Acr0P(pub u32);
impl Group6Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Acr0P {
    #[inline(always)]
    fn default() -> Group6Acr0P {
        Group6Acr0P(0)
    }
}
impl core::fmt::Debug for Group6Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Acr1P(pub u32);
impl Group6Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Acr1P {
    #[inline(always)]
    fn default() -> Group6Acr1P {
        Group6Acr1P(0)
    }
}
impl core::fmt::Debug for Group6Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Acr2P(pub u32);
impl Group6Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Acr2P {
    #[inline(always)]
    fn default() -> Group6Acr2P {
        Group6Acr2P(0)
    }
}
impl core::fmt::Debug for Group6Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Acr3P(pub u32);
impl Group6Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Acr3P {
    #[inline(always)]
    fn default() -> Group6Acr3P {
        Group6Acr3P(0)
    }
}
impl core::fmt::Debug for Group6Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6AcrB(pub u32);
impl Group6AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6AcrB {
    #[inline(always)]
    fn default() -> Group6AcrB {
        Group6AcrB(0)
    }
}
impl core::fmt::Debug for Group6AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Amr0P(pub u32);
impl Group6Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Amr0P {
    #[inline(always)]
    fn default() -> Group6Amr0P {
        Group6Amr0P(0)
    }
}
impl core::fmt::Debug for Group6Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Amr1P(pub u32);
impl Group6Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Amr1P {
    #[inline(always)]
    fn default() -> Group6Amr1P {
        Group6Amr1P(0)
    }
}
impl core::fmt::Debug for Group6Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Amr2P(pub u32);
impl Group6Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Amr2P {
    #[inline(always)]
    fn default() -> Group6Amr2P {
        Group6Amr2P(0)
    }
}
impl core::fmt::Debug for Group6Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6Amr3P(pub u32);
impl Group6Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6Amr3P {
    #[inline(always)]
    fn default() -> Group6Amr3P {
        Group6Amr3P(0)
    }
}
impl core::fmt::Debug for Group6Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group6AmrB(pub u32);
impl Group6AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group6AmrB {
    #[inline(always)]
    fn default() -> Group6AmrB {
        Group6AmrB(0)
    }
}
impl core::fmt::Debug for Group6AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group6AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group6AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group6AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Acr0P(pub u32);
impl Group7Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Acr0P {
    #[inline(always)]
    fn default() -> Group7Acr0P {
        Group7Acr0P(0)
    }
}
impl core::fmt::Debug for Group7Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Acr1P(pub u32);
impl Group7Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Acr1P {
    #[inline(always)]
    fn default() -> Group7Acr1P {
        Group7Acr1P(0)
    }
}
impl core::fmt::Debug for Group7Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Acr2P(pub u32);
impl Group7Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Acr2P {
    #[inline(always)]
    fn default() -> Group7Acr2P {
        Group7Acr2P(0)
    }
}
impl core::fmt::Debug for Group7Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Acr3P(pub u32);
impl Group7Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Acr3P {
    #[inline(always)]
    fn default() -> Group7Acr3P {
        Group7Acr3P(0)
    }
}
impl core::fmt::Debug for Group7Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7AcrB(pub u32);
impl Group7AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7AcrB {
    #[inline(always)]
    fn default() -> Group7AcrB {
        Group7AcrB(0)
    }
}
impl core::fmt::Debug for Group7AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Amr0P(pub u32);
impl Group7Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Amr0P {
    #[inline(always)]
    fn default() -> Group7Amr0P {
        Group7Amr0P(0)
    }
}
impl core::fmt::Debug for Group7Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Amr1P(pub u32);
impl Group7Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Amr1P {
    #[inline(always)]
    fn default() -> Group7Amr1P {
        Group7Amr1P(0)
    }
}
impl core::fmt::Debug for Group7Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Amr2P(pub u32);
impl Group7Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Amr2P {
    #[inline(always)]
    fn default() -> Group7Amr2P {
        Group7Amr2P(0)
    }
}
impl core::fmt::Debug for Group7Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7Amr3P(pub u32);
impl Group7Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7Amr3P {
    #[inline(always)]
    fn default() -> Group7Amr3P {
        Group7Amr3P(0)
    }
}
impl core::fmt::Debug for Group7Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group7AmrB(pub u32);
impl Group7AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group7AmrB {
    #[inline(always)]
    fn default() -> Group7AmrB {
        Group7AmrB(0)
    }
}
impl core::fmt::Debug for Group7AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group7AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group7AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group7AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Acr0P(pub u32);
impl Group8Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Acr0P {
    #[inline(always)]
    fn default() -> Group8Acr0P {
        Group8Acr0P(0)
    }
}
impl core::fmt::Debug for Group8Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Acr1P(pub u32);
impl Group8Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Acr1P {
    #[inline(always)]
    fn default() -> Group8Acr1P {
        Group8Acr1P(0)
    }
}
impl core::fmt::Debug for Group8Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Acr2P(pub u32);
impl Group8Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Acr2P {
    #[inline(always)]
    fn default() -> Group8Acr2P {
        Group8Acr2P(0)
    }
}
impl core::fmt::Debug for Group8Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Acr3P(pub u32);
impl Group8Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Acr3P {
    #[inline(always)]
    fn default() -> Group8Acr3P {
        Group8Acr3P(0)
    }
}
impl core::fmt::Debug for Group8Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8AcrB(pub u32);
impl Group8AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8AcrB {
    #[inline(always)]
    fn default() -> Group8AcrB {
        Group8AcrB(0)
    }
}
impl core::fmt::Debug for Group8AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Amr0P(pub u32);
impl Group8Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Amr0P {
    #[inline(always)]
    fn default() -> Group8Amr0P {
        Group8Amr0P(0)
    }
}
impl core::fmt::Debug for Group8Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Amr1P(pub u32);
impl Group8Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Amr1P {
    #[inline(always)]
    fn default() -> Group8Amr1P {
        Group8Amr1P(0)
    }
}
impl core::fmt::Debug for Group8Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Amr2P(pub u32);
impl Group8Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Amr2P {
    #[inline(always)]
    fn default() -> Group8Amr2P {
        Group8Amr2P(0)
    }
}
impl core::fmt::Debug for Group8Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8Amr3P(pub u32);
impl Group8Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8Amr3P {
    #[inline(always)]
    fn default() -> Group8Amr3P {
        Group8Amr3P(0)
    }
}
impl core::fmt::Debug for Group8Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group8AmrB(pub u32);
impl Group8AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group8AmrB {
    #[inline(always)]
    fn default() -> Group8AmrB {
        Group8AmrB(0)
    }
}
impl core::fmt::Debug for Group8AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group8AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group8AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group8AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Code register0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Acr0P(pub u32);
impl Group9Acr0P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Acr0P {
    #[inline(always)]
    fn default() -> Group9Acr0P {
        Group9Acr0P(0)
    }
}
impl core::fmt::Debug for Group9Acr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Acr0P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Acr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Acr0P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Acr1P(pub u32);
impl Group9Acr1P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Acr1P {
    #[inline(always)]
    fn default() -> Group9Acr1P {
        Group9Acr1P(0)
    }
}
impl core::fmt::Debug for Group9Acr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Acr1P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Acr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Acr1P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Acr2P(pub u32);
impl Group9Acr2P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Acr2P {
    #[inline(always)]
    fn default() -> Group9Acr2P {
        Group9Acr2P(0)
    }
}
impl core::fmt::Debug for Group9Acr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Acr2P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Acr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Acr2P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Code register3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Acr3P(pub u32);
impl Group9Acr3P {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Acr3P {
    #[inline(always)]
    fn default() -> Group9Acr3P {
        Group9Acr3P(0)
    }
}
impl core::fmt::Debug for Group9Acr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Acr3P")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Acr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Acr3P {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Basic Acceptance Code register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9AcrB(pub u32);
impl Group9AcrB {
    #[doc = "Acceptance code"]
    #[must_use]
    #[inline(always)]
    pub const fn ac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance code"]
    #[inline(always)]
    pub const fn set_ac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9AcrB {
    #[inline(always)]
    fn default() -> Group9AcrB {
        Group9AcrB(0)
    }
}
impl core::fmt::Debug for Group9AcrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9AcrB")
            .field("ac", &self.ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9AcrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9AcrB {{ ac: {=u8:?} }}", self.ac())
    }
}
#[doc = "Peli Acceptance Mask register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Amr0P(pub u32);
impl Group9Amr0P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Amr0P {
    #[inline(always)]
    fn default() -> Group9Amr0P {
        Group9Amr0P(0)
    }
}
impl core::fmt::Debug for Group9Amr0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Amr0P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Amr0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Amr0P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Amr1P(pub u32);
impl Group9Amr1P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Amr1P {
    #[inline(always)]
    fn default() -> Group9Amr1P {
        Group9Amr1P(0)
    }
}
impl core::fmt::Debug for Group9Amr1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Amr1P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Amr1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Amr1P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Amr2P(pub u32);
impl Group9Amr2P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Amr2P {
    #[inline(always)]
    fn default() -> Group9Amr2P {
        Group9Amr2P(0)
    }
}
impl core::fmt::Debug for Group9Amr2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Amr2P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Amr2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Amr2P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Acceptance Mask register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9Amr3P(pub u32);
impl Group9Amr3P {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9Amr3P {
    #[inline(always)]
    fn default() -> Group9Amr3P {
        Group9Amr3P(0)
    }
}
impl core::fmt::Debug for Group9Amr3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9Amr3P")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9Amr3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9Amr3P {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Basic Acceptance Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group9AmrB(pub u32);
impl Group9AmrB {
    #[doc = "Acceptance mask"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Acceptance mask"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Group9AmrB {
    #[inline(always)]
    fn default() -> Group9AmrB {
        Group9AmrB(0)
    }
}
impl core::fmt::Debug for Group9AmrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Group9AmrB")
            .field("am", &self.am())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Group9AmrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Group9AmrB {{ am: {=u8:?} }}", self.am())
    }
}
#[doc = "Peli Interrupt Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IerP(pub u32);
impl IerP {
    #[doc = "Receive interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt enable"]
    #[inline(always)]
    pub const fn set_eie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data overrun interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn doie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data overrun interrupt enable"]
    #[inline(always)]
    pub const fn set_doie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Error passive interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn epie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt enable"]
    #[inline(always)]
    pub const fn set_epie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arbitration lost interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost interrupt enable"]
    #[inline(always)]
    pub const fn set_alie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn beie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error interrupt enable"]
    #[inline(always)]
    pub const fn set_beie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IerP {
    #[inline(always)]
    fn default() -> IerP {
        IerP(0)
    }
}
impl core::fmt::Debug for IerP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IerP")
            .field("rie", &self.rie())
            .field("tie", &self.tie())
            .field("eie", &self.eie())
            .field("doie", &self.doie())
            .field("epie", &self.epie())
            .field("alie", &self.alie())
            .field("beie", &self.beie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IerP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IerP {{ rie: {=bool:?}, tie: {=bool:?}, eie: {=bool:?}, doie: {=bool:?}, epie: {=bool:?}, alie: {=bool:?}, beie: {=bool:?} }}",
            self.rie(),
            self.tie(),
            self.eie(),
            self.doie(),
            self.epie(),
            self.alie(),
            self.beie()
        )
    }
}
#[doc = "Interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrB(pub u32);
impl IrB {
    #[doc = "Receive interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt"]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt"]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ei(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt"]
    #[inline(always)]
    pub const fn set_ei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data overrun interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn doi(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data overrun interrupt"]
    #[inline(always)]
    pub const fn set_doi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IrB {
    #[inline(always)]
    fn default() -> IrB {
        IrB(0)
    }
}
impl core::fmt::Debug for IrB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrB")
            .field("ri", &self.ri())
            .field("ti", &self.ti())
            .field("ei", &self.ei())
            .field("doi", &self.doi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IrB {{ ri: {=bool:?}, ti: {=bool:?}, ei: {=bool:?}, doi: {=bool:?} }}",
            self.ri(),
            self.ti(),
            self.ei(),
            self.doi()
        )
    }
}
#[doc = "Interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrP(pub u32);
impl IrP {
    #[doc = "Receive interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt"]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt"]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ei(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt"]
    #[inline(always)]
    pub const fn set_ei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data overrun interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn doi(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data overrun interrupt"]
    #[inline(always)]
    pub const fn set_doi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Error passive interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn epi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt"]
    #[inline(always)]
    pub const fn set_epi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Arbitration lost interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ali(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration lost interrupt"]
    #[inline(always)]
    pub const fn set_ali(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn bei(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus error interrupt"]
    #[inline(always)]
    pub const fn set_bei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IrP {
    #[inline(always)]
    fn default() -> IrP {
        IrP(0)
    }
}
impl core::fmt::Debug for IrP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrP")
            .field("ri", &self.ri())
            .field("ti", &self.ti())
            .field("ei", &self.ei())
            .field("doi", &self.doi())
            .field("epi", &self.epi())
            .field("ali", &self.ali())
            .field("bei", &self.bei())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IrP {{ ri: {=bool:?}, ti: {=bool:?}, ei: {=bool:?}, doi: {=bool:?}, epi: {=bool:?}, ali: {=bool:?}, bei: {=bool:?} }}",
            self.ri(),
            self.ti(),
            self.ei(),
            self.doi(),
            self.epi(),
            self.ali(),
            self.bei()
        )
    }
}
#[doc = "Peli Mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModP(pub u32);
impl ModP {
    #[doc = "Reset mode"]
    #[must_use]
    #[inline(always)]
    pub const fn rm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reset mode"]
    #[inline(always)]
    pub const fn set_rm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Listen only mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Listen only mode"]
    #[inline(always)]
    pub const fn set_lom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Self test mode"]
    #[must_use]
    #[inline(always)]
    pub const fn stm(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Self test mode"]
    #[inline(always)]
    pub const fn set_stm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Acceptance filter mode"]
    #[must_use]
    #[inline(always)]
    pub const fn afm(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Acceptance filter mode"]
    #[inline(always)]
    pub const fn set_afm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ModP {
    #[inline(always)]
    fn default() -> ModP {
        ModP(0)
    }
}
impl core::fmt::Debug for ModP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ModP")
            .field("rm", &self.rm())
            .field("lom", &self.lom())
            .field("stm", &self.stm())
            .field("afm", &self.afm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ModP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ModP {{ rm: {=bool:?}, lom: {=bool:?}, stm: {=bool:?}, afm: {=bool:?} }}",
            self.rm(),
            self.lom(),
            self.stm(),
            self.afm()
        )
    }
}
#[doc = "Peli RX Error Counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxerrP(pub u32);
impl RxerrP {
    #[doc = "RX error counter register"]
    #[must_use]
    #[inline(always)]
    pub const fn rxerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX error counter register"]
    #[inline(always)]
    pub const fn set_rxerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RxerrP {
    #[inline(always)]
    fn default() -> RxerrP {
        RxerrP(0)
    }
}
impl core::fmt::Debug for RxerrP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxerrP")
            .field("rxerr", &self.rxerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxerrP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxerrP {{ rxerr: {=u8:?} }}", self.rxerr())
    }
}
#[doc = "Peli Send Frame Format register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SffP(pub u32);
impl SffP {
    #[doc = "Data length code bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data length code bit"]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Remote transmission request"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Remote transmission request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Frame format"]
    #[must_use]
    #[inline(always)]
    pub const fn ff(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn set_ff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SffP {
    #[inline(always)]
    fn default() -> SffP {
        SffP(0)
    }
}
impl core::fmt::Debug for SffP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SffP")
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ff", &self.ff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SffP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SffP {{ dlc: {=u8:?}, rtr: {=bool:?}, ff: {=bool:?} }}",
            self.dlc(),
            self.rtr(),
            self.ff()
        )
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Receive buffer status"]
    #[must_use]
    #[inline(always)]
    pub const fn rbs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive buffer status"]
    #[inline(always)]
    pub const fn set_rbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data overrun status"]
    #[must_use]
    #[inline(always)]
    pub const fn dos(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data overrun status"]
    #[inline(always)]
    pub const fn set_dos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit buffer status"]
    #[must_use]
    #[inline(always)]
    pub const fn tbs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer status"]
    #[inline(always)]
    pub const fn set_tbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmission complete status"]
    #[must_use]
    #[inline(always)]
    pub const fn tcs(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission complete status"]
    #[inline(always)]
    pub const fn set_tcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive status"]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive status"]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit status"]
    #[must_use]
    #[inline(always)]
    pub const fn ts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit status"]
    #[inline(always)]
    pub const fn set_ts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Error status"]
    #[must_use]
    #[inline(always)]
    pub const fn es(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Error status"]
    #[inline(always)]
    pub const fn set_es(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus status"]
    #[must_use]
    #[inline(always)]
    pub const fn bs(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus status"]
    #[inline(always)]
    pub const fn set_bs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("rbs", &self.rbs())
            .field("dos", &self.dos())
            .field("tbs", &self.tbs())
            .field("tcs", &self.tcs())
            .field("rs", &self.rs())
            .field("ts", &self.ts())
            .field("es", &self.es())
            .field("bs", &self.bs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ rbs: {=bool:?}, dos: {=bool:?}, tbs: {=bool:?}, tcs: {=bool:?}, rs: {=bool:?}, ts: {=bool:?}, es: {=bool:?}, bs: {=bool:?} }}",
            self.rbs(),
            self.dos(),
            self.tbs(),
            self.tcs(),
            self.rs(),
            self.ts(),
            self.es(),
            self.bs()
        )
    }
}
#[doc = "Peli TX Data register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata0P(pub u32);
impl Txdata0P {
    #[doc = "DATA0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata0P {
    #[inline(always)]
    fn default() -> Txdata0P {
        Txdata0P(0)
    }
}
impl core::fmt::Debug for Txdata0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata0P")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata0P {{ data0: {=u8:?} }}", self.data0())
    }
}
#[doc = "Peli TX Data register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata1P(pub u32);
impl Txdata1P {
    #[doc = "DATA1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata1P {
    #[inline(always)]
    fn default() -> Txdata1P {
        Txdata1P(0)
    }
}
impl core::fmt::Debug for Txdata1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata1P")
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata1P {{ data1: {=u8:?} }}", self.data1())
    }
}
#[doc = "Peli TX Data register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata2P(pub u32);
impl Txdata2P {
    #[doc = "DATA2"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA2"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata2P {
    #[inline(always)]
    fn default() -> Txdata2P {
        Txdata2P(0)
    }
}
impl core::fmt::Debug for Txdata2P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata2P")
            .field("data2", &self.data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata2P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata2P {{ data2: {=u8:?} }}", self.data2())
    }
}
#[doc = "Peli TX Data register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata3P(pub u32);
impl Txdata3P {
    #[doc = "DATA3"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA3"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata3P {
    #[inline(always)]
    fn default() -> Txdata3P {
        Txdata3P(0)
    }
}
impl core::fmt::Debug for Txdata3P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata3P")
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata3P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata3P {{ data3: {=u8:?} }}", self.data3())
    }
}
#[doc = "Peli TX Data register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata4P(pub u32);
impl Txdata4P {
    #[doc = "DATA4"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA4"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata4P {
    #[inline(always)]
    fn default() -> Txdata4P {
        Txdata4P(0)
    }
}
impl core::fmt::Debug for Txdata4P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata4P")
            .field("data4", &self.data4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata4P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata4P {{ data4: {=u8:?} }}", self.data4())
    }
}
#[doc = "Peli TX Data register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata5P(pub u32);
impl Txdata5P {
    #[doc = "DATA5"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA5"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata5P {
    #[inline(always)]
    fn default() -> Txdata5P {
        Txdata5P(0)
    }
}
impl core::fmt::Debug for Txdata5P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata5P")
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata5P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata5P {{ data5: {=u8:?} }}", self.data5())
    }
}
#[doc = "Peli TX Data register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata6P(pub u32);
impl Txdata6P {
    #[doc = "DATA6"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA6"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata6P {
    #[inline(always)]
    fn default() -> Txdata6P {
        Txdata6P(0)
    }
}
impl core::fmt::Debug for Txdata6P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata6P")
            .field("data6", &self.data6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata6P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata6P {{ data6: {=u8:?} }}", self.data6())
    }
}
#[doc = "Peli TX Data register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata7P(pub u32);
impl Txdata7P {
    #[doc = "DATA7"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA7"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata7P {
    #[inline(always)]
    fn default() -> Txdata7P {
        Txdata7P(0)
    }
}
impl core::fmt::Debug for Txdata7P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata7P")
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata7P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata7P {{ data7: {=u8:?} }}", self.data7())
    }
}
#[doc = "Peli TX Data register 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata8P(pub u32);
impl Txdata8P {
    #[doc = "DATA8"]
    #[must_use]
    #[inline(always)]
    pub const fn data8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA8"]
    #[inline(always)]
    pub const fn set_data8(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata8P {
    #[inline(always)]
    fn default() -> Txdata8P {
        Txdata8P(0)
    }
}
impl core::fmt::Debug for Txdata8P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata8P")
            .field("data8", &self.data8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata8P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata8P {{ data8: {=u8:?} }}", self.data8())
    }
}
#[doc = "Peli TX Data register 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata9P(pub u32);
impl Txdata9P {
    #[doc = "DATA9"]
    #[must_use]
    #[inline(always)]
    pub const fn data9(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA9"]
    #[inline(always)]
    pub const fn set_data9(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txdata9P {
    #[inline(always)]
    fn default() -> Txdata9P {
        Txdata9P(0)
    }
}
impl core::fmt::Debug for Txdata9P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdata9P")
            .field("data9", &self.data9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdata9P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdata9P {{ data9: {=u8:?} }}", self.data9())
    }
}
#[doc = "Basic TX Data register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr0B(pub u32);
impl Txdr0B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr0B {
    #[inline(always)]
    fn default() -> Txdr0B {
        Txdr0B(0)
    }
}
impl core::fmt::Debug for Txdr0B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr0B")
            .field("txdr0", &self.txdr0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr0B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr0B {{ txdr0: {=u32:?} }}", self.txdr0())
    }
}
#[doc = "Basic TX Data register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr1B(pub u32);
impl Txdr1B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr1B {
    #[inline(always)]
    fn default() -> Txdr1B {
        Txdr1B(0)
    }
}
impl core::fmt::Debug for Txdr1B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr1B")
            .field("txdr1", &self.txdr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr1B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr1B {{ txdr1: {=u32:?} }}", self.txdr1())
    }
}
#[doc = "Basic Send Data register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr2B(pub u32);
impl Txdr2B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr2B {
    #[inline(always)]
    fn default() -> Txdr2B {
        Txdr2B(0)
    }
}
impl core::fmt::Debug for Txdr2B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr2B")
            .field("txdr2", &self.txdr2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr2B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr2B {{ txdr2: {=u32:?} }}", self.txdr2())
    }
}
#[doc = "Basic TX Data register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr3B(pub u32);
impl Txdr3B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr3B {
    #[inline(always)]
    fn default() -> Txdr3B {
        Txdr3B(0)
    }
}
impl core::fmt::Debug for Txdr3B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr3B")
            .field("txdr3", &self.txdr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr3B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr3B {{ txdr3: {=u32:?} }}", self.txdr3())
    }
}
#[doc = "Basic TX Data register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr4B(pub u32);
impl Txdr4B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr4(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr4B {
    #[inline(always)]
    fn default() -> Txdr4B {
        Txdr4B(0)
    }
}
impl core::fmt::Debug for Txdr4B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr4B")
            .field("txdr4", &self.txdr4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr4B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr4B {{ txdr4: {=u32:?} }}", self.txdr4())
    }
}
#[doc = "Basic TX Data register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr5B(pub u32);
impl Txdr5B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr5(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr5B {
    #[inline(always)]
    fn default() -> Txdr5B {
        Txdr5B(0)
    }
}
impl core::fmt::Debug for Txdr5B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr5B")
            .field("txdr5", &self.txdr5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr5B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr5B {{ txdr5: {=u32:?} }}", self.txdr5())
    }
}
#[doc = "Basic TX Data register 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr6B(pub u32);
impl Txdr6B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr6(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr6B {
    #[inline(always)]
    fn default() -> Txdr6B {
        Txdr6B(0)
    }
}
impl core::fmt::Debug for Txdr6B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr6B")
            .field("txdr6", &self.txdr6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr6B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr6B {{ txdr6: {=u32:?} }}", self.txdr6())
    }
}
#[doc = "Basic TX Data register 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdr7B(pub u32);
impl Txdr7B {
    #[doc = "Transmit data register"]
    #[must_use]
    #[inline(always)]
    pub const fn txdr7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit data register"]
    #[inline(always)]
    pub const fn set_txdr7(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Txdr7B {
    #[inline(always)]
    fn default() -> Txdr7B {
        Txdr7B(0)
    }
}
impl core::fmt::Debug for Txdr7B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txdr7B")
            .field("txdr7", &self.txdr7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txdr7B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txdr7B {{ txdr7: {=u32:?} }}", self.txdr7())
    }
}
#[doc = "Peli TX Error Counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxerrP(pub u32);
impl TxerrP {
    #[doc = "TX error counter register"]
    #[must_use]
    #[inline(always)]
    pub const fn txerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX error counter register"]
    #[inline(always)]
    pub const fn set_txerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TxerrP {
    #[inline(always)]
    fn default() -> TxerrP {
        TxerrP(0)
    }
}
impl core::fmt::Debug for TxerrP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxerrP")
            .field("txerr", &self.txerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxerrP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxerrP {{ txerr: {=u8:?} }}", self.txerr())
    }
}
#[doc = "Basic TX ID register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txid0B(pub u32);
impl Txid0B {
    #[doc = "Identifier bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn id3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 3"]
    #[inline(always)]
    pub const fn set_id3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Identifier bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn id4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 4"]
    #[inline(always)]
    pub const fn set_id4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Identifier bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn id5(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 5"]
    #[inline(always)]
    pub const fn set_id5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Identifier bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn id6(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 6"]
    #[inline(always)]
    pub const fn set_id6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Identifier bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn id7(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 7"]
    #[inline(always)]
    pub const fn set_id7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Identifier bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn id8(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 8"]
    #[inline(always)]
    pub const fn set_id8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Identifier bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn id9(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 9"]
    #[inline(always)]
    pub const fn set_id9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Identifier bit 10"]
    #[must_use]
    #[inline(always)]
    pub const fn id10(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 10"]
    #[inline(always)]
    pub const fn set_id10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Txid0B {
    #[inline(always)]
    fn default() -> Txid0B {
        Txid0B(0)
    }
}
impl core::fmt::Debug for Txid0B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txid0B")
            .field("id3", &self.id3())
            .field("id4", &self.id4())
            .field("id5", &self.id5())
            .field("id6", &self.id6())
            .field("id7", &self.id7())
            .field("id8", &self.id8())
            .field("id9", &self.id9())
            .field("id10", &self.id10())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txid0B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txid0B {{ id3: {=bool:?}, id4: {=bool:?}, id5: {=bool:?}, id6: {=bool:?}, id7: {=bool:?}, id8: {=bool:?}, id9: {=bool:?}, id10: {=bool:?} }}",
            self.id3(),
            self.id4(),
            self.id5(),
            self.id6(),
            self.id7(),
            self.id8(),
            self.id9(),
            self.id10()
        )
    }
}
#[doc = "Peli TX ID register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txid0P(pub u32);
impl Txid0P {
    #[doc = "Identifier bit 21"]
    #[must_use]
    #[inline(always)]
    pub const fn id21(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 21"]
    #[inline(always)]
    pub const fn set_id21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Identifier bit 22"]
    #[must_use]
    #[inline(always)]
    pub const fn id22(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 22"]
    #[inline(always)]
    pub const fn set_id22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Identifier bit 23"]
    #[must_use]
    #[inline(always)]
    pub const fn id23(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 23"]
    #[inline(always)]
    pub const fn set_id23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Identifier bit 24"]
    #[must_use]
    #[inline(always)]
    pub const fn id24(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 24"]
    #[inline(always)]
    pub const fn set_id24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Identifier bit 25"]
    #[must_use]
    #[inline(always)]
    pub const fn id25(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 25"]
    #[inline(always)]
    pub const fn set_id25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Identifier bit 26"]
    #[must_use]
    #[inline(always)]
    pub const fn id26(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 26"]
    #[inline(always)]
    pub const fn set_id26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Identifier bit 27"]
    #[must_use]
    #[inline(always)]
    pub const fn id27(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 27"]
    #[inline(always)]
    pub const fn set_id27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Identifier bit 28"]
    #[must_use]
    #[inline(always)]
    pub const fn id28(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 28"]
    #[inline(always)]
    pub const fn set_id28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Txid0P {
    #[inline(always)]
    fn default() -> Txid0P {
        Txid0P(0)
    }
}
impl core::fmt::Debug for Txid0P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txid0P")
            .field("id21", &self.id21())
            .field("id22", &self.id22())
            .field("id23", &self.id23())
            .field("id24", &self.id24())
            .field("id25", &self.id25())
            .field("id26", &self.id26())
            .field("id27", &self.id27())
            .field("id28", &self.id28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txid0P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txid0P {{ id21: {=bool:?}, id22: {=bool:?}, id23: {=bool:?}, id24: {=bool:?}, id25: {=bool:?}, id26: {=bool:?}, id27: {=bool:?}, id28: {=bool:?} }}",
            self.id21(),
            self.id22(),
            self.id23(),
            self.id24(),
            self.id25(),
            self.id26(),
            self.id27(),
            self.id28()
        )
    }
}
#[doc = "Basic TX ID register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txid1B(pub u32);
impl Txid1B {
    #[doc = "Data length code"]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data length code"]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Remote transmission request"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Remote transmission request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Identifier bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn id0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 0"]
    #[inline(always)]
    pub const fn set_id0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Identifier bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn id1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 1"]
    #[inline(always)]
    pub const fn set_id1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Identifier bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn id2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 2"]
    #[inline(always)]
    pub const fn set_id2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Txid1B {
    #[inline(always)]
    fn default() -> Txid1B {
        Txid1B(0)
    }
}
impl core::fmt::Debug for Txid1B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txid1B")
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("id0", &self.id0())
            .field("id1", &self.id1())
            .field("id2", &self.id2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txid1B {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txid1B {{ dlc: {=u8:?}, rtr: {=bool:?}, id0: {=bool:?}, id1: {=bool:?}, id2: {=bool:?} }}",
            self.dlc(),
            self.rtr(),
            self.id0(),
            self.id1(),
            self.id2()
        )
    }
}
#[doc = "Peli TX ID register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txid1P(pub u32);
impl Txid1P {
    #[doc = "Identifier bit 13"]
    #[must_use]
    #[inline(always)]
    pub const fn id13(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 13"]
    #[inline(always)]
    pub const fn set_id13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Identifier bit 14"]
    #[must_use]
    #[inline(always)]
    pub const fn id14(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 14"]
    #[inline(always)]
    pub const fn set_id14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Identifier bit 15"]
    #[must_use]
    #[inline(always)]
    pub const fn id15(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 15"]
    #[inline(always)]
    pub const fn set_id15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Identifier bit 16"]
    #[must_use]
    #[inline(always)]
    pub const fn id16(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 16"]
    #[inline(always)]
    pub const fn set_id16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Identifier bit 17"]
    #[must_use]
    #[inline(always)]
    pub const fn id17(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 17"]
    #[inline(always)]
    pub const fn set_id17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Identifier bit 18"]
    #[must_use]
    #[inline(always)]
    pub const fn id18(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 18"]
    #[inline(always)]
    pub const fn set_id18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Identifier bit 19"]
    #[must_use]
    #[inline(always)]
    pub const fn id19(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 19"]
    #[inline(always)]
    pub const fn set_id19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Identifier bit 20"]
    #[must_use]
    #[inline(always)]
    pub const fn id20(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Identifier bit 20"]
    #[inline(always)]
    pub const fn set_id20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Txid1P {
    #[inline(always)]
    fn default() -> Txid1P {
        Txid1P(0)
    }
}
impl core::fmt::Debug for Txid1P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txid1P")
            .field("id13", &self.id13())
            .field("id14", &self.id14())
            .field("id15", &self.id15())
            .field("id16", &self.id16())
            .field("id17", &self.id17())
            .field("id18", &self.id18())
            .field("id19", &self.id19())
            .field("id20", &self.id20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txid1P {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txid1P {{ id13: {=bool:?}, id14: {=bool:?}, id15: {=bool:?}, id16: {=bool:?}, id17: {=bool:?}, id18: {=bool:?}, id19: {=bool:?}, id20: {=bool:?} }}",
            self.id13(),
            self.id14(),
            self.id15(),
            self.id16(),
            self.id17(),
            self.id18(),
            self.id19(),
            self.id20()
        )
    }
}
