#[doc = "Register `GICD_ITARGETSR55` reader"]
pub type R = crate::R<GicdItargetsr55Spec>;
#[doc = "Register `GICD_ITARGETSR55` writer"]
pub type W = crate::W<GicdItargetsr55Spec>;
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type Int220R = crate::FieldReader;
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type Int220W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type Int221R = crate::FieldReader;
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type Int221W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type Int222R = crate::FieldReader;
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type Int222W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type Int223R = crate::FieldReader;
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type Int223W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> Int220R {
        Int220R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> Int221R {
        Int221R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> Int222R {
        Int222R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> Int223R {
        Int223R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR55")
            .field("int220", &self.int220())
            .field("int221", &self.int221())
            .field("int222", &self.int222())
            .field("int223", &self.int223())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> Int220W<GicdItargetsr55Spec> {
        Int220W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> Int221W<GicdItargetsr55Spec> {
        Int221W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> Int222W<GicdItargetsr55Spec> {
        Int222W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> Int223W<GicdItargetsr55Spec> {
        Int223W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 220 - 223\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr55Spec;
impl crate::RegisterSpec for GicdItargetsr55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr55::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr55Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr55::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR55 to value 0"]
impl crate::Resettable for GicdItargetsr55Spec {
    const RESET_VALUE: u32 = 0;
}
