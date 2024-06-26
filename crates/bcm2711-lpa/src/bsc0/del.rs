#[doc = "Register `DEL` reader"]
pub type R = crate::R<DelSpec>;
#[doc = "Register `DEL` writer"]
pub type W = crate::W<DelSpec>;
#[doc = "Field `REDL` reader - Delay before reading after a rising edge"]
pub type RedlR = crate::FieldReader<u16>;
#[doc = "Field `REDL` writer - Delay before reading after a rising edge"]
pub type RedlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FEDL` reader - Delay before reading after a falling edge"]
pub type FedlR = crate::FieldReader<u16>;
#[doc = "Field `FEDL` writer - Delay before reading after a falling edge"]
pub type FedlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    pub fn redl(&self) -> RedlR {
        RedlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    pub fn fedl(&self) -> FedlR {
        FedlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEL")
            .field("fedl", &self.fedl())
            .field("redl", &self.redl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> RedlW<DelSpec> {
        RedlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn fedl(&mut self) -> FedlW<DelSpec> {
        FedlW::new(self, 16)
    }
}
#[doc = "Data delay (Values must be under CDIV / 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`del::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`del::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelSpec;
impl crate::RegisterSpec for DelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`del::R`](R) reader structure"]
impl crate::Readable for DelSpec {}
#[doc = "`write(|w| ..)` method takes [`del::W`](W) writer structure"]
impl crate::Writable for DelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEL to value 0x0030_0030"]
impl crate::Resettable for DelSpec {
    const RESET_VALUE: u32 = 0x0030_0030;
}
