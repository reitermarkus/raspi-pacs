#[doc = "Register `GICD_ITARGETSR40` reader"]
pub type R = crate::R<GicdItargetsr40Spec>;
#[doc = "Register `GICD_ITARGETSR40` writer"]
pub type W = crate::W<GicdItargetsr40Spec>;
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type Int160R = crate::FieldReader;
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type Int160W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type Int161R = crate::FieldReader;
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type Int161W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type Int162R = crate::FieldReader;
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type Int162W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type Int163R = crate::FieldReader;
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type Int163W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> Int160R {
        Int160R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> Int161R {
        Int161R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> Int162R {
        Int162R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> Int163R {
        Int163R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR40")
            .field("int160", &self.int160())
            .field("int161", &self.int161())
            .field("int162", &self.int162())
            .field("int163", &self.int163())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> Int160W<GicdItargetsr40Spec> {
        Int160W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> Int161W<GicdItargetsr40Spec> {
        Int161W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> Int162W<GicdItargetsr40Spec> {
        Int162W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> Int163W<GicdItargetsr40Spec> {
        Int163W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 160 - 163\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr40Spec;
impl crate::RegisterSpec for GicdItargetsr40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr40::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr40Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr40::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR40 to value 0"]
impl crate::Resettable for GicdItargetsr40Spec {
    const RESET_VALUE: u32 = 0;
}
