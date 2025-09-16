#[doc = "Register `COMP_CRV` reader"]
pub type R = crate::R<CompCrvSpec>;
#[doc = "Register `COMP_CRV` writer"]
pub type W = crate::W<CompCrvSpec>;
#[doc = "Field `CRV_SEL` reader - Comparator external reference voltage select"]
pub type CrvSelR = crate::FieldReader;
#[doc = "Field `CRV_SEL` writer - Comparator external reference voltage select"]
pub type CrvSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRV_EN` reader - Comparator external reference voltage enable"]
pub type CrvEnR = crate::BitReader;
#[doc = "Field `CRV_EN` writer - Comparator external reference voltage enable"]
pub type CrvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRV_SRC` reader - Comparator external reference voltage source select"]
pub type CrvSrcR = crate::BitReader;
#[doc = "Field `CRV_SRC` writer - Comparator external reference voltage source select"]
pub type CrvSrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    pub fn crv_sel(&self) -> CrvSelR {
        CrvSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    pub fn crv_en(&self) -> CrvEnR {
        CrvEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    pub fn crv_src(&self) -> CrvSrcR {
        CrvSrcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Comparator external reference voltage select"]
    #[inline(always)]
    pub fn crv_sel(&mut self) -> CrvSelW<'_, CompCrvSpec> {
        CrvSelW::new(self, 0)
    }
    #[doc = "Bit 4 - Comparator external reference voltage enable"]
    #[inline(always)]
    pub fn crv_en(&mut self) -> CrvEnW<'_, CompCrvSpec> {
        CrvEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Comparator external reference voltage source select"]
    #[inline(always)]
    pub fn crv_src(&mut self) -> CrvSrcW<'_, CompCrvSpec> {
        CrvSrcW::new(self, 5)
    }
}
#[doc = "COMP Extern Reference Voltage\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_crv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_crv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
