use crate::println;
use x86_64::structures::paging::{page_table::FrameError, PageTable};
use x86_64::{registers::control::Cr3, PhysAddr, VirtAddr};

pub fn init_kernel_page_table() {
    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let l4_table = unsafe { get_table_refmut(phys, 0) }; // UEFI page table is Identity Mapping

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let l3_table = unsafe { get_table_refmut(phys, 0) };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }
    println!(
        "{}",
        translate_addr(VirtAddr::new(0x1000), 0).unwrap().as_u64()
    );
}

unsafe fn get_table_refmut(start: PhysAddr, offset: u64) -> &'static mut PageTable {
    let virt = start.as_u64() + offset;
    let page_table_ptr = VirtAddr::new(virt).as_mut_ptr();
    &mut *page_table_ptr
}

unsafe fn get_table_ref(start: PhysAddr, offset: u64) -> &'static PageTable {
    let virt = start.as_u64() + offset;
    let page_table_ptr = VirtAddr::new(virt).as_ptr();
    &*page_table_ptr
}

fn translate_addr(addr: VirtAddr, physical_offset: u64) -> Option<PhysAddr> {
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];

    let (mut frame, _) = Cr3::read();
    for &index in &table_indexes {
        let phys = PhysAddr::new(physical_offset + frame.start_address().as_u64());
        let table = unsafe { get_table_ref(phys, physical_offset) };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
