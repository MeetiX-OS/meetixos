/*! Virtual address */

use core::{
    fmt,
    fmt::{
        Debug,
        Display
    },
    iter::Step,
    ops::Deref
};

use crate::{
    addr::{
        Address,
        HwAddrBase
    },
    arch::addr::hw_virt_addr::HwVirtAddr
};

/**
 * Hardware 64bit wide virtual address.
 *
 * Encapsulates the hardware implementation of the virtual address for the
 * compiling target architecture.
 *
 * The inner address is responsible to perform validity checks, eventual
 * truncating and creation
 */
#[repr(transparent)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Ord, PartialOrd)]
#[derive(Hash)]
pub struct VirtAddr {
    m_hw_virt_addr: HwVirtAddr
}

impl VirtAddr {
    /**
     * Returns this `VirtAddr` value as constant raw pointer
     */
    pub fn as_ptr<T>(&self) -> *const T {
        *self.m_hw_virt_addr as *const T
    }

    /**
     * Returns this `VirtAddr` value as mutable raw pointer
     */
    pub fn as_ptr_mut<T>(&self) -> *mut T {
        *self.m_hw_virt_addr as *mut T
    }

    /**
     * Returns this `VirtAddr` value as immutable reference
     */
    pub unsafe fn as_ref<T>(&self) -> &T {
        &*self.as_ptr()
    }

    /**
     * Returns this `VirtAddr` value as mutable reference
     */
    pub unsafe fn as_ref_mut<T>(&self) -> &mut T {
        &mut *self.as_ptr_mut()
    }
}

impl Address for VirtAddr {
    const MAX: Self = Self { m_hw_virt_addr: HwVirtAddr::MAX };
}

impl Default for VirtAddr {
    #[inline]
    fn default() -> Self {
        Self::null()
    }
}

impl From<usize> for VirtAddr {
    #[inline]
    fn from(raw_virt_addr: usize) -> Self {
        Self { m_hw_virt_addr: HwVirtAddr::from(raw_virt_addr) }
    }
}

impl<T> From<*const T> for VirtAddr {
    #[inline]
    fn from(virt_addr_ptr: *const T) -> Self {
        Self::from(virt_addr_ptr as usize)
    }
}

impl<T> From<*mut T> for VirtAddr {
    #[inline]
    fn from(virt_addr_ptr: *mut T) -> Self {
        Self::from(virt_addr_ptr as usize)
    }
}

impl Deref for VirtAddr {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.m_hw_virt_addr.deref()
    }
}

impl Debug for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VirtAddr({:#018x})", **self)
    }
}

impl Display for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#018x}", **self)
    }
}

impl Step for VirtAddr {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        HwVirtAddr::steps_between(&start.m_hw_virt_addr, &end.m_hw_virt_addr)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(check_phys_addr) =
            HwVirtAddr::forward_checked(start.m_hw_virt_addr, count)
        {
            Some(Self { m_hw_virt_addr: check_phys_addr })
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(check_phys_addr) =
            HwVirtAddr::backward_checked(start.m_hw_virt_addr, count)
        {
            Some(Self { m_hw_virt_addr: check_phys_addr })
        } else {
            None
        }
    }
}
