#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `CDIV` reader - Divide the source clock"]
pub type CdivR = crate::FieldReader<u16>;
#[doc = "Field `CDIV` writer - Divide the source clock"]
pub type CdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Divide the source clock"]
    #[inline(always)]
    pub fn cdiv(&self) -> CdivR {
        CdivR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIV").field("cdiv", &self.cdiv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Divide the source clock"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CdivW<DivSpec> {
        CdivW::new(self, 0)
    }
}
#[doc = "Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0x05dc"]
impl crate::Resettable for DivSpec {
    const RESET_VALUE: u32 = 0x05dc;
}
