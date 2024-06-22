#[doc = "Register `GICC_DIR` writer"]
pub type W = crate::W<GiccDirSpec>;
impl core::fmt::Debug for crate::generic::Reg<GiccDirSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Deactivate Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_dir::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiccDirSpec;
impl crate::RegisterSpec for GiccDirSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gicc_dir::W`](W) writer structure"]
impl crate::Writable for GiccDirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICC_DIR to value 0"]
impl crate::Resettable for GiccDirSpec {
    const RESET_VALUE: u32 = 0;
}
