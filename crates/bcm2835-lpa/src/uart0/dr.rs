#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - DATA"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - DATA"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FE` reader - FE"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - FE"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - PE"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - PE"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - BE"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - BE"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - OE"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - OE"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("data", &self.data())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .field("be", &self.be())
            .field("oe", &self.oe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<DrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<DrSpec> {
        FeW::new(self, 8)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<DrSpec> {
        PeW::new(self, 9)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<DrSpec> {
        BeW::new(self, 10)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<DrSpec> {
        OeW::new(self, 11)
    }
}
#[doc = "Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
