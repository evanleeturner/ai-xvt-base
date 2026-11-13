// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa5e10: i32;
    static mut DAT_00aa5e68: i32;
    static mut DAT_00aa5aac: i16;
    static mut DAT_00a91d18: u8;
    static mut DAT_00aa5e38: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004f36d0() {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_00aa5e10 holds 10 ints and the records at DAT_00a91d18 are
    // 0x562 bytes apart, read where the original reads them.
    unsafe {
        let table = &raw mut DAT_00aa5e10;
        let mut p = table;
        let mut i: i32 = 10;
        while i != 0 {
            p.write_unaligned(0);
            p = p.add(1);
            i = i.wrapping_sub(1);
        }
        DAT_00aa5e68 = 0;
        if 0 < DAT_00aa5aac {
            let mut pb = (&raw mut DAT_00a91d18) as *const u8;
            let mut n: i32 = DAT_00aa5aac as i32;
            loop {
                if pb.byte_add(0xc).read() != 0 {
                    let slot = table.add(pb.read() as usize);
                    slot.write_unaligned(slot.read_unaligned().wrapping_add(1));
                }
                pb = pb.byte_add(0x562);
                n = n.wrapping_sub(1);
                if n == 0 {
                    break;
                }
            }
        }
        let end = (&raw mut DAT_00aa5e38) as *const i32;
        let mut q = table as *const i32;
        loop {
            if q.read_unaligned() != 0 {
                DAT_00aa5e68 = DAT_00aa5e68.wrapping_add(1);
            }
            q = q.add(1);
            if q >= end {
                break;
            }
        }
    }
}
