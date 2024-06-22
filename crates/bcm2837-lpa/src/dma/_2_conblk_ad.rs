#[doc = "Register `2_CONBLK_AD` reader"]
pub type R = crate::R<_2ConblkAdSpec>;
#[doc = "Register `2_CONBLK_AD` writer"]
pub type W = crate::W<_2ConblkAdSpec>;
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
        f.debug_struct("2_CONBLK_AD")
            .field("scb_addr", &self.scb_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Control Block Address"]
    #[inline(always)]
    #[must_use]
    pub fn scb_addr(&mut self) -> ScbAddrW<_2ConblkAdSpec> {
        ScbAddrW::new(self, 0)
    }
}
#[doc = "DMA Channel 2 Control Block Address\n\nYou can [`read`](crate::Reg::read) this register and get [`_2_conblk_ad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_2_conblk_ad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2ConblkAdSpec;
impl crate::RegisterSpec for _2ConblkAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2_conblk_ad::R`](R) reader structure"]
impl crate::Readable for _2ConblkAdSpec {}
#[doc = "`write(|w| ..)` method takes [`_2_conblk_ad::W`](W) writer structure"]
impl crate::Writable for _2ConblkAdSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets 2_CONBLK_AD to value 0"]
impl crate::Resettable for _2ConblkAdSpec {
    const RESET_VALUE: u32 = 0;
}
