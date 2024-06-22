#[doc = "Register `GICD_ITARGETSR14` reader"]
pub type R = crate::R<GicdItargetsr14Spec>;
#[doc = "Register `GICD_ITARGETSR14` writer"]
pub type W = crate::W<GicdItargetsr14Spec>;
#[doc = "Field `INT56` reader - Interrupt 56"]
pub type Int56R = crate::FieldReader;
#[doc = "Field `INT56` writer - Interrupt 56"]
pub type Int56W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT57` reader - Interrupt 57"]
pub type Int57R = crate::FieldReader;
#[doc = "Field `INT57` writer - Interrupt 57"]
pub type Int57W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT58` reader - Interrupt 58"]
pub type Int58R = crate::FieldReader;
#[doc = "Field `INT58` writer - Interrupt 58"]
pub type Int58W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT59` reader - Interrupt 59"]
pub type Int59R = crate::FieldReader;
#[doc = "Field `INT59` writer - Interrupt 59"]
pub type Int59W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    pub fn int56(&self) -> Int56R {
        Int56R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    pub fn int57(&self) -> Int57R {
        Int57R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    pub fn int58(&self) -> Int58R {
        Int58R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    pub fn int59(&self) -> Int59R {
        Int59R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR14")
            .field("int56", &self.int56())
            .field("int57", &self.int57())
            .field("int58", &self.int58())
            .field("int59", &self.int59())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 56"]
    #[inline(always)]
    #[must_use]
    pub fn int56(&mut self) -> Int56W<GicdItargetsr14Spec> {
        Int56W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 57"]
    #[inline(always)]
    #[must_use]
    pub fn int57(&mut self) -> Int57W<GicdItargetsr14Spec> {
        Int57W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 58"]
    #[inline(always)]
    #[must_use]
    pub fn int58(&mut self) -> Int58W<GicdItargetsr14Spec> {
        Int58W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 59"]
    #[inline(always)]
    #[must_use]
    pub fn int59(&mut self) -> Int59W<GicdItargetsr14Spec> {
        Int59W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 56 - 59\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr14Spec;
impl crate::RegisterSpec for GicdItargetsr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr14::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr14Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr14::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR14 to value 0"]
impl crate::Resettable for GicdItargetsr14Spec {
    const RESET_VALUE: u32 = 0;
}
