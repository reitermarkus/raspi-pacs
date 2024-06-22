#[doc = "Register `GICD_ITARGETSR42` reader"]
pub type R = crate::R<GicdItargetsr42Spec>;
#[doc = "Register `GICD_ITARGETSR42` writer"]
pub type W = crate::W<GicdItargetsr42Spec>;
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type Int168R = crate::FieldReader;
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type Int168W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type Int169R = crate::FieldReader;
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type Int169W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type Int170R = crate::FieldReader;
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type Int170W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type Int171R = crate::FieldReader;
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type Int171W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> Int168R {
        Int168R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> Int169R {
        Int169R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> Int170R {
        Int170R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> Int171R {
        Int171R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR42")
            .field("int168", &self.int168())
            .field("int169", &self.int169())
            .field("int170", &self.int170())
            .field("int171", &self.int171())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> Int168W<GicdItargetsr42Spec> {
        Int168W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> Int169W<GicdItargetsr42Spec> {
        Int169W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> Int170W<GicdItargetsr42Spec> {
        Int170W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> Int171W<GicdItargetsr42Spec> {
        Int171W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 168 - 171\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr42Spec;
impl crate::RegisterSpec for GicdItargetsr42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr42::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr42Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr42::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR42 to value 0"]
impl crate::Resettable for GicdItargetsr42Spec {
    const RESET_VALUE: u32 = 0;
}
