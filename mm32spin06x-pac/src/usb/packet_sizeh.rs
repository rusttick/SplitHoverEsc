#[doc = "Register `PACKET_SIZEH` reader"]
pub type R = crate::R<PacketSizehSpec>;
#[doc = "Register `PACKET_SIZEH` writer"]
pub type W = crate::W<PacketSizehSpec>;
#[doc = "Field `SIZE1` reader - USB Max Packet Size"]
pub type Size1R = crate::FieldReader;
#[doc = "Field `SIZE1` writer - USB Max Packet Size"]
pub type Size1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USB Max Packet Size"]
    #[inline(always)]
    pub fn size1(&self) -> Size1R {
        Size1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USB Max Packet Size"]
    #[inline(always)]
    pub fn size1(&mut self) -> Size1W<'_, PacketSizehSpec> {
        Size1W::new(self, 0)
    }
}
#[doc = "PACKET SIZEH register\n\nYou can [`read`](crate::Reg::read) this register and get [`packet_sizeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packet_sizeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketSizehSpec;
impl crate::RegisterSpec for PacketSizehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packet_sizeh::R`](R) reader structure"]
impl crate::Readable for PacketSizehSpec {}
#[doc = "`write(|w| ..)` method takes [`packet_sizeh::W`](W) writer structure"]
impl crate::Writable for PacketSizehSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PACKET_SIZEH to value 0x40"]
impl crate::Resettable for PacketSizehSpec {
    const RESET_VALUE: u32 = 0x40;
}
