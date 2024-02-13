mod fixed_size_block;

use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::{VirtAddr};
use fixed_size_block::FixedSizeBlockAllocator;


#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024;


pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {

    // Create a page range for the heap. Determine the page address for the heap's start and end, and then create a Page
    // range from the start to the end.
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // For each page in the page range, allocate a frame and map it to the page with flags, using the frame allocator
    // passed in as an argument.
    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator) }?.flush()
    }

    // Initialize the allocator with the heap's start and size.
    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); }
    Ok(())
}


pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl <A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        // Create a wrapper around the spin::Mutex type, which will be used to lock the allocator.
        Locked { inner: spin::Mutex::new(inner) }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        // Lock the inner spin::Mutex and return a guard that will unlock the mutex when dropped.
        self.inner.lock()
    }
}


fn align_up(addr: usize, align: usize) -> usize {
    // Align a given "addr" upwards to the specified "align" value
    (addr + align - 1) & !(align - 1)
}
