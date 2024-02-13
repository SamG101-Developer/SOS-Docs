use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;

use crate::{hlt_loop, print, println};
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 { self as u8 }
    fn as_usize(self) -> usize { usize::from(self.as_u8()) }
}


lazy_static! {
    // The Interrupt Descriptor Table (IDT). The IDT contains a list of interrupt handlers, which are functions that are
    // called when an interrupt occurs. The IDT is used to handle hardware and software interrupts.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            // Set the default handlers for the CPU exceptions.
            // idt.divide_error.set_handler_fn(divide_error_handler);
            // idt.debug.set_handler_fn(debug_handler);
            // idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
            idt.breakpoint.set_handler_fn(breakpoint_handler);
            // idt.overflow.set_handler_fn(overflow_handler);
            // idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
            // idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
            // idt.device_not_available.set_handler_fn(device_not_available_handler);
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            // idt.invalid_tss.set_handler_fn(invalid_tss_handler);
            // idt.segment_not_present.set_handler_fn(segment_not_present_handler);
            // idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
            // idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
            idt.page_fault.set_handler_fn(page_fault_handler);
            // idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
            // idt.alignment_check.set_handler_fn(alignment_check_handler);
            // idt.machine_check.set_handler_fn(machine_check_handler);
            // idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
            // idt.virtualization.set_handler_fn(virtualization_handler);
            // idt.vmm_communication_exception.set_handler_fn(vmm_communication_exception_handler);
            // idt.security_exception.set_handler_fn(security_exception_handler);

            // Set the user defined interrupts. These are the timer and keyboard interrupts.
            idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
            idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        }

        idt
    };
}


pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    // Notify the PIC that the interrupt has been handled (this is a non-terminating interrupt, so the PIC needs to know
    // when the interrupt has been handled.
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8()); }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    // Read from port 0x60, which is the PS/2 controller data port. This port is used to read scancodes from the
    // keyboard, and will contain the "key pressed". Read the scancode and create a task to handle the scancode.
    let mut port = Port::new(0x60);
    let scancode = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    // Notify the PIC that the interrupt has been handled (this is a non-terminating interrupt, so the PIC needs to know
    // when the interrupt has been handled.
    unsafe { PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8()); }
}
