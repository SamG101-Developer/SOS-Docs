use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{PageTable, OffsetPageTable, Page, PhysFrame, Mapper, Size4KiB, FrameAllocator};
use x86_64::{VirtAddr, PhysAddr};


pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    // Create an active level 4 page table, and then create an OffsetPageTable from it. This uses the physical memory
    // offset argument.
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}


unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    // Get the physical address of the level 4 table frame from the CR3 register, and then calculate the virtual address
    // using the physical memory offset argument and physical address. The virtual address is returned.
    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();

    let page_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();
    &mut *page_table_ptr
}


pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        // Create a boot info frame allocator from the memory map and set the next frame to 0.
        BootInfoFrameAllocator { memory_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
        // Get usable regions and transform them into address ranges by mapping each region to a range between its start
        // and end addresses. Then, transform the address ranges into frame addresses by stepping through each range by
        // 4096. Finally, transform the frame addresses into physical frames.
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let address_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());

        let frame_addresses = address_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        // To allocate a frame, get the next usable frame and increment the next frame counter. Return the frame.
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
