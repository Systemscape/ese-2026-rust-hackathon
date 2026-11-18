/*
 * Adapted from: https://github.com/embassy-rs/embassy/blob/2b69fd062148e2b1ec678b04b01499c1af5fd39e/examples/rp235x/memory.x
 */

MEMORY {
    /* RP2354: 2 MiB stacked flash (XIP) */
    FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
    /* SRAM0–7, striped mapping */
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
    /* SRAM8/9, direct mapping, z. B. für separate Stacks (Core1) */
    SRAM8 : ORIGIN = 0x20080000, LENGTH = 4K
    SRAM9 : ORIGIN = 0x20081000, LENGTH = 4K
}

SECTIONS {
    /* Boot-ROM-Block (IMAGE_DEF), muss in den ersten 4K des Flash liegen */
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

/* .text erst nach dem Boot-Block beginnen */
/*
 * _stext = ADDR(.start_block) + SIZEOF(.start_block);
 * führt teilweise zu Linkerwarnungen:
 * warning: linker stderr: rust-lld: address (0x1000013c) of section .text is not a multiple of alignment (8)
 */
_stext = ALIGN(ADDR(.start_block) + SIZEOF(.start_block), 8);

SECTIONS {
    /* picotool Binary-Info-Einträge */
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    /* Abschließender Block, z. B. für Signaturen */
    .end_block : ALIGN(4)
    {
        __end_block_addr = .;
        KEEP(*(.end_block));
    } > FLASH
} INSERT AFTER .uninit;

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);

