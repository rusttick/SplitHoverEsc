#[doc = "Register `EP4_FIFO` reader"]
pub type R = crate::R<Ep4FifoSpec>;
#[doc = "Register `EP4_FIFO` writer"]
pub type W = crate::W<Ep4FifoSpec>;
#[doc = "Field `EP4_FIFO` reader - EP4 FIFO port"]
pub type Ep4FifoR = crate::FieldReader;
#[doc = "Field `EP4_FIFO` writer - EP4 FIFO port"]
pub type Ep4FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP4 FIFO port"]
    #[inline(always)]
    pub fn ep4_fifo(&self) -> Ep4FifoR {
        Ep4FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP4 FIFO port"]
    #[inline(always)]
    pub fn ep4_fifo(&mut self) -> Ep4FifoW<'_, Ep4FifoSpec> {
        Ep4FifoW::new(self, 0)
    }
}
#[doc = "EP4 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep4_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep4_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep4FifoSpec;
impl crate::RegisterSpec for Ep4FifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep4_fifo::R`](R) reader structure"]
impl crate::Readable for Ep4FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ep4_fifo::W`](W) writer structure"]
impl crate::Writable for Ep4FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP4_FIFO to value 0"]
impl crate::Resettable for Ep4FifoSpec {}
