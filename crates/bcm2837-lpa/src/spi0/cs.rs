#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `CS` reader - Chip select"]
pub type CsR = crate::FieldReader;
#[doc = "Field `CS` writer - Chip select"]
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clear the FIFO(s)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clear {
    #[doc = "1: `1`"]
    Tx = 1,
    #[doc = "2: `10`"]
    Rx = 2,
    #[doc = "3: `11`"]
    Both = 3,
}
impl From<Clear> for u8 {
    #[inline(always)]
    fn from(variant: Clear) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clear {
    type Ux = u8;
}
impl crate::IsEnum for Clear {}
#[doc = "Field `CLEAR` reader - Clear the FIFO(s)"]
pub type ClearR = crate::FieldReader<Clear>;
impl ClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clear> {
        match self.bits {
            1 => Some(Clear::Tx),
            2 => Some(Clear::Rx),
            3 => Some(Clear::Both),
            _ => None,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == Clear::Tx
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == Clear::Rx
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Clear::Both
    }
}
#[doc = "Field `CLEAR` writer - Clear the FIFO(s)"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clear>;
impl<'a, REG> ClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Tx)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Rx)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Clear::Both)
    }
}
#[doc = "Field `CSPOL` reader - Chip select polarity"]
pub type CspolR = crate::BitReader;
#[doc = "Field `CSPOL` writer - Chip select polarity"]
pub type CspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA` reader - Transfer active"]
pub type TaR = crate::BitReader;
#[doc = "Field `TA` writer - Transfer active"]
pub type TaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - Enable DMA"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - Enable DMA"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTD` reader - Interrupt on done"]
pub type IntdR = crate::BitReader;
#[doc = "Field `INTD` writer - Interrupt on done"]
pub type IntdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR` reader - Interrupt on RX"]
pub type IntrR = crate::BitReader;
#[doc = "Field `INTR` writer - Interrupt on RX"]
pub type IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCS` reader - Automatically deassert chip select"]
pub type AdcsR = crate::BitReader;
#[doc = "Field `ADCS` writer - Automatically deassert chip select"]
pub type AdcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Read enable"]
pub type RenR = crate::BitReader;
#[doc = "Field `REN` writer - Read enable"]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN` reader - LoSSI enable"]
pub type LenR = crate::BitReader;
#[doc = "Field `LEN` writer - LoSSI enable"]
pub type LenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMONO` reader - "]
pub type LmonoR = crate::BitReader;
#[doc = "Field `LMONO` writer - "]
pub type LmonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE_EN` reader - "]
pub type TeEnR = crate::BitReader;
#[doc = "Field `TE_EN` writer - "]
pub type TeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Transfer is done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `RXD` reader - RX FIFO contains data"]
pub type RxdR = crate::BitReader;
#[doc = "Field `TXD` reader - TX FIFO can accept data"]
pub type TxdR = crate::BitReader;
#[doc = "Field `RXR` reader - RX FIFO has data to be read"]
pub type RxrR = crate::BitReader;
#[doc = "Field `RXF` reader - RX FIFO full"]
pub type RxfR = crate::BitReader;
#[doc = "Field `CSPOL0` reader - Chip select 0 polarity"]
pub type Cspol0R = crate::BitReader;
#[doc = "Field `CSPOL0` writer - Chip select 0 polarity"]
pub type Cspol0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPOL1` reader - Chip select 1 polarity"]
pub type Cspol1R = crate::BitReader;
#[doc = "Field `CSPOL1` writer - Chip select 1 polarity"]
pub type Cspol1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPOL2` reader - Chip select 2 polarity"]
pub type Cspol2R = crate::BitReader;
#[doc = "Field `CSPOL2` writer - Chip select 2 polarity"]
pub type Cspol2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_LEN` reader - Enable DMA in LoSSI mode"]
pub type DmaLenR = crate::BitReader;
#[doc = "Field `DMA_LEN` writer - Enable DMA in LoSSI mode"]
pub type DmaLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEN_LONG` reader - Enable long data word in LoSSI mode"]
pub type LenLongR = crate::BitReader;
#[doc = "Field `LEN_LONG` writer - Enable long data word in LoSSI mode"]
pub type LenLongW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Chip select"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO(s)"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Chip select polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CspolR {
        CspolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer active"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on done"]
    #[inline(always)]
    pub fn intd(&self) -> IntdR {
        IntdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    pub fn intr(&self) -> IntrR {
        IntrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Automatically deassert chip select"]
    #[inline(always)]
    pub fn adcs(&self) -> AdcsR {
        AdcsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Read enable"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LoSSI enable"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn lmono(&self) -> LmonoR {
        LmonoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn te_en(&self) -> TeEnR {
        TeEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer is done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO contains data"]
    #[inline(always)]
    pub fn rxd(&self) -> RxdR {
        RxdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TX FIFO can accept data"]
    #[inline(always)]
    pub fn txd(&self) -> TxdR {
        TxdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RX FIFO has data to be read"]
    #[inline(always)]
    pub fn rxr(&self) -> RxrR {
        RxrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RX FIFO full"]
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Chip select 0 polarity"]
    #[inline(always)]
    pub fn cspol0(&self) -> Cspol0R {
        Cspol0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Chip select 1 polarity"]
    #[inline(always)]
    pub fn cspol1(&self) -> Cspol1R {
        Cspol1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Chip select 2 polarity"]
    #[inline(always)]
    pub fn cspol2(&self) -> Cspol2R {
        Cspol2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable DMA in LoSSI mode"]
    #[inline(always)]
    pub fn dma_len(&self) -> DmaLenR {
        DmaLenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable long data word in LoSSI mode"]
    #[inline(always)]
    pub fn len_long(&self) -> LenLongR {
        LenLongR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("len_long", &self.len_long())
            .field("dma_len", &self.dma_len())
            .field("cspol2", &self.cspol2())
            .field("cspol1", &self.cspol1())
            .field("cspol0", &self.cspol0())
            .field("rxf", &self.rxf())
            .field("rxr", &self.rxr())
            .field("txd", &self.txd())
            .field("rxd", &self.rxd())
            .field("done", &self.done())
            .field("te_en", &self.te_en())
            .field("lmono", &self.lmono())
            .field("len", &self.len())
            .field("ren", &self.ren())
            .field("adcs", &self.adcs())
            .field("intr", &self.intr())
            .field("intd", &self.intd())
            .field("dmaen", &self.dmaen())
            .field("ta", &self.ta())
            .field("cspol", &self.cspol())
            .field("clear", &self.clear())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .field("cs", &self.cs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Chip select"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<CsSpec> {
        CsW::new(self, 0)
    }
    #[doc = "Bit 2 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<CsSpec> {
        CphaW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<CsSpec> {
        CpolW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Clear the FIFO(s)"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<CsSpec> {
        ClearW::new(self, 4)
    }
    #[doc = "Bit 6 - Chip select polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol(&mut self) -> CspolW<CsSpec> {
        CspolW::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer active"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TaW<CsSpec> {
        TaW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<CsSpec> {
        DmaenW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on done"]
    #[inline(always)]
    #[must_use]
    pub fn intd(&mut self) -> IntdW<CsSpec> {
        IntdW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> IntrW<CsSpec> {
        IntrW::new(self, 10)
    }
    #[doc = "Bit 11 - Automatically deassert chip select"]
    #[inline(always)]
    #[must_use]
    pub fn adcs(&mut self) -> AdcsW<CsSpec> {
        AdcsW::new(self, 11)
    }
    #[doc = "Bit 12 - Read enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> RenW<CsSpec> {
        RenW::new(self, 12)
    }
    #[doc = "Bit 13 - LoSSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<CsSpec> {
        LenW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn lmono(&mut self) -> LmonoW<CsSpec> {
        LmonoW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn te_en(&mut self) -> TeEnW<CsSpec> {
        TeEnW::new(self, 15)
    }
    #[doc = "Bit 21 - Chip select 0 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol0(&mut self) -> Cspol0W<CsSpec> {
        Cspol0W::new(self, 21)
    }
    #[doc = "Bit 22 - Chip select 1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol1(&mut self) -> Cspol1W<CsSpec> {
        Cspol1W::new(self, 22)
    }
    #[doc = "Bit 23 - Chip select 2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cspol2(&mut self) -> Cspol2W<CsSpec> {
        Cspol2W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable DMA in LoSSI mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma_len(&mut self) -> DmaLenW<CsSpec> {
        DmaLenW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable long data word in LoSSI mode"]
    #[inline(always)]
    #[must_use]
    pub fn len_long(&mut self) -> LenLongW<CsSpec> {
        LenLongW::new(self, 25)
    }
}
#[doc = "Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0x0004_1000"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0x0004_1000;
}
