#[doc = "Register `EXRDFIFO_EN` reader"]
pub type R = crate::R<ExrdfifoEnSpec>;
#[doc = "Register `EXRDFIFO_EN` writer"]
pub type W = crate::W<ExrdfifoEnSpec>;
#[doc = "Field `ENABLE` reader - Enable the extension FIFO"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable the extension FIFO"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXRDFIFO_EN")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ExrdfifoEnSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Enable the extension data register\n\nYou can [`read`](crate::Reg::read) this register and get [`exrdfifo_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrdfifo_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExrdfifoEnSpec;
impl crate::RegisterSpec for ExrdfifoEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exrdfifo_en::R`](R) reader structure"]
impl crate::Readable for ExrdfifoEnSpec {}
#[doc = "`write(|w| ..)` method takes [`exrdfifo_en::W`](W) writer structure"]
impl crate::Writable for ExrdfifoEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_EN to value 0"]
impl crate::Resettable for ExrdfifoEnSpec {
    const RESET_VALUE: u32 = 0;
}
