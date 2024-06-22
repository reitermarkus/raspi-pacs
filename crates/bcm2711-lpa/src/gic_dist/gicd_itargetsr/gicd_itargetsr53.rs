#[doc = "Register `GICD_ITARGETSR53` reader"]
pub type R = crate::R<GicdItargetsr53Spec>;
#[doc = "Register `GICD_ITARGETSR53` writer"]
pub type W = crate::W<GicdItargetsr53Spec>;
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type Int212R = crate::FieldReader;
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type Int212W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type Int213R = crate::FieldReader;
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type Int213W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type Int214R = crate::FieldReader;
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type Int214W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type Int215R = crate::FieldReader;
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type Int215W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> Int212R {
        Int212R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> Int213R {
        Int213R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> Int214R {
        Int214R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> Int215R {
        Int215R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR53")
            .field("int212", &self.int212())
            .field("int213", &self.int213())
            .field("int214", &self.int214())
            .field("int215", &self.int215())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> Int212W<GicdItargetsr53Spec> {
        Int212W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> Int213W<GicdItargetsr53Spec> {
        Int213W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> Int214W<GicdItargetsr53Spec> {
        Int214W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> Int215W<GicdItargetsr53Spec> {
        Int215W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 212 - 215\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr53Spec;
impl crate::RegisterSpec for GicdItargetsr53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr53::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr53Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr53::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR53 to value 0"]
impl crate::Resettable for GicdItargetsr53Spec {
    const RESET_VALUE: u32 = 0;
}
