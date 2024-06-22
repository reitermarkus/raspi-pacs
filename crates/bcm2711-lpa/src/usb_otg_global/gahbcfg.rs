#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GintR = crate::BitReader;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Maximum AXI burst length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Burst {
    #[doc = "0: `0`"]
    _4 = 0,
    #[doc = "1: `1`"]
    _3 = 1,
    #[doc = "2: `10`"]
    _2 = 2,
    #[doc = "3: `11`"]
    _1 = 3,
}
impl From<Burst> for u8 {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Burst {
    type Ux = u8;
}
impl crate::IsEnum for Burst {}
#[doc = "Field `AXI_BURST` reader - Maximum AXI burst length"]
pub type AxiBurstR = crate::FieldReader<Burst>;
impl AxiBurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            0 => Burst::_4,
            1 => Burst::_3,
            2 => Burst::_2,
            3 => Burst::_1,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Burst::_4
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Burst::_3
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Burst::_2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Burst::_1
    }
}
#[doc = "Field `AXI_BURST` writer - Maximum AXI burst length"]
pub type AxiBurstW<'a, REG> = crate::FieldWriter<'a, REG, 2, Burst, crate::Safe>;
impl<'a, REG> AxiBurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::_4)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::_3)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::_2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::_1)
    }
}
#[doc = "Field `AXI_WAIT` reader - Wait for all AXI writes before signaling DMA"]
pub type AxiWaitR = crate::BitReader;
#[doc = "Field `AXI_WAIT` writer - Wait for all AXI writes before signaling DMA"]
pub type AxiWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TxfelvlR = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TxfelvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PtxfelvlR = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PtxfelvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GintR {
        GintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Maximum AXI burst length"]
    #[inline(always)]
    pub fn axi_burst(&self) -> AxiBurstR {
        AxiBurstR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    pub fn axi_wait(&self) -> AxiWaitR {
        AxiWaitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TxfelvlR {
        TxfelvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PtxfelvlR {
        PtxfelvlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("gint", &self.gint())
            .field("axi_wait", &self.axi_wait())
            .field("axi_burst", &self.axi_burst())
            .field("dmaen", &self.dmaen())
            .field("txfelvl", &self.txfelvl())
            .field("ptxfelvl", &self.ptxfelvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GintW<GahbcfgSpec> {
        GintW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Maximum AXI burst length"]
    #[inline(always)]
    #[must_use]
    pub fn axi_burst(&mut self) -> AxiBurstW<GahbcfgSpec> {
        AxiBurstW::new(self, 1)
    }
    #[doc = "Bit 4 - Wait for all AXI writes before signaling DMA"]
    #[inline(always)]
    #[must_use]
    pub fn axi_wait(&mut self) -> AxiWaitW<GahbcfgSpec> {
        AxiWaitW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<GahbcfgSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TxfelvlW<GahbcfgSpec> {
        TxfelvlW::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PtxfelvlW<GahbcfgSpec> {
        PtxfelvlW::new(self, 8)
    }
}
#[doc = "OTG_HS AHB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {
    const RESET_VALUE: u32 = 0;
}
