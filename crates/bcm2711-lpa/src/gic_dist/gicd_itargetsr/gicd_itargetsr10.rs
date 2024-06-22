#[doc = "Register `GICD_ITARGETSR10` reader"]
pub type R = crate::R<GicdItargetsr10Spec>;
#[doc = "Register `GICD_ITARGETSR10` writer"]
pub type W = crate::W<GicdItargetsr10Spec>;
#[doc = "Field `INT40` reader - Interrupt 40"]
pub type Int40R = crate::FieldReader;
#[doc = "Field `INT40` writer - Interrupt 40"]
pub type Int40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT41` reader - Interrupt 41"]
pub type Int41R = crate::FieldReader;
#[doc = "Field `INT41` writer - Interrupt 41"]
pub type Int41W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT42` reader - Interrupt 42"]
pub type Int42R = crate::FieldReader;
#[doc = "Field `INT42` writer - Interrupt 42"]
pub type Int42W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT43` reader - Interrupt 43"]
pub type Int43R = crate::FieldReader;
#[doc = "Field `INT43` writer - Interrupt 43"]
pub type Int43W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    pub fn int40(&self) -> Int40R {
        Int40R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    pub fn int41(&self) -> Int41R {
        Int41R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    pub fn int42(&self) -> Int42R {
        Int42R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    pub fn int43(&self) -> Int43R {
        Int43R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR10")
            .field("int40", &self.int40())
            .field("int41", &self.int41())
            .field("int42", &self.int42())
            .field("int43", &self.int43())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 40"]
    #[inline(always)]
    #[must_use]
    pub fn int40(&mut self) -> Int40W<GicdItargetsr10Spec> {
        Int40W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 41"]
    #[inline(always)]
    #[must_use]
    pub fn int41(&mut self) -> Int41W<GicdItargetsr10Spec> {
        Int41W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 42"]
    #[inline(always)]
    #[must_use]
    pub fn int42(&mut self) -> Int42W<GicdItargetsr10Spec> {
        Int42W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 43"]
    #[inline(always)]
    #[must_use]
    pub fn int43(&mut self) -> Int43W<GicdItargetsr10Spec> {
        Int43W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 40 - 43\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr10Spec;
impl crate::RegisterSpec for GicdItargetsr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr10::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr10Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr10::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR10 to value 0"]
impl crate::Resettable for GicdItargetsr10Spec {
    const RESET_VALUE: u32 = 0;
}
