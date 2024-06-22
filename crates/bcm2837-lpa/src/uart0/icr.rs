#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RIMIC` writer - RIMIC"]
pub type RimicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSMIC` writer - CTSMIC"]
pub type CtsmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDMIC` writer - DCDMIC"]
pub type DcdmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRMIC` writer - DSRMIC"]
pub type DsrmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIC` writer - RXIC"]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - TXIC"]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - RTIC"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - FEIC"]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - PEIC"]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - BEIC"]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - OEIC"]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - RIMIC"]
    #[inline(always)]
    #[must_use]
    pub fn rimic(&mut self) -> RimicW<IcrSpec> {
        RimicW::new(self, 0)
    }
    #[doc = "Bit 1 - CTSMIC"]
    #[inline(always)]
    #[must_use]
    pub fn ctsmic(&mut self) -> CtsmicW<IcrSpec> {
        CtsmicW::new(self, 1)
    }
    #[doc = "Bit 2 - DCDMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dcdmic(&mut self) -> DcdmicW<IcrSpec> {
        DcdmicW::new(self, 2)
    }
    #[doc = "Bit 3 - DSRMIC"]
    #[inline(always)]
    #[must_use]
    pub fn dsrmic(&mut self) -> DsrmicW<IcrSpec> {
        DsrmicW::new(self, 3)
    }
    #[doc = "Bit 4 - RXIC"]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RxicW<IcrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - TXIC"]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TxicW<IcrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - RTIC"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - FEIC"]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FeicW<IcrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - PEIC"]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PeicW<IcrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - BEIC"]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BeicW<IcrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - OEIC"]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OeicW<IcrSpec> {
        OeicW::new(self, 10)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
