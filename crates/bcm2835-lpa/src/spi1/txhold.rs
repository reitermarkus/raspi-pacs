#[doc = "Register `TXHOLD%s` reader"]
pub type R = crate::R<TxholdSpec>;
#[doc = "Register `TXHOLD%s` writer"]
pub type W = crate::W<TxholdSpec>;
#[doc = "Field `DATA` reader - FIFO data access"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - FIFO data access"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXHOLD")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<TxholdSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Writing to the FIFO will maintain CS at the end of the access\n\nYou can [`read`](crate::Reg::read) this register and get [`txhold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txhold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxholdSpec;
impl crate::RegisterSpec for TxholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txhold::R`](R) reader structure"]
impl crate::Readable for TxholdSpec {}
#[doc = "`write(|w| ..)` method takes [`txhold::W`](W) writer structure"]
impl crate::Writable for TxholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXHOLD%s to value 0"]
impl crate::Resettable for TxholdSpec {
    const RESET_VALUE: u32 = 0;
}
