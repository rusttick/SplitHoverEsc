#[doc = "Register `TXID0_P` reader"]
pub type R = crate::R<Txid0PSpec>;
#[doc = "Register `TXID0_P` writer"]
pub type W = crate::W<Txid0PSpec>;
#[doc = "Field `ID21` reader - Identifier bit 21"]
pub type Id21R = crate::BitReader;
#[doc = "Field `ID21` writer - Identifier bit 21"]
pub type Id21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID22` reader - Identifier bit 22"]
pub type Id22R = crate::BitReader;
#[doc = "Field `ID22` writer - Identifier bit 22"]
pub type Id22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID23` reader - Identifier bit 23"]
pub type Id23R = crate::BitReader;
#[doc = "Field `ID23` writer - Identifier bit 23"]
pub type Id23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID24` reader - Identifier bit 24"]
pub type Id24R = crate::BitReader;
#[doc = "Field `ID24` writer - Identifier bit 24"]
pub type Id24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID25` reader - Identifier bit 25"]
pub type Id25R = crate::BitReader;
#[doc = "Field `ID25` writer - Identifier bit 25"]
pub type Id25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID26` reader - Identifier bit 26"]
pub type Id26R = crate::BitReader;
#[doc = "Field `ID26` writer - Identifier bit 26"]
pub type Id26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID27` reader - Identifier bit 27"]
pub type Id27R = crate::BitReader;
#[doc = "Field `ID27` writer - Identifier bit 27"]
pub type Id27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID28` reader - Identifier bit 28"]
pub type Id28R = crate::BitReader;
#[doc = "Field `ID28` writer - Identifier bit 28"]
pub type Id28W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Identifier bit 21"]
    #[inline(always)]
    pub fn id21(&self) -> Id21R {
        Id21R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 22"]
    #[inline(always)]
    pub fn id22(&self) -> Id22R {
        Id22R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 23"]
    #[inline(always)]
    pub fn id23(&self) -> Id23R {
        Id23R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 24"]
    #[inline(always)]
    pub fn id24(&self) -> Id24R {
        Id24R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 25"]
    #[inline(always)]
    pub fn id25(&self) -> Id25R {
        Id25R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 26"]
    #[inline(always)]
    pub fn id26(&self) -> Id26R {
        Id26R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 27"]
    #[inline(always)]
    pub fn id27(&self) -> Id27R {
        Id27R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 28"]
    #[inline(always)]
    pub fn id28(&self) -> Id28R {
        Id28R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 21"]
    #[inline(always)]
    pub fn id21(&mut self) -> Id21W<'_, Txid0PSpec> {
        Id21W::new(self, 0)
    }
    #[doc = "Bit 1 - Identifier bit 22"]
    #[inline(always)]
    pub fn id22(&mut self) -> Id22W<'_, Txid0PSpec> {
        Id22W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier bit 23"]
    #[inline(always)]
    pub fn id23(&mut self) -> Id23W<'_, Txid0PSpec> {
        Id23W::new(self, 2)
    }
    #[doc = "Bit 3 - Identifier bit 24"]
    #[inline(always)]
    pub fn id24(&mut self) -> Id24W<'_, Txid0PSpec> {
        Id24W::new(self, 3)
    }
    #[doc = "Bit 4 - Identifier bit 25"]
    #[inline(always)]
    pub fn id25(&mut self) -> Id25W<'_, Txid0PSpec> {
        Id25W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 26"]
    #[inline(always)]
    pub fn id26(&mut self) -> Id26W<'_, Txid0PSpec> {
        Id26W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 27"]
    #[inline(always)]
    pub fn id27(&mut self) -> Id27W<'_, Txid0PSpec> {
        Id27W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 28"]
    #[inline(always)]
    pub fn id28(&mut self) -> Id28W<'_, Txid0PSpec> {
        Id28W::new(self, 7)
    }
}
#[doc = "Peli TX ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txid0_p::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid0_p::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txid0PSpec;
impl crate::RegisterSpec for Txid0PSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid0_p::R`](R) reader structure"]
impl crate::Readable for Txid0PSpec {}
#[doc = "`write(|w| ..)` method takes [`txid0_p::W`](W) writer structure"]
impl crate::Writable for Txid0PSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXID0_P to value 0"]
impl crate::Resettable for Txid0PSpec {}
