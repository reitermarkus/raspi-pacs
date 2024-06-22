#[doc = "Register `BAUDH` reader"]
pub type R = crate::R<BaudhSpec>;
#[doc = "Register `BAUDH` writer"]
pub type W = crate::W<BaudhSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "High bits of baudrate when DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`baudh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudhSpec;
impl crate::RegisterSpec for BaudhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`baudh::R`](R) reader structure"]
impl crate::Readable for BaudhSpec {}
#[doc = "`write(|w| ..)` method takes [`baudh::W`](W) writer structure"]
impl crate::Writable for BaudhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BAUDH to value 0"]
impl crate::Resettable for BaudhSpec {
    const RESET_VALUE: u8 = 0;
}
