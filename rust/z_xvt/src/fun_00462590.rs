// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004153d0(param_1: i32, param_2: i32, param_3: i32, param_4: u32);
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller supplies an address inside the game's own data block.
    unsafe { (addr as usize as *const i32).read_unaligned() }
}

unsafe fn wr(addr: i32, v: i32) {
    // SAFETY: the caller supplies an address inside the game's own data block.
    unsafe { (addr as usize as *mut i32).write_unaligned(v) }
}

unsafe fn conv(src: i32, dst: *mut u8, count: u32) {
    let mut s = src as usize as *const u8;
    let mut d = dst;
    let mut n: u32 = 0;
    // SAFETY: the source points at the game's palette data and dst at a
    // local buffer large enough for fewer than 0x400 entries of 3 bytes.
    unsafe {
        if count != 0 {
            loop {
                *d = *s >> 2;
                *d.add(1) = *s.add(1) >> 2;
                *d.add(2) = *s.add(2) >> 2;
                s = s.add(4);
                d = d.add(3);
                n = n.wrapping_add(1);
                if n >= count {
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00462590(param_1: *mut i32) {
    let mut local_c00 = [0u8; 3072];
    let base = param_1 as usize as i32;
    // SAFETY: param_1 points at the game's own block; all offsets read and
    // written are those the original uses, and the local buffer is only
    // filled with fewer than 0x400 three-byte entries.
    unsafe {
        let mut i_var3 = rd(base).wrapping_add(base);
        if rd(base.wrapping_add(0x2c)) == 0x18 {
            let u_var6 = rd(base.wrapping_add(0x30)) as u32;
            wr(base.wrapping_add(0x14), i_var3.wrapping_sub(base));
            if u_var6 < 0x400 {
                let src = rd(base.wrapping_add(0xc)).wrapping_add(base);
                conv(src, local_c00.as_mut_ptr(), u_var6);
                FUN_004153d0(local_c00.as_mut_ptr() as usize as i32, i_var3, 0, u_var6);
                i_var3 = i_var3.wrapping_add(rd(base.wrapping_add(0x30)).wrapping_mul(2));
            }
        }
        let mut local_c04: u32 = 0;
        if rd(base.wrapping_add(0x18)) != 0 {
            loop {
                let i_var5 = rd(base
                    .wrapping_add(rd(base.wrapping_add(0x10)))
                    .wrapping_add((local_c04 as i32).wrapping_mul(4)))
                .wrapping_add(base);
                wr(i_var5.wrapping_add(0xc), i_var3.wrapping_sub(i_var5));
                if rd(i_var5.wrapping_add(0x24)) == 0x18
                    && (rd(i_var5.wrapping_add(0x28)) as u32) < 0x400
                {
                    let cnt = rd(i_var5.wrapping_add(0x28)) as u32;
                    let src = rd(i_var5.wrapping_add(4)).wrapping_add(i_var5);
                    conv(src, local_c00.as_mut_ptr(), cnt);
                    FUN_004153d0(
                        local_c00.as_mut_ptr() as usize as i32,
                        i_var3,
                        0,
                        rd(i_var5.wrapping_add(0x28)) as u32,
                    );
                    i_var3 = i_var3.wrapping_add(rd(i_var5.wrapping_add(0x28)).wrapping_mul(2));
                }
                local_c04 = local_c04.wrapping_add(1);
                if local_c04 >= rd(base.wrapping_add(0x18)) as u32 {
                    break;
                }
            }
        }
    }
}
