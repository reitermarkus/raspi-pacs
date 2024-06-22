#[doc = "Register `GICD_ITARGETSR11` reader"]
pub type R = crate::R<GicdItargetsr11Spec>;
#[doc = "Register `GICD_ITARGETSR11` writer"]
pub type W = crate::W<GicdItargetsr11Spec>;
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type Int44R = crate::FieldReader;
#[doc = "Field `INT44` writer - Interrupt 44"]
pub type Int44W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type Int45R = crate::FieldReader;
#[doc = "Field `INT45` writer - Interrupt 45"]
pub type Int45W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type Int46R = crate::FieldReader;
#[doc = "Field `INT46` writer - Interrupt 46"]
pub type Int46W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type Int47R = crate::FieldReader;
#[doc = "Field `INT47` writer - Interrupt 47"]
pub type Int47W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> Int44R {
        Int44R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> Int45R {
        Int45R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> Int46R {
        Int46R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> Int47R {
        Int47R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR11")
            .field("int44", &self.int44())
            .field("int45", &self.int45())
            .field("int46", &self.int46())
            .field("int47", &self.int47())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn int44(&mut self) -> Int44W<GicdItargetsr11Spec> {
        Int44W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn int45(&mut self) -> Int45W<GicdItargetsr11Spec> {
        Int45W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn int46(&mut self) -> Int46W<GicdItargetsr11Spec> {
        Int46W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn int47(&mut self) -> Int47W<GicdItargetsr11Spec> {
        Int47W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 44 - 47\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr11Spec;
impl crate::RegisterSpec for GicdItargetsr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr11::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr11Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr11::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR11 to value 0"]
impl crate::Resettable for GicdItargetsr11Spec {
    const RESET_VALUE: u32 = 0;
}
