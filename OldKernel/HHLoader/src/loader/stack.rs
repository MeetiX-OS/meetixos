/*! Loader stack management */

use shared::{
    info::vm_layout::VMLayoutArea,
    logger::{
        debug,
        trace
    },
    mem::paging::{
        flags::PDirFlags,
        flush::MapFlusher
    }
};

use crate::mem::{
    paging::{
        allocators::HHLPageDirAllocator,
        paging_current_page_dir
    },
    vm_layout::vml_core_layout
};

/**
 * Allocates an maps the stack for the OldKernel core
 */
pub fn loader_stack_setup_core_stack() -> VMLayoutArea {
    let stack_area = vml_core_layout().kern_stack_area();
    debug!("Mapping OldKernel stack at: {}", stack_area);

    /* map the stack area */
    let mapping_res =
        paging_current_page_dir().map_range(stack_area.as_frame_range(),
                                            &HHLPageDirAllocator,
                                            PDirFlags::new().set_present()
                                                            .set_readable()
                                                            .set_writeable()
                                                            .set_no_execute()
                                                            .build());
    match mapping_res {
        Ok(map_flusher) => map_flusher.flush(),
        Err(err) => {
            trace!("\n{:?}", paging_current_page_dir());
            panic!("Failed to map OldKernel stack: cause: {}", err)
        }
    }

    /* zero-fill the area of the allocated stack */
    let stack_area_slice = unsafe {
        core::slice::from_raw_parts_mut(stack_area.start_addr().as_ptr_mut::<u8>(),
                                        stack_area.size())
    };
    stack_area_slice.fill(0);

    stack_area.clone()
}