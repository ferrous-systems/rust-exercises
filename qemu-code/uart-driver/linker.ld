/* RAM starts at 0x40000000 but if we ask to load the kernel there, QEMU will not load a DTB */

MEMORY {
    RAM : ORIGIN = 0x40100000, LENGTH = 128M
}

ENTRY(_start)
SECTIONS {
    .startup ORIGIN(RAM) : {
        *(.text.startup)
    } > RAM
    .text : { *(.text .text*) } > RAM
    .rodata : { *(.rodata .rodata*) } > RAM
    .data : { *(.data .data*) } > RAM
    .bss : { *(.bss .bss* COMMON) } > RAM
    /DISCARD/ : {
        *(.note .note*)
    }
    . = ALIGN(16);
    . += 0x100000; /* 1024kB of stack memory */
    stack_top = .;
}
