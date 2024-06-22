#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `BIT_COUNT` reader - Number of bits left to be processed."]
pub type BitCountR = crate::FieldReader;
#[doc = "Field `BIT_COUNT` writer - Number of bits left to be processed."]
pub type BitCountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BUSY` reader - Indicates a transfer is ongoing"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Indicates a transfer is ongoing"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EMPTY` reader - RX FIFO is empty"]
pub type RxEmptyR = crate::BitReader;
#[doc = "Field `RX_EMPTY` writer - RX FIFO is empty"]
pub type RxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - RX FIFO is full"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `RX_FULL` writer - RX FIFO is full"]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - TX FIFO is empty"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - TX FIFO is empty"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FULL` reader - TX FIFO is full"]
pub type TxFullR = crate::BitReader;
#[doc = "Field `TX_FULL` writer - TX FIFO is full"]
pub type TxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LEVEL` reader - Number of entries in RX FIFO"]
pub type RxLevelR = crate::FieldReader;
#[doc = "Field `RX_LEVEL` writer - Number of entries in RX FIFO"]
pub type RxLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_LEVEL` reader - Number of entries in TX FIFO"]
pub type TxLevelR = crate::FieldReader;
#[doc = "Field `TX_LEVEL` writer - Number of entries in TX FIFO"]
pub type TxLevelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - Number of bits left to be processed."]
    #[inline(always)]
    pub fn bit_count(&self) -> BitCountR {
        BitCountR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Indicates a transfer is ongoing"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO is empty"]
    #[inline(always)]
    pub fn rx_empty(&self) -> RxEmptyR {
        RxEmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO is full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO is empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TX FIFO is full"]
    #[inline(always)]
    pub fn tx_full(&self) -> TxFullR {
        TxFullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of entries in RX FIFO"]
    #[inline(always)]
    pub fn rx_level(&self) -> RxLevelR {
        RxLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of entries in TX FIFO"]
    #[inline(always)]
    pub fn tx_level(&self) -> TxLevelR {
        TxLevelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("tx_level", &self.tx_level())
            .field("rx_level", &self.rx_level())
            .field("tx_full", &self.tx_full())
            .field("tx_empty", &self.tx_empty())
            .field("rx_full", &self.rx_full())
            .field("rx_empty", &self.rx_empty())
            .field("busy", &self.busy())
            .field("bit_count", &self.bit_count())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Number of bits left to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn bit_count(&mut self) -> BitCountW<StatSpec> {
        BitCountW::new(self, 0)
    }
    #[doc = "Bit 6 - Indicates a transfer is ongoing"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatSpec> {
        BusyW::new(self, 6)
    }
    #[doc = "Bit 7 - RX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_empty(&mut self) -> RxEmptyW<StatSpec> {
        RxEmptyW::new(self, 7)
    }
    #[doc = "Bit 8 - RX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RxFullW<StatSpec> {
        RxFullW::new(self, 8)
    }
    #[doc = "Bit 9 - TX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<StatSpec> {
        TxEmptyW::new(self, 9)
    }
    #[doc = "Bit 10 - TX FIFO is full"]
    #[inline(always)]
    #[must_use]
    pub fn tx_full(&mut self) -> TxFullW<StatSpec> {
        TxFullW::new(self, 10)
    }
    #[doc = "Bits 16:19 - Number of entries in RX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rx_level(&mut self) -> RxLevelW<StatSpec> {
        RxLevelW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of entries in TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_level(&mut self) -> TxLevelW<StatSpec> {
        TxLevelW::new(self, 24)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
