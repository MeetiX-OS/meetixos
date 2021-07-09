/*! # MeetiX OldKernel
 *
 * Implements the MeetiX OldKernel core
 */

#![no_std]
#![no_main]
#![feature(panic_info_message, alloc_error_handler, const_fn_trait_bound)]

extern crate alloc;

use shared::{
    info::descriptor::LoaderInfo,
    logger::{
        info,
        log,
        trace
    }
};

use crate::{
    cmdline::{
        cmdline_info,
        cmdline_info_init
    },
    interrupt::interrupt_init,
    log::{
        log_enable_buffering,
        log_init
    },
    mem::{
        heap::heap_init,
        paging::paging_unmap_loader,
        phys::phys_init,
        vm_layout::vml_init_from_loader_info
    },
    symbols::symbols_init,
    version::KERN_VERSION
};

mod cmdline;
mod interrupt;
mod log;
mod mem;
mod panic;
mod symbols;
mod version;

/**
 * Rust OldKernel entry point, here is where the OldKernel core starts his
 * execution
 */
#[no_mangle]
pub unsafe extern "C" fn kern_start(loader_info: &LoaderInfo) {
    /* initialize the command line boot from the loader boot */
    cmdline_info_init(loader_info);

    /* initialize the logger, to be able to print in a formatted way */
    log_init();

    /* print the OldKernel header */
    print_header();

    /* initialize the OldKernel <VMLayout> from the LoaderInfo given */
    info!("Initializing OldKernel VM Layout...");
    vml_init_from_loader_info(loader_info);

    /* initialize the physical memory allocator */
    info!("Initializing OldKernel Physical Memory Management...");
    phys_init(loader_info.bitmap_allocated_bits());

    /* initialize the OldKernel heap */
    info!("Initializing OldKernel Heap...");
    heap_init();

    /* initialize the global OldKernel symbols */
    info!("Initializing OldKernel Symbols...");
    symbols_init(loader_info);

    /* unmap the OldKernel loader, after this call <loader_info> will be invalid */
    info!("Unmapping OldKernel Loader...");
    paging_unmap_loader(loader_info);

    /* enable logging buffering */
    info!("Enabling Buffered OldKernel Logging...");
    log_enable_buffering(false);

    /* initialize interrupt management */
    info!("Initializing Interrupt Management...");
    interrupt_init();

    loop {}
}

/**
 * Prints the header in the logging
 */
fn print_header() {
    info!(target: "aa", "MeetiX OldKernel v{}", KERN_VERSION);
    info!("...Hoping you will use this OS as your primarily OS, maybe one day...");

    trace!("Booted By {}", cmdline_info().bootloader_name());
    trace!("Commandline: {}", cmdline_info().cmdline_args());
}