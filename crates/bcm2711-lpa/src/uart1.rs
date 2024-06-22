#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_io: [u8; 0x04],
    _reserved_1_ier: [u8; 0x04],
    iir: Iir,
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scratch: Scratch,
    _reserved8: [u8; 0x03],
    cntl: Cntl,
    stat: Stat,
    baud: Baud,
}
impl RegisterBlock {
    #[doc = "0x00 - Lower bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudl(&self) -> &Baudl {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - I/O Data"]
    #[inline(always)]
    pub const fn io(&self) -> &Io {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - High bits of baudrate when DLAB is set"]
    #[inline(always)]
    pub const fn baudh(&self) -> &Baudh {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Interrupt Identify"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        &self.iir
    }
    #[doc = "0x0c - Line control"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratch"]
    #[inline(always)]
    pub const fn scratch(&self) -> &Scratch {
        &self.scratch
    }
    #[doc = "0x20 - Control"]
    #[inline(always)]
    pub const fn cntl(&self) -> &Cntl {
        &self.cntl
    }
    #[doc = "0x24 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x28 - Baudrate"]
    #[inline(always)]
    pub const fn baud(&self) -> &Baud {
        &self.baud
    }
}
#[doc = "IO (rw) register accessor: I/O Data\n\nYou can [`read`](crate::Reg::read) this register and get [`io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
#[doc(alias = "IO")]
pub type Io = crate::Reg<io::IoSpec>;
#[doc = "I/O Data"]
pub mod io;
#[doc = "BAUDL (rw) register accessor: Lower bits of baudrate when DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`baudl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudl`]
module"]
#[doc(alias = "BAUDL")]
pub type Baudl = crate::Reg<baudl::BaudlSpec>;
#[doc = "Lower bits of baudrate when DLAB is set"]
pub mod baudl;
#[doc = "IER (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "BAUDH (rw) register accessor: High bits of baudrate when DLAB is set\n\nYou can [`read`](crate::Reg::read) this register and get [`baudh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudh`]
module"]
#[doc(alias = "BAUDH")]
pub type Baudh = crate::Reg<baudh::BaudhSpec>;
#[doc = "High bits of baudrate when DLAB is set"]
pub mod baudh;
#[doc = "IIR (rw) register accessor: Interrupt Identify\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identify"]
pub mod iir;
#[doc = "LCR (rw) register accessor: Line control\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line control"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control"]
pub mod mcr;
#[doc = "LSR (rw) register accessor: Line Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: Modem Status\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status"]
pub mod msr;
#[doc = "SCRATCH (rw) register accessor: Scratch\n\nYou can [`read`](crate::Reg::read) this register and get [`scratch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scratch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scratch`]
module"]
#[doc(alias = "SCRATCH")]
pub type Scratch = crate::Reg<scratch::ScratchSpec>;
#[doc = "Scratch"]
pub mod scratch;
#[doc = "CNTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl`]
module"]
#[doc(alias = "CNTL")]
pub type Cntl = crate::Reg<cntl::CntlSpec>;
#[doc = "Control"]
pub mod cntl;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status"]
pub mod stat;
#[doc = "BAUD (rw) register accessor: Baudrate\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
#[doc(alias = "BAUD")]
pub type Baud = crate::Reg<baud::BaudSpec>;
#[doc = "Baudrate"]
pub mod baud;
