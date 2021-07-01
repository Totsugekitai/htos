use crate::println;
use bit_field::BitField;
use core::fmt::*;
use core::marker::PhantomData;
use htlib::mutex::*;

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: InterruptDescriptor<HandlerFunc>,
    pub debug: InterruptDescriptor<HandlerFunc>,
    pub non_maskable_interrupt: InterruptDescriptor<HandlerFunc>,
    pub breakpoint: InterruptDescriptor<HandlerFunc>,
    pub overflow: InterruptDescriptor<HandlerFunc>,
    pub bound_range_exceeded: InterruptDescriptor<HandlerFunc>,
    pub invalid_opcode: InterruptDescriptor<HandlerFunc>,
    pub device_not_available: InterruptDescriptor<HandlerFunc>,
    pub double_fault: InterruptDescriptor<HandlerFuncWithErrCode>,
    coprocessor_segment_overrun: InterruptDescriptor<HandlerFunc>,
    pub invalid_tss: InterruptDescriptor<HandlerFuncWithErrCode>,
    pub segment_not_present: InterruptDescriptor<HandlerFuncWithErrCode>,
    pub stack_segment_fault: InterruptDescriptor<HandlerFuncWithErrCode>,
    pub general_protection_fault: InterruptDescriptor<HandlerFuncWithErrCode>,
    pub page_fault: InterruptDescriptor<PageFaultHandlerFunc>,
    reserved_1: InterruptDescriptor<HandlerFunc>,
    pub x87_floating_point: InterruptDescriptor<HandlerFunc>,
    pub alignment_check: InterruptDescriptor<HandlerFuncWithErrCode>,
    pub machine_check: InterruptDescriptor<HandlerFunc>,
    pub simd_floating_point: InterruptDescriptor<HandlerFunc>,
    pub virtualization: InterruptDescriptor<HandlerFunc>,
    reserved_2: [InterruptDescriptor<HandlerFunc>; 9],
    pub security_exception: InterruptDescriptor<HandlerFuncWithErrCode>,
    reserved_3: InterruptDescriptor<HandlerFunc>,
    user_defined: [InterruptDescriptor<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: InterruptDescriptor::missing(),
            debug: InterruptDescriptor::missing(),
            non_maskable_interrupt: InterruptDescriptor::missing(),
            breakpoint: InterruptDescriptor::missing(),
            overflow: InterruptDescriptor::missing(),
            bound_range_exceeded: InterruptDescriptor::missing(),
            invalid_opcode: InterruptDescriptor::missing(),
            device_not_available: InterruptDescriptor::missing(),
            double_fault: InterruptDescriptor::missing(),
            coprocessor_segment_overrun: InterruptDescriptor::missing(),
            invalid_tss: InterruptDescriptor::missing(),
            segment_not_present: InterruptDescriptor::missing(),
            stack_segment_fault: InterruptDescriptor::missing(),
            general_protection_fault: InterruptDescriptor::missing(),
            page_fault: InterruptDescriptor::missing(),
            reserved_1: InterruptDescriptor::missing(),
            x87_floating_point: InterruptDescriptor::missing(),
            alignment_check: InterruptDescriptor::missing(),
            machine_check: InterruptDescriptor::missing(),
            simd_floating_point: InterruptDescriptor::missing(),
            virtualization: InterruptDescriptor::missing(),
            reserved_2: [InterruptDescriptor::missing(); 9],
            security_exception: InterruptDescriptor::missing(),
            reserved_3: InterruptDescriptor::missing(),
            user_defined: [InterruptDescriptor::missing(); 256 - 32],
        }
    }

    pub fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            limit: (core::mem::size_of::<Self>() - 1) as u16,
            base: self as *const _ as u64,
        }
    }

    pub fn load(&self) {
        let aptr = IDT.as_mut_ptr();
        let ptr = &self.pointer();
        println!("{:x}", ptr as *const _ as u64);
        println!("limit: {:x}, base: {:x}", ptr.limit, ptr.base);
        println!("{:x}", aptr as u64);
        unsafe {
            asm!("lidt [{}]", in(reg) ptr);
        }
    }
}

#[repr(C, packed)]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InterruptDescriptor<F> {
    func_ptr_low: u16,
    gdt_selector: u16,
    options: InterruptDescriptorOptions,
    func_ptr_mid: u16,
    func_ptr_high: u32,
    reserved: u32,
    phantom_func: PhantomData<F>,
}

impl<F> InterruptDescriptor<F> {
    pub const fn missing() -> Self {
        InterruptDescriptor {
            gdt_selector: 0,
            func_ptr_low: 0,
            func_ptr_mid: 0,
            func_ptr_high: 0,
            options: InterruptDescriptorOptions::minimal(),
            reserved: 0,
            phantom_func: PhantomData,
        }
    }

    pub fn set_handler_addr(&mut self, addr: u64) -> &mut InterruptDescriptorOptions {
        self.func_ptr_low = addr as u16;
        self.func_ptr_mid = (addr >> 16) as u16;
        self.func_ptr_high = (addr >> 32) as u32;
        self.gdt_selector = 0; // TODO
        self.options.set_present(true)
    }
}

macro_rules! impl_set_handler_fn {
    ($f:ty) => {
        impl InterruptDescriptor<$f> {
            pub fn set_handler_fn(&mut self, handler: $f) -> &mut InterruptDescriptorOptions {
                self.set_handler_addr(handler as u64)
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
impl_set_handler_fn!(PageFaultHandlerFunc);

impl<F> Debug for InterruptDescriptor<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct("InterruptDescriptor");
        s.field("func_ptr_low", &self.func_ptr_low);
        s.field("gdt_selector", &self.gdt_selector);
        s.field("options", &self.options);
        s.field("func_ptr_mid", &self.func_ptr_mid);
        s.field("func_ptr_high", &self.func_ptr_high);
        s.finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InterruptDescriptorOptions(u16);

impl InterruptDescriptorOptions {
    const fn minimal() -> Self {
        InterruptDescriptorOptions(0b1110_0000_0000)
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultHandlerFunc =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

#[derive(Clone)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl Debug for InterruptStackFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("rip", &self.rip);
        s.finish()
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

static IDT: SpinMutex<InterruptDescriptorTable> = SpinMutex::new(InterruptDescriptorTable::new());

pub fn init_idt() {
    println!("{:x}", &IDT as *const _ as u64);
    let mut idt = IDT.lock();

    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.non_maskable_interrupt
        .set_handler_fn(non_maskable_interrupt_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded
        .set_handler_fn(bound_range_exceeded_handler);

    idt.load();
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
