#[doc = "Register `EP0_FIFO` reader"]
pub type R = crate::R<Ep0FifoSpec>;
#[doc = "Register `EP0_FIFO` writer"]
pub type W = crate::W<Ep0FifoSpec>;
#[doc = "Field `EP0_FIFO` reader - EP0 FIFO port"]
pub type Ep0FifoR = crate::FieldReader;
#[doc = "Field `EP0_FIFO` writer - EP0 FIFO port"]
pub type Ep0FifoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EP0 FIFO port"]
    #[inline(always)]
    pub fn ep0_fifo(&self) -> Ep0FifoR {
        Ep0FifoR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EP0 FIFO port"]
    #[inline(always)]
    pub fn ep0_fifo(&mut self) -> Ep0FifoW<'_, Ep0FifoSpec> {
        Ep0FifoW::new(self, 0)
    }
}
#[doc = "EP0 FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0FifoSpec;
impl crate::RegisterSpec for Ep0FifoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0_fifo::R`](R) reader structure"]
impl crate::Readable for Ep0FifoSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0_fifo::W`](W) writer structure"]
impl crate::Writable for Ep0FifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EP0_FIFO to value 0"]
impl crate::Resettable for Ep0FifoSpec {}
