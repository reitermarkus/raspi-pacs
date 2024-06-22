#[doc = "Register `C` reader"]
pub type R = crate::R<CSpec>;
#[doc = "Register `C` writer"]
pub type W = crate::W<CSpec>;
#[doc = "Field `READ` reader - Transfer is read"]
pub type ReadR = crate::BitReader;
#[doc = "Field `READ` writer - Transfer is read"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` reader - Clear the FIFO"]
pub type ClearR = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear the FIFO"]
pub type ClearW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ST` reader - Start transfer"]
pub type StR = crate::BitReader;
#[doc = "Field `ST` writer - Start transfer"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTD` reader - Interrupt on done"]
pub type IntdR = crate::BitReader;
#[doc = "Field `INTD` writer - Interrupt on done"]
pub type IntdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTT` reader - Interrupt on TX"]
pub type InttR = crate::BitReader;
#[doc = "Field `INTT` writer - Interrupt on TX"]
pub type InttW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR` reader - Interrupt on RX"]
pub type IntrR = crate::BitReader;
#[doc = "Field `INTR` writer - Interrupt on RX"]
pub type IntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer is read"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Start transfer"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt on done"]
    #[inline(always)]
    pub fn intd(&self) -> IntdR {
        IntdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on TX"]
    #[inline(always)]
    pub fn intt(&self) -> InttR {
        InttR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    pub fn intr(&self) -> IntrR {
        IntrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C")
            .field("i2cen", &self.i2cen())
            .field("intr", &self.intr())
            .field("intt", &self.intt())
            .field("intd", &self.intd())
            .field("st", &self.st())
            .field("clear", &self.clear())
            .field("read", &self.read())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer is read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<CSpec> {
        ReadW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<CSpec> {
        ClearW::new(self, 4)
    }
    #[doc = "Bit 7 - Start transfer"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<CSpec> {
        StW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt on done"]
    #[inline(always)]
    #[must_use]
    pub fn intd(&mut self) -> IntdW<CSpec> {
        IntdW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on TX"]
    #[inline(always)]
    #[must_use]
    pub fn intt(&mut self) -> InttW<CSpec> {
        InttW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> IntrW<CSpec> {
        IntrW::new(self, 10)
    }
    #[doc = "Bit 15 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2cenW<CSpec> {
        I2cenW::new(self, 15)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSpec;
impl crate::RegisterSpec for CSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c::R`](R) reader structure"]
impl crate::Readable for CSpec {}
#[doc = "`write(|w| ..)` method takes [`c::W`](W) writer structure"]
impl crate::Writable for CSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C to value 0"]
impl crate::Resettable for CSpec {
    const RESET_VALUE: u32 = 0;
}
