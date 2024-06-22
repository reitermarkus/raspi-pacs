#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Field `CTS` reader - CTS"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - CTS"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR` reader - DSR"]
pub type DsrR = crate::BitReader;
#[doc = "Field `DSR` writer - DSR"]
pub type DsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` reader - DCD"]
pub type DcdR = crate::BitReader;
#[doc = "Field `DCD` writer - DCD"]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFE` reader - RXFE"]
pub type RxfeR = crate::BitReader;
#[doc = "Field `RXFE` writer - RXFE"]
pub type RxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFF` reader - TXFF"]
pub type TxffR = crate::BitReader;
#[doc = "Field `TXFF` writer - TXFF"]
pub type TxffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFF` reader - RXFF"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFF` writer - RXFF"]
pub type RxffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFE` writer - TXFE"]
pub type TxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - RI"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - RI"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FR")
            .field("cts", &self.cts())
            .field("dsr", &self.dsr())
            .field("dcd", &self.dcd())
            .field("busy", &self.busy())
            .field("rxfe", &self.rxfe())
            .field("txff", &self.txff())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("ri", &self.ri())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<FrSpec> {
        CtsW::new(self, 0)
    }
    #[doc = "Bit 1 - DSR"]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DsrW<FrSpec> {
        DsrW::new(self, 1)
    }
    #[doc = "Bit 2 - DCD"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DcdW<FrSpec> {
        DcdW::new(self, 2)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<FrSpec> {
        BusyW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFE"]
    #[inline(always)]
    #[must_use]
    pub fn rxfe(&mut self) -> RxfeW<FrSpec> {
        RxfeW::new(self, 4)
    }
    #[doc = "Bit 5 - TXFF"]
    #[inline(always)]
    #[must_use]
    pub fn txff(&mut self) -> TxffW<FrSpec> {
        TxffW::new(self, 5)
    }
    #[doc = "Bit 6 - RXFF"]
    #[inline(always)]
    #[must_use]
    pub fn rxff(&mut self) -> RxffW<FrSpec> {
        RxffW::new(self, 6)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    #[must_use]
    pub fn txfe(&mut self) -> TxfeW<FrSpec> {
        TxfeW::new(self, 7)
    }
    #[doc = "Bit 8 - RI"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<FrSpec> {
        RiW::new(self, 8)
    }
}
#[doc = "Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FR to value 0"]
impl crate::Resettable for FrSpec {
    const RESET_VALUE: u32 = 0;
}
