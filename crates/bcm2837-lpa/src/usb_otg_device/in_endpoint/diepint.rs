#[doc = "Register `DIEPINT` reader"]
pub type R = crate::R<DiepintSpec>;
#[doc = "Register `DIEPINT` writer"]
pub type W = crate::W<DiepintSpec>;
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - Timeout condition"]
pub type TocR = crate::BitReader;
#[doc = "Field `TOC` writer - Timeout condition"]
pub type TocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - IN token received when TxFIFO is empty"]
pub type IttxfeR = crate::BitReader;
#[doc = "Field `ITTXFE` writer - IN token received when TxFIFO is empty"]
pub type IttxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - IN endpoint NAK effective"]
pub type InepneR = crate::BitReader;
#[doc = "Field `INEPNE` writer - IN endpoint NAK effective"]
pub type InepneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` reader - Transmit Fifo Underrun"]
pub type TxfifoudrnR = crate::BitReader;
#[doc = "Field `TXFIFOUDRN` writer - Transmit Fifo Underrun"]
pub type TxfifoudrnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNA` reader - Buffer not available interrupt"]
pub type BnaR = crate::BitReader;
#[doc = "Field `BNA` writer - Buffer not available interrupt"]
pub type BnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub type PktdrpstsR = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub type PktdrpstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Babble error interrupt"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Babble error interrupt"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&self) -> IttxfeR {
        IttxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&self) -> InepneR {
        InepneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TxfifoudrnR {
        TxfifoudrnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&self) -> BnaR {
        BnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PktdrpstsR {
        PktdrpstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT")
            .field("xfrc", &self.xfrc())
            .field("epdisd", &self.epdisd())
            .field("toc", &self.toc())
            .field("ittxfe", &self.ittxfe())
            .field("inepne", &self.inepne())
            .field("txfe", &self.txfe())
            .field("txfifoudrn", &self.txfifoudrn())
            .field("bna", &self.bna())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("berr", &self.berr())
            .field("nak", &self.nak())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XfrcW<DiepintSpec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EpdisdW<DiepintSpec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TocW<DiepintSpec> {
        TocW::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> IttxfeW<DiepintSpec> {
        IttxfeW::new(self, 4)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    #[must_use]
    pub fn inepne(&mut self) -> InepneW<DiepintSpec> {
        InepneW::new(self, 6)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrn(&mut self) -> TxfifoudrnW<DiepintSpec> {
        TxfifoudrnW::new(self, 8)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BnaW<DiepintSpec> {
        BnaW::new(self, 9)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PktdrpstsW<DiepintSpec> {
        PktdrpstsW::new(self, 11)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BerrW<DiepintSpec> {
        BerrW::new(self, 12)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<DiepintSpec> {
        NakW::new(self, 13)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiepintSpec;
impl crate::RegisterSpec for DiepintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint::R`](R) reader structure"]
impl crate::Readable for DiepintSpec {}
#[doc = "`write(|w| ..)` method takes [`diepint::W`](W) writer structure"]
impl crate::Writable for DiepintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT to value 0x80"]
impl crate::Resettable for DiepintSpec {
    const RESET_VALUE: u32 = 0x80;
}
