#[doc = "Register `CLKT` reader"]
pub type R = crate::R<ClktSpec>;
#[doc = "Register `CLKT` writer"]
pub type W = crate::W<ClktSpec>;
#[doc = "Field `TOUT` reader - Number of SCL clock cycles to wait"]
pub type ToutR = crate::FieldReader<u16>;
#[doc = "Field `TOUT` writer - Number of SCL clock cycles to wait"]
pub type ToutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKT").field("tout", &self.tout()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of SCL clock cycles to wait"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<ClktSpec> {
        ToutW::new(self, 0)
    }
}
#[doc = "Clock stretch timeout (broken on 283x)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClktSpec;
impl crate::RegisterSpec for ClktSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkt::R`](R) reader structure"]
impl crate::Readable for ClktSpec {}
#[doc = "`write(|w| ..)` method takes [`clkt::W`](W) writer structure"]
impl crate::Writable for ClktSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKT to value 0"]
impl crate::Resettable for ClktSpec {
    const RESET_VALUE: u32 = 0;
}
