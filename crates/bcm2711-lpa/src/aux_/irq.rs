#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IrqSpec>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IrqSpec>;
#[doc = "Field `UART_1` reader - UART1 interrupt active"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART_1` writer - UART1 interrupt active"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    pub fn uart_1(&self) -> Uart1R {
        Uart1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    pub fn spi_2(&self) -> Spi2R {
        Spi2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("spi_2", &self.spi_2())
            .field("spi_1", &self.spi_1())
            .field("uart_1", &self.uart_1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - UART1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_1(&mut self) -> Uart1W<IrqSpec> {
        Uart1W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> Spi1W<IrqSpec> {
        Spi1W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> Spi2W<IrqSpec> {
        Spi2W::new(self, 2)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSpec;
impl crate::RegisterSpec for IrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IrqSpec {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IrqSpec {
    const RESET_VALUE: u32 = 0;
}
