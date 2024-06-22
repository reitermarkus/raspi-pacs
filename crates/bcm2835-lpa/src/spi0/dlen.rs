#[doc = "Register `DLEN` reader"]
pub type R = crate::R<DlenSpec>;
#[doc = "Register `DLEN` writer"]
pub type W = crate::W<DlenSpec>;
#[doc = "Field `DLEN` reader - Data length"]
pub type DlenR = crate::FieldReader<u16>;
#[doc = "Field `DLEN` writer - Data length"]
pub type DlenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    pub fn dlen(&self) -> DlenR {
        DlenR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLEN").field("dlen", &self.dlen()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Data length"]
    #[inline(always)]
    #[must_use]
    pub fn dlen(&mut self) -> DlenW<DlenSpec> {
        DlenW::new(self, 0)
    }
}
#[doc = "Data length\n\nYou can [`read`](crate::Reg::read) this register and get [`dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlenSpec;
impl crate::RegisterSpec for DlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlen::R`](R) reader structure"]
impl crate::Readable for DlenSpec {}
#[doc = "`write(|w| ..)` method takes [`dlen::W`](W) writer structure"]
impl crate::Writable for DlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DlenSpec {
    const RESET_VALUE: u32 = 0;
}
