#[doc = "Register `0_CONBLK_AD` reader"]
pub type R = crate::R<_0ConblkAdSpec>;
#[doc = "Register `0_CONBLK_AD` writer"]
pub type W = crate::W<_0ConblkAdSpec>;
#[doc = "Field `SCB_ADDR` reader - Control Block Address"]
pub type ScbAddrR = crate::FieldReader<u32>;
#[doc = "Field `SCB_ADDR` writer - Control Block Address"]
pub type ScbAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - Control Block Address"]
    #[inline(always)]
    pub fn scb_addr(&self) -> ScbAddrR {
        ScbAddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("0_CONBLK_AD")
            .field("scb_addr", &self.scb_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Control Block Address"]
    #[inline(always)]
    #[must_use]
    pub fn scb_addr(&mut self) -> ScbAddrW<_0ConblkAdSpec> {
        ScbAddrW::new(self, 0)
    }
}
#[doc = "DMA Channel 0 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_conblk_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_conblk_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0ConblkAdSpec;
impl crate::RegisterSpec for _0ConblkAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_conblk_ad::R`](R) reader structure"]
impl crate::Readable for _0ConblkAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_0_conblk_ad::W`](W) writer structure"]
impl crate::Writable for _0ConblkAdSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 0_CONBLK_AD to value 0"]
impl crate::Resettable for _0ConblkAdSpec {
    const RESET_VALUE: u32 = 0;
}
