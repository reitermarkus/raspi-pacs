#[doc = "Register `DOEPEACHMSK1` reader"]
pub type R = crate::R<Doepeachmsk1Spec>;
#[doc = "Register `DOEPEACHMSK1` writer"]
pub type W = crate::W<Doepeachmsk1Spec>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EpdmR = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EpdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - Timeout condition mask"]
pub type TomR = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask"]
pub type TomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type IttxfemskR = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type IttxfemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type InepnmmR = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type InepnmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type InepnemR = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type InepnemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFURM` reader - OUT packet error mask"]
pub type TxfurmR = crate::BitReader;
#[doc = "Field `TXFURM` writer - OUT packet error mask"]
pub type TxfurmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BimR = crate::BitReader;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRM` reader - Bubble error interrupt mask"]
pub type BerrmR = crate::BitReader;
#[doc = "Field `BERRM` writer - Bubble error interrupt mask"]
pub type BerrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK interrupt mask"]
pub type NakmR = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK interrupt mask"]
pub type NakmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETM` reader - NYET interrupt mask"]
pub type NyetmR = crate::BitReader;
#[doc = "Field `NYETM` writer - NYET interrupt mask"]
pub type NyetmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EpdmR {
        EpdmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&self) -> TomR {
        TomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> IttxfemskR {
        IttxfemskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> InepnmmR {
        InepnmmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> InepnemR {
        InepnemR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TxfurmR {
        TxfurmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BimR {
        BimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&self) -> BerrmR {
        BerrmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NakmR {
        NakmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&self) -> NyetmR {
        NyetmR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPEACHMSK1")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("tom", &self.tom())
            .field("ittxfemsk", &self.ittxfemsk())
            .field("inepnmm", &self.inepnmm())
            .field("inepnem", &self.inepnem())
            .field("txfurm", &self.txfurm())
            .field("bim", &self.bim())
            .field("berrm", &self.berrm())
            .field("nakm", &self.nakm())
            .field("nyetm", &self.nyetm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XfrcmW<Doepeachmsk1Spec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EpdmW<Doepeachmsk1Spec> {
        EpdmW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TomW<Doepeachmsk1Spec> {
        TomW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> IttxfemskW<Doepeachmsk1Spec> {
        IttxfemskW::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnmm(&mut self) -> InepnmmW<Doepeachmsk1Spec> {
        InepnmmW::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> InepnemW<Doepeachmsk1Spec> {
        InepnemW::new(self, 6)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TxfurmW<Doepeachmsk1Spec> {
        TxfurmW::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn bim(&mut self) -> BimW<Doepeachmsk1Spec> {
        BimW::new(self, 9)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BerrmW<Doepeachmsk1Spec> {
        BerrmW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NakmW<Doepeachmsk1Spec> {
        NakmW::new(self, 13)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nyetm(&mut self) -> NyetmW<Doepeachmsk1Spec> {
        NyetmW::new(self, 14)
    }
}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepeachmsk1Spec;
impl crate::RegisterSpec for Doepeachmsk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepeachmsk1::R`](R) reader structure"]
impl crate::Readable for Doepeachmsk1Spec {}
#[doc = "`write(|w| ..)` method takes [`doepeachmsk1::W`](W) writer structure"]
impl crate::Writable for Doepeachmsk1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for Doepeachmsk1Spec {
    const RESET_VALUE: u32 = 0;
}
