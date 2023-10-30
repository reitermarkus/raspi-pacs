#[doc = "Register `ARG2` reader"]
pub type R = crate::R<ARG2_SPEC>;
#[doc = "Register `ARG2` writer"]
pub type W = crate::W<ARG2_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ARG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
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
#[doc = "Argument for ACMD23 command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARG2_SPEC;
impl crate::RegisterSpec for ARG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg2::R`](R) reader structure"]
impl crate::Readable for ARG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arg2::W`](W) writer structure"]
impl crate::Writable for ARG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARG2 to value 0"]
impl crate::Resettable for ARG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}