#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "Register `S` writer"]
pub type W = crate::W<SSpec>;
#[doc = "Field `TA` reader - Transfer active"]
pub type TaR = crate::BitReader;
#[doc = "Field `DONE` reader - Transfer done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Transfer done"]
pub type DoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXW` reader - FIFO needs to be written"]
pub type TxwR = crate::BitReader;
#[doc = "Field `RXR` reader - FIFO needs to be read"]
pub type RxrR = crate::BitReader;
#[doc = "Field `TXD` reader - FIFO has space for at least one byte"]
pub type TxdR = crate::BitReader;
#[doc = "Field `RXD` reader - FIFO contains at least one byte"]
pub type RxdR = crate::BitReader;
#[doc = "Field `TXE` reader - FIFO is empty. Nothing to transmit"]
pub type TxeR = crate::BitReader;
#[doc = "Field `RXF` reader - FIFO is full. Can't receive anything else"]
pub type RxfR = crate::BitReader;
#[doc = "Field `ERR` reader - Error: No ack"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - Error: No ack"]
pub type ErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLKT` reader - Clock stretch timeout"]
pub type ClktR = crate::BitReader;
#[doc = "Field `CLKT` writer - Clock stretch timeout"]
pub type ClktW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer active"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO needs to be written"]
    #[inline(always)]
    pub fn txw(&self) -> TxwR {
        TxwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO needs to be read"]
    #[inline(always)]
    pub fn rxr(&self) -> RxrR {
        RxrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO has space for at least one byte"]
    #[inline(always)]
    pub fn txd(&self) -> TxdR {
        TxdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO contains at least one byte"]
    #[inline(always)]
    pub fn rxd(&self) -> RxdR {
        RxdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO is empty. Nothing to transmit"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO is full. Can't receive anything else"]
    #[inline(always)]
    pub fn rxf(&self) -> RxfR {
        RxfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error: No ack"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock stretch timeout"]
    #[inline(always)]
    pub fn clkt(&self) -> ClktR {
        ClktR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S")
            .field("clkt", &self.clkt())
            .field("err", &self.err())
            .field("rxf", &self.rxf())
            .field("txe", &self.txe())
            .field("rxd", &self.rxd())
            .field("txd", &self.txd())
            .field("rxr", &self.rxr())
            .field("txw", &self.txw())
            .field("done", &self.done())
            .field("ta", &self.ta())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<SSpec> {
        DoneW::new(self, 1)
    }
    #[doc = "Bit 8 - Error: No ack"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<SSpec> {
        ErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock stretch timeout"]
    #[inline(always)]
    #[must_use]
    pub fn clkt(&mut self) -> ClktW<SSpec> {
        ClktW::new(self, 9)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSpec;
impl crate::RegisterSpec for SSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for SSpec {}
#[doc = "`write(|w| ..)` method takes [`s::W`](W) writer structure"]
impl crate::Writable for SSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0302;
}
#[doc = "`reset()` method sets S to value 0x50"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u32 = 0x50;
}
