#[doc = "Register `GICD_ITARGETSR7` reader"]
pub type R = crate::R<GicdItargetsr7Spec>;
#[doc = "Register `GICD_ITARGETSR7` writer"]
pub type W = crate::W<GicdItargetsr7Spec>;
#[doc = "Field `INT28` reader - Interrupt 28"]
pub type Int28R = crate::FieldReader;
#[doc = "Field `INT28` writer - Interrupt 28"]
pub type Int28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT29` reader - Interrupt 29"]
pub type Int29R = crate::FieldReader;
#[doc = "Field `INT29` writer - Interrupt 29"]
pub type Int29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT30` reader - Interrupt 30"]
pub type Int30R = crate::FieldReader;
#[doc = "Field `INT30` writer - Interrupt 30"]
pub type Int30W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT31` reader - Interrupt 31"]
pub type Int31R = crate::FieldReader;
#[doc = "Field `INT31` writer - Interrupt 31"]
pub type Int31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    pub fn int28(&self) -> Int28R {
        Int28R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    pub fn int29(&self) -> Int29R {
        Int29R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    pub fn int30(&self) -> Int30R {
        Int30R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    pub fn int31(&self) -> Int31R {
        Int31R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR7")
            .field("int28", &self.int28())
            .field("int29", &self.int29())
            .field("int30", &self.int30())
            .field("int31", &self.int31())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 28"]
    #[inline(always)]
    #[must_use]
    pub fn int28(&mut self) -> Int28W<GicdItargetsr7Spec> {
        Int28W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 29"]
    #[inline(always)]
    #[must_use]
    pub fn int29(&mut self) -> Int29W<GicdItargetsr7Spec> {
        Int29W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 30"]
    #[inline(always)]
    #[must_use]
    pub fn int30(&mut self) -> Int30W<GicdItargetsr7Spec> {
        Int30W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 31"]
    #[inline(always)]
    #[must_use]
    pub fn int31(&mut self) -> Int31W<GicdItargetsr7Spec> {
        Int31W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 28 - 31\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr7Spec;
impl crate::RegisterSpec for GicdItargetsr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr7::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr7Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr7::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR7 to value 0"]
impl crate::Resettable for GicdItargetsr7Spec {
    const RESET_VALUE: u32 = 0;
}
