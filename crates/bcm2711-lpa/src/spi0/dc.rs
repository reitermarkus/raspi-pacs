#[doc = "Register `DC` reader"]
pub type R = crate::R<DcSpec>;
#[doc = "Register `DC` writer"]
pub type W = crate::W<DcSpec>;
#[doc = "Field `TDREQ` reader - DMA Write request threshold"]
pub type TdreqR = crate::FieldReader;
#[doc = "Field `TDREQ` writer - DMA Write request threshold"]
pub type TdreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPANIC` reader - DMA write panic threshold"]
pub type TpanicR = crate::FieldReader;
#[doc = "Field `TPANIC` writer - DMA write panic threshold"]
pub type TpanicW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RDREQ` reader - DMA read request threshold"]
pub type RdreqR = crate::FieldReader;
#[doc = "Field `RDREQ` writer - DMA read request threshold"]
pub type RdreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RPANIC` reader - DMA read panic threshold"]
pub type RpanicR = crate::FieldReader;
#[doc = "Field `RPANIC` writer - DMA read panic threshold"]
pub type RpanicW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    pub fn tdreq(&self) -> TdreqR {
        TdreqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    pub fn tpanic(&self) -> TpanicR {
        TpanicR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    pub fn rdreq(&self) -> RdreqR {
        RdreqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    pub fn rpanic(&self) -> RpanicR {
        RpanicR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DC")
            .field("rpanic", &self.rpanic())
            .field("rdreq", &self.rdreq())
            .field("tpanic", &self.tpanic())
            .field("tdreq", &self.tdreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Write request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tdreq(&mut self) -> TdreqW<DcSpec> {
        TdreqW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA write panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tpanic(&mut self) -> TpanicW<DcSpec> {
        TpanicW::new(self, 8)
    }
    #[doc = "Bits 16:23 - DMA read request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rdreq(&mut self) -> RdreqW<DcSpec> {
        RdreqW::new(self, 16)
    }
    #[doc = "Bits 24:31 - DMA read panic threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rpanic(&mut self) -> RpanicW<DcSpec> {
        RpanicW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcSpec;
impl crate::RegisterSpec for DcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc::R`](R) reader structure"]
impl crate::Readable for DcSpec {}
#[doc = "`write(|w| ..)` method takes [`dc::W`](W) writer structure"]
impl crate::Writable for DcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC to value 0x3020_1050"]
impl crate::Resettable for DcSpec {
    const RESET_VALUE: u32 = 0x3020_1050;
}
