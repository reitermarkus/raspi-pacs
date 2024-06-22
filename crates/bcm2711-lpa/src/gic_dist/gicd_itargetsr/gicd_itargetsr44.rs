#[doc = "Register `GICD_ITARGETSR44` reader"]
pub type R = crate::R<GicdItargetsr44Spec>;
#[doc = "Register `GICD_ITARGETSR44` writer"]
pub type W = crate::W<GicdItargetsr44Spec>;
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type Int176R = crate::FieldReader;
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type Int176W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type Int177R = crate::FieldReader;
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type Int177W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type Int178R = crate::FieldReader;
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type Int178W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type Int179R = crate::FieldReader;
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type Int179W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> Int176R {
        Int176R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> Int177R {
        Int177R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> Int178R {
        Int178R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> Int179R {
        Int179R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR44")
            .field("int176", &self.int176())
            .field("int177", &self.int177())
            .field("int178", &self.int178())
            .field("int179", &self.int179())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> Int176W<GicdItargetsr44Spec> {
        Int176W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> Int177W<GicdItargetsr44Spec> {
        Int177W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> Int178W<GicdItargetsr44Spec> {
        Int178W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> Int179W<GicdItargetsr44Spec> {
        Int179W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 176 - 179\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr44Spec;
impl crate::RegisterSpec for GicdItargetsr44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr44::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr44Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr44::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR44 to value 0"]
impl crate::Resettable for GicdItargetsr44Spec {
    const RESET_VALUE: u32 = 0;
}
