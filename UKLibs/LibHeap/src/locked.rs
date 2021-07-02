/*! Generics-customizable locked `Heap` */

use core::{
    alloc::{
        GlobalAlloc,
        Layout
    },
    ptr,
    ptr::NonNull
};

use sync::{
    mutex::{
        BackRawMutex,
        Mutex
    },
    Lazy
};

use crate::{
    Heap,
    HeapMemorySupplier
};

/**
 * Callback used by the `RawLazyLockedHeap` to obtain the
 * `sync::BackRawMutex` implementation
 */
pub type RawLazyMutexSupplier<M> = fn() -> Option<M>;

/**
 * Locked heap with a customizable `sync::Mutex` backend that is lazily
 * initialized
 *
 * This allow the use of the struct as `global_allocator` using constant
 * initialization
 */
pub struct RawLazyLockedHeap<M>
    where M: BackRawMutex + 'static {
    m_lazy_locked_heap: Lazy<Mutex<M, Heap>, LazyHeapInitializer<M>>
}

impl<M> RawLazyLockedHeap<M> where M: BackRawMutex + 'static {
    /**
     * Constructs a `RawLazyLockedHeap` without initialize the internal
     * `sync::Mutex<Heap>`
     */
    pub const unsafe fn new(raw_mutex_supplier: RawLazyMutexSupplier<M>,
                            mem_supplier: HeapMemorySupplier)
                            -> Self {
        Self { m_lazy_locked_heap:
                   Lazy::new(LazyHeapInitializer::new(raw_mutex_supplier, mem_supplier)) }
    }

    /**
     * Forces the initialization of this lazy `Heap`
     */
    pub fn force_init(&self) {
        self.m_lazy_locked_heap.lock().memory_in_use();
    }

    /**
     * Returns the total amount of memory returned by the
     * `HeapMemorySupplier`
     */
    pub fn memory_from_supplier(&self) -> usize {
        self.m_lazy_locked_heap.lock().memory_from_supplier()
    }

    /**
     * Returns the total amount of in-use memory (allocated)
     */
    pub fn memory_in_use(&self) -> usize {
        self.m_lazy_locked_heap.lock().memory_in_use()
    }

    /**
     * Returns the amount of currently available memory
     */
    pub fn memory_available(&self) -> usize {
        self.m_lazy_locked_heap.lock().memory_available()
    }
}

unsafe impl<M> GlobalAlloc for RawLazyLockedHeap<M> where M: BackRawMutex {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.m_lazy_locked_heap
            .lock()
            .allocate(layout)
            .map_or(ptr::null_mut(), |nn_ptr| nn_ptr.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(nn_ptr) = NonNull::new(ptr) {
            self.m_lazy_locked_heap.lock().deallocate(nn_ptr, layout);
        } else {
            panic!("GlobalAlloc::dealloc(): Tried to free a null-pointer");
        }
    }
}

/**
 * Concrete type for the `FnOnce` trait used by the `sync::Lazy`
 */
struct LazyHeapInitializer<T>
    where T: BackRawMutex {
    m_raw_mutex_supplier: RawLazyMutexSupplier<T>,
    m_mem_supplier: HeapMemorySupplier
}

impl<T> LazyHeapInitializer<T> where T: BackRawMutex {
    /**
     * Constructs a `LazyHeapInitializer`
     */
    const fn new(raw_mutex_supplier: RawLazyMutexSupplier<T>,
                 mem_supplier: HeapMemorySupplier)
                 -> Self {
        Self { m_raw_mutex_supplier: raw_mutex_supplier,
               m_mem_supplier: mem_supplier }
    }
}

impl<T> FnOnce<()> for LazyHeapInitializer<T> where T: BackRawMutex {
    type Output = Mutex<T, Heap>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        let raw_mutex =
            (self.m_raw_mutex_supplier)().expect("Failed to lazy obtain `BackRawMutex`");
        Mutex::raw_new(raw_mutex, unsafe {
            Heap::new(self.m_mem_supplier).expect("Failed to lazy initialize the Heap")
        })
    }
}