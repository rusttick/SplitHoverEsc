#[doc = "COMP1_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp1Csr(pub u32);
impl Comp1Csr {
    #[doc = "Comparator 1 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 1 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 1 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 1 output selection"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 1 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 1 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 1 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator 1 output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 1 output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 1 output status"]
    #[must_use]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 output status"]
    #[inline(always)]
    pub const fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator 1 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 1 lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp1Csr {
    #[inline(always)]
    fn default() -> Comp1Csr {
        Comp1Csr(0)
    }
}
impl core::fmt::Debug for Comp1Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp1Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("out", &self.out())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("sta", &self.sta())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp1Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp1Csr {{ en: {=bool:?}, mode: {=u8:?}, out: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, sta: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.out(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.sta(),
            self.lock()
        )
    }
}
#[doc = "COMP2_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp2Csr(pub u32);
impl Comp2Csr {
    #[doc = "Comparator 2 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 2 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 2 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 2 output selection"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 2 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 2 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 2 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator 2 output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 2 output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 2 output status"]
    #[must_use]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 output status"]
    #[inline(always)]
    pub const fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator 2 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 2 lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp2Csr {
    #[inline(always)]
    fn default() -> Comp2Csr {
        Comp2Csr(0)
    }
}
impl core::fmt::Debug for Comp2Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp2Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("out", &self.out())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("sta", &self.sta())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp2Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp2Csr {{ en: {=bool:?}, mode: {=u8:?}, out: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, sta: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.out(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.sta(),
            self.lock()
        )
    }
}
#[doc = "COMP3_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp3Csr(pub u32);
impl Comp3Csr {
    #[doc = "Comparator 3 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 3 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 3 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 3 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 3 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 3 output selection"]
    #[inline(always)]
    pub const fn set_out_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 3 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 3 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 3 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 3 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator 3 output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 3 output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 3 output status"]
    #[must_use]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 3 output status"]
    #[inline(always)]
    pub const fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator 3 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 3 lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp3Csr {
    #[inline(always)]
    fn default() -> Comp3Csr {
        Comp3Csr(0)
    }
}
impl core::fmt::Debug for Comp3Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp3Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("out_sel", &self.out_sel())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("sta", &self.sta())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp3Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp3Csr {{ en: {=bool:?}, mode: {=u8:?}, out_sel: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, sta: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.out_sel(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.sta(),
            self.lock()
        )
    }
}
#[doc = "COMP4_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp4Csr(pub u32);
impl Comp4Csr {
    #[doc = "Comparator 4 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 4 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 4 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 4 inverting input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inm(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 4 inverting input selection"]
    #[inline(always)]
    pub const fn set_inm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Comparator 4 normal phase input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 4 normal phase input selection"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Comparator 4 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 4 output selection"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 4 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 4 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 4 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator 4 output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 4 output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 4 output status"]
    #[must_use]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 output status"]
    #[inline(always)]
    pub const fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator 4 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp4Csr {
    #[inline(always)]
    fn default() -> Comp4Csr {
        Comp4Csr(0)
    }
}
impl core::fmt::Debug for Comp4Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp4Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("inm", &self.inm())
            .field("inp", &self.inp())
            .field("out", &self.out())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("sta", &self.sta())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp4Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp4Csr {{ en: {=bool:?}, mode: {=u8:?}, inm: {=u8:?}, inp: {=u8:?}, out: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, sta: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.inm(),
            self.inp(),
            self.out(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.sta(),
            self.lock()
        )
    }
}
#[doc = "COMP4_POLL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp4Poll(pub u32);
impl Comp4Poll {
    #[doc = "Comparator 4 polling enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 polling enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 4 polling channel"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 polling channel"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 4 Polling inverting input fix"]
    #[must_use]
    #[inline(always)]
    pub const fn fixn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 4 Polling inverting input fix"]
    #[inline(always)]
    pub const fn set_fixn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 4 polling wait cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 4 polling wait cycle"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Comparator 4 Polling output"]
    #[must_use]
    #[inline(always)]
    pub const fn pout(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 4 Polling output"]
    #[inline(always)]
    pub const fn set_pout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Comp4Poll {
    #[inline(always)]
    fn default() -> Comp4Poll {
        Comp4Poll(0)
    }
}
impl core::fmt::Debug for Comp4Poll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp4Poll")
            .field("en", &self.en())
            .field("ch", &self.ch())
            .field("fixn", &self.fixn())
            .field("period", &self.period())
            .field("pout", &self.pout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp4Poll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp4Poll {{ en: {=bool:?}, ch: {=bool:?}, fixn: {=bool:?}, period: {=u8:?}, pout: {=u8:?} }}",
            self.en(),
            self.ch(),
            self.fixn(),
            self.period(),
            self.pout()
        )
    }
}
#[doc = "COMP5_CSR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp5Csr(pub u32);
impl Comp5Csr {
    #[doc = "Comparator 5 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 5 mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 5 mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Comparator 5 inverting input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inm(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 5 inverting input selection"]
    #[inline(always)]
    pub const fn set_inm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Comparator 5 normal phase input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 5 normal phase input selection"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Comparator 5 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator 5 output selection"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Comparator 5 output polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 output polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Comparator 5 hysteresis"]
    #[must_use]
    #[inline(always)]
    pub const fn hyst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Comparator 5 hysteresis"]
    #[inline(always)]
    pub const fn set_hyst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Comparator 5 output filter"]
    #[must_use]
    #[inline(always)]
    pub const fn oflt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 5 output filter"]
    #[inline(always)]
    pub const fn set_oflt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Comparator 5 output status"]
    #[must_use]
    #[inline(always)]
    pub const fn sta(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 output status"]
    #[inline(always)]
    pub const fn set_sta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Comparator 5 lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Comp5Csr {
    #[inline(always)]
    fn default() -> Comp5Csr {
        Comp5Csr(0)
    }
}
impl core::fmt::Debug for Comp5Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp5Csr")
            .field("en", &self.en())
            .field("mode", &self.mode())
            .field("inm", &self.inm())
            .field("inp", &self.inp())
            .field("out", &self.out())
            .field("pol", &self.pol())
            .field("hyst", &self.hyst())
            .field("oflt", &self.oflt())
            .field("sta", &self.sta())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp5Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp5Csr {{ en: {=bool:?}, mode: {=u8:?}, inm: {=u8:?}, inp: {=u8:?}, out: {=u8:?}, pol: {=bool:?}, hyst: {=u8:?}, oflt: {=u8:?}, sta: {=bool:?}, lock: {=bool:?} }}",
            self.en(),
            self.mode(),
            self.inm(),
            self.inp(),
            self.out(),
            self.pol(),
            self.hyst(),
            self.oflt(),
            self.sta(),
            self.lock()
        )
    }
}
#[doc = "COMP5_POLL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp5Poll(pub u32);
impl Comp5Poll {
    #[doc = "Comparator 5 polling enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 polling enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Comparator 5 polling channel"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 polling channel"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Comparator 5 Polling inverting input fix"]
    #[must_use]
    #[inline(always)]
    pub const fn fixn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator 5 Polling inverting input fix"]
    #[inline(always)]
    pub const fn set_fixn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Comparator 5 polling wait cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 5 polling wait cycle"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Comparator 5 Polling output"]
    #[must_use]
    #[inline(always)]
    pub const fn pout(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Comparator 5 Polling output"]
    #[inline(always)]
    pub const fn set_pout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Comp5Poll {
    #[inline(always)]
    fn default() -> Comp5Poll {
        Comp5Poll(0)
    }
}
impl core::fmt::Debug for Comp5Poll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Comp5Poll")
            .field("en", &self.en())
            .field("ch", &self.ch())
            .field("fixn", &self.fixn())
            .field("period", &self.period())
            .field("pout", &self.pout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Comp5Poll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Comp5Poll {{ en: {=bool:?}, ch: {=bool:?}, fixn: {=bool:?}, period: {=u8:?}, pout: {=u8:?} }}",
            self.en(),
            self.ch(),
            self.fixn(),
            self.period(),
            self.pout()
        )
    }
}
#[doc = "COMP_CRV"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompCrv(pub u32);
impl CompCrv {
    #[doc = "Comparator external referencevoltage select"]
    #[must_use]
    #[inline(always)]
    pub const fn crv_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Comparator external referencevoltage select"]
    #[inline(always)]
    pub const fn set_crv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Comparator external referencevoltage enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator external referencevoltage enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Comparator external refer-ence voltage source select"]
    #[must_use]
    #[inline(always)]
    pub const fn crv_src(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator external refer-ence voltage source select"]
    #[inline(always)]
    pub const fn set_crv_src(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CompCrv {
    #[inline(always)]
    fn default() -> CompCrv {
        CompCrv(0)
    }
}
impl core::fmt::Debug for CompCrv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompCrv")
            .field("crv_sel", &self.crv_sel())
            .field("en", &self.en())
            .field("crv_src", &self.crv_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CompCrv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CompCrv {{ crv_sel: {=u8:?}, en: {=bool:?}, crv_src: {=bool:?} }}",
            self.crv_sel(),
            self.en(),
            self.crv_src()
        )
    }
}
