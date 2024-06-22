#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `FE` writer - FE"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` writer - PE"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` writer - BE"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` writer - OE"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<EcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<EcrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<EcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<EcrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<EcrSpec> {
        OeW::new(self, 3)
    }
}
#[doc = "Error Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
