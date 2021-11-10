use uefi::table::boot::*;
use x86_64::structures::paging::mapper::*;

pub fn get_memory_descriptor_page_table_is_inside(
    start: usize,
    mm_iter: impl ExactSizeIterator<Item = &MemoryDescriptor>,
) {
    const PAGE_SIZE: u64 = 0x1000; // This info is from maystorm
    for mdesc in mm_iter {
        let frame_head = mdesc.phys_start;
        let frame_size = mdesc.page_count * PAGE_SIZE;
        let frame_tail = frame_head + frame_size - 1; // This var is inside memdesc!!!
        if frame_head <= start && start <= frame_tail {}
    }
}
