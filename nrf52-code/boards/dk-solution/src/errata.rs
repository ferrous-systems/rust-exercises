/// USBD cannot be enabled
pub unsafe fn e187a() {
    (0x4006_EC00 as *mut u32).write_volatile(0x9375);
    (0x4006_ED14 as *mut u32).write_volatile(3);
    (0x4006_EC00 as *mut u32).write_volatile(0x9375);
}

/// USBD cannot be enabled
pub unsafe fn e187b() {
    (0x4006_EC00 as *mut u32).write_volatile(0x9375);
    (0x4006_ED14 as *mut u32).write_volatile(0);
    (0x4006_EC00 as *mut u32).write_volatile(0x9375);
}
