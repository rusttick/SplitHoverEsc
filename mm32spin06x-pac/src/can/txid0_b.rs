#[doc = "Register `TXID0_B` reader"]
pub type R = crate::R<Txid0BSpec>;
#[doc = "Register `TXID0_B` writer"]
pub type W = crate::W<Txid0BSpec>;
#[doc = "Field `ID3` reader - Identifier bit 3"]
pub type Id3R = crate::BitReader;
#[doc = "Field `ID3` writer - Identifier bit 3"]
pub type Id3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID4` reader - Identifier bit 4"]
pub type Id4R = crate::BitReader;
#[doc = "Field `ID4` writer - Identifier bit 4"]
pub type Id4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID5` reader - Identifier bit 5"]
pub type Id5R = crate::BitReader;
#[doc = "Field `ID5` writer - Identifier bit 5"]
pub type Id5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID6` reader - Identifier bit 6"]
pub type Id6R = crate::BitReader;
#[doc = "Field `ID6` writer - Identifier bit 6"]
pub type Id6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID7` reader - Identifier bit 7"]
pub type Id7R = crate::BitReader;
#[doc = "Field `ID7` writer - Identifier bit 7"]
pub type Id7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID8` reader - Identifier bit 8"]
pub type Id8R = crate::BitReader;
#[doc = "Field `ID8` writer - Identifier bit 8"]
pub type Id8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID9` reader - Identifier bit 9"]
pub type Id9R = crate::BitReader;
#[doc = "Field `ID9` writer - Identifier bit 9"]
pub type Id9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID10` reader - Identifier bit 10"]
pub type Id10R = crate::BitReader;
#[doc = "Field `ID10` writer - Identifier bit 10"]
pub type Id10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    pub fn id3(&self) -> Id3R {
        Id3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    pub fn id4(&self) -> Id4R {
        Id4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    pub fn id5(&self) -> Id5R {
        Id5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    pub fn id6(&self) -> Id6R {
        Id6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    pub fn id7(&self) -> Id7R {
        Id7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    pub fn id8(&self) -> Id8R {
        Id8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    pub fn id9(&self) -> Id9R {
        Id9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    pub fn id10(&self) -> Id10R {
        Id10R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Identifier bit 3"]
    #[inline(always)]
    pub fn id3(&mut self) -> Id3W<'_, Txid0BSpec> {
        Id3W::new(self, 0)
    }
    #[doc = "Bit 1 - Identifier bit 4"]
    #[inline(always)]
    pub fn id4(&mut self) -> Id4W<'_, Txid0BSpec> {
        Id4W::new(self, 1)
    }
    #[doc = "Bit 2 - Identifier bit 5"]
    #[inline(always)]
    pub fn id5(&mut self) -> Id5W<'_, Txid0BSpec> {
        Id5W::new(self, 2)
    }
    #[doc = "Bit 3 - Identifier bit 6"]
    #[inline(always)]
    pub fn id6(&mut self) -> Id6W<'_, Txid0BSpec> {
        Id6W::new(self, 3)
    }
    #[doc = "Bit 4 - Identifier bit 7"]
    #[inline(always)]
    pub fn id7(&mut self) -> Id7W<'_, Txid0BSpec> {
        Id7W::new(self, 4)
    }
    #[doc = "Bit 5 - Identifier bit 8"]
    #[inline(always)]
    pub fn id8(&mut self) -> Id8W<'_, Txid0BSpec> {
        Id8W::new(self, 5)
    }
    #[doc = "Bit 6 - Identifier bit 9"]
    #[inline(always)]
    pub fn id9(&mut self) -> Id9W<'_, Txid0BSpec> {
        Id9W::new(self, 6)
    }
    #[doc = "Bit 7 - Identifier bit 10"]
    #[inline(always)]
    pub fn id10(&mut self) -> Id10W<'_, Txid0BSpec> {
        Id10W::new(self, 7)
    }
}
#[doc = "Basic TX ID register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`txid0_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txid0_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Txid0BSpec;
impl crate::RegisterSpec for Txid0BSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txid0_b::R`](R) reader structure"]
impl crate::Readable for Txid0BSpec {}
#[doc = "`write(|w| ..)` method takes [`txid0_b::W`](W) writer structure"]
impl crate::Writable for Txid0BSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXID0_B to value 0"]
impl crate::Resettable for Txid0BSpec {}
