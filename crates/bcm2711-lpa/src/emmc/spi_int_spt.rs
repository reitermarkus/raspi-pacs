#[doc = "Register `SPI_INT_SPT` reader"]
pub type R = crate::R<SpiIntSptSpec>;
#[doc = "Register `SPI_INT_SPT` writer"]
pub type W = crate::W<SpiIntSptSpec>;
#[doc = "Field `SELECT` reader - "]
pub type SelectR = crate::FieldReader;
#[doc = "Field `SELECT` writer - "]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INT_SPT")
            .field("select", &self.select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<SpiIntSptSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "Interrupts in SPI mode depend on CS\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_spt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_spt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiIntSptSpec;
impl crate::RegisterSpec for SpiIntSptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_int_spt::R`](R) reader structure"]
impl crate::Readable for SpiIntSptSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_int_spt::W`](W) writer structure"]
impl crate::Writable for SpiIntSptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_INT_SPT to value 0"]
impl crate::Resettable for SpiIntSptSpec {
    const RESET_VALUE: u32 = 0;
}
