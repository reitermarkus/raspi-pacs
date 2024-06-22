#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cntl0: Cntl0,
    cntl1: Cntl1,
    stat: Stat,
    peek: Peek,
    io: [Io; 4],
    txhold: [Txhold; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control 0"]
    #[inline(always)]
    pub const fn cntl0(&self) -> &Cntl0 {
        &self.cntl0
    }
    #[doc = "0x04 - Control 1"]
    #[inline(always)]
    pub const fn cntl1(&self) -> &Cntl1 {
        &self.cntl1
    }
    #[doc = "0x08 - Status"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x0c - Read the RXFIFO without removing an entry"]
    #[inline(always)]
    pub const fn peek(&self) -> &Peek {
        &self.peek
    }
    #[doc = "0x10..0x20 - Writing to the FIFO will deassert CS at the end of the access"]
    #[inline(always)]
    pub const fn io(&self, n: usize) -> &Io {
        &self.io[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Writing to the FIFO will deassert CS at the end of the access"]
    #[inline(always)]
    pub fn io_iter(&self) -> impl Iterator<Item = &Io> {
        self.io.iter()
    }
    #[doc = "0x20..0x30 - Writing to the FIFO will maintain CS at the end of the access"]
    #[inline(always)]
    pub const fn txhold(&self, n: usize) -> &Txhold {
        &self.txhold[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Writing to the FIFO will maintain CS at the end of the access"]
    #[inline(always)]
    pub fn txhold_iter(&self) -> impl Iterator<Item = &Txhold> {
        self.txhold.iter()
    }
}
#[doc = "CNTL0 (rw) register accessor: Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl0`]
module"]
#[doc(alias = "CNTL0")]
pub type Cntl0 = crate::Reg<cntl0::Cntl0Spec>;
#[doc = "Control 0"]
pub mod cntl0;
#[doc = "CNTL1 (rw) register accessor: Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntl1`]
module"]
#[doc(alias = "CNTL1")]
pub type Cntl1 = crate::Reg<cntl1::Cntl1Spec>;
#[doc = "Control 1"]
pub mod cntl1;
#[doc = "STAT (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status"]
pub mod stat;
#[doc = "PEEK (r) register accessor: Read the RXFIFO without removing an entry\n\nYou can [`read`](crate::Reg::read) this register and get [`peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek`]
module"]
#[doc(alias = "PEEK")]
pub type Peek = crate::Reg<peek::PeekSpec>;
#[doc = "Read the RXFIFO without removing an entry"]
pub mod peek;
#[doc = "IO (rw) register accessor: Writing to the FIFO will deassert CS at the end of the access\n\nYou can [`read`](crate::Reg::read) this register and get [`io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io`]
module"]
#[doc(alias = "IO")]
pub type Io = crate::Reg<io::IoSpec>;
#[doc = "Writing to the FIFO will deassert CS at the end of the access"]
pub mod io;
#[doc = "TXHOLD (rw) register accessor: Writing to the FIFO will maintain CS at the end of the access\n\nYou can [`read`](crate::Reg::read) this register and get [`txhold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txhold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txhold`]
module"]
#[doc(alias = "TXHOLD")]
pub type Txhold = crate::Reg<txhold::TxholdSpec>;
#[doc = "Writing to the FIFO will maintain CS at the end of the access"]
pub mod txhold;
