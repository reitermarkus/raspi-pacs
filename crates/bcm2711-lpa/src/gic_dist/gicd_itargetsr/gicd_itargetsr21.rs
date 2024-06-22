#[doc = "Register `GICD_ITARGETSR21` reader"]
pub type R = crate::R<GicdItargetsr21Spec>;
#[doc = "Register `GICD_ITARGETSR21` writer"]
pub type W = crate::W<GicdItargetsr21Spec>;
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type Int84R = crate::FieldReader;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type Int84W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type Int85R = crate::FieldReader;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type Int85W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type Int86R = crate::FieldReader;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type Int86W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type Int87R = crate::FieldReader;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type Int87W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> Int84R {
        Int84R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> Int85R {
        Int85R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> Int86R {
        Int86R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> Int87R {
        Int87R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR21")
            .field("int84", &self.int84())
            .field("int85", &self.int85())
            .field("int86", &self.int86())
            .field("int87", &self.int87())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> Int84W<GicdItargetsr21Spec> {
        Int84W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> Int85W<GicdItargetsr21Spec> {
        Int85W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> Int86W<GicdItargetsr21Spec> {
        Int86W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> Int87W<GicdItargetsr21Spec> {
        Int87W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 84 - 87\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr21Spec;
impl crate::RegisterSpec for GicdItargetsr21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr21::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr21Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr21::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR21 to value 0"]
impl crate::Resettable for GicdItargetsr21Spec {
    const RESET_VALUE: u32 = 0;
}
