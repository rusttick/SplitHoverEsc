#[doc = "Register `PACKET_SIZEL` reader"]
pub type R = crate::R<PacketSizelSpec>;
#[doc = "Register `PACKET_SIZEL` writer"]
pub type W = crate::W<PacketSizelSpec>;
#[doc = "Field `SIZE0` reader - USB Max Packet Size"]
pub type Size0R = crate::FieldReader;
#[doc = "Field `SIZE0` writer - USB Max Packet Size"]
pub type Size0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - USB Max Packet Size"]
    #[inline(always)]
    pub fn size0(&self) -> Size0R {
        Size0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USB Max Packet Size"]
    #[inline(always)]
    pub fn size0(&mut self) -> Size0W<'_, PacketSizelSpec> {
        Size0W::new(self, 0)
    }
}
#[doc = "PACKET SIZEL register\n\nYou can [`read`](crate::Reg::read) this register and get [`packet_sizel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packet_sizel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketSizelSpec;
impl crate::RegisterSpec for PacketSizelSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`packet_sizel::R`](R) reader structure"]
impl crate::Readable for PacketSizelSpec {}
#[doc = "`write(|w| ..)` method takes [`packet_sizel::W`](W) writer structure"]
impl crate::Writable for PacketSizelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PACKET_SIZEL to value 0x40"]
impl crate::Resettable for PacketSizelSpec {
    const RESET_VALUE: u16 = 0x40;
}
