#[doc = "Register `DOEPINT` reader"]
pub type R = crate::R<DoepintSpec>;
#[doc = "Register `DOEPINT` writer"]
pub type W = crate::W<DoepintSpec>;
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - SETUP phase done"]
pub type StupR = crate::BitReader;
#[doc = "Field `STUP` writer - SETUP phase done"]
pub type StupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub type OtepdisR = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub type OtepdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2bstupR = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2bstupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET interrupt"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - NYET interrupt"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> StupR {
        StupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OtepdisR {
        OtepdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2bstupR {
        B2bstupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("stup", &self.stup())
            .field("otepdis", &self.otepdis())
            .field("b2bstup", &self.b2bstup())
            .field("nyet", &self.nyet())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XfrcW<DoepintSpec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EpdisdW<DoepintSpec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> StupW<DoepintSpec> {
        StupW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OtepdisW<DoepintSpec> {
        OtepdisW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2bstupW<DoepintSpec> {
        B2bstupW::new(self, 6)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NyetW<DoepintSpec> {
        NyetW::new(self, 14)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepintSpec;
impl crate::RegisterSpec for DoepintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint::R`](R) reader structure"]
impl crate::Readable for DoepintSpec {}
#[doc = "`write(|w| ..)` method takes [`doepint::W`](W) writer structure"]
impl crate::Writable for DoepintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT to value 0x80"]
impl crate::Resettable for DoepintSpec {
    const RESET_VALUE: u32 = 0x80;
}
