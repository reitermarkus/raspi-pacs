#[doc = "Register `GICD_IPRIORITYR25` reader"]
pub type R = crate::R<GicdIpriorityr25Spec>;
#[doc = "Register `GICD_IPRIORITYR25` writer"]
pub type W = crate::W<GicdIpriorityr25Spec>;
#[doc = "Field `H264_0` reader - H264 0"]
pub type H264_0R = crate::FieldReader;
#[doc = "Field `H264_0` writer - H264 0"]
pub type H264_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_1` reader - H264 1"]
pub type H264_1R = crate::FieldReader;
#[doc = "Field `H264_1` writer - H264 1"]
pub type H264_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `H264_2` reader - H264 2"]
pub type H264_2R = crate::FieldReader;
#[doc = "Field `H264_2` writer - H264 2"]
pub type H264_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `JPEG` reader - JPEG"]
pub type JpegR = crate::FieldReader;
#[doc = "Field `JPEG` writer - JPEG"]
pub type JpegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - H264 0"]
    #[inline(always)]
    pub fn h264_0(&self) -> H264_0R {
        H264_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - H264 1"]
    #[inline(always)]
    pub fn h264_1(&self) -> H264_1R {
        H264_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - H264 2"]
    #[inline(always)]
    pub fn h264_2(&self) -> H264_2R {
        H264_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - JPEG"]
    #[inline(always)]
    pub fn jpeg(&self) -> JpegR {
        JpegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR25")
            .field("h264_0", &self.h264_0())
            .field("h264_1", &self.h264_1())
            .field("h264_2", &self.h264_2())
            .field("jpeg", &self.jpeg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - H264 0"]
    #[inline(always)]
    #[must_use]
    pub fn h264_0(&mut self) -> H264_0W<GicdIpriorityr25Spec> {
        H264_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - H264 1"]
    #[inline(always)]
    #[must_use]
    pub fn h264_1(&mut self) -> H264_1W<GicdIpriorityr25Spec> {
        H264_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - H264 2"]
    #[inline(always)]
    #[must_use]
    pub fn h264_2(&mut self) -> H264_2W<GicdIpriorityr25Spec> {
        H264_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - JPEG"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JpegW<GicdIpriorityr25Spec> {
        JpegW::new(self, 24)
    }
}
#[doc = "Interrupt Priority 100 - 103 (Lower is first)\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_ipriorityr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ipriorityr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdIpriorityr25Spec;
impl crate::RegisterSpec for GicdIpriorityr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr25::R`](R) reader structure"]
impl crate::Readable for GicdIpriorityr25Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr25::W`](W) writer structure"]
impl crate::Writable for GicdIpriorityr25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR25 to value 0"]
impl crate::Resettable for GicdIpriorityr25Spec {
    const RESET_VALUE: u32 = 0;
}
