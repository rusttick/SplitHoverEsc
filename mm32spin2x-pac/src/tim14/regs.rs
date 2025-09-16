#[doc = "auto-reload register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arr(pub u32);
impl Arr {
    #[doc = "auto-reload value"]
    #[must_use]
    #[inline(always)]
    pub const fn arr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "auto-reload value"]
    #[inline(always)]
    pub const fn set_arr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Arr {
    #[inline(always)]
    fn default() -> Arr {
        Arr(0)
    }
}
impl core::fmt::Debug for Arr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arr").field("arr", &self.arr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Arr {{ arr: {=u16:?} }}", self.arr())
    }
}
#[doc = "capture/compare enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccer(pub u32);
impl Ccer {
    #[doc = "Capture/Compare 1 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1e(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output enable"]
    #[inline(always)]
    pub const fn set_cc1e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 output polarity"]
    #[inline(always)]
    pub const fn set_cc1p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 1 complementary output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1np(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 complementary output polarity"]
    #[inline(always)]
    pub const fn set_cc1np(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ccer {
    #[inline(always)]
    fn default() -> Ccer {
        Ccer(0)
    }
}
impl core::fmt::Debug for Ccer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccer")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccer {{ cc1e: {=bool:?}, cc1p: {=bool:?}, cc1np: {=bool:?} }}",
            self.cc1e(),
            self.cc1p(),
            self.cc1np()
        )
    }
}
#[doc = "capture/compare mode register 1 (input mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Input(pub u32);
impl Ccmr1Input {
    #[doc = "Capture/compare 1 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/compare 1 selection"]
    #[inline(always)]
    pub const fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input capture 1 prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn ic1psc(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Input capture 1 prescaler"]
    #[inline(always)]
    pub const fn set_ic1psc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Input capture 1 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn ic1f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Input capture 1 filter"]
    #[inline(always)]
    pub const fn set_ic1f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Ccmr1Input {
    #[inline(always)]
    fn default() -> Ccmr1Input {
        Ccmr1Input(0)
    }
}
impl core::fmt::Debug for Ccmr1Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr1Input")
            .field("cc1s", &self.cc1s())
            .field("ic1psc", &self.ic1psc())
            .field("ic1f", &self.ic1f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr1Input {{ cc1s: {=u8:?}, ic1psc: {=u8:?}, ic1f: {=u8:?} }}",
            self.cc1s(),
            self.ic1psc(),
            self.ic1f()
        )
    }
}
#[doc = "capture/compare mode register 1 (output mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccmr1Output(pub u32);
impl Ccmr1Output {
    #[doc = "Capture/Compare 2 output enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1s(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Capture/Compare 2 output enable"]
    #[inline(always)]
    pub const fn set_cc1s(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output compare 1 fast enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1fe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 fast enable"]
    #[inline(always)]
    pub const fn set_oc1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output compare 1 preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1pe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output compare 1 preload enable"]
    #[inline(always)]
    pub const fn set_oc1pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output compare 1 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn oc1m(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Output compare 1 mode"]
    #[inline(always)]
    pub const fn set_oc1m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for Ccmr1Output {
    #[inline(always)]
    fn default() -> Ccmr1Output {
        Ccmr1Output(0)
    }
}
impl core::fmt::Debug for Ccmr1Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccmr1Output")
            .field("cc1s", &self.cc1s())
            .field("oc1fe", &self.oc1fe())
            .field("oc1pe", &self.oc1pe())
            .field("oc1m", &self.oc1m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccmr1Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccmr1Output {{ cc1s: {=u8:?}, oc1fe: {=bool:?}, oc1pe: {=bool:?}, oc1m: {=u8:?} }}",
            self.cc1s(),
            self.oc1fe(),
            self.oc1pe(),
            self.oc1m()
        )
    }
}
#[doc = "capture/compare register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Capture/Compare 1 value"]
    #[must_use]
    #[inline(always)]
    pub const fn ccr1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture/Compare 1 value"]
    #[inline(always)]
    pub const fn set_ccr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
impl core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr1").field("ccr1", &self.ccr1()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ccr1 {{ ccr1: {=u16:?} }}", self.ccr1())
    }
}
#[doc = "counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter value"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "Counter enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable"]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Update disable"]
    #[must_use]
    #[inline(always)]
    pub const fn udis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Update disable"]
    #[inline(always)]
    pub const fn set_udis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Update request source"]
    #[must_use]
    #[inline(always)]
    pub const fn urs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Update request source"]
    #[inline(always)]
    pub const fn set_urs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto-reload preload enable"]
    #[must_use]
    #[inline(always)]
    pub const fn arpe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-reload preload enable"]
    #[inline(always)]
    pub const fn set_arpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clock division"]
    #[must_use]
    #[inline(always)]
    pub const fn ckd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Clock division"]
    #[inline(always)]
    pub const fn set_ckd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("cen", &self.cen())
            .field("udis", &self.udis())
            .field("urs", &self.urs())
            .field("arpe", &self.arpe())
            .field("ckd", &self.ckd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {=bool:?}, arpe: {=bool:?}, ckd: {=u8:?} }}",
            self.cen(),
            self.udis(),
            self.urs(),
            self.arpe(),
            self.ckd()
        )
    }
}
#[doc = "DMA/Interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dier(pub u32);
impl Dier {
    #[doc = "Update interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt enable"]
    #[inline(always)]
    pub const fn set_uie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub const fn set_cc1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dier {
    #[inline(always)]
    fn default() -> Dier {
        Dier(0)
    }
}
impl core::fmt::Debug for Dier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dier")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dier {{ uie: {=bool:?}, cc1ie: {=bool:?} }}",
            self.uie(),
            self.cc1ie()
        )
    }
}
#[doc = "event generation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Egr(pub u32);
impl Egr {
    #[doc = "Update generation"]
    #[must_use]
    #[inline(always)]
    pub const fn ug(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update generation"]
    #[inline(always)]
    pub const fn set_ug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 generation"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1g(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 generation"]
    #[inline(always)]
    pub const fn set_cc1g(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Egr {
    #[inline(always)]
    fn default() -> Egr {
        Egr(0)
    }
}
impl core::fmt::Debug for Egr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Egr")
            .field("ug", &self.ug())
            .field("cc1g", &self.cc1g())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Egr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Egr {{ ug: {=bool:?}, cc1g: {=bool:?} }}",
            self.ug(),
            self.cc1g()
        )
    }
}
#[doc = "prescaler"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psc(pub u32);
impl Psc {
    #[doc = "Prescaler value"]
    #[must_use]
    #[inline(always)]
    pub const fn psc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Prescaler value"]
    #[inline(always)]
    pub const fn set_psc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psc {
    #[inline(always)]
    fn default() -> Psc {
        Psc(0)
    }
}
impl core::fmt::Debug for Psc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psc").field("psc", &self.psc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Psc {{ psc: {=u16:?} }}", self.psc())
    }
}
#[doc = "status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Update interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn uif(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Update interrupt flag"]
    #[inline(always)]
    pub const fn set_uif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1if(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 interrupt flag"]
    #[inline(always)]
    pub const fn set_cc1if(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cc1of(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub const fn set_cc1of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc1of", &self.cc1of())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ uif: {=bool:?}, cc1if: {=bool:?}, cc1of: {=bool:?} }}",
            self.uif(),
            self.cc1if(),
            self.cc1of()
        )
    }
}
