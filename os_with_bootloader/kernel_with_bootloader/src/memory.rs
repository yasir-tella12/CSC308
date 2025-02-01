use x86_64::{
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator, OffsetPageTable},
    VirtAddr,
};

/// Initializes paging by mapping virtual memory to physical memory.
pub unsafe fn init_memory_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut x86_64::structures::paging::PageTable {
    let table_addr = physical_memory_offset.as_u64() + 0x1000;
    &mut *(table_addr as *mut _)
}
