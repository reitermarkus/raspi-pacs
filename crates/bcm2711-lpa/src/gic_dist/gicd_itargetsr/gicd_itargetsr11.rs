#[doc = "Register `GICD_ITARGETSR11` reader"]
pub type R = crate::R<GICD_ITARGETSR11_SPEC>;
#[doc = "Register `GICD_ITARGETSR11` writer"]
pub type W = crate::W<GICD_ITARGETSR11_SPEC>;
#[doc = "Field `INT44` reader - Interrupt 44"]
pub type INT44_R = crate::FieldReader;
#[doc = "Field `INT44` writer - Interrupt 44"]
pub type INT44_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT45` reader - Interrupt 45"]
pub type INT45_R = crate::FieldReader;
#[doc = "Field `INT45` writer - Interrupt 45"]
pub type INT45_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT46` reader - Interrupt 46"]
pub type INT46_R = crate::FieldReader;
#[doc = "Field `INT46` writer - Interrupt 46"]
pub type INT46_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `INT47` reader - Interrupt 47"]
pub type INT47_R = crate::FieldReader;
#[doc = "Field `INT47` writer - Interrupt 47"]
pub type INT47_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    pub fn int44(&self) -> INT44_R {
        INT44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    pub fn int45(&self) -> INT45_R {
        INT45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    pub fn int46(&self) -> INT46_R {
        INT46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    pub fn int47(&self) -> INT47_R {
        INT47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR11")
            .field("int44", &format_args!("{}", self.int44().bits()))
            .field("int45", &format_args!("{}", self.int45().bits()))
            .field("int46", &format_args!("{}", self.int46().bits()))
            .field("int47", &format_args!("{}", self.int47().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 44"]
    #[inline(always)]
    #[must_use]
    pub fn int44(&mut self) -> INT44_W<GICD_ITARGETSR11_SPEC, 0> {
        INT44_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 45"]
    #[inline(always)]
    #[must_use]
    pub fn int45(&mut self) -> INT45_W<GICD_ITARGETSR11_SPEC, 8> {
        INT45_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 46"]
    #[inline(always)]
    #[must_use]
    pub fn int46(&mut self) -> INT46_W<GICD_ITARGETSR11_SPEC, 16> {
        INT46_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 47"]
    #[inline(always)]
    #[must_use]
    pub fn int47(&mut self) -> INT47_W<GICD_ITARGETSR11_SPEC, 24> {
        INT47_W::new(self)
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
#[doc = "Interrupt Processor Target 44 - 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR11_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr11::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr11::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR11 to value 0"]
impl crate::Resettable for GICD_ITARGETSR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
