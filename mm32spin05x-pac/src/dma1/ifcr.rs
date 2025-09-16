#[doc = "Register `IFCR` reader"]
pub type R = crate::R<IfcrSpec>;
#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `CGIF1` reader - channel 1 global interrupt clear"]
pub type Cgif1R = crate::BitReader;
#[doc = "Field `CGIF1` writer - channel 1 global interrupt clear"]
pub type Cgif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` reader - channel 1 transfer complete clear"]
pub type Ctcif1R = crate::BitReader;
#[doc = "Field `CTCIF1` writer - channel 1 transfer complete clear"]
pub type Ctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` reader - channel 1 half transfer clear"]
pub type Chtif1R = crate::BitReader;
#[doc = "Field `CHTIF1` writer - channel 1 half transfer clear"]
pub type Chtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` reader - channel 1 transfer error clear"]
pub type Cteif1R = crate::BitReader;
#[doc = "Field `CTEIF1` writer - channel 1 transfer error clear"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF2` reader - channel 2 global interrupt clear"]
pub type Cgif2R = crate::BitReader;
#[doc = "Field `CGIF2` writer - channel 2 global interrupt clear"]
pub type Cgif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` reader - channel 2 transfer complete clear"]
pub type Ctcif2R = crate::BitReader;
#[doc = "Field `CTCIF2` writer - channel 2 transfer complete clear"]
pub type Ctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` reader - channel 2 half transfer clear"]
pub type Chtif2R = crate::BitReader;
#[doc = "Field `CHTIF2` writer - channel 2 half transfer clear"]
pub type Chtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` reader - channel 2 transfer error clear"]
pub type Cteif2R = crate::BitReader;
#[doc = "Field `CTEIF2` writer - channel 2 transfer error clear"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF3` reader - channel 3 global interrupt clear"]
pub type Cgif3R = crate::BitReader;
#[doc = "Field `CGIF3` writer - channel 3 global interrupt clear"]
pub type Cgif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` reader - channel 3 transfer complete clear"]
pub type Ctcif3R = crate::BitReader;
#[doc = "Field `CTCIF3` writer - channel 3 transfer complete clear"]
pub type Ctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` reader - channel 3 half transfer clear"]
pub type Chtif3R = crate::BitReader;
#[doc = "Field `CHTIF3` writer - channel 3 half transfer clear"]
pub type Chtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` reader - channel 3 transfer error clear"]
pub type Cteif3R = crate::BitReader;
#[doc = "Field `CTEIF3` writer - channel 3 transfer error clear"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF4` reader - channel 4 global interrupt clear"]
pub type Cgif4R = crate::BitReader;
#[doc = "Field `CGIF4` writer - channel 4 global interrupt clear"]
pub type Cgif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` reader - channel 4 transfer complete clear"]
pub type Ctcif4R = crate::BitReader;
#[doc = "Field `CTCIF4` writer - channel 4 transfer complete clear"]
pub type Ctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` reader - channel 4 half transfer clear"]
pub type Chtif4R = crate::BitReader;
#[doc = "Field `CHTIF4` writer - channel 4 half transfer clear"]
pub type Chtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` reader - channel 4 transfer error clear"]
pub type Cteif4R = crate::BitReader;
#[doc = "Field `CTEIF4` writer - channel 4 transfer error clear"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGIF5` reader - channel 5 global interrupt clear"]
pub type Cgif5R = crate::BitReader;
#[doc = "Field `CGIF5` writer - channel 5 global interrupt clear"]
pub type Cgif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` reader - channel 5 transfer complete clear"]
pub type Ctcif5R = crate::BitReader;
#[doc = "Field `CTCIF5` writer - channel 5 transfer complete clear"]
pub type Ctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` reader - channel 5 half transfer clear"]
pub type Chtif5R = crate::BitReader;
#[doc = "Field `CHTIF5` writer - channel 5 half transfer clear"]
pub type Chtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` reader - channel 5 transfer error clear"]
pub type Cteif5R = crate::BitReader;
#[doc = "Field `CTEIF5` writer - channel 5 transfer error clear"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - channel 1 global interrupt clear"]
    #[inline(always)]
    pub fn cgif1(&self) -> Cgif1R {
        Cgif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - channel 1 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif1(&self) -> Ctcif1R {
        Ctcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - channel 1 half transfer clear"]
    #[inline(always)]
    pub fn chtif1(&self) -> Chtif1R {
        Chtif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - channel 1 transfer error clear"]
    #[inline(always)]
    pub fn cteif1(&self) -> Cteif1R {
        Cteif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel 2 global interrupt clear"]
    #[inline(always)]
    pub fn cgif2(&self) -> Cgif2R {
        Cgif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - channel 2 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif2(&self) -> Ctcif2R {
        Ctcif2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - channel 2 half transfer clear"]
    #[inline(always)]
    pub fn chtif2(&self) -> Chtif2R {
        Chtif2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - channel 2 transfer error clear"]
    #[inline(always)]
    pub fn cteif2(&self) -> Cteif2R {
        Cteif2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - channel 3 global interrupt clear"]
    #[inline(always)]
    pub fn cgif3(&self) -> Cgif3R {
        Cgif3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - channel 3 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif3(&self) -> Ctcif3R {
        Ctcif3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - channel 3 half transfer clear"]
    #[inline(always)]
    pub fn chtif3(&self) -> Chtif3R {
        Chtif3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - channel 3 transfer error clear"]
    #[inline(always)]
    pub fn cteif3(&self) -> Cteif3R {
        Cteif3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - channel 4 global interrupt clear"]
    #[inline(always)]
    pub fn cgif4(&self) -> Cgif4R {
        Cgif4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - channel 4 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif4(&self) -> Ctcif4R {
        Ctcif4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - channel 4 half transfer clear"]
    #[inline(always)]
    pub fn chtif4(&self) -> Chtif4R {
        Chtif4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - channel 4 transfer error clear"]
    #[inline(always)]
    pub fn cteif4(&self) -> Cteif4R {
        Cteif4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - channel 5 global interrupt clear"]
    #[inline(always)]
    pub fn cgif5(&self) -> Cgif5R {
        Cgif5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - channel 5 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif5(&self) -> Ctcif5R {
        Ctcif5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - channel 5 half transfer clear"]
    #[inline(always)]
    pub fn chtif5(&self) -> Chtif5R {
        Chtif5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - channel 5 transfer error clear"]
    #[inline(always)]
    pub fn cteif5(&self) -> Cteif5R {
        Cteif5R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel 1 global interrupt clear"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> Cgif1W<'_, IfcrSpec> {
        Cgif1W::new(self, 0)
    }
    #[doc = "Bit 1 - channel 1 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> Ctcif1W<'_, IfcrSpec> {
        Ctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - channel 1 half transfer clear"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> Chtif1W<'_, IfcrSpec> {
        Chtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - channel 1 transfer error clear"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<'_, IfcrSpec> {
        Cteif1W::new(self, 3)
    }
    #[doc = "Bit 4 - channel 2 global interrupt clear"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> Cgif2W<'_, IfcrSpec> {
        Cgif2W::new(self, 4)
    }
    #[doc = "Bit 5 - channel 2 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> Ctcif2W<'_, IfcrSpec> {
        Ctcif2W::new(self, 5)
    }
    #[doc = "Bit 6 - channel 2 half transfer clear"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> Chtif2W<'_, IfcrSpec> {
        Chtif2W::new(self, 6)
    }
    #[doc = "Bit 7 - channel 2 transfer error clear"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<'_, IfcrSpec> {
        Cteif2W::new(self, 7)
    }
    #[doc = "Bit 8 - channel 3 global interrupt clear"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> Cgif3W<'_, IfcrSpec> {
        Cgif3W::new(self, 8)
    }
    #[doc = "Bit 9 - channel 3 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> Ctcif3W<'_, IfcrSpec> {
        Ctcif3W::new(self, 9)
    }
    #[doc = "Bit 10 - channel 3 half transfer clear"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> Chtif3W<'_, IfcrSpec> {
        Chtif3W::new(self, 10)
    }
    #[doc = "Bit 11 - channel 3 transfer error clear"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<'_, IfcrSpec> {
        Cteif3W::new(self, 11)
    }
    #[doc = "Bit 12 - channel 4 global interrupt clear"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> Cgif4W<'_, IfcrSpec> {
        Cgif4W::new(self, 12)
    }
    #[doc = "Bit 13 - channel 4 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> Ctcif4W<'_, IfcrSpec> {
        Ctcif4W::new(self, 13)
    }
    #[doc = "Bit 14 - channel 4 half transfer clear"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> Chtif4W<'_, IfcrSpec> {
        Chtif4W::new(self, 14)
    }
    #[doc = "Bit 15 - channel 4 transfer error clear"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<'_, IfcrSpec> {
        Cteif4W::new(self, 15)
    }
    #[doc = "Bit 16 - channel 5 global interrupt clear"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> Cgif5W<'_, IfcrSpec> {
        Cgif5W::new(self, 16)
    }
    #[doc = "Bit 17 - channel 5 transfer complete clear"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> Ctcif5W<'_, IfcrSpec> {
        Ctcif5W::new(self, 17)
    }
    #[doc = "Bit 18 - channel 5 half transfer clear"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> Chtif5W<'_, IfcrSpec> {
        Chtif5W::new(self, 18)
    }
    #[doc = "Bit 19 - channel 5 transfer error clear"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<'_, IfcrSpec> {
        Cteif5W::new(self, 19)
    }
}
#[doc = "IFCR\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifcr::R`](R) reader structure"]
impl crate::Readable for IfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}
