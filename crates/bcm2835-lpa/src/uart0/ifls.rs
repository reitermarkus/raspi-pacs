#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "Field `TXIFLSEL` reader - TXIFLSEL"]
pub type TxiflselR = crate::FieldReader;
#[doc = "Field `TXIFLSEL` writer - TXIFLSEL"]
pub type TxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXIFLSEL` reader - RXIFLSEL"]
pub type RxiflselR = crate::FieldReader;
#[doc = "Field `RXIFLSEL` writer - RXIFLSEL"]
pub type RxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TxiflselR {
        TxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RxiflselR {
        RxiflselR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFLS")
            .field("txiflsel", &self.txiflsel())
            .field("rxiflsel", &self.rxiflsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - TXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TxiflselW<IflsSpec> {
        TxiflselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - RXIFLSEL"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RxiflselW<IflsSpec> {
        RxiflselW::new(self, 3)
    }
}
#[doc = "Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0;
}
