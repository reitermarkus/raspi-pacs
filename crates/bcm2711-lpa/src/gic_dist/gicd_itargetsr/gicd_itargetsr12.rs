#[doc = "Register `GICD_ITARGETSR12` reader"]
pub type R = crate::R<GicdItargetsr12Spec>;
#[doc = "Register `GICD_ITARGETSR12` writer"]
pub type W = crate::W<GicdItargetsr12Spec>;
#[doc = "Field `INT48` reader - Interrupt 48"]
pub type Int48R = crate::FieldReader;
#[doc = "Field `INT48` writer - Interrupt 48"]
pub type Int48W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT49` reader - Interrupt 49"]
pub type Int49R = crate::FieldReader;
#[doc = "Field `INT49` writer - Interrupt 49"]
pub type Int49W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT50` reader - Interrupt 50"]
pub type Int50R = crate::FieldReader;
#[doc = "Field `INT50` writer - Interrupt 50"]
pub type Int50W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT51` reader - Interrupt 51"]
pub type Int51R = crate::FieldReader;
#[doc = "Field `INT51` writer - Interrupt 51"]
pub type Int51W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    pub fn int48(&self) -> Int48R {
        Int48R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    pub fn int49(&self) -> Int49R {
        Int49R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    pub fn int50(&self) -> Int50R {
        Int50R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    pub fn int51(&self) -> Int51R {
        Int51R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR12")
            .field("int48", &self.int48())
            .field("int49", &self.int49())
            .field("int50", &self.int50())
            .field("int51", &self.int51())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 48"]
    #[inline(always)]
    #[must_use]
    pub fn int48(&mut self) -> Int48W<GicdItargetsr12Spec> {
        Int48W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 49"]
    #[inline(always)]
    #[must_use]
    pub fn int49(&mut self) -> Int49W<GicdItargetsr12Spec> {
        Int49W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 50"]
    #[inline(always)]
    #[must_use]
    pub fn int50(&mut self) -> Int50W<GicdItargetsr12Spec> {
        Int50W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 51"]
    #[inline(always)]
    #[must_use]
    pub fn int51(&mut self) -> Int51W<GicdItargetsr12Spec> {
        Int51W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 48 - 51\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr12Spec;
impl crate::RegisterSpec for GicdItargetsr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr12::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr12Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr12::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR12 to value 0"]
impl crate::Resettable for GicdItargetsr12Spec {
    const RESET_VALUE: u32 = 0;
}
