#[doc = "Register `CONFIG0` reader"]
pub type R = crate::R<Config0Spec>;
#[doc = "Register `CONFIG0` writer"]
pub type W = crate::W<Config0Spec>;
#[doc = "Field `IRQEN` reader - Enable the interrupt when data is available"]
pub type IrqenR = crate::BitReader;
#[doc = "Field `IRQEN` writer - Enable the interrupt when data is available"]
pub type IrqenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    pub fn irqen(&self) -> IrqenR {
        IrqenR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG0")
            .field("irqen", &self.irqen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IrqenW<Config0Spec> {
        IrqenW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config0Spec;
impl crate::RegisterSpec for Config0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config0::R`](R) reader structure"]
impl crate::Readable for Config0Spec {}
#[doc = "`write(|w| ..)` method takes [`config0::W`](W) writer structure"]
impl crate::Writable for Config0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
