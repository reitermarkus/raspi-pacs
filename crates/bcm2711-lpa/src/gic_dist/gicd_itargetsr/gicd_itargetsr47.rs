#[doc = "Register `GICD_ITARGETSR47` reader"]
pub type R = crate::R<GicdItargetsr47Spec>;
#[doc = "Register `GICD_ITARGETSR47` writer"]
pub type W = crate::W<GicdItargetsr47Spec>;
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type Int188R = crate::FieldReader;
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type Int188W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type Int189R = crate::FieldReader;
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type Int189W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type Int190R = crate::FieldReader;
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type Int190W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type Int191R = crate::FieldReader;
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type Int191W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> Int188R {
        Int188R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> Int189R {
        Int189R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> Int190R {
        Int190R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> Int191R {
        Int191R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR47")
            .field("int188", &self.int188())
            .field("int189", &self.int189())
            .field("int190", &self.int190())
            .field("int191", &self.int191())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> Int188W<GicdItargetsr47Spec> {
        Int188W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> Int189W<GicdItargetsr47Spec> {
        Int189W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> Int190W<GicdItargetsr47Spec> {
        Int190W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> Int191W<GicdItargetsr47Spec> {
        Int191W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 188 - 191\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr47Spec;
impl crate::RegisterSpec for GicdItargetsr47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr47::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr47Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr47::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR47 to value 0"]
impl crate::Resettable for GicdItargetsr47Spec {
    const RESET_VALUE: u32 = 0;
}
