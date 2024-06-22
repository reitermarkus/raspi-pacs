#[doc = "Register `GICD_ITARGETSR52` reader"]
pub type R = crate::R<GicdItargetsr52Spec>;
#[doc = "Register `GICD_ITARGETSR52` writer"]
pub type W = crate::W<GicdItargetsr52Spec>;
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type Int208R = crate::FieldReader;
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type Int208W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type Int209R = crate::FieldReader;
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type Int209W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type Int210R = crate::FieldReader;
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type Int210W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type Int211R = crate::FieldReader;
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type Int211W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> Int208R {
        Int208R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> Int209R {
        Int209R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> Int210R {
        Int210R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> Int211R {
        Int211R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR52")
            .field("int208", &self.int208())
            .field("int209", &self.int209())
            .field("int210", &self.int210())
            .field("int211", &self.int211())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> Int208W<GicdItargetsr52Spec> {
        Int208W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> Int209W<GicdItargetsr52Spec> {
        Int209W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> Int210W<GicdItargetsr52Spec> {
        Int210W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> Int211W<GicdItargetsr52Spec> {
        Int211W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 208 - 211\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr52Spec;
impl crate::RegisterSpec for GicdItargetsr52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr52::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr52Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr52::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR52 to value 0"]
impl crate::Resettable for GicdItargetsr52Spec {
    const RESET_VALUE: u32 = 0;
}
