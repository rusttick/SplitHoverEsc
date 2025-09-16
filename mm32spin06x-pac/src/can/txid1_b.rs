#[doc = "Register `TXID1_B` reader"]
pub type R = crate::R<Txid1BSpec>;
#[doc = "Register `TXID1_B` writer"]
pub type W = crate::W<Txid1BSpec>;
#[doc = "Field `DLC` reader - Data length code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID0` reader - Identifier bit 0"]
pub type Id0R = crate::BitReader;
#[doc = "Field `ID0` writer - Identifier bit 0"]
pub type Id0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID1` reader - Identifier bit 1"]
pub type Id1R = crate::BitReader;
#[doc = "Field `ID1` writer - Identifier bit 1"]
pub type Id1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID2` reader - Identifier bit 2"]
pub type Id2R = crate::BitReader;
#[doc = "Field `ID2` writer - Identifier bit 2"]
pub type Id2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    pub fn id0(&self) -> Id0R {
        Id0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    pub fn id1(&self) -> Id1R {
        Id1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    pub fn id2(&self) -> Id2R {
        Id2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, Txid1BSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 4 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<'_, Txid1BSpec> {
        RtrW::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 0"]
    #[inline(always)]
    pub fn id0(&mut self) -> Id0W<'_, Txid1BSpec> {
        Id0W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 1"]
    #[inline(always)]
    pub fn id1(&mut self) -> Id1W<'_, Txid1BSpec> {
        Id1W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 2"]
    #[inline(always)]
    pub fn id2(&mut self) -> Id2W<'_, Txid1BSpec> {
        Id2W::new(self, 7)
    }
}
#[doc = "Basic TX ID register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`txid1_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid1_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txid1BSpec;
impl crate::RegisterSpec for Txid1BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid1_b::R`](R) reader structure"]
impl crate::Readable for Txid1BSpec {}
#[doc = "`write(|w| ..)` method takes [`txid1_b::W`](W) writer structure"]
impl crate::Writable for Txid1BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXID1_B to value 0"]
impl crate::Resettable for Txid1BSpec {}
