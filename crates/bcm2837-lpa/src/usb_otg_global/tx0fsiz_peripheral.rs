#[doc = "Register `TX0FSIZ_Peripheral` reader"]
pub type R = crate::R<Tx0fsizPeripheralSpec>;
#[doc = "Register `TX0FSIZ_Peripheral` writer"]
pub type W = crate::W<Tx0fsizPeripheralSpec>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type Tx0fsaR = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type Tx0fsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type Tx0fdR = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type Tx0fdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> Tx0fsaR {
        Tx0fsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> Tx0fdR {
        Tx0fdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX0FSIZ_Peripheral")
            .field("tx0fsa", &self.tx0fsa())
            .field("tx0fd", &self.tx0fd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> Tx0fsaW<Tx0fsizPeripheralSpec> {
        Tx0fsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> Tx0fdW<Tx0fsizPeripheralSpec> {
        Tx0fdW::new(self, 16)
    }
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`tx0fsiz_peripheral::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx0fsiz_peripheral::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tx0fsizPeripheralSpec;
impl crate::RegisterSpec for Tx0fsizPeripheralSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx0fsiz_peripheral::R`](R) reader structure"]
impl crate::Readable for Tx0fsizPeripheralSpec {}
#[doc = "`write(|w| ..)` method takes [`tx0fsiz_peripheral::W`](W) writer structure"]
impl crate::Writable for Tx0fsizPeripheralSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX0FSIZ_Peripheral to value 0x0200"]
impl crate::Resettable for Tx0fsizPeripheralSpec {
    const RESET_VALUE: u32 = 0x0200;
}
