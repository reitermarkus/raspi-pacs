#[doc = "Register `GICD_ITARGETSR23` reader"]
pub type R = crate::R<GicdItargetsr23Spec>;
#[doc = "Register `GICD_ITARGETSR23` writer"]
pub type W = crate::W<GicdItargetsr23Spec>;
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type Int92R = crate::FieldReader;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type Int92W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type Int93R = crate::FieldReader;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type Int93W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type Int94R = crate::FieldReader;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type Int94W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type Int95R = crate::FieldReader;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type Int95W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> Int92R {
        Int92R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> Int93R {
        Int93R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> Int94R {
        Int94R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> Int95R {
        Int95R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR23")
            .field("int92", &self.int92())
            .field("int93", &self.int93())
            .field("int94", &self.int94())
            .field("int95", &self.int95())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> Int92W<GicdItargetsr23Spec> {
        Int92W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> Int93W<GicdItargetsr23Spec> {
        Int93W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> Int94W<GicdItargetsr23Spec> {
        Int94W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> Int95W<GicdItargetsr23Spec> {
        Int95W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 92 - 95\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr23Spec;
impl crate::RegisterSpec for GicdItargetsr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr23::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr23Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr23::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR23 to value 0"]
impl crate::Resettable for GicdItargetsr23Spec {
    const RESET_VALUE: u32 = 0;
}
