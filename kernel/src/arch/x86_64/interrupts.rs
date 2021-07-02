use crate::println;
use htlib::mutex::*;

use x86_64::structures::idt::*;

static IDT: SpinMutex<InterruptDescriptorTable> = SpinMutex::new(InterruptDescriptorTable::new());

pub fn init_idt() {
    let mut idt = IDT.lock();

    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.non_maskable_interrupt
        .set_handler_fn(non_maskable_interrupt_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded
        .set_handler_fn(bound_range_exceeded_handler);

    unsafe {
        idt.load_unsafe();
    }
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE_ERROR\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: NON_MASKABLE_INTERRUPT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
    loop {}
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BOUND_RANGE_EXCEEDED\n{:#?}", stack_frame);
    loop {}
}
