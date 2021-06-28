use core::fmt::*;
use core::marker::PhantomData;

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: InterruptDescriptor<HandlerFunc>,
    pub debug: InterruptDescriptor<HandlerFunc>,
    pub non_maskable_interrupt: InterruptDescriptor<HandlerFunc>,
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

#[derive(Clone, Copy, Debug)]
pub struct InterruptDescriptorOptions(u16);

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
