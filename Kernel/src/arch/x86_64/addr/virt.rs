/*! x86_64 virtual address implementation */

use core::ops::Deref;

use crate::addr::HwAddrBase;

/**
 * x86_64 virtual address implementation.
 *
 * This wrapper ensures canonical addresses
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct HwVirtAddr {
    m_raw_virt_addr: usize
}

impl HwAddrBase for HwVirtAddr {
    /* No methods to implement */
}

impl From<usize> for HwVirtAddr {
    fn from(raw_virt_addr: usize) -> Self {
        Self { m_raw_virt_addr: ((raw_virt_addr << 16) as isize >> 16) as usize }
    }
}

impl Deref for HwVirtAddr {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.m_raw_virt_addr
    }
}