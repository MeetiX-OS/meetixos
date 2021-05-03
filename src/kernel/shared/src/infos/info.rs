/*! # HAL Boot Informations
 *
 * Implements the bootloader independent informations structure
 */

use os::str_utils;

#[cfg(feature = "loader_stage")]
use crate::arch::boot::HwBootInfos;
#[cfg(feature = "loader_stage")]
use crate::infos::BootMemAreas;
use crate::infos::{
    CmdLineArgs,
    VMLayout
};

/** Size in bytes of the bootloader name store into [`BootInfosInner`]
 *
 * [`BootInfosInner`]: crate::infos::info::BootInfosInner
 */
pub(crate) const BOOTLOADER_NAME_LEN_MAX: usize = 64;

/** It is initialized by the [`BootInfos::from()`] implementation
 *
 * [`BootInfos::from()`]: crate::infos::info::BootInfos::from
 */
static mut BOOT_INFOS_INNER: Option<BootInfosInner> = None;

/** # Common Bootloader Informations
 *
 * Stores an immutable reference to the common bootloader's informations
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfos {
    m_inner: &'static BootInfosInner
}

impl BootInfos {
    /** # Constructs a `BootInfos`
     *
     * The returned instance is already filled and valid
     */
    pub fn obtain() -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_some(),
                    "HAL haven't initialized boot informations");
            Self { m_inner: BOOT_INFOS_INNER.as_mut().unwrap() }
        }
    }

    /** Returns the slice to the kernel's command line
     */
    pub fn cmdline_args(&self) -> &'static CmdLineArgs {
        &self.m_inner.m_cmdline_args
    }

    /** Returns the [`BootMemAreas`] collection
     *
     * [`BootMemAreas`]: crate::infos::mem_area::BootMemAreas
     */
    #[cfg(feature = "loader_stage")]
    pub fn mem_areas(&self) -> &'static BootMemAreas {
        &self.m_inner.m_mem_areas
    }

    /** Returns the [`VMLayout`] collection
     *
     * [`VMLayout`]: crate::infos::vm_layout::VMLayout
     */
    pub fn vm_layout(&self) -> &'static VMLayout {
        &self.m_inner.m_vm_layout
    }
}

#[cfg(feature = "loader_stage")]
impl From<*const u8> for BootInfos {
    /** Initializes the global inner informations from the given raw
     * information pointer then constructs the `BootInfos` instance.
     *
     * Used by the higher half loader to initialize his instance of the
     * `BootInfosInner`
     */
    fn from(raw_info_ptr: *const u8) -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_none(), "Tried to re-initialize inner BootInfos");
        }

        /* obtain the informations inner and store to the global struct */
        let inner_infos = HwBootInfos::obtain_inner_from_arch_infos(raw_info_ptr);
        unsafe {
            BOOT_INFOS_INNER = Some(inner_infos);
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }
}

#[cfg(not(feature = "loader_stage"))]
impl From<&Self> for BootInfos {
    /** Initializes the global inner informations cloning the given instance
     * then constructs the `BootInfos` instance.
     *
     * Used by the kernel core to clone the higher half loader's instance of
     * the `BootInfosInner` into the higher half instance
     */
    fn from(rhs: &BootInfos) -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_none(), "Tried to re-initialize inner BootInfos");
        }

        /* clone the infos informations inner and store to our global copy */
        unsafe {
            BOOT_INFOS_INNER = Some(rhs.m_inner.clone());
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }
}

/** # Boot Informations Inner
 *
 * Defines the container of the common infos informations that is
 * initialized and instantiated once across all the HAL/kernel instance
 */
#[derive(Debug)]
pub(crate) struct BootInfosInner {
    #[cfg(feature = "loader_stage")]
    m_mem_areas: BootMemAreas,
    m_cmdline_args: CmdLineArgs,
    m_vm_layout: VMLayout,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX]
}

#[cfg(feature = "loader_stage")]
impl BootInfosInner {
    /** # Constructs a `BootInfosInner`
     *
     * The returned instance copies the given buffers into his
     */
    pub(crate) fn new(raw_cmdline: &str,
                      mem_areas: BootMemAreas,
                      bootloader_name: &str)
                      -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&name_buffer, bootloader_name);

        Self { m_cmdline_args: CmdLineArgs::new(raw_cmdline),
               m_mem_areas: mem_areas,
               m_vm_layout: VMLayout::new_zero(),
               m_bootloader_name: name_buffer }
    }
}

#[cfg(not(feature = "loader_stage"))]
impl Clone for BootInfosInner {
    fn clone(&self) -> Self {
        Self { m_cmdline_args: self.m_cmdline_args.clone(),
               m_vm_layout: self.m_vm_layout.clone(),
               m_bootloader_name: self.m_bootloader_name.clone() }
    }
}

/** # Hardware Boot Informations Base Interface
 *
 * Defines the method that is required by the [`BootInfosInner`]
 *
 * [`BootInfosInner`]: crate::infos::info::BootInfosInner
 */
#[cfg(feature = "loader_stage")]
pub(crate) trait HwBootInfosBase {
    /** # Constructs a `BootInfosInner`
     *
     * The instance returned is expected to be filled by the architecture
     * dependent code using the bootloaders informations given via raw
     * pointer
     */
    fn obtain_inner_from_arch_infos(raw_boot_infos_ptr: *const u8) -> BootInfosInner;
}
