#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `DATA_READY` reader - Receive FIFO has at least 1 byte"]
pub type DataReadyR = crate::BitReader;
#[doc = "Field `DATA_READY` writer - Receive FIFO has at least 1 byte"]
pub type DataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_READY` reader - Transmit FIFO is empty"]
pub type TxReadyR = crate::BitReader;
#[doc = "Field `TX_READY` writer - Transmit FIFO is empty"]
pub type TxReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    pub fn data_ready(&self) -> DataReadyR {
        DataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO is empty"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TxReadyR {
        TxReadyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("tx_ready", &self.tx_ready())
            .field("data_ready", &self.data_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO has at least 1 byte"]
    #[inline(always)]
    #[must_use]
    pub fn data_ready(&mut self) -> DataReadyW<IerSpec> {
        DataReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ready(&mut self) -> TxReadyW<IerSpec> {
        TxReadyW::new(self, 1)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
