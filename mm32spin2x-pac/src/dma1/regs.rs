#[doc = "CCR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "Channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half transfer interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half transfer interrupt enable"]
    #[inline(always)]
    pub const fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer error interrupt enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Circular mode"]
    #[must_use]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Circular mode"]
    #[inline(always)]
    pub const fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral increment mode"]
    #[inline(always)]
    pub const fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Memory increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Memory increment mode"]
    #[inline(always)]
    pub const fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral size"]
    #[must_use]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral size"]
    #[inline(always)]
    pub const fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority level"]
    #[inline(always)]
    pub const fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Memory to memory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Memory to memory mode"]
    #[inline(always)]
    pub const fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto-Reload Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn are(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Reload Enable bit"]
    #[inline(always)]
    pub const fn set_are(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        f.debug_struct("Ccr1")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("are", &self.are())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr1 {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {=bool:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {=u8:?}, msize: {=u8:?}, pl: {=u8:?}, mem2mem: {=bool:?}, are: {=bool:?} }}",
            self.en(),
            self.tcie(),
            self.htie(),
            self.teie(),
            self.dir(),
            self.circ(),
            self.pinc(),
            self.minc(),
            self.psize(),
            self.msize(),
            self.pl(),
            self.mem2mem(),
            self.are()
        )
    }
}
#[doc = "CCR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2(pub u32);
impl Ccr2 {
    #[doc = "Channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half transfer interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half transfer interrupt enable"]
    #[inline(always)]
    pub const fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer error interrupt enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Circular mode"]
    #[must_use]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Circular mode"]
    #[inline(always)]
    pub const fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral increment mode"]
    #[inline(always)]
    pub const fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Memory increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Memory increment mode"]
    #[inline(always)]
    pub const fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral size"]
    #[must_use]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral size"]
    #[inline(always)]
    pub const fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority level"]
    #[inline(always)]
    pub const fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Memory to memory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Memory to memory mode"]
    #[inline(always)]
    pub const fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto-Reload Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn are(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Reload Enable bit"]
    #[inline(always)]
    pub const fn set_are(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        Ccr2(0)
    }
}
impl core::fmt::Debug for Ccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr2")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("are", &self.are())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr2 {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {=bool:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {=u8:?}, msize: {=u8:?}, pl: {=u8:?}, mem2mem: {=bool:?}, are: {=bool:?} }}",
            self.en(),
            self.tcie(),
            self.htie(),
            self.teie(),
            self.dir(),
            self.circ(),
            self.pinc(),
            self.minc(),
            self.psize(),
            self.msize(),
            self.pl(),
            self.mem2mem(),
            self.are()
        )
    }
}
#[doc = "CCR3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3(pub u32);
impl Ccr3 {
    #[doc = "Channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half transfer interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half transfer interrupt enable"]
    #[inline(always)]
    pub const fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer error interrupt enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Circular mode"]
    #[must_use]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Circular mode"]
    #[inline(always)]
    pub const fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral increment mode"]
    #[inline(always)]
    pub const fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Memory increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Memory increment mode"]
    #[inline(always)]
    pub const fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral size"]
    #[must_use]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral size"]
    #[inline(always)]
    pub const fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority level"]
    #[inline(always)]
    pub const fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Memory to memory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Memory to memory mode"]
    #[inline(always)]
    pub const fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto-Reload Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn are(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Reload Enable bit"]
    #[inline(always)]
    pub const fn set_are(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        Ccr3(0)
    }
}
impl core::fmt::Debug for Ccr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr3")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("are", &self.are())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr3 {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {=bool:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {=u8:?}, msize: {=u8:?}, pl: {=u8:?}, mem2mem: {=bool:?}, are: {=bool:?} }}",
            self.en(),
            self.tcie(),
            self.htie(),
            self.teie(),
            self.dir(),
            self.circ(),
            self.pinc(),
            self.minc(),
            self.psize(),
            self.msize(),
            self.pl(),
            self.mem2mem(),
            self.are()
        )
    }
}
#[doc = "CCR4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4(pub u32);
impl Ccr4 {
    #[doc = "Channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half transfer interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half transfer interrupt enable"]
    #[inline(always)]
    pub const fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer error interrupt enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Circular mode"]
    #[must_use]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Circular mode"]
    #[inline(always)]
    pub const fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral increment mode"]
    #[inline(always)]
    pub const fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Memory increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Memory increment mode"]
    #[inline(always)]
    pub const fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral size"]
    #[must_use]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral size"]
    #[inline(always)]
    pub const fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority level"]
    #[inline(always)]
    pub const fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Memory to memory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Memory to memory mode"]
    #[inline(always)]
    pub const fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto-Reload Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn are(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Reload Enable bit"]
    #[inline(always)]
    pub const fn set_are(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        Ccr4(0)
    }
}
impl core::fmt::Debug for Ccr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr4")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("are", &self.are())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr4 {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {=bool:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {=u8:?}, msize: {=u8:?}, pl: {=u8:?}, mem2mem: {=bool:?}, are: {=bool:?} }}",
            self.en(),
            self.tcie(),
            self.htie(),
            self.teie(),
            self.dir(),
            self.circ(),
            self.pinc(),
            self.minc(),
            self.psize(),
            self.msize(),
            self.pl(),
            self.mem2mem(),
            self.are()
        )
    }
}
#[doc = "CCR5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr5(pub u32);
impl Ccr5 {
    #[doc = "Channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half transfer interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn htie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half transfer interrupt enable"]
    #[inline(always)]
    pub const fn set_htie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transfer error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer error interrupt enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Circular mode"]
    #[must_use]
    #[inline(always)]
    pub const fn circ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Circular mode"]
    #[inline(always)]
    pub const fn set_circ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Peripheral increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pinc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral increment mode"]
    #[inline(always)]
    pub const fn set_pinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Memory increment mode"]
    #[must_use]
    #[inline(always)]
    pub const fn minc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Memory increment mode"]
    #[inline(always)]
    pub const fn set_minc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Peripheral size"]
    #[must_use]
    #[inline(always)]
    pub const fn psize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Peripheral size"]
    #[inline(always)]
    pub const fn set_psize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn msize(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_msize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Channel priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Channel priority level"]
    #[inline(always)]
    pub const fn set_pl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Memory to memory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mem2mem(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Memory to memory mode"]
    #[inline(always)]
    pub const fn set_mem2mem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Auto-Reload Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn are(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Auto-Reload Enable bit"]
    #[inline(always)]
    pub const fn set_are(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ccr5 {
    #[inline(always)]
    fn default() -> Ccr5 {
        Ccr5(0)
    }
}
impl core::fmt::Debug for Ccr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr5")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("are", &self.are())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr5 {{ en: {=bool:?}, tcie: {=bool:?}, htie: {=bool:?}, teie: {=bool:?}, dir: {=bool:?}, circ: {=bool:?}, pinc: {=bool:?}, minc: {=bool:?}, psize: {=u8:?}, msize: {=u8:?}, pl: {=u8:?}, mem2mem: {=bool:?}, are: {=bool:?} }}",
            self.en(),
            self.tcie(),
            self.htie(),
            self.teie(),
            self.dir(),
            self.circ(),
            self.pinc(),
            self.minc(),
            self.psize(),
            self.msize(),
            self.pl(),
            self.mem2mem(),
            self.are()
        )
    }
}
#[doc = "CMAR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar1(pub u32);
impl Cmar1 {
    #[doc = "Memory address"]
    #[must_use]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory address"]
    #[inline(always)]
    pub const fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar1 {
    #[inline(always)]
    fn default() -> Cmar1 {
        Cmar1(0)
    }
}
impl core::fmt::Debug for Cmar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmar1").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmar1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmar1 {{ ma: {=u32:?} }}", self.ma())
    }
}
#[doc = "CMAR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar2(pub u32);
impl Cmar2 {
    #[doc = "Memory address"]
    #[must_use]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory address"]
    #[inline(always)]
    pub const fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar2 {
    #[inline(always)]
    fn default() -> Cmar2 {
        Cmar2(0)
    }
}
impl core::fmt::Debug for Cmar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmar2").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmar2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmar2 {{ ma: {=u32:?} }}", self.ma())
    }
}
#[doc = "CMAR3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar3(pub u32);
impl Cmar3 {
    #[doc = "Memory address"]
    #[must_use]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory address"]
    #[inline(always)]
    pub const fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar3 {
    #[inline(always)]
    fn default() -> Cmar3 {
        Cmar3(0)
    }
}
impl core::fmt::Debug for Cmar3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmar3").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmar3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmar3 {{ ma: {=u32:?} }}", self.ma())
    }
}
#[doc = "CMAR4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar4(pub u32);
impl Cmar4 {
    #[doc = "Memory address"]
    #[must_use]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory address"]
    #[inline(always)]
    pub const fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar4 {
    #[inline(always)]
    fn default() -> Cmar4 {
        Cmar4(0)
    }
}
impl core::fmt::Debug for Cmar4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmar4").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmar4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmar4 {{ ma: {=u32:?} }}", self.ma())
    }
}
#[doc = "CMAR5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmar5(pub u32);
impl Cmar5 {
    #[doc = "Memory address"]
    #[must_use]
    #[inline(always)]
    pub const fn ma(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory address"]
    #[inline(always)]
    pub const fn set_ma(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmar5 {
    #[inline(always)]
    fn default() -> Cmar5 {
        Cmar5(0)
    }
}
impl core::fmt::Debug for Cmar5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmar5").field("ma", &self.ma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmar5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmar5 {{ ma: {=u32:?} }}", self.ma())
    }
}
#[doc = "CNDTR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr1(pub u32);
impl Cndtr1 {
    #[doc = "Number of data to transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data to transfer"]
    #[inline(always)]
    pub const fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr1 {
    #[inline(always)]
    fn default() -> Cndtr1 {
        Cndtr1(0)
    }
}
impl core::fmt::Debug for Cndtr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr1").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr1 {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[doc = "CNDTR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr2(pub u32);
impl Cndtr2 {
    #[doc = "Number of data to transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data to transfer"]
    #[inline(always)]
    pub const fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr2 {
    #[inline(always)]
    fn default() -> Cndtr2 {
        Cndtr2(0)
    }
}
impl core::fmt::Debug for Cndtr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr2").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr2 {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[doc = "CNDTR3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr3(pub u32);
impl Cndtr3 {
    #[doc = "Number of data to transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data to transfer"]
    #[inline(always)]
    pub const fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr3 {
    #[inline(always)]
    fn default() -> Cndtr3 {
        Cndtr3(0)
    }
}
impl core::fmt::Debug for Cndtr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr3").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr3 {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[doc = "CNDTR4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr4(pub u32);
impl Cndtr4 {
    #[doc = "Number of data to transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data to transfer"]
    #[inline(always)]
    pub const fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr4 {
    #[inline(always)]
    fn default() -> Cndtr4 {
        Cndtr4(0)
    }
}
impl core::fmt::Debug for Cndtr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr4").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr4 {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[doc = "CNDTR5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cndtr5(pub u32);
impl Cndtr5 {
    #[doc = "Number of data to transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ndt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data to transfer"]
    #[inline(always)]
    pub const fn set_ndt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cndtr5 {
    #[inline(always)]
    fn default() -> Cndtr5 {
        Cndtr5(0)
    }
}
impl core::fmt::Debug for Cndtr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cndtr5").field("ndt", &self.ndt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cndtr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cndtr5 {{ ndt: {=u16:?} }}", self.ndt())
    }
}
#[doc = "CPAR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar1(pub u32);
impl Cpar1 {
    #[doc = "Peripheral address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar1 {
    #[inline(always)]
    fn default() -> Cpar1 {
        Cpar1(0)
    }
}
impl core::fmt::Debug for Cpar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar1").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar1 {{ pa: {=u32:?} }}", self.pa())
    }
}
#[doc = "CPAR2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar2(pub u32);
impl Cpar2 {
    #[doc = "Peripheral address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar2 {
    #[inline(always)]
    fn default() -> Cpar2 {
        Cpar2(0)
    }
}
impl core::fmt::Debug for Cpar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar2").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar2 {{ pa: {=u32:?} }}", self.pa())
    }
}
#[doc = "CPAR3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar3(pub u32);
impl Cpar3 {
    #[doc = "Peripheral address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar3 {
    #[inline(always)]
    fn default() -> Cpar3 {
        Cpar3(0)
    }
}
impl core::fmt::Debug for Cpar3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar3").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar3 {{ pa: {=u32:?} }}", self.pa())
    }
}
#[doc = "CPAR4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar4(pub u32);
impl Cpar4 {
    #[doc = "Peripheral address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar4 {
    #[inline(always)]
    fn default() -> Cpar4 {
        Cpar4(0)
    }
}
impl core::fmt::Debug for Cpar4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar4").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar4 {{ pa: {=u32:?} }}", self.pa())
    }
}
#[doc = "CPAR5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpar5(pub u32);
impl Cpar5 {
    #[doc = "Peripheral address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Peripheral address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cpar5 {
    #[inline(always)]
    fn default() -> Cpar5 {
        Cpar5(0)
    }
}
impl core::fmt::Debug for Cpar5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpar5").field("pa", &self.pa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpar5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpar5 {{ pa: {=u32:?} }}", self.pa())
    }
}
#[doc = "IFCR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifcr(pub u32);
impl Ifcr {
    #[doc = "channel 1 global interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cgif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 global interrupt clear"]
    #[inline(always)]
    pub const fn set_cgif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "channel 1 transfer complete clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 transfer complete clear"]
    #[inline(always)]
    pub const fn set_ctcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "channel 1 half transfer clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chtif1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 half transfer clear"]
    #[inline(always)]
    pub const fn set_chtif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "channel 1 transfer error clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 transfer error clear"]
    #[inline(always)]
    pub const fn set_cteif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "channel 2 global interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cgif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 global interrupt clear"]
    #[inline(always)]
    pub const fn set_cgif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "channel 2 transfer complete clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 transfer complete clear"]
    #[inline(always)]
    pub const fn set_ctcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "channel 2 half transfer clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chtif2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 half transfer clear"]
    #[inline(always)]
    pub const fn set_chtif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "channel 2 transfer error clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 transfer error clear"]
    #[inline(always)]
    pub const fn set_cteif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "channel 3 global interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cgif3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 global interrupt clear"]
    #[inline(always)]
    pub const fn set_cgif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "channel 3 transfer complete clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 transfer complete clear"]
    #[inline(always)]
    pub const fn set_ctcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "channel 3 half transfer clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chtif3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 half transfer clear"]
    #[inline(always)]
    pub const fn set_chtif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "channel 3 transfer error clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 transfer error clear"]
    #[inline(always)]
    pub const fn set_cteif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "channel 4 global interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cgif4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 global interrupt clear"]
    #[inline(always)]
    pub const fn set_cgif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "channel 4 transfer complete clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif4(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 transfer complete clear"]
    #[inline(always)]
    pub const fn set_ctcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "channel 4 half transfer clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chtif4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 half transfer clear"]
    #[inline(always)]
    pub const fn set_chtif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "channel 4 transfer error clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 transfer error clear"]
    #[inline(always)]
    pub const fn set_cteif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "channel 5 global interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cgif5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 global interrupt clear"]
    #[inline(always)]
    pub const fn set_cgif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "channel 5 transfer complete clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ctcif5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 transfer complete clear"]
    #[inline(always)]
    pub const fn set_ctcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "channel 5 half transfer clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chtif5(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 half transfer clear"]
    #[inline(always)]
    pub const fn set_chtif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "channel 5 transfer error clear"]
    #[must_use]
    #[inline(always)]
    pub const fn cteif5(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 transfer error clear"]
    #[inline(always)]
    pub const fn set_cteif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Ifcr {
    #[inline(always)]
    fn default() -> Ifcr {
        Ifcr(0)
    }
}
impl core::fmt::Debug for Ifcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifcr")
            .field("cgif1", &self.cgif1())
            .field("ctcif1", &self.ctcif1())
            .field("chtif1", &self.chtif1())
            .field("cteif1", &self.cteif1())
            .field("cgif2", &self.cgif2())
            .field("ctcif2", &self.ctcif2())
            .field("chtif2", &self.chtif2())
            .field("cteif2", &self.cteif2())
            .field("cgif3", &self.cgif3())
            .field("ctcif3", &self.ctcif3())
            .field("chtif3", &self.chtif3())
            .field("cteif3", &self.cteif3())
            .field("cgif4", &self.cgif4())
            .field("ctcif4", &self.ctcif4())
            .field("chtif4", &self.chtif4())
            .field("cteif4", &self.cteif4())
            .field("cgif5", &self.cgif5())
            .field("ctcif5", &self.ctcif5())
            .field("chtif5", &self.chtif5())
            .field("cteif5", &self.cteif5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ifcr {{ cgif1: {=bool:?}, ctcif1: {=bool:?}, chtif1: {=bool:?}, cteif1: {=bool:?}, cgif2: {=bool:?}, ctcif2: {=bool:?}, chtif2: {=bool:?}, cteif2: {=bool:?}, cgif3: {=bool:?}, ctcif3: {=bool:?}, chtif3: {=bool:?}, cteif3: {=bool:?}, cgif4: {=bool:?}, ctcif4: {=bool:?}, chtif4: {=bool:?}, cteif4: {=bool:?}, cgif5: {=bool:?}, ctcif5: {=bool:?}, chtif5: {=bool:?}, cteif5: {=bool:?} }}",
            self.cgif1(),
            self.ctcif1(),
            self.chtif1(),
            self.cteif1(),
            self.cgif2(),
            self.ctcif2(),
            self.chtif2(),
            self.cteif2(),
            self.cgif3(),
            self.ctcif3(),
            self.chtif3(),
            self.cteif3(),
            self.cgif4(),
            self.ctcif4(),
            self.chtif4(),
            self.cteif4(),
            self.cgif5(),
            self.ctcif5(),
            self.chtif5(),
            self.cteif5()
        )
    }
}
#[doc = "ISR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "channel 1 global interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gif1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 global interrupt flag"]
    #[inline(always)]
    pub const fn set_gif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "channel 1 transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "channel 1 half transfer flag"]
    #[must_use]
    #[inline(always)]
    pub const fn htif1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 half transfer flag"]
    #[inline(always)]
    pub const fn set_htif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "channel 1 transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "channel 1 transfer error flag"]
    #[inline(always)]
    pub const fn set_teif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "channel 2 global interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 global interrupt flag"]
    #[inline(always)]
    pub const fn set_gif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "channel 2 transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "channel 2 half transfer flag"]
    #[must_use]
    #[inline(always)]
    pub const fn htif2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 half transfer flag"]
    #[inline(always)]
    pub const fn set_htif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "channel 2 transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teif2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "channel 2 transfer error flag"]
    #[inline(always)]
    pub const fn set_teif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "channel 3 global interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gif3(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 global interrupt flag"]
    #[inline(always)]
    pub const fn set_gif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "channel 3 transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "channel 3 half transfer flag"]
    #[must_use]
    #[inline(always)]
    pub const fn htif3(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 half transfer flag"]
    #[inline(always)]
    pub const fn set_htif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "channel 3 transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teif3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "channel 3 transfer error flag"]
    #[inline(always)]
    pub const fn set_teif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "channel 4 global interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gif4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 global interrupt flag"]
    #[inline(always)]
    pub const fn set_gif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "channel 4 transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif4(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "channel 4 half transfer flag"]
    #[must_use]
    #[inline(always)]
    pub const fn htif4(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 half transfer flag"]
    #[inline(always)]
    pub const fn set_htif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "channel 4 transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teif4(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "channel 4 transfer error flag"]
    #[inline(always)]
    pub const fn set_teif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "channel 5 global interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gif5(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 global interrupt flag"]
    #[inline(always)]
    pub const fn set_gif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "channel 5 transfer complete flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcif5(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 transfer complete flag"]
    #[inline(always)]
    pub const fn set_tcif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "channel 5 half transfer flag"]
    #[must_use]
    #[inline(always)]
    pub const fn htif5(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 half transfer flag"]
    #[inline(always)]
    pub const fn set_htif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "channel 5 transfer error flag"]
    #[must_use]
    #[inline(always)]
    pub const fn teif5(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "channel 5 transfer error flag"]
    #[inline(always)]
    pub const fn set_teif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
            .field("gif1", &self.gif1())
            .field("tcif1", &self.tcif1())
            .field("htif1", &self.htif1())
            .field("teif1", &self.teif1())
            .field("gif2", &self.gif2())
            .field("tcif2", &self.tcif2())
            .field("htif2", &self.htif2())
            .field("teif2", &self.teif2())
            .field("gif3", &self.gif3())
            .field("tcif3", &self.tcif3())
            .field("htif3", &self.htif3())
            .field("teif3", &self.teif3())
            .field("gif4", &self.gif4())
            .field("tcif4", &self.tcif4())
            .field("htif4", &self.htif4())
            .field("teif4", &self.teif4())
            .field("gif5", &self.gif5())
            .field("tcif5", &self.tcif5())
            .field("htif5", &self.htif5())
            .field("teif5", &self.teif5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ gif1: {=bool:?}, tcif1: {=bool:?}, htif1: {=bool:?}, teif1: {=bool:?}, gif2: {=bool:?}, tcif2: {=bool:?}, htif2: {=bool:?}, teif2: {=bool:?}, gif3: {=bool:?}, tcif3: {=bool:?}, htif3: {=bool:?}, teif3: {=bool:?}, gif4: {=bool:?}, tcif4: {=bool:?}, htif4: {=bool:?}, teif4: {=bool:?}, gif5: {=bool:?}, tcif5: {=bool:?}, htif5: {=bool:?}, teif5: {=bool:?} }}",
            self.gif1(),
            self.tcif1(),
            self.htif1(),
            self.teif1(),
            self.gif2(),
            self.tcif2(),
            self.htif2(),
            self.teif2(),
            self.gif3(),
            self.tcif3(),
            self.htif3(),
            self.teif3(),
            self.gif4(),
            self.tcif4(),
            self.htif4(),
            self.teif4(),
            self.gif5(),
            self.tcif5(),
            self.htif5(),
            self.teif5()
        )
    }
}
