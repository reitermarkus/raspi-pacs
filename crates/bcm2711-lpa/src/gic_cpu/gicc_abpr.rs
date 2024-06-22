#[doc = "Register `GICC_ABPR` reader"]
pub type R = crate::R<GiccAbprSpec>;
#[doc = "Register `GICC_ABPR` writer"]
pub type W = crate::W<GiccAbprSpec>;
#[doc = "Field `BINARY_POINT` reader - Split point between group priority and subpriority"]
pub type BinaryPointR = crate::FieldReader;
#[doc = "Field `BINARY_POINT` writer - Split point between group priority and subpriority"]
pub type BinaryPointW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    pub fn binary_point(&self) -> BinaryPointR {
        BinaryPointR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICC_ABPR")
            .field("binary_point", &self.binary_point())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BinaryPointW<GiccAbprSpec> {
        BinaryPointW::new(self, 0)
    }
}
#[doc = "Aliased Binary Point\n\nYou can [`read`](crate::Reg::read) this register and get [`gicc_abpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_abpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccAbprSpec;
impl crate::RegisterSpec for GiccAbprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicc_abpr::R`](R) reader structure"]
impl crate::Readable for GiccAbprSpec {}
#[doc = "`write(|w| ..)` method takes [`gicc_abpr::W`](W) writer structure"]
impl crate::Writable for GiccAbprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_ABPR to value 0"]
impl crate::Resettable for GiccAbprSpec {
    const RESET_VALUE: u32 = 0;
}
