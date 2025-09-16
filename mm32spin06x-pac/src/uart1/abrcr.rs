#[doc = "Register `ABRCR` reader"]
pub type R = crate::R<AbrcrSpec>;
#[doc = "Register `ABRCR` writer"]
pub type W = crate::W<AbrcrSpec>;
#[doc = "Field `Abren` reader - Automatic baud rate enable"]
pub type AbrenR = crate::BitReader;
#[doc = "Field `Abren` writer - Automatic baud rate enable"]
pub type AbrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Abr_bitcnt` reader - Automatic baud rate detection length"]
pub type AbrBitcntR = crate::FieldReader;
#[doc = "Field `Abr_bitcnt` writer - Automatic baud rate detection length"]
pub type AbrBitcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Former_edge` reader - Auto baud rate previous edge selection"]
pub type FormerEdgeR = crate::BitReader;
#[doc = "Field `Former_edge` writer - Auto baud rate previous edge selection"]
pub type FormerEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Later_edge` reader - Automatic baud rate after edge selection"]
pub type LaterEdgeR = crate::BitReader;
#[doc = "Field `Later_edge` writer - Automatic baud rate after edge selection"]
pub type LaterEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> AbrenR {
        AbrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    pub fn abr_bitcnt(&self) -> AbrBitcntR {
        AbrBitcntR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    pub fn former_edge(&self) -> FormerEdgeR {
        FormerEdgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    pub fn later_edge(&self) -> LaterEdgeR {
        LaterEdgeR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic baud rate enable"]
    #[inline(always)]
    pub fn abren(&mut self) -> AbrenW<'_, AbrcrSpec> {
        AbrenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Automatic baud rate detection length"]
    #[inline(always)]
    pub fn abr_bitcnt(&mut self) -> AbrBitcntW<'_, AbrcrSpec> {
        AbrBitcntW::new(self, 1)
    }
    #[doc = "Bit 3 - Auto baud rate previous edge selection"]
    #[inline(always)]
    pub fn former_edge(&mut self) -> FormerEdgeW<'_, AbrcrSpec> {
        FormerEdgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic baud rate after edge selection"]
    #[inline(always)]
    pub fn later_edge(&mut self) -> LaterEdgeW<'_, AbrcrSpec> {
        LaterEdgeW::new(self, 4)
    }
}
#[doc = "Automatic Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`abrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AbrcrSpec;
impl crate::RegisterSpec for AbrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abrcr::R`](R) reader structure"]
impl crate::Readable for AbrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`abrcr::W`](W) writer structure"]
impl crate::Writable for AbrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ABRCR to value 0"]
impl crate::Resettable for AbrcrSpec {}
