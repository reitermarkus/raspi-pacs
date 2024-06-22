#[doc = "Register `A` reader"]
pub type R = crate::R<ASpec>;
#[doc = "Register `A` writer"]
pub type W = crate::W<ASpec>;
#[doc = "Field `ADDR` reader - Slave address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Slave address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A").field("addr", &self.addr()).finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<ASpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ASpec;
impl crate::RegisterSpec for ASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a::R`](R) reader structure"]
impl crate::Readable for ASpec {}
#[doc = "`write(|w| ..)` method takes [`a::W`](W) writer structure"]
impl crate::Writable for ASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A to value 0"]
impl crate::Resettable for ASpec {
    const RESET_VALUE: u32 = 0;
}
