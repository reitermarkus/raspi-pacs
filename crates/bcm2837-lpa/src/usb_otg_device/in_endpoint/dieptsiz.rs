#[doc = "Register `DIEPTSIZ` reader"]
pub type R = crate::R<DieptsizSpec>;
#[doc = "Register `DIEPTSIZ` writer"]
pub type W = crate::W<DieptsizSpec>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XfrsizR = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ")
            .field("xfrsiz", &self.xfrsiz())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XfrsizW<DieptsizSpec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PktcntW<DieptsizSpec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "Transfer size\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieptsizSpec;
impl crate::RegisterSpec for DieptsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz::R`](R) reader structure"]
impl crate::Readable for DieptsizSpec {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz::W`](W) writer structure"]
impl crate::Writable for DieptsizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ to value 0"]
impl crate::Resettable for DieptsizSpec {
    const RESET_VALUE: u32 = 0;
}
