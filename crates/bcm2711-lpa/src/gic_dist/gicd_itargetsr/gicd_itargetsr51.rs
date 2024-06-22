#[doc = "Register `GICD_ITARGETSR51` reader"]
pub type R = crate::R<GicdItargetsr51Spec>;
#[doc = "Register `GICD_ITARGETSR51` writer"]
pub type W = crate::W<GicdItargetsr51Spec>;
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type Int204R = crate::FieldReader;
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type Int204W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type Int205R = crate::FieldReader;
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type Int205W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type Int206R = crate::FieldReader;
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type Int206W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type Int207R = crate::FieldReader;
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type Int207W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> Int204R {
        Int204R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> Int205R {
        Int205R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> Int206R {
        Int206R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> Int207R {
        Int207R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR51")
            .field("int204", &self.int204())
            .field("int205", &self.int205())
            .field("int206", &self.int206())
            .field("int207", &self.int207())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> Int204W<GicdItargetsr51Spec> {
        Int204W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> Int205W<GicdItargetsr51Spec> {
        Int205W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> Int206W<GicdItargetsr51Spec> {
        Int206W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> Int207W<GicdItargetsr51Spec> {
        Int207W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 204 - 207\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr51Spec;
impl crate::RegisterSpec for GicdItargetsr51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr51::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr51Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr51::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR51 to value 0"]
impl crate::Resettable for GicdItargetsr51Spec {
    const RESET_VALUE: u32 = 0;
}
