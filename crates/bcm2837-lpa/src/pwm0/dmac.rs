#[doc = "Register `DMAC` reader"]
pub type R = crate::R<DmacSpec>;
#[doc = "Register `DMAC` writer"]
pub type W = crate::W<DmacSpec>;
#[doc = "Field `DREQ` reader - DMA threshold for DREQ signal"]
pub type DreqR = crate::FieldReader;
#[doc = "Field `DREQ` writer - DMA threshold for DREQ signal"]
pub type DreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PANIC` reader - DMA threshold for panic signal"]
pub type PanicR = crate::FieldReader;
#[doc = "Field `PANIC` writer - DMA threshold for panic signal"]
pub type PanicW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENAB` reader - DMA enabled"]
pub type EnabR = crate::BitReader;
#[doc = "Field `ENAB` writer - DMA enabled"]
pub type EnabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    pub fn dreq(&self) -> DreqR {
        DreqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    pub fn panic(&self) -> PanicR {
        PanicR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    pub fn enab(&self) -> EnabR {
        EnabR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC")
            .field("enab", &self.enab())
            .field("panic", &self.panic())
            .field("dreq", &self.dreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DreqW<DmacSpec> {
        DreqW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    #[must_use]
    pub fn panic(&mut self) -> PanicW<DmacSpec> {
        PanicW::new(self, 8)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> EnabW<DmacSpec> {
        EnabW::new(self, 31)
    }
}
#[doc = "DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacSpec;
impl crate::RegisterSpec for DmacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac::R`](R) reader structure"]
impl crate::Readable for DmacSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac::W`](W) writer structure"]
impl crate::Writable for DmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC to value 0"]
impl crate::Resettable for DmacSpec {
    const RESET_VALUE: u32 = 0;
}
