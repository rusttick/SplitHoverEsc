#[doc = "Register `TXID1_P` reader"]
pub type R = crate::R<Txid1PSpec>;
#[doc = "Register `TXID1_P` writer"]
pub type W = crate::W<Txid1PSpec>;
#[doc = "Field `ID13` reader - Identifier bit 13"]
pub type Id13R = crate::BitReader;
#[doc = "Field `ID13` writer - Identifier bit 13"]
pub type Id13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID14` reader - Identifier bit 14"]
pub type Id14R = crate::BitReader;
#[doc = "Field `ID14` writer - Identifier bit 14"]
pub type Id14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID15` reader - Identifier bit 15"]
pub type Id15R = crate::BitReader;
#[doc = "Field `ID15` writer - Identifier bit 15"]
pub type Id15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID16` reader - Identifier bit 16"]
pub type Id16R = crate::BitReader;
#[doc = "Field `ID16` writer - Identifier bit 16"]
pub type Id16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID17` reader - Identifier bit 17"]
pub type Id17R = crate::BitReader;
#[doc = "Field `ID17` writer - Identifier bit 17"]
pub type Id17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID18` reader - Identifier bit 18"]
pub type Id18R = crate::BitReader;
#[doc = "Field `ID18` writer - Identifier bit 18"]
pub type Id18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID19` reader - Identifier bit 19"]
pub type Id19R = crate::BitReader;
#[doc = "Field `ID19` writer - Identifier bit 19"]
pub type Id19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID20` reader - Identifier bit 20"]
pub type Id20R = crate::BitReader;
#[doc = "Field `ID20` writer - Identifier bit 20"]
pub type Id20W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    pub fn id13(&self) -> Id13R {
        Id13R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    pub fn id14(&self) -> Id14R {
        Id14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    pub fn id15(&self) -> Id15R {
        Id15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    pub fn id16(&self) -> Id16R {
        Id16R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    pub fn id17(&self) -> Id17R {
        Id17R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    pub fn id18(&self) -> Id18R {
        Id18R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    pub fn id19(&self) -> Id19R {
        Id19R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    pub fn id20(&self) -> Id20R {
        Id20R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 13"]
    #[inline(always)]
    pub fn id13(&mut self) -> Id13W<'_, Txid1PSpec> {
        Id13W::new(self, 0)
    }
    #[doc = "Bit 1 - Identifier bit 14"]
    #[inline(always)]
    pub fn id14(&mut self) -> Id14W<'_, Txid1PSpec> {
        Id14W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier bit 15"]
    #[inline(always)]
    pub fn id15(&mut self) -> Id15W<'_, Txid1PSpec> {
        Id15W::new(self, 2)
    }
    #[doc = "Bit 3 - Identifier bit 16"]
    #[inline(always)]
    pub fn id16(&mut self) -> Id16W<'_, Txid1PSpec> {
        Id16W::new(self, 3)
    }
    #[doc = "Bit 4 - Identifier bit 17"]
    #[inline(always)]
    pub fn id17(&mut self) -> Id17W<'_, Txid1PSpec> {
        Id17W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 18"]
    #[inline(always)]
    pub fn id18(&mut self) -> Id18W<'_, Txid1PSpec> {
        Id18W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 19"]
    #[inline(always)]
    pub fn id19(&mut self) -> Id19W<'_, Txid1PSpec> {
        Id19W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 20"]
    #[inline(always)]
    pub fn id20(&mut self) -> Id20W<'_, Txid1PSpec> {
        Id20W::new(self, 7)
    }
}
#[doc = "Peli TX ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txid1_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid1_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txid1PSpec;
impl crate::RegisterSpec for Txid1PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid1_p::R`](R) reader structure"]
impl crate::Readable for Txid1PSpec {}
#[doc = "`write(|w| ..)` method takes [`txid1_p::W`](W) writer structure"]
impl crate::Writable for Txid1PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXID1_P to value 0"]
impl crate::Resettable for Txid1PSpec {}
