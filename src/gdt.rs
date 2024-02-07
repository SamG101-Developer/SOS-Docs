/*
The Global Descriptor Table is a binary data structure for x86_64 architectures. It contains entries telling the CPU
about memory segments. A segment is a contiguous chunk of memory. The GDT is used to define the memory areas that can
be addressed by a program running in the CPU.
 */

use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use lazy_static::lazy_static;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;


// The Task State Segment (TSS) is a special structure for x86_64 architectures. It holds information about a task,
// which is a program running on the CPU. The TSS contains 2 stacks (privilege and interrupt), and a pointer to an I/O
// map.
lazy_static! {
    static ref TSS: TaskStateSegment = {
        // Create the TSS structure.
        let mut tss = TaskStateSegment::new();

        // Define the 0th IST entry to be the double fault stack (any IST entry can be used for any interrupt). Write
        // the top address of the double fault stack to the 0th IST entry, so the 0th IST entry now points to the top of
        // the double fault stack.
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            // The stack grows downwards, from high addresses to low addresses. The stack pointer points to the top of
            // the stack, so the stack pointer should point to the end of the stack. The stack pointer should be a
            // virtual address, so we need to convert the stack pointer to a virtual address.
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end   = stack_start + STACK_SIZE;
            stack_end
        };

        // Return the TSS structure.
        tss
    };
}


lazy_static! {
    // Create a new GDT with a code segment, and a TSS segment.
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector,})
    };
}


// Keep the different selectors all in one place.
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}


pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    // Load the GDT. Reload the code segment register (CS), and load the Task State Segment (TSS) into the CPU.
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector)
    }
}
