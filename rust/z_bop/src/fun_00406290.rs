// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00a08104: *mut u8;
    static mut DAT_009a1fe8: *mut u8;
    static mut DAT_009a8e18: *mut u8;
    static mut DAT_009e9640: i32;
    static mut DAT_009e96fd: [u8; 0];
    static mut DAT_009e96dd: [u8; 0];
    static mut DAT_009e968a: [u8; 0];
    static mut DAT_009e9699: [u8; 0];
    static mut DAT_0051a3de: [u16; 0];
    static mut DAT_009d6990: [u32; 0];
    static mut DAT_009d8c88: [u8; 0];
    static mut DAT_009d8c84: [u8; 0];
    static mut DAT_009d9155: [u8; 0];
    static mut DAT_0051c5aa: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_004065d0(param_1: i32, param_2: i32, param_3: i32) -> u32;
    fn FUN_0042e720(param_1: u32, param_2: i32);
    fn FUN_00407910(param_1: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_00406290(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: u32,
) -> i32 {
    // SAFETY: all globals and callees are the game's own memory; offsets are
    // computed exactly as the original does.
    unsafe {
        let i_var6: *mut u8 = DAT_00a08104;
        let i_var1: i32 = (param_1 as u32).wrapping_mul(0x23) as i32;
        let i_var5: i32 = DAT_009a1fe8
            .offset(0x1b)
            .offset(i_var1 as isize)
            .cast::<i32>()
            .read_unaligned();
        let i_var9: i32 = (param_2 as u32).wrapping_mul(4) as i32;
        if *DAT_00a08104.offset(i_var9 as isize).offset(0x370) == 0 {
            return -1;
        }
        if *DAT_00a08104.offset(i_var9 as isize).offset(0x372) == 0 {
            return -1;
        }
        let u_var7: u32 = FUN_004065d0(param_1, param_2, param_3);
        if u_var7 == 0xffff_ffff {
            return -1;
        }
        let p = DAT_00a08104.offset(0x254);
        *p = (*p).wrapping_add(1);
        let tbl = (&raw mut DAT_0051a3de).cast::<u16>();
        if i_var5 != -1 {
            let i_tmp = (i_var5 as u32).wrapping_mul(0x5bd) as i32;
            let q = (&raw mut DAT_009e96fd)
                .cast::<u8>()
                .offset(i_tmp as isize)
                .cast::<i16>();
            let s = q.read_unaligned().wrapping_add(1);
            q.write_unaligned(s);
            let q = (&raw mut DAT_009e96dd)
                .cast::<u8>()
                .offset(i_tmp as isize)
                .cast::<u32>();
            let u = q
                .read_unaligned()
                .wrapping_sub(u32::from(tbl.offset(param_3 as isize).read()));
            q.write_unaligned(u);
        }
        let idx = *(&raw mut DAT_009d8c88).cast::<u8>().offset(
            ((*DAT_009a1fe8.offset(0x16).offset(i_var1 as isize) as u32).wrapping_mul(0x566))
                as isize,
        );
        let arr = (&raw mut DAT_009d6990).cast::<u32>().offset(idx as isize);
        arr.write(
            arr.read()
                .wrapping_sub(u32::from(tbl.offset(param_3 as isize).read())),
        );
        FUN_0042e720(u_var7, i_var5);
        let i_var8 = ((*DAT_009a1fe8.offset(0x16).offset(i_var1 as isize) as u32)
            .wrapping_mul(0x566)) as i32;
        if *(&raw mut DAT_009d8c84).cast::<u8>().offset(i_var8 as isize) != 0x15
            && *(&raw mut DAT_009d9155).cast::<u8>().offset(i_var8 as isize) != 0x15
        {
            let pc = DAT_00a08104.offset(0x372).offset(i_var9 as isize);
            *pc = (*pc).wrapping_sub(1);
        }
        let i_var8 = u_var7.wrapping_sub(DAT_009e9640 as u32) as i32;
        let mut i_var1 = (i_var8 as u32).wrapping_mul(10) as i32;
        if param_4 < 2 {
            let s = DAT_00a08104.offset(0x23b).cast::<i16>().read_unaligned();
            *DAT_009a8e18.offset(1).offset(i_var1 as isize) = ((s as i32) / 0xec) as u8;
            if 6 < *DAT_009a8e18.offset(1).offset(i_var1 as isize) {
                *DAT_009a8e18.offset(1).offset(i_var1 as isize) = 6;
            }
        }
        i_var1 = (i_var8 as u32).wrapping_mul(10) as i32;
        'lab: {
            if i_var5 == -1 {
                let v = i_var6.offset(0x42).cast::<u16>().read_unaligned();
                DAT_009a8e18
                    .offset(4)
                    .offset(i_var1 as isize)
                    .cast::<u16>()
                    .write_unaligned(v);
                let v = i_var6.offset(0x46).cast::<u16>().read_unaligned();
                DAT_009a8e18
                    .offset(2)
                    .offset(i_var1 as isize)
                    .cast::<u16>()
                    .write_unaligned(v);
                let u_var4 = i_var6.offset(0x42).cast::<u16>().read_unaligned();
                if u_var4 != 0xffff && u_var4 < 0x8000 {
                    let v = DAT_009a1fe8
                        .offset((u32::from(u_var4).wrapping_mul(0x23)) as isize)
                        .cast::<u16>()
                        .read_unaligned();
                    DAT_009a8e18
                        .offset(6)
                        .offset(i_var1 as isize)
                        .cast::<u16>()
                        .write_unaligned(v);
                    break 'lab;
                }
            } else {
                let i_tmp = (i_var5 as u32).wrapping_mul(0x5bd) as i32;
                let v = (&raw mut DAT_009e968a)
                    .cast::<u8>()
                    .offset(i_tmp as isize)
                    .cast::<u16>()
                    .read_unaligned();
                DAT_009a8e18
                    .offset(4)
                    .offset(i_var1 as isize)
                    .cast::<u16>()
                    .write_unaligned(v);
                let v = (&raw mut DAT_009e9699)
                    .cast::<u8>()
                    .offset(i_tmp as isize)
                    .cast::<u16>()
                    .read_unaligned();
                DAT_009a8e18
                    .offset(2)
                    .offset(i_var1 as isize)
                    .cast::<u16>()
                    .write_unaligned(v);
                let us = (&raw mut DAT_009e968a)
                    .cast::<u8>()
                    .offset(i_tmp as isize)
                    .cast::<u16>()
                    .read_unaligned();
                if us != 0xffff {
                    let v = DAT_009a1fe8
                        .offset((u32::from(us).wrapping_mul(0x23)) as isize)
                        .cast::<u16>()
                        .read_unaligned();
                    DAT_009a8e18
                        .offset(6)
                        .offset(i_var1 as isize)
                        .cast::<u16>()
                        .write_unaligned(v);
                    break 'lab;
                }
            }
            DAT_009a8e18
                .offset(6)
                .offset(i_var1 as isize)
                .cast::<u16>()
                .write_unaligned(0);
        }
        *DAT_009a8e18.offset(i_var1 as isize) = i_var5 as u8;
        let u_var4 = DAT_009a8e18
            .offset(4)
            .offset(i_var1 as isize)
            .cast::<u16>()
            .read_unaligned();
        if u_var4 != 0xffff {
            let i_tmp = DAT_009a1fe8
                .offset((u32::from(u_var4).wrapping_mul(0x23)) as isize)
                .offset(0x1b)
                .cast::<i32>()
                .read_unaligned();
            if i_tmp != -1 {
                FUN_00407910(i_var8 as u16);
            }
        }
        if param_4 < 2 {
            let k = *(&raw mut DAT_0051c5aa).cast::<u8>().offset(
                (param_4.wrapping_add((*DAT_00a08104.offset(4) as u32).wrapping_mul(0x101)))
                    as isize,
            );
            let k4 = (u32::from(k)).wrapping_mul(4) as isize;
            if *DAT_00a08104.offset(0x376).offset(k4) <= *DAT_00a08104.offset(0x372).offset(k4) {
                let pb = DAT_00a08104.offset(0x235).offset(param_4 as isize);
                *pb &= 0x7f;
                return i_var8;
            }
            let pb = DAT_00a08104.offset(0x235).offset(param_4 as isize);
            *pb |= 0x80;
        }
        i_var8
    }
}
