#[doc = "Register `BOOT_TIMEOUT` reader"]
pub type R = crate::R<BootTimeoutSpec>;
#[doc = "Register `BOOT_TIMEOUT` writer"]
pub type W = crate::W<BootTimeoutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Number of SD clock cycles to wait for boot\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootTimeoutSpec;
impl crate::RegisterSpec for BootTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_timeout::R`](R) reader structure"]
impl crate::Readable for BootTimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_timeout::W`](W) writer structure"]
impl crate::Writable for BootTimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_TIMEOUT to value 0"]
impl crate::Resettable for BootTimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
