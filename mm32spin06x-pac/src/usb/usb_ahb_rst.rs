#[doc = "Register `USB_AHB_RST` reader"]
pub type R = crate::R<UsbAhbRstSpec>;
#[doc = "Register `USB_AHB_RST` writer"]
pub type W = crate::W<UsbAhbRstSpec>;
#[doc = "Field `EP0` reader - Endpoint 0 reset"]
pub type Ep0R = crate::BitReader;
#[doc = "Field `EP0` writer - Endpoint 0 reset"]
pub type Ep0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1` reader - Endpoint 1 reset"]
pub type Ep1R = crate::BitReader;
#[doc = "Field `EP1` writer - Endpoint 1 reset"]
pub type Ep1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2` reader - Endpoint 2 reset"]
pub type Ep2R = crate::BitReader;
#[doc = "Field `EP2` writer - Endpoint 2 reset"]
pub type Ep2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3` reader - Endpoint 3 reset"]
pub type Ep3R = crate::BitReader;
#[doc = "Field `EP3` writer - Endpoint 3 reset"]
pub type Ep3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 reset"]
    #[inline(always)]
    pub fn ep0(&self) -> Ep0R {
        Ep0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 reset"]
    #[inline(always)]
    pub fn ep1(&self) -> Ep1R {
        Ep1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 reset"]
    #[inline(always)]
    pub fn ep2(&self) -> Ep2R {
        Ep2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 reset"]
    #[inline(always)]
    pub fn ep3(&self) -> Ep3R {
        Ep3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 reset"]
    #[inline(always)]
    pub fn ep0(&mut self) -> Ep0W<'_, UsbAhbRstSpec> {
        Ep0W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 1 reset"]
    #[inline(always)]
    pub fn ep1(&mut self) -> Ep1W<'_, UsbAhbRstSpec> {
        Ep1W::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 2 reset"]
    #[inline(always)]
    pub fn ep2(&mut self) -> Ep2W<'_, UsbAhbRstSpec> {
        Ep2W::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 3 reset"]
    #[inline(always)]
    pub fn ep3(&mut self) -> Ep3W<'_, UsbAhbRstSpec> {
        Ep3W::new(self, 3)
    }
}
#[doc = "USB AHB RST register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ahb_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ahb_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbAhbRstSpec;
impl crate::RegisterSpec for UsbAhbRstSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usb_ahb_rst::R`](R) reader structure"]
impl crate::Readable for UsbAhbRstSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_ahb_rst::W`](W) writer structure"]
impl crate::Writable for UsbAhbRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_AHB_RST to value 0"]
impl crate::Resettable for UsbAhbRstSpec {}
