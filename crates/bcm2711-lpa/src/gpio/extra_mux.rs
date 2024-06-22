#[doc = "Register `EXTRA_MUX` reader"]
pub type R = crate::R<ExtraMuxSpec>;
#[doc = "Register `EXTRA_MUX` writer"]
pub type W = crate::W<ExtraMuxSpec>;
#[doc = "Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio {
    #[doc = "0: Connect the newer SD host"]
    Sdhost = 0,
    #[doc = "1: Connect Arasan SD/EMMC host"]
    Arasan = 1,
}
impl From<Sdio> for bool {
    #[inline(always)]
    fn from(variant: Sdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO` reader - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
pub type SdioR = crate::BitReader<Sdio>;
impl SdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio {
        match self.bits {
            false => Sdio::Sdhost,
            true => Sdio::Arasan,
        }
    }
    #[doc = "Connect the newer SD host"]
    #[inline(always)]
    pub fn is_sdhost(&self) -> bool {
        *self == Sdio::Sdhost
    }
    #[doc = "Connect Arasan SD/EMMC host"]
    #[inline(always)]
    pub fn is_arasan(&self) -> bool {
        *self == Sdio::Arasan
    }
}
#[doc = "Field `SDIO` writer - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
pub type SdioW<'a, REG> = crate::BitWriter<'a, REG, Sdio>;
impl<'a, REG> SdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect the newer SD host"]
    #[inline(always)]
    pub fn sdhost(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::Sdhost)
    }
    #[doc = "Connect Arasan SD/EMMC host"]
    #[inline(always)]
    pub fn arasan(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::Arasan)
    }
}
impl R {
    #[doc = "Bit 1 - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTRA_MUX")
            .field("sdio", &self.sdio())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Switch peripheral connection to undocumented SDIO pins used on Pi 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SdioW<ExtraMuxSpec> {
        SdioW::new(self, 1)
    }
}
#[doc = "Undocumented multiplexing bits\n\nYou can [`read`](crate::Reg::read) this register and get [`extra_mux::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extra_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtraMuxSpec;
impl crate::RegisterSpec for ExtraMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extra_mux::R`](R) reader structure"]
impl crate::Readable for ExtraMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`extra_mux::W`](W) writer structure"]
impl crate::Writable for ExtraMuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
