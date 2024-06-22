#[doc = "Register `GICD_IPRIORITYR26` reader"]
pub type R = crate::R<GicdIpriorityr26Spec>;
#[doc = "Register `GICD_IPRIORITYR26` writer"]
pub type W = crate::W<GicdIpriorityr26Spec>;
#[doc = "Field `ISP` reader - ISP"]
pub type IspR = crate::FieldReader;
#[doc = "Field `ISP` writer - ISP"]
pub type IspW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB` reader - USB"]
pub type UsbR = crate::FieldReader;
#[doc = "Field `USB` writer - USB"]
pub type UsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `V3D` reader - V3D"]
pub type V3dR = crate::FieldReader;
#[doc = "Field `V3D` writer - V3D"]
pub type V3dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSPOSER` reader - Transposer"]
pub type TransposerR = crate::FieldReader;
#[doc = "Field `TRANSPOSER` writer - Transposer"]
pub type TransposerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    pub fn isp(&self) -> IspR {
        IspR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    pub fn v3d(&self) -> V3dR {
        V3dR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    pub fn transposer(&self) -> TransposerR {
        TransposerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR26")
            .field("isp", &self.isp())
            .field("usb", &self.usb())
            .field("v3d", &self.v3d())
            .field("transposer", &self.transposer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - ISP"]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> IspW<GicdIpriorityr26Spec> {
        IspW::new(self, 0)
    }
    #[doc = "Bits 8:15 - USB"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> UsbW<GicdIpriorityr26Spec> {
        UsbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - V3D"]
    #[inline(always)]
    #[must_use]
    pub fn v3d(&mut self) -> V3dW<GicdIpriorityr26Spec> {
        V3dW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transposer"]
    #[inline(always)]
    #[must_use]
    pub fn transposer(&mut self) -> TransposerW<GicdIpriorityr26Spec> {
        TransposerW::new(self, 24)
    }
}
#[doc = "Interrupt Priority 104 - 107 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr26Spec;
impl crate::RegisterSpec for GicdIpriorityr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr26::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr26Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr26::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR26 to value 0"]
impl crate::Resettable for GicdIpriorityr26Spec {
    const RESET_VALUE: u32 = 0;
}
