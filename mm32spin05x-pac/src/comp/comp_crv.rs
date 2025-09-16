#[doc = "Register `COMP_CRV` reader"]
pub type R = crate::R<CompCrvSpec>;
#[doc = "Register `COMP_CRV` writer"]
pub type W = crate::W<CompCrvSpec>;
#[doc = "Field `CRV_SEL` reader - Comparator external referencevoltage select"]
pub type CrvSelR = crate::FieldReader;
#[doc = "Field `CRV_SEL` writer - Comparator external referencevoltage select"]
pub type CrvSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN` reader - Comparator external referencevoltage enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator external referencevoltage enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRV_SRC` reader - Comparator external refer-ence voltage source select"]
pub type CrvSrcR = crate::BitReader;
#[doc = "Field `CRV_SRC` writer - Comparator external refer-ence voltage source select"]
pub type CrvSrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Comparator external referencevoltage select"]
    #[inline(always)]
    pub fn crv_sel(&self) -> CrvSelR {
        CrvSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Comparator external referencevoltage enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator external refer-ence voltage source select"]
    #[inline(always)]
    pub fn crv_src(&self) -> CrvSrcR {
        CrvSrcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator external referencevoltage select"]
    #[inline(always)]
    pub fn crv_sel(&mut self) -> CrvSelW<'_, CompCrvSpec> {
        CrvSelW::new(self, 0)
    }
    #[doc = "Bit 4 - Comparator external referencevoltage enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CompCrvSpec> {
        EnW::new(self, 4)
    }
    #[doc = "Bit 5 - Comparator external refer-ence voltage source select"]
    #[inline(always)]
    pub fn crv_src(&mut self) -> CrvSrcW<'_, CompCrvSpec> {
        CrvSrcW::new(self, 5)
    }
}
#[doc = "COMP_CRV\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_crv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_crv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompCrvSpec;
impl crate::RegisterSpec for CompCrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_crv::R`](R) reader structure"]
impl crate::Readable for CompCrvSpec {}
#[doc = "`write(|w| ..)` method takes [`comp_crv::W`](W) writer structure"]
impl crate::Writable for CompCrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP_CRV to value 0"]
impl crate::Resettable for CompCrvSpec {}
