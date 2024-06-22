#[doc = "Register `GICD_ITARGETSR8` reader"]
pub type R = crate::R<GicdItargetsr8Spec>;
#[doc = "Register `GICD_ITARGETSR8` writer"]
pub type W = crate::W<GicdItargetsr8Spec>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type Int32R = crate::FieldReader;
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type Int32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type Int33R = crate::FieldReader;
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type Int33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type Int34R = crate::FieldReader;
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type Int34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type Int35R = crate::FieldReader;
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type Int35W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> Int32R {
        Int32R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> Int33R {
        Int33R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> Int34R {
        Int34R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> Int35R {
        Int35R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR8")
            .field("int32", &self.int32())
            .field("int33", &self.int33())
            .field("int34", &self.int34())
            .field("int35", &self.int35())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> Int32W<GicdItargetsr8Spec> {
        Int32W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> Int33W<GicdItargetsr8Spec> {
        Int33W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> Int34W<GicdItargetsr8Spec> {
        Int34W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> Int35W<GicdItargetsr8Spec> {
        Int35W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 32 - 35\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr8Spec;
impl crate::RegisterSpec for GicdItargetsr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr8::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr8Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr8::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR8 to value 0"]
impl crate::Resettable for GicdItargetsr8Spec {
    const RESET_VALUE: u32 = 0;
}
