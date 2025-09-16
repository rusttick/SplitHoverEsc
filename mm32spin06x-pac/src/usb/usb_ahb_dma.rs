#[doc = "Register `USB_AHB_DMA` reader"]
pub type R = crate::R<UsbAhbDmaSpec>;
#[doc = "Register `USB_AHB_DMA` writer"]
pub type W = crate::W<UsbAhbDmaSpec>;
#[doc = "Field `CH0_BS` reader - Channel 0 burst set"]
pub type Ch0BsR = crate::FieldReader;
#[doc = "Field `CH0_BS` writer - Channel 0 burst set"]
pub type Ch0BsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_BS` reader - Channel 1 burst set"]
pub type Ch1BsR = crate::FieldReader;
#[doc = "Field `CH1_BS` writer - Channel 1 burst set"]
pub type Ch1BsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_BS` reader - Channel 2 burst set"]
pub type Ch2BsR = crate::FieldReader;
#[doc = "Field `CH2_BS` writer - Channel 2 burst set"]
pub type Ch2BsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3_BS` reader - Channel 3 burst set"]
pub type Ch3BsR = crate::FieldReader;
#[doc = "Field `CH3_BS` writer - Channel 3 burst set"]
pub type Ch3BsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Channel 0 burst set"]
    #[inline(always)]
    pub fn ch0_bs(&self) -> Ch0BsR {
        Ch0BsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 burst set"]
    #[inline(always)]
    pub fn ch1_bs(&self) -> Ch1BsR {
        Ch1BsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 burst set"]
    #[inline(always)]
    pub fn ch2_bs(&self) -> Ch2BsR {
        Ch2BsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Channel 3 burst set"]
    #[inline(always)]
    pub fn ch3_bs(&self) -> Ch3BsR {
        Ch3BsR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 burst set"]
    #[inline(always)]
    pub fn ch0_bs(&mut self) -> Ch0BsW<'_, UsbAhbDmaSpec> {
        Ch0BsW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 burst set"]
    #[inline(always)]
    pub fn ch1_bs(&mut self) -> Ch1BsW<'_, UsbAhbDmaSpec> {
        Ch1BsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Channel 2 burst set"]
    #[inline(always)]
    pub fn ch2_bs(&mut self) -> Ch2BsW<'_, UsbAhbDmaSpec> {
        Ch2BsW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Channel 3 burst set"]
    #[inline(always)]
    pub fn ch3_bs(&mut self) -> Ch3BsW<'_, UsbAhbDmaSpec> {
        Ch3BsW::new(self, 6)
    }
}
#[doc = "USB AHB DMA register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ahb_dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ahb_dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbAhbDmaSpec;
impl crate::RegisterSpec for UsbAhbDmaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`usb_ahb_dma::R`](R) reader structure"]
impl crate::Readable for UsbAhbDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_ahb_dma::W`](W) writer structure"]
impl crate::Writable for UsbAhbDmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_AHB_DMA to value 0"]
impl crate::Resettable for UsbAhbDmaSpec {}
