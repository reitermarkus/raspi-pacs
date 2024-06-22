#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<Status0Spec>;
#[doc = "Field `EMPTY` reader - "]
pub type EmptyR = crate::BitReader;
#[doc = "Field `FULL` reader - "]
pub type FullR = crate::BitReader;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS0")
            .field("full", &self.full())
            .field("empty", &self.empty())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status0Spec;
impl crate::RegisterSpec for Status0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for Status0Spec {}
