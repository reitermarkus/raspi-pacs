#[doc = "Register `GRXFSIZ` reader"]
pub type R = crate::R<GrxfsizSpec>;
#[doc = "Register `GRXFSIZ` writer"]
pub type W = crate::W<GrxfsizSpec>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RxfdR = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RxfdR {
        RxfdR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXFSIZ")
            .field("rxfd", &self.rxfd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RxfdW<GrxfsizSpec> {
        RxfdW::new(self, 0)
    }
}
#[doc = "OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrxfsizSpec;
impl crate::RegisterSpec for GrxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxfsiz::R`](R) reader structure"]
impl crate::Readable for GrxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`grxfsiz::W`](W) writer structure"]
impl crate::Writable for GrxfsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRXFSIZ to value 0x0200"]
impl crate::Resettable for GrxfsizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
