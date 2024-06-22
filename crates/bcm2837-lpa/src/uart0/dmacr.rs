#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `RXDMAE` reader - RXDMAE"]
pub type RxdmaeR = crate::BitReader;
#[doc = "Field `RXDMAE` writer - RXDMAE"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAE` reader - TXDMAE"]
pub type TxdmaeR = crate::BitReader;
#[doc = "Field `TXDMAE` writer - TXDMAE"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAONERR` reader - DMAONERR"]
pub type DmaonerrR = crate::BitReader;
#[doc = "Field `DMAONERR` writer - DMAONERR"]
pub type DmaonerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DmaonerrR {
        DmaonerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACR")
            .field("rxdmae", &self.rxdmae())
            .field("txdmae", &self.txdmae())
            .field("dmaonerr", &self.dmaonerr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - RXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<DmacrSpec> {
        RxdmaeW::new(self, 0)
    }
    #[doc = "Bit 1 - TXDMAE"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<DmacrSpec> {
        TxdmaeW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAONERR"]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DmaonerrW<DmacrSpec> {
        DmaonerrW::new(self, 2)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DmacrSpec {
    const RESET_VALUE: u32 = 0;
}
