#[doc = "RTC alarm high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcAlrh(pub u16);
impl RtcAlrh {
    #[doc = "RTC alarm high"]
    #[must_use]
    #[inline(always)]
    pub const fn alr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC alarm high"]
    #[inline(always)]
    pub const fn set_alr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcAlrh {
    #[inline(always)]
    fn default() -> RtcAlrh {
        RtcAlrh(0)
    }
}
impl core::fmt::Debug for RtcAlrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcAlrh").field("alr", &self.alr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcAlrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcAlrh {{ alr: {=u16:?} }}", self.alr())
    }
}
#[doc = "RTC alarm low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcAlrl(pub u16);
impl RtcAlrl {
    #[doc = "RTC alarm low"]
    #[must_use]
    #[inline(always)]
    pub const fn alr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC alarm low"]
    #[inline(always)]
    pub const fn set_alr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcAlrl {
    #[inline(always)]
    fn default() -> RtcAlrl {
        RtcAlrl(0)
    }
}
impl core::fmt::Debug for RtcAlrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcAlrl").field("alr", &self.alr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcAlrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcAlrl {{ alr: {=u16:?} }}", self.alr())
    }
}
#[doc = "RTC counter high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCnth(pub u16);
impl RtcCnth {
    #[doc = "RTC counter high"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC counter high"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcCnth {
    #[inline(always)]
    fn default() -> RtcCnth {
        RtcCnth(0)
    }
}
impl core::fmt::Debug for RtcCnth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcCnth").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcCnth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcCnth {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "RTC counter low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCntl(pub u16);
impl RtcCntl {
    #[doc = "RTC counter low"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC counter low"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcCntl {
    #[inline(always)]
    fn default() -> RtcCntl {
        RtcCntl(0)
    }
}
impl core::fmt::Debug for RtcCntl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcCntl").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcCntl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcCntl {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "RTC configuration high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCrh(pub u16);
impl RtcCrh {
    #[doc = "Second interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn secie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Second interrupt enable"]
    #[inline(always)]
    pub const fn set_secie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Alarm interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alrie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm interrupt enable"]
    #[inline(always)]
    pub const fn set_alrie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Overflow interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn owie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub const fn set_owie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for RtcCrh {
    #[inline(always)]
    fn default() -> RtcCrh {
        RtcCrh(0)
    }
}
impl core::fmt::Debug for RtcCrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcCrh")
            .field("secie", &self.secie())
            .field("alrie", &self.alrie())
            .field("owie", &self.owie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcCrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RtcCrh {{ secie: {=bool:?}, alrie: {=bool:?}, owie: {=bool:?} }}",
            self.secie(),
            self.alrie(),
            self.owie()
        )
    }
}
#[doc = "RTC configuration low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcCrl(pub u16);
impl RtcCrl {
    #[doc = "Second flag"]
    #[must_use]
    #[inline(always)]
    pub const fn secf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Second flag"]
    #[inline(always)]
    pub const fn set_secf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Alarm flag"]
    #[must_use]
    #[inline(always)]
    pub const fn alrf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm flag"]
    #[inline(always)]
    pub const fn set_alrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Overflow flag"]
    #[must_use]
    #[inline(always)]
    pub const fn owf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow flag"]
    #[inline(always)]
    pub const fn set_owf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Registers synchronized flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Registers synchronized flag"]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Configuration flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cnf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration flag"]
    #[inline(always)]
    pub const fn set_cnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "RTC operation OFF"]
    #[must_use]
    #[inline(always)]
    pub const fn rtoff(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RTC operation OFF"]
    #[inline(always)]
    pub const fn set_rtoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "RTC alarm loop enable"]
    #[must_use]
    #[inline(always)]
    pub const fn alpen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RTC alarm loop enable"]
    #[inline(always)]
    pub const fn set_alpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
}
impl Default for RtcCrl {
    #[inline(always)]
    fn default() -> RtcCrl {
        RtcCrl(0)
    }
}
impl core::fmt::Debug for RtcCrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcCrl")
            .field("secf", &self.secf())
            .field("alrf", &self.alrf())
            .field("owf", &self.owf())
            .field("rsf", &self.rsf())
            .field("cnf", &self.cnf())
            .field("rtoff", &self.rtoff())
            .field("alpen", &self.alpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcCrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RtcCrl {{ secf: {=bool:?}, alrf: {=bool:?}, owf: {=bool:?}, rsf: {=bool:?}, cnf: {=bool:?}, rtoff: {=bool:?}, alpen: {=bool:?} }}",
            self.secf(),
            self.alrf(),
            self.owf(),
            self.rsf(),
            self.cnf(),
            self.rtoff(),
            self.alpen()
        )
    }
}
#[doc = "RTC prescaler divider factor high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcDivh(pub u16);
impl RtcDivh {
    #[doc = "RTC clock divider high"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "RTC clock divider high"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for RtcDivh {
    #[inline(always)]
    fn default() -> RtcDivh {
        RtcDivh(0)
    }
}
impl core::fmt::Debug for RtcDivh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcDivh").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcDivh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcDivh {{ div: {=u8:?} }}", self.div())
    }
}
#[doc = "RTC prescaler divider factor low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcDivl(pub u16);
impl RtcDivl {
    #[doc = "RTC clock divider low"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC clock divider low"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcDivl {
    #[inline(always)]
    fn default() -> RtcDivl {
        RtcDivl(0)
    }
}
impl core::fmt::Debug for RtcDivl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcDivl").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcDivl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcDivl {{ div: {=u16:?} }}", self.div())
    }
}
#[doc = "RTC millisecond alarm high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcMsrh(pub u16);
impl RtcMsrh {
    #[doc = "RTC msec high"]
    #[must_use]
    #[inline(always)]
    pub const fn msr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "RTC msec high"]
    #[inline(always)]
    pub const fn set_msr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for RtcMsrh {
    #[inline(always)]
    fn default() -> RtcMsrh {
        RtcMsrh(0)
    }
}
impl core::fmt::Debug for RtcMsrh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcMsrh").field("msr", &self.msr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcMsrh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcMsrh {{ msr: {=u8:?} }}", self.msr())
    }
}
#[doc = "RTC millisecond alarm low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcMsrl(pub u16);
impl RtcMsrl {
    #[doc = "RTC msec low"]
    #[must_use]
    #[inline(always)]
    pub const fn msr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC msec low"]
    #[inline(always)]
    pub const fn set_msr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcMsrl {
    #[inline(always)]
    fn default() -> RtcMsrl {
        RtcMsrl(0)
    }
}
impl core::fmt::Debug for RtcMsrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcMsrl").field("msr", &self.msr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcMsrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcMsrl {{ msr: {=u16:?} }}", self.msr())
    }
}
#[doc = "RTC Prescaler load high register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcPrlh(pub u16);
impl RtcPrlh {
    #[doc = "RTC prescaler reload value high"]
    #[must_use]
    #[inline(always)]
    pub const fn prl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "RTC prescaler reload value high"]
    #[inline(always)]
    pub const fn set_prl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for RtcPrlh {
    #[inline(always)]
    fn default() -> RtcPrlh {
        RtcPrlh(0)
    }
}
impl core::fmt::Debug for RtcPrlh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcPrlh").field("prl", &self.prl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcPrlh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcPrlh {{ prl: {=u8:?} }}", self.prl())
    }
}
#[doc = "RTC Prescaler load low register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcPrll(pub u16);
impl RtcPrll {
    #[doc = "RTC prescaler reload value low"]
    #[must_use]
    #[inline(always)]
    pub const fn prl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RTC prescaler reload value low"]
    #[inline(always)]
    pub const fn set_prl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for RtcPrll {
    #[inline(always)]
    fn default() -> RtcPrll {
        RtcPrll(0)
    }
}
impl core::fmt::Debug for RtcPrll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcPrll").field("prl", &self.prl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcPrll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RtcPrll {{ prl: {=u16:?} }}", self.prl())
    }
}
