#[doc = "Register `MOD_P` reader"]
pub type R = crate::R<ModPSpec>;
#[doc = "Register `MOD_P` writer"]
pub type W = crate::W<ModPSpec>;
#[doc = "Field `RM` reader - Reset mode"]
pub type RmR = crate::BitReader;
#[doc = "Field `RM` writer - Reset mode"]
pub type RmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOM` reader - Listen only mode"]
pub type LomR = crate::BitReader;
#[doc = "Field `LOM` writer - Listen only mode"]
pub type LomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STM` reader - Self test mode"]
pub type StmR = crate::BitReader;
#[doc = "Field `STM` writer - Self test mode"]
pub type StmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFM` reader - Acceptance filter mode"]
pub type AfmR = crate::BitReader;
#[doc = "Field `AFM` writer - Acceptance filter mode"]
pub type AfmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    pub fn lom(&self) -> LomR {
        LomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm(&self) -> AfmR {
        AfmR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset mode"]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<'_, ModPSpec> {
        RmW::new(self, 0)
    }
    #[doc = "Bit 1 - Listen only mode"]
    #[inline(always)]
    pub fn lom(&mut self) -> LomW<'_, ModPSpec> {
        LomW::new(self, 1)
    }
    #[doc = "Bit 2 - Self test mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> StmW<'_, ModPSpec> {
        StmW::new(self, 2)
    }
    #[doc = "Bit 3 - Acceptance filter mode"]
    #[inline(always)]
    pub fn afm(&mut self) -> AfmW<'_, ModPSpec> {
        AfmW::new(self, 3)
    }
}
#[doc = "Peli Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModPSpec;
impl crate::RegisterSpec for ModPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_p::R`](R) reader structure"]
impl crate::Readable for ModPSpec {}
#[doc = "`write(|w| ..)` method takes [`mod_p::W`](W) writer structure"]
impl crate::Writable for ModPSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD_P to value 0x01"]
impl crate::Resettable for ModPSpec {
    const RESET_VALUE: u32 = 0x01;
}
