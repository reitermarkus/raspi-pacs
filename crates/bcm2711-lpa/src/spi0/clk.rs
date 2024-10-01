#[doc = "Register `CLK` reader"]
pub type R = crate::R<ClkSpec>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<ClkSpec>;
#[doc = "Field `CDIV` reader - Clock divider"]
pub type CdivR = crate::FieldReader<u16>;
#[doc = "Field `CDIV` writer - Clock divider"]
pub type CdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock divider"]
    #[inline(always)]
    pub fn cdiv(&self) -> CdivR {
        CdivR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK").field("cdiv", &self.cdiv()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CdivW<ClkSpec> {
        CdivW::new(self, 0)
    }
}
#[doc = "Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSpec;
impl crate::RegisterSpec for ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for ClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for ClkSpec {
    const RESET_VALUE: u32 = 0;
}
