#[doc = "Register `BAUDL` reader"]
pub type R = crate::R<BaudlSpec>;
#[doc = "Register `BAUDL` writer"]
pub type W = crate::W<BaudlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Lower bits of baudrate when DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`baudl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudlSpec;
impl crate::RegisterSpec for BaudlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`baudl::R`](R) reader structure"]
impl crate::Readable for BaudlSpec {}
#[doc = "`write(|w| ..)` method takes [`baudl::W`](W) writer structure"]
impl crate::Writable for BaudlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BAUDL to value 0"]
impl crate::Resettable for BaudlSpec {
    const RESET_VALUE: u8 = 0;
}
