#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Register `STA` writer"]
pub type W = crate::W<StaSpec>;
#[doc = "Field `FULL1` reader - FIFO full"]
pub type Full1R = crate::BitReader;
#[doc = "Field `FULL1` writer - FIFO full"]
pub type Full1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPT1` reader - FIFO empty"]
pub type Empt1R = crate::BitReader;
#[doc = "Field `EMPT1` writer - FIFO empty"]
pub type Empt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WERR1` reader - FIFO write error"]
pub type Werr1R = crate::BitReader;
#[doc = "Field `WERR1` writer - FIFO write error"]
pub type Werr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR1` reader - FIFO read error"]
pub type Rerr1R = crate::BitReader;
#[doc = "Field `RERR1` writer - FIFO read error"]
pub type Rerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO1` reader - Channel 1 gap occurred"]
pub type Gapo1R = crate::BitReader;
#[doc = "Field `GAPO1` writer - Channel 1 gap occurred"]
pub type Gapo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO2` reader - Channel 2 gap occurred"]
pub type Gapo2R = crate::BitReader;
#[doc = "Field `GAPO2` writer - Channel 2 gap occurred"]
pub type Gapo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO3` reader - Channel 3 gap occurred"]
pub type Gapo3R = crate::BitReader;
#[doc = "Field `GAPO3` writer - Channel 3 gap occurred"]
pub type Gapo3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPO4` reader - Channel 4 gap occurred"]
pub type Gapo4R = crate::BitReader;
#[doc = "Field `GAPO4` writer - Channel 4 gap occurred"]
pub type Gapo4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bus error"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA1` reader - Channel 1 state"]
pub type Sta1R = crate::BitReader;
#[doc = "Field `STA1` writer - Channel 1 state"]
pub type Sta1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA2` reader - Channel 2 state"]
pub type Sta2R = crate::BitReader;
#[doc = "Field `STA2` writer - Channel 2 state"]
pub type Sta2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA3` reader - Channel 3 state"]
pub type Sta3R = crate::BitReader;
#[doc = "Field `STA3` writer - Channel 3 state"]
pub type Sta3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA4` reader - Channel 4 state"]
pub type Sta4R = crate::BitReader;
#[doc = "Field `STA4` writer - Channel 4 state"]
pub type Sta4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    pub fn full1(&self) -> Full1R {
        Full1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    pub fn empt1(&self) -> Empt1R {
        Empt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    pub fn werr1(&self) -> Werr1R {
        Werr1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    pub fn rerr1(&self) -> Rerr1R {
        Rerr1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    pub fn gapo1(&self) -> Gapo1R {
        Gapo1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    pub fn gapo2(&self) -> Gapo2R {
        Gapo2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    pub fn gapo3(&self) -> Gapo3R {
        Gapo3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    pub fn gapo4(&self) -> Gapo4R {
        Gapo4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    pub fn sta1(&self) -> Sta1R {
        Sta1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    pub fn sta2(&self) -> Sta2R {
        Sta2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    pub fn sta3(&self) -> Sta3R {
        Sta3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    pub fn sta4(&self) -> Sta4R {
        Sta4R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STA")
            .field("sta4", &self.sta4())
            .field("sta3", &self.sta3())
            .field("sta2", &self.sta2())
            .field("sta1", &self.sta1())
            .field("berr", &self.berr())
            .field("gapo4", &self.gapo4())
            .field("gapo3", &self.gapo3())
            .field("gapo2", &self.gapo2())
            .field("gapo1", &self.gapo1())
            .field("rerr1", &self.rerr1())
            .field("werr1", &self.werr1())
            .field("empt1", &self.empt1())
            .field("full1", &self.full1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> Full1W<StaSpec> {
        Full1W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn empt1(&mut self) -> Empt1W<StaSpec> {
        Empt1W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO write error"]
    #[inline(always)]
    #[must_use]
    pub fn werr1(&mut self) -> Werr1W<StaSpec> {
        Werr1W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO read error"]
    #[inline(always)]
    #[must_use]
    pub fn rerr1(&mut self) -> Rerr1W<StaSpec> {
        Rerr1W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo1(&mut self) -> Gapo1W<StaSpec> {
        Gapo1W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo2(&mut self) -> Gapo2W<StaSpec> {
        Gapo2W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 3 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo3(&mut self) -> Gapo3W<StaSpec> {
        Gapo3W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 4 gap occurred"]
    #[inline(always)]
    #[must_use]
    pub fn gapo4(&mut self) -> Gapo4W<StaSpec> {
        Gapo4W::new(self, 7)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BerrW<StaSpec> {
        BerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta1(&mut self) -> Sta1W<StaSpec> {
        Sta1W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta2(&mut self) -> Sta2W<StaSpec> {
        Sta2W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta3(&mut self) -> Sta3W<StaSpec> {
        Sta3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 state"]
    #[inline(always)]
    #[must_use]
    pub fn sta4(&mut self) -> Sta4W<StaSpec> {
        Sta4W::new(self, 12)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`write(|w| ..)` method takes [`sta::W`](W) writer structure"]
impl crate::Writable for StaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}
