#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GotgintSpec>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GotgintSpec>;
#[doc = "Field `SEDET` reader - Session end detected"]
pub type SedetR = crate::BitReader;
#[doc = "Field `SEDET` writer - Session end detected"]
pub type SedetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRSSCHG` reader - Session request success status change"]
pub type SrsschgR = crate::BitReader;
#[doc = "Field `SRSSCHG` writer - Session request success status change"]
pub type SrsschgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNSSCHG` reader - Host negotiation success status change"]
pub type HnsschgR = crate::BitReader;
#[doc = "Field `HNSSCHG` writer - Host negotiation success status change"]
pub type HnsschgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGDET` reader - Host negotiation detected"]
pub type HngdetR = crate::BitReader;
#[doc = "Field `HNGDET` writer - Host negotiation detected"]
pub type HngdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTOCHG` reader - A-device timeout change"]
pub type AdtochgR = crate::BitReader;
#[doc = "Field `ADTOCHG` writer - A-device timeout change"]
pub type AdtochgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCDNE` reader - Debounce done"]
pub type DbcdneR = crate::BitReader;
#[doc = "Field `DBCDNE` writer - Debounce done"]
pub type DbcdneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    pub fn sedet(&self) -> SedetR {
        SedetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srsschg(&self) -> SrsschgR {
        SrsschgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    pub fn hnsschg(&self) -> HnsschgR {
        HnsschgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    pub fn hngdet(&self) -> HngdetR {
        HngdetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    pub fn adtochg(&self) -> AdtochgR {
        AdtochgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    pub fn dbcdne(&self) -> DbcdneR {
        DbcdneR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sedet", &self.sedet())
            .field("srsschg", &self.srsschg())
            .field("hnsschg", &self.hnsschg())
            .field("hngdet", &self.hngdet())
            .field("adtochg", &self.adtochg())
            .field("dbcdne", &self.dbcdne())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Session end detected"]
    #[inline(always)]
    #[must_use]
    pub fn sedet(&mut self) -> SedetW<GotgintSpec> {
        SedetW::new(self, 2)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    #[must_use]
    pub fn srsschg(&mut self) -> SrsschgW<GotgintSpec> {
        SrsschgW::new(self, 8)
    }
    #[doc = "Bit 9 - Host negotiation success status change"]
    #[inline(always)]
    #[must_use]
    pub fn hnsschg(&mut self) -> HnsschgW<GotgintSpec> {
        HnsschgW::new(self, 9)
    }
    #[doc = "Bit 17 - Host negotiation detected"]
    #[inline(always)]
    #[must_use]
    pub fn hngdet(&mut self) -> HngdetW<GotgintSpec> {
        HngdetW::new(self, 17)
    }
    #[doc = "Bit 18 - A-device timeout change"]
    #[inline(always)]
    #[must_use]
    pub fn adtochg(&mut self) -> AdtochgW<GotgintSpec> {
        AdtochgW::new(self, 18)
    }
    #[doc = "Bit 19 - Debounce done"]
    #[inline(always)]
    #[must_use]
    pub fn dbcdne(&mut self) -> DbcdneW<GotgintSpec> {
        DbcdneW::new(self, 19)
    }
}
#[doc = "OTG_HS interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GotgintSpec;
impl crate::RegisterSpec for GotgintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GotgintSpec {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GotgintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GotgintSpec {
    const RESET_VALUE: u32 = 0;
}
