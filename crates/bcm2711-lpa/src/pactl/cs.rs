#[doc = "Register `CS` reader"]
pub type R = crate::R<CsSpec>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CsSpec>;
#[doc = "Field `SPI_0` reader - SPI0 interrupt active"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI_0` writer - SPI0 interrupt active"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_1` reader - SPI1 interrupt active"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI_1` writer - SPI1 interrupt active"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_2` reader - SPI2 interrupt active"]
pub type Spi2R = crate::BitReader;
#[doc = "Field `SPI_2` writer - SPI2 interrupt active"]
pub type Spi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_3` reader - SPI3 interrupt active"]
pub type Spi3R = crate::BitReader;
#[doc = "Field `SPI_3` writer - SPI3 interrupt active"]
pub type Spi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_4` reader - SPI4 interrupt active"]
pub type Spi4R = crate::BitReader;
#[doc = "Field `SPI_4` writer - SPI4 interrupt active"]
pub type Spi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_5` reader - SPI5 interrupt active"]
pub type Spi5R = crate::BitReader;
#[doc = "Field `SPI_5` writer - SPI5 interrupt active"]
pub type Spi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_6` reader - SPI6 interrupt active"]
pub type Spi6R = crate::BitReader;
#[doc = "Field `SPI_6` writer - SPI6 interrupt active"]
pub type Spi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_0` reader - I2C0 interrupt active"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C_0` writer - I2C0 interrupt active"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_1` reader - I2C1 interrupt active"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C_1` writer - I2C1 interrupt active"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_2` reader - I2C2 interrupt active"]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C_2` writer - I2C2 interrupt active"]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_3` reader - I2C3 interrupt active"]
pub type I2c3R = crate::BitReader;
#[doc = "Field `I2C_3` writer - I2C3 interrupt active"]
pub type I2c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_4` reader - I2C4 interrupt active"]
pub type I2c4R = crate::BitReader;
#[doc = "Field `I2C_4` writer - I2C4 interrupt active"]
pub type I2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_5` reader - I2C5 interrupt active"]
pub type I2c5R = crate::BitReader;
#[doc = "Field `I2C_5` writer - I2C5 interrupt active"]
pub type I2c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_6` reader - I2C6 interrupt active"]
pub type I2c6R = crate::BitReader;
#[doc = "Field `I2C_6` writer - I2C6 interrupt active"]
pub type I2c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_7` reader - I2C7 interrupt active"]
pub type I2c7R = crate::BitReader;
#[doc = "Field `I2C_7` writer - I2C7 interrupt active"]
pub type I2c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_5` reader - UART5 interrupt active"]
pub type Uart5R = crate::BitReader;
#[doc = "Field `UART_5` writer - UART5 interrupt active"]
pub type Uart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_4` reader - UART4 interrupt active"]
pub type Uart4R = crate::BitReader;
#[doc = "Field `UART_4` writer - UART4 interrupt active"]
pub type Uart4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_3` reader - UART3 interrupt active"]
pub type Uart3R = crate::BitReader;
#[doc = "Field `UART_3` writer - UART3 interrupt active"]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_2` reader - UART2 interrupt active"]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART_2` writer - UART2 interrupt active"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_0` reader - UART0 interrupt active"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART_0` writer - UART0 interrupt active"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI0 interrupt active"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 3 - SPI3 interrupt active"]
    #[inline(always)]
    pub fn spi_3(&self) -> Spi3R {
        Spi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI4 interrupt active"]
    #[inline(always)]
    pub fn spi_4(&self) -> Spi4R {
        Spi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI5 interrupt active"]
    #[inline(always)]
    pub fn spi_5(&self) -> Spi5R {
        Spi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI6 interrupt active"]
    #[inline(always)]
    pub fn spi_6(&self) -> Spi6R {
        Spi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C0 interrupt active"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C1 interrupt active"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C2 interrupt active"]
    #[inline(always)]
    pub fn i2c_2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C3 interrupt active"]
    #[inline(always)]
    pub fn i2c_3(&self) -> I2c3R {
        I2c3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C4 interrupt active"]
    #[inline(always)]
    pub fn i2c_4(&self) -> I2c4R {
        I2c4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C5 interrupt active"]
    #[inline(always)]
    pub fn i2c_5(&self) -> I2c5R {
        I2c5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C6 interrupt active"]
    #[inline(always)]
    pub fn i2c_6(&self) -> I2c6R {
        I2c6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C7 interrupt active"]
    #[inline(always)]
    pub fn i2c_7(&self) -> I2c7R {
        I2c7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - UART5 interrupt active"]
    #[inline(always)]
    pub fn uart_5(&self) -> Uart5R {
        Uart5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART4 interrupt active"]
    #[inline(always)]
    pub fn uart_4(&self) -> Uart4R {
        Uart4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART3 interrupt active"]
    #[inline(always)]
    pub fn uart_3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART2 interrupt active"]
    #[inline(always)]
    pub fn uart_2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART0 interrupt active"]
    #[inline(always)]
    pub fn uart_0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("spi_0", &self.spi_0())
            .field("spi_1", &self.spi_1())
            .field("spi_2", &self.spi_2())
            .field("spi_3", &self.spi_3())
            .field("spi_4", &self.spi_4())
            .field("spi_5", &self.spi_5())
            .field("spi_6", &self.spi_6())
            .field("i2c_0", &self.i2c_0())
            .field("i2c_1", &self.i2c_1())
            .field("i2c_2", &self.i2c_2())
            .field("i2c_3", &self.i2c_3())
            .field("i2c_4", &self.i2c_4())
            .field("i2c_5", &self.i2c_5())
            .field("i2c_6", &self.i2c_6())
            .field("i2c_7", &self.i2c_7())
            .field("uart_5", &self.uart_5())
            .field("uart_4", &self.uart_4())
            .field("uart_3", &self.uart_3())
            .field("uart_2", &self.uart_2())
            .field("uart_0", &self.uart_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_0(&mut self) -> Spi0W<CsSpec> {
        Spi0W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_1(&mut self) -> Spi1W<CsSpec> {
        Spi1W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_2(&mut self) -> Spi2W<CsSpec> {
        Spi2W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_3(&mut self) -> Spi3W<CsSpec> {
        Spi3W::new(self, 3)
    }
    #[doc = "Bit 4 - SPI4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_4(&mut self) -> Spi4W<CsSpec> {
        Spi4W::new(self, 4)
    }
    #[doc = "Bit 5 - SPI5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_5(&mut self) -> Spi5W<CsSpec> {
        Spi5W::new(self, 5)
    }
    #[doc = "Bit 6 - SPI6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn spi_6(&mut self) -> Spi6W<CsSpec> {
        Spi6W::new(self, 6)
    }
    #[doc = "Bit 8 - I2C0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_0(&mut self) -> I2c0W<CsSpec> {
        I2c0W::new(self, 8)
    }
    #[doc = "Bit 9 - I2C1 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_1(&mut self) -> I2c1W<CsSpec> {
        I2c1W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_2(&mut self) -> I2c2W<CsSpec> {
        I2c2W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_3(&mut self) -> I2c3W<CsSpec> {
        I2c3W::new(self, 11)
    }
    #[doc = "Bit 12 - I2C4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_4(&mut self) -> I2c4W<CsSpec> {
        I2c4W::new(self, 12)
    }
    #[doc = "Bit 13 - I2C5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_5(&mut self) -> I2c5W<CsSpec> {
        I2c5W::new(self, 13)
    }
    #[doc = "Bit 14 - I2C6 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_6(&mut self) -> I2c6W<CsSpec> {
        I2c6W::new(self, 14)
    }
    #[doc = "Bit 15 - I2C7 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_7(&mut self) -> I2c7W<CsSpec> {
        I2c7W::new(self, 15)
    }
    #[doc = "Bit 16 - UART5 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_5(&mut self) -> Uart5W<CsSpec> {
        Uart5W::new(self, 16)
    }
    #[doc = "Bit 17 - UART4 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_4(&mut self) -> Uart4W<CsSpec> {
        Uart4W::new(self, 17)
    }
    #[doc = "Bit 18 - UART3 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_3(&mut self) -> Uart3W<CsSpec> {
        Uart3W::new(self, 18)
    }
    #[doc = "Bit 19 - UART2 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_2(&mut self) -> Uart2W<CsSpec> {
        Uart2W::new(self, 19)
    }
    #[doc = "Bit 20 - UART0 interrupt active"]
    #[inline(always)]
    #[must_use]
    pub fn uart_0(&mut self) -> Uart0W<CsSpec> {
        Uart0W::new(self, 20)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsSpec;
impl crate::RegisterSpec for CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CsSpec {
    const RESET_VALUE: u32 = 0;
}
