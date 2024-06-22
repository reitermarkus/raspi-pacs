#[doc = "Register `LTOH` reader"]
pub type R = crate::R<LtohSpec>;
#[doc = "Register `LTOH` writer"]
pub type W = crate::W<LtohSpec>;
#[doc = "Field `TOH` reader - Output hold delay"]
pub type TohR = crate::FieldReader;
#[doc = "Field `TOH` writer - Output hold delay"]
pub type TohW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    pub fn toh(&self) -> TohR {
        TohR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTOH").field("toh", &self.toh()).finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Output hold delay"]
    #[inline(always)]
    #[must_use]
    pub fn toh(&mut self) -> TohW<LtohSpec> {
        TohW::new(self, 0)
    }
}
#[doc = "LoSSI output hold delay\n\nYou can [`read`](crate::Reg::read) this register and get [`ltoh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltoh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LtohSpec;
impl crate::RegisterSpec for LtohSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltoh::R`](R) reader structure"]
impl crate::Readable for LtohSpec {}
#[doc = "`write(|w| ..)` method takes [`ltoh::W`](W) writer structure"]
impl crate::Writable for LtohSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTOH to value 0x01"]
impl crate::Resettable for LtohSpec {
    const RESET_VALUE: u32 = 0x01;
}
