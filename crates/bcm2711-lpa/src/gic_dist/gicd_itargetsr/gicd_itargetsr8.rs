#[doc = "Register `GICD_ITARGETSR8` reader"]
pub type R = crate::R<GICD_ITARGETSR8_SPEC>;
#[doc = "Register `GICD_ITARGETSR8` writer"]
pub type W = crate::W<GICD_ITARGETSR8_SPEC>;
#[doc = "Field `INT32` reader - Interrupt 32"]
pub type INT32_R = crate::FieldReader;
#[doc = "Field `INT32` writer - Interrupt 32"]
pub type INT32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT33` reader - Interrupt 33"]
pub type INT33_R = crate::FieldReader;
#[doc = "Field `INT33` writer - Interrupt 33"]
pub type INT33_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT34` reader - Interrupt 34"]
pub type INT34_R = crate::FieldReader;
#[doc = "Field `INT34` writer - Interrupt 34"]
pub type INT34_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT35` reader - Interrupt 35"]
pub type INT35_R = crate::FieldReader;
#[doc = "Field `INT35` writer - Interrupt 35"]
pub type INT35_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    pub fn int32(&self) -> INT32_R {
        INT32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    pub fn int33(&self) -> INT33_R {
        INT33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    pub fn int34(&self) -> INT34_R {
        INT34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    pub fn int35(&self) -> INT35_R {
        INT35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR8")
            .field("int32", &format_args!("{}", self.int32().bits()))
            .field("int33", &format_args!("{}", self.int33().bits()))
            .field("int34", &format_args!("{}", self.int34().bits()))
            .field("int35", &format_args!("{}", self.int35().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 32"]
    #[inline(always)]
    #[must_use]
    pub fn int32(&mut self) -> INT32_W<GICD_ITARGETSR8_SPEC, 0> {
        INT32_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 33"]
    #[inline(always)]
    #[must_use]
    pub fn int33(&mut self) -> INT33_W<GICD_ITARGETSR8_SPEC, 8> {
        INT33_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 34"]
    #[inline(always)]
    #[must_use]
    pub fn int34(&mut self) -> INT34_W<GICD_ITARGETSR8_SPEC, 16> {
        INT34_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 35"]
    #[inline(always)]
    #[must_use]
    pub fn int35(&mut self) -> INT35_W<GICD_ITARGETSR8_SPEC, 24> {
        INT35_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Processor Target 32 - 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR8_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr8::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr8::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR8 to value 0"]
impl crate::Resettable for GICD_ITARGETSR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
