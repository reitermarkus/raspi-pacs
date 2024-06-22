#[doc = "Register `GICD_ITARGETSR41` reader"]
pub type R = crate::R<GicdItargetsr41Spec>;
#[doc = "Register `GICD_ITARGETSR41` writer"]
pub type W = crate::W<GicdItargetsr41Spec>;
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type Int164R = crate::FieldReader;
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type Int164W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type Int165R = crate::FieldReader;
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type Int165W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type Int166R = crate::FieldReader;
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type Int166W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type Int167R = crate::FieldReader;
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type Int167W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> Int164R {
        Int164R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> Int165R {
        Int165R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> Int166R {
        Int166R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> Int167R {
        Int167R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR41")
            .field("int164", &self.int164())
            .field("int165", &self.int165())
            .field("int166", &self.int166())
            .field("int167", &self.int167())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> Int164W<GicdItargetsr41Spec> {
        Int164W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> Int165W<GicdItargetsr41Spec> {
        Int165W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> Int166W<GicdItargetsr41Spec> {
        Int166W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> Int167W<GicdItargetsr41Spec> {
        Int167W::new(self, 24)
    }
}
#[doc = "Interrupt Processor Target 164 - 167\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_itargetsr41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_itargetsr41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdItargetsr41Spec;
impl crate::RegisterSpec for GicdItargetsr41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr41::R`](R) reader structure"]
impl crate::Readable for GicdItargetsr41Spec {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr41::W`](W) writer structure"]
impl crate::Writable for GicdItargetsr41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR41 to value 0"]
impl crate::Resettable for GicdItargetsr41Spec {
    const RESET_VALUE: u32 = 0;
}
