#[doc = "Register `GICD_TYPER` reader"]
pub type R = crate::R<GicdTyperSpec>;
#[doc = "Field `IT_LINES_NUMBER` reader - Interrupt line number"]
pub type ItLinesNumberR = crate::FieldReader;
#[doc = "Field `CPU_NUMBER` reader - CPU number"]
pub type CpuNumberR = crate::FieldReader;
#[doc = "Field `SECURITY_EXTENSION` reader - Security extension implemented"]
pub type SecurityExtensionR = crate::BitReader;
#[doc = "Field `LSPI` reader - Lockable SPI count"]
pub type LspiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Interrupt line number"]
    #[inline(always)]
    pub fn it_lines_number(&self) -> ItLinesNumberR {
        ItLinesNumberR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - CPU number"]
    #[inline(always)]
    pub fn cpu_number(&self) -> CpuNumberR {
        CpuNumberR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 10 - Security extension implemented"]
    #[inline(always)]
    pub fn security_extension(&self) -> SecurityExtensionR {
        SecurityExtensionR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Lockable SPI count"]
    #[inline(always)]
    pub fn lspi(&self) -> LspiR {
        LspiR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_TYPER")
            .field("it_lines_number", &self.it_lines_number())
            .field("cpu_number", &self.cpu_number())
            .field("security_extension", &self.security_extension())
            .field("lspi", &self.lspi())
            .finish()
    }
}
#[doc = "Interrupt Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gicd_typer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicdTyperSpec;
impl crate::RegisterSpec for GicdTyperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_typer::R`](R) reader structure"]
impl crate::Readable for GicdTyperSpec {}
#[doc = "`reset()` method sets GICD_TYPER to value 0"]
impl crate::Resettable for GicdTyperSpec {
    const RESET_VALUE: u32 = 0;
}
