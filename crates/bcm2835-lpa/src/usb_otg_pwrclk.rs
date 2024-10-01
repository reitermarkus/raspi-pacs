#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcgcctl: Pcgcctl,
}
impl RegisterBlock {
    #[doc = "0x00 - power and clock gating control"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &Pcgcctl {
        &self.pcgcctl
    }
}
#[doc = "PCGCCTL (rw) register accessor: power and clock gating control\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
#[doc(alias = "PCGCCTL")]
pub type Pcgcctl = crate::Reg<pcgcctl::PcgcctlSpec>;
#[doc = "power and clock gating control"]
pub mod pcgcctl;
