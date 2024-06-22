#[doc = "Register `IO` reader"]
pub type R = crate::R<IoSpec>;
#[doc = "Register `IO` writer"]
pub type W = crate::W<IoSpec>;
#[doc = "Field `DATA` reader - FIFO access"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - FIFO access"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO access"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO").field("data", &self.data()).finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO access"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<IoSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "I/O Data\n\nYou can [`read`](crate::Reg::read) this register and get [`io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoSpec;
impl crate::RegisterSpec for IoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io::R`](R) reader structure"]
impl crate::Readable for IoSpec {}
#[doc = "`write(|w| ..)` method takes [`io::W`](W) writer structure"]
impl crate::Writable for IoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO to value 0"]
impl crate::Resettable for IoSpec {
    const RESET_VALUE: u32 = 0;
}
