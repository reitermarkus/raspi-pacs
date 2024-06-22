#[doc = "Register `HCDMA` reader"]
pub type R = crate::R<HcdmaSpec>;
#[doc = "Register `HCDMA` writer"]
pub type W = crate::W<HcdmaSpec>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DmaaddrW<HcdmaSpec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "DMA address\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcdmaSpec;
impl crate::RegisterSpec for HcdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdma::R`](R) reader structure"]
impl crate::Readable for HcdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`hcdma::W`](W) writer structure"]
impl crate::Writable for HcdmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCDMA to value 0"]
impl crate::Resettable for HcdmaSpec {
    const RESET_VALUE: u32 = 0;
}
