// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_006ccad0: u16;
    static mut DAT_006ccad2: u16;
    static mut DAT_006cca90: [u8; 0];
    static mut DAT_006cca92: [u8; 0];
    static mut DAT_006d4988: *mut u8;
    static mut DAT_006db71c: *mut u8;
    static mut DAT_00732940: *mut u8;
    static mut DAT_00732e54: *mut u8;
    static mut DAT_006db6c0: u16;
    static mut DAT_007394c4: u8;
    static mut DAT_007394c3: u8;
    static mut DAT_007394d2: u8;
    static mut DAT_007394d0: u8;
    static mut DAT_007394c5: u8;
    static mut DAT_007394c7: u8;
    static mut _DAT_007394d0: u32;
    static mut DAT_007394c2: i8;
    static mut DAT_007394cc: i8;
    static mut DAT_007394ca: i8;
    static mut DAT_007394cb: i8;
    static mut DAT_0073abc1: i8;
    static mut DAT_007394e8: [u8; 0];
    static mut DAT_007394e4: [u8; 0];
    static mut DAT_007394c8: u16;
    static mut DAT_007394ce: u16;
    static mut DAT_0073b374: u16;
    static mut DAT_0073b370: u8;
    static mut DAT_0073b371: u8;
    static mut DAT_0073b372: u8;
    static mut DAT_0073abc0: u8;
    static mut DAT_00536e8c: i8;
    static mut DAT_007092c1: i8;
    static mut DAT_0073abc4: i32;
    static mut DAT_0073abc8: i32;
    static mut DAT_0073abcc: i32;
    static mut DAT_0070ab14: i32;
    static mut DAT_0070ab18: i32;
    static mut DAT_0070ac20: i32;
    static mut DAT_0073abd0: [u8; 0];
    static mut DAT_00709fc4: [u8; 0];
    static mut DAT_006d4a68: [u8; 0];
    static mut DAT_006da3e0: [u8; 0];
    static mut DAT_006da412: u8;
    static mut DAT_006da414: u8;
    static mut DAT_005247e8: [u8; 0];
    static mut DAT_0052686c: [u8; 0];
    static mut DAT_0052686a: [u8; 0];
    static mut DAT_00524778: [u8; 0];
    static mut DAT_00524788: [u8; 0];
    static mut DAT_00524870: [u8; 0];
    static mut DAT_00524880: [u8; 0];
    static mut DAT_00524898: [u8; 0];
    static mut DAT_005248b8: [u8; 0];
    static mut DAT_00524798: [u8; 0];
    static mut DAT_005247c0: [u8; 0];
    static mut DAT_0052a650: [u8; 0];
    static mut DAT_0052a7b0: [u8; 0];
    static mut DAT_0052a948: [u8; 0];
    static mut DAT_0052aae0: [u8; 0];
    static mut DAT_0052ac78: [u8; 0];
    static mut DAT_0052b080: [u8; 0];
    static mut DAT_0052b081: [u8; 0];
    static mut DAT_0052b082: [u8; 0];
    static mut DAT_0052b083: [u8; 0];
    static mut DAT_0052b087: [u8; 0];
    static mut DAT_0052b089: [u8; 0];
    static mut DAT_0052b08d: [u8; 0];
    static mut DAT_0052b093: [u8; 0];
    static mut DAT_0052b097: [u8; 0];
    static mut DAT_0052b099: [u8; 0];
    static mut DAT_0052b0a8: [u8; 0];
    static mut DAT_0052b0aa: [u8; 0];
    static mut DAT_0052b0ac: [u8; 0];
    static mut DAT_0052b0ae: [u8; 0];
    static mut DAT_0052b0b0: [u8; 0];
    static mut DAT_0052b0b4: [u8; 0];
    static mut DAT_0052b0b6: [u8; 0];
    static mut DAT_0052b0ba: [u8; 0];
    static mut DAT_0052b13c: [u8; 0];
    static mut DAT_0052b157: [u8; 0];
    static mut DAT_0052b159: [u8; 0];
    static mut DAT_0052b15b: [u8; 0];
    static mut DAT_0052b15d: [u8; 0];
    static mut DAT_0070ad42: [u8; 0];
    static mut DAT_0070ad45: [u8; 0];
    static mut DAT_0070ad46: [u8; 0];
    static mut DAT_0070ad4a: [u8; 0];
    static mut DAT_0070ad56: [u8; 0];
    static mut DAT_0070ad40: [u8; 0];
    static mut DAT_0070ad2c: [u8; 0];
    static mut DAT_0070ad18: [u8; 0];
    static mut DAT_0070ad92: [u8; 0];
    static mut DAT_0070ad93: [u8; 0];
    static mut DAT_0070b252: [u8; 0];
    static mut DAT_0070b216: [u8; 0];
    static mut DAT_0070b210: [u8; 0];
    static mut DAT_0070b213: [u8; 0];
    static mut DAT_00703ca0: [u8; 0];
    static mut DAT_00524948: [i8; 0];
    static mut DAT_00524934: [i8; 0];
    static mut DAT_00524920: [i8; 0];
    static mut DAT_00524914: [i8; 0];
    static mut DAT_00524904: [i8; 0];
    static mut DAT_0071b730: [u8; 0];
    static mut DAT_0071b734: [u8; 0];
    static mut DAT_0071b73e: [u8; 0];
    static mut DAT_0071b742: [u8; 0];
    static mut DAT_0071b757: [u8; 0];
    static mut DAT_0071b758: [u8; 0];
    static mut DAT_0071b761: [u8; 0];
    static mut DAT_0071b763: [u8; 0];
    static mut DAT_0071b765: [u8; 0];
    static mut DAT_0071b766: [u8; 0];
    static mut DAT_0071b767: [u8; 0];
    static mut DAT_0071b768: [u8; 0];
    static mut DAT_0071b769: [u8; 0];
    static mut DAT_0071b76a: [u8; 0];
    static mut DAT_0071b79d: [u8; 0];
    static mut DAT_0071b7a1: [u8; 0];
    static mut DAT_0071b7a5: [u8; 0];
    static mut DAT_0071b7a9: [u8; 0];
    static mut DAT_0071b7ad: [u8; 0];
    static mut DAT_0071b7b1: [u8; 0];
    static mut DAT_0071b7b5: [u8; 0];
    static mut DAT_0071b7b7: [u8; 0];
    static mut DAT_0071b7b9: [u8; 0];
    static mut DAT_0071b7bb: [u8; 0];
    static mut DAT_0071b7bd: [u8; 0];
    static mut DAT_0071b7bf: [u8; 0];
    static mut DAT_0071b7c1: [u8; 0];
    static mut DAT_0071b7c3: [u8; 0];
    static mut DAT_0071b7c5: [u8; 0];
    static mut DAT_0071b825: [u8; 0];
    static mut DAT_0071b885: [u8; 0];
    static mut DAT_0071b8e5: [u8; 0];
    static mut DAT_0071b917: [u8; 0];
    static mut DAT_0071b949: [u8; 0];
    static mut DAT_0071b97b: [u8; 0];
    static mut DAT_0071b987: [u8; 0];
    static mut DAT_0071b993: [u8; 0];
    static mut DAT_0071b99f: [u8; 0];
    static mut DAT_0071b9af: [u8; 0];
    static mut DAT_0071b9bf: [u8; 0];
    static mut DAT_0071b9c1: [u8; 0];
    static mut DAT_0071b9c3: [u8; 0];
    static mut DAT_0071b9c5: [u8; 0];
    static mut DAT_0071b9c7: [u8; 0];
    static mut DAT_0071b9c9: [u8; 0];
    static mut DAT_0071b9d9: [u8; 0];
    static mut DAT_0071b9e9: [u8; 0];
    static mut DAT_0071ba49: [u8; 0];
    static mut DAT_0071baa9: [u8; 0];
    static mut DAT_0071badb: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_0040f050(a: u16);
    fn FUN_004a7aa0(a: u32) -> u32;
    fn FUN_0042a230(a: i32, b: u32, c: i32, d: i16, e: i16, f: i16);
    fn FUN_00478320(a: u32, b: u16) -> u32;
    fn FUN_004490f0(a: u32) -> i32;
    fn FUN_00449790(a: u32, b: u32) -> i32;
    fn FUN_0044a600(a: u32, b: u32) -> u32;
    fn FUN_00481380(a: *mut i8) -> i32;
    fn FUN_00478470() -> u16;
    fn FUN_004296b0(a: u16);
    fn FUN_00429310(a: u32);
    fn FUN_004213b0(a: i32);
}

unsafe fn ld8(p: *const u8, o: u32) -> u8 {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).read() }
}
unsafe fn ld16(p: *const u8, o: u32) -> u16 {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).cast::<u16>().read_unaligned() }
}
unsafe fn ld32(p: *const u8, o: u32) -> u32 {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).cast::<u32>().read_unaligned() }
}
unsafe fn st8(p: *mut u8, o: u32, v: u8) {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).write(v) }
}
unsafe fn st16(p: *mut u8, o: u32, v: u16) {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).cast::<u16>().write_unaligned(v) }
}
unsafe fn st32(p: *mut u8, o: u32, v: u32) {
    // SAFETY: the caller supplies a pointer into the game's own memory.
    unsafe { p.wrapping_add(o as usize).cast::<u32>().write_unaligned(v) }
}

unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting on zero as the original does.
    unsafe {
        asm!("cdq", "idiv {d:e}", d = in(reg) b, inout("eax") a => q, out("edx") _, options(nomem, nostack));
    }
    q
}

unsafe fn cmps(mut n: u32, mut a: *const i8, mut b: *const i8, mut flag: bool) -> (u32, bool) {
    loop {
        if n == 0 {
            break;
        }
        n -= 1;
        // SAFETY: both strings are the game's own NUL-padded buffers.
        flag = unsafe { *a == *b };
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        if !flag {
            break;
        }
    }
    (n, flag)
}

fn sx(v: u16) -> u32 {
    v as i16 as i32 as u32
}
fn sxi(v: i16) -> u32 {
    v as i32 as u32
}

macro_rules! ad {
    ($x:ident) => {
        (&raw mut $x).cast::<u8>()
    };
}
macro_rules! ip {
    ($x:expr) => {
        (($x) as u32 as usize) as *mut u8
    };
}
macro_rules! sp {
    ($p:expr) => {
        (ld32(($p) as *const u8, 0x1f) as usize) as *mut u8
    };
}
macro_rules! l8 {
    ($p:expr, $o:expr) => {
        ld8(($p) as *const u8, ($o) as u32)
    };
}
macro_rules! l16 {
    ($p:expr, $o:expr) => {
        ld16(($p) as *const u8, ($o) as u32)
    };
}
macro_rules! l32 {
    ($p:expr, $o:expr) => {
        ld32(($p) as *const u8, ($o) as u32)
    };
}
macro_rules! s8 {
    ($p:expr, $o:expr, $v:expr) => {
        st8(($p) as *mut u8, ($o) as u32, ($v) as u8)
    };
}
macro_rules! s16 {
    ($p:expr, $o:expr, $v:expr) => {
        st16(($p) as *mut u8, ($o) as u32, ($v) as u16)
    };
}
macro_rules! s32 {
    ($p:expr, $o:expr, $v:expr) => {
        st32(($p) as *mut u8, ($o) as u32, ($v) as u32)
    };
}
macro_rules! r566 {
    () => {
        (DAT_0073b374 as u32).wrapping_mul(0x566)
    };
}

#[unsafe(no_mangle)]
#[allow(
    non_snake_case,
    unused_assignments,
    unused_variables,
    unused_mut,
    unused_unsafe
)]
pub unsafe extern "stdcall" fn FUN_0041f670() -> u16 {
    // SAFETY: every global, callee and pointer here is the game's own memory
    // and code, read and written at the offsets the original uses.
    unsafe {
        let mut local_14: i32 = 0;
        let mut uVar20: u16 = DAT_006ccad0;
        let mut uVar26: u16 = DAT_006ccad0;
        while uVar26 < DAT_006ccad2
            && l8!(
                DAT_006d4988,
                (uVar26 as u32).wrapping_mul(0x23).wrapping_add(3)
            ) != 0
        {
            uVar26 = uVar26.wrapping_add(1);
        }
        if DAT_006ccad2 <= uVar26 {
            return 0xffff;
        }
        let mut uVar27: u32 = uVar26 as u32;
        FUN_0040f050(uVar26);
        let mut iVar15: i32 = uVar27.wrapping_mul(0x23) as i32;
        let puVar32: *mut u8 = DAT_006d4988.wrapping_add(iVar15 as u32 as usize);
        s8!(sp!(puVar32), 0, 7);
        s8!(sp!(puVar32), 0x86, DAT_007394c4);
        s8!(sp!(puVar32), 0x87, DAT_007394c3);
        s16!(sp!(puVar32), 0x81, 0);
        s16!(sp!(puVar32), 0x7f, 0);
        s32!(
            ad!(DAT_007394e8),
            uVar27.wrapping_mul(0xc),
            uVar27.wrapping_sub(uVar20 as u32)
        );
        let puVar2: *mut u8 = DAT_006db71c
            .wrapping_add(uVar27.wrapping_sub(uVar20 as u32).wrapping_mul(0x4c) as usize);
        s32!(sp!(puVar32), 0xab, puVar2 as usize);
        s32!(sp!(puVar32), 0xa3, 0);
        s32!(sp!(puVar32), 0xa7, 0);
        s8!(DAT_006d4988, (2 + iVar15) as u32, 0x10);
        s8!(DAT_006d4988, (3 + iVar15) as u32, 200);
        s16!(DAT_006d4988, iVar15, DAT_006db6c0);
        DAT_006db6c0 = DAT_006db6c0.wrapping_add(1);
        let mut bVar16: u8 = l8!(ad!(DAT_005247e8), l8!(ad!(DAT_0070ad42), r566!()));
        let mut uVar24: u32 = (((((DAT_0073b374 as u32).wrapping_mul(0x159)) >> 8) << 8)
            | bVar16 as u32)
            & 0xffff00ff;
        let mut bVar10: u8 = uVar24 as u8;
        DAT_007394d2 = bVar10;
        uVar20 = l16!(ad!(DAT_006cca90), (DAT_0073b370 as u32) * 4);
        uVar26 = uVar20;
        while uVar26 < l16!(ad!(DAT_006cca92), (DAT_0073b370 as u32) * 4)
            && l8!(
                DAT_006d4988,
                (uVar26 as u32).wrapping_mul(0x23).wrapping_add(3)
            ) != 0
        {
            uVar26 = uVar26.wrapping_add(1);
        }
        let uStack_1d: u8 = (uVar26 >> 8) as u8;
        let cStack_1e: i8 = uVar26 as i8;
        if l16!(ad!(DAT_006cca92), (DAT_0073b370 as u32) * 4) <= uVar26 {
            return 0xffff;
        }
        let uVar6: u32 = uVar26 as u32;
        FUN_0040f050(uVar26);
        iVar15 = l32!(ad!(DAT_0070b252), r566!()) as i32;
        let mut bVar8: i32;
        if iVar15 == -1
            || (l8!(ad!(DAT_0070ad56), r566!()) as u32) != _DAT_007394d0
            || DAT_00536e8c == 0
        {
            bVar8 = 0;
            s32!(
                DAT_006d4988,
                0x1bu32.wrapping_add(uVar6.wrapping_mul(0x23)),
                0xffffffffu32
            );
        } else {
            s32!(
                ad!(DAT_0071b730),
                (iVar15 as u32).wrapping_mul(0x5bd),
                uVar6
            );
            bVar8 = 1;
            s32!(
                DAT_006d4988,
                0x1bu32.wrapping_add(uVar6.wrapping_mul(0x23)),
                iVar15
            );
            local_14 = iVar15;
        }
        let iVar7: u32 = uVar6.wrapping_mul(0x23);
        iVar15 = l32!(ad!(DAT_0070b252), r566!()) as i32;
        let mut iVar18: i32 = uVar6.wrapping_sub(uVar20 as u32) as i32;
        s32!(ad!(DAT_007394e4), uVar6.wrapping_mul(0xc), iVar18);
        iVar18 = (iVar18 as u32).wrapping_mul(0x436) as i32;
        s32!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0xa7,
            (DAT_00732940 as usize as u32).wrapping_add(iVar18 as u32)
        );
        DAT_00732e54 = DAT_00732940.wrapping_add(iVar18 as u32 as usize);
        s32!(ad!(DAT_0073abd0), uVar6.wrapping_mul(4), uVar27);
        s32!(DAT_00732e54, 0x432, puVar32 as usize);
        s16!(DAT_00732e54, 0x384, l16!(puVar32, 0));
        s8!(DAT_006d4988, 3 + iVar7, bVar10);
        s16!(DAT_006d4988, iVar7, DAT_006db6c0);
        DAT_006db6c0 = DAT_006db6c0.wrapping_add(1);
        uVar27 = bVar16 as u32;
        uVar20 = l16!(ad!(DAT_0052686c), uVar27 * 0x16);
        if (uVar24 as u16 as i16) == 0x3a {
            uVar20 = uVar20.wrapping_shl(3);
        }
        let mut sVar21: i16;
        if uVar20 < 0x2000 {
            sVar21 = uVar20.wrapping_shl(2) as i16;
        } else {
            sVar21 = 0x7fff;
        }
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x7d, sVar21);
        uVar24 = FUN_004a7aa0(uVar24);
        s8!(DAT_00732e54, 4, uVar24);
        uVar24 = l32!(DAT_00732e54, 4);
        uVar20 = (uVar24 as u8) as u16;
        s8!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0x86,
            DAT_007394c4
        );
        let piVar3: *mut u8 = DAT_006d4988.wrapping_add((0x1f + iVar7) as usize);
        DAT_0073b371 = l8!(ip!(l32!(piVar3, 0)), 0x86);
        s8!(ip!(l32!(piVar3, 0)), 0x87, DAT_007394c3);
        s8!(DAT_006d4988, 2 + iVar7, DAT_0073b370);
        s8!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0,
            l8!(ad!(DAT_0052686a), uVar27 * 0x16)
        );
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x81, 0);
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x7f, 0);
        s32!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 2, 0);
        s8!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0x88,
            l8!(ad!(DAT_0070ad4a), r566!())
        );
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x83, uVar26);
        s8!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x85, bVar10);
        s8!(DAT_006d4988, 0x16 + iVar7, DAT_0073b374 as u8);
        s8!(DAT_00732e54, 5, DAT_007394c2);
        if DAT_007394c2 == -1 {
            DAT_007394c2 = cStack_1e;
        }
        s8!(DAT_00732e54, 0xae, DAT_0073b372);
        bVar16 = l8!(DAT_00732e54, 0xae);
        s8!(DAT_00732e54, 0xb0, DAT_007394d0);
        let mut uVar13: u32 = l32!(DAT_00732e54, 0xb0);
        let mut uVar12: u16 = DAT_007394c5 as u16;
        if DAT_007394cc != 0 {
            uVar12 = 0;
        }
        s8!(DAT_00732e54, 0xaf, uVar12);
        s32!(DAT_00732e54, 0xb9, 0);
        s32!(DAT_00732e54, 0xb5, l32!(DAT_00732e54, 0xb9));
        s32!(DAT_00732e54, 0xb1, l32!(DAT_00732e54, 0xb5));
        iVar18 = r566!() as i32;
        let mut uVar28: u32;
        if l8!(ad!(DAT_0070b216), iVar18) == 0 || l8!(ad!(DAT_0070b210), iVar18) != 0 {
            s32!(
                DAT_00732e54,
                0,
                l16!(
                    ad!(DAT_006d4a68),
                    (DAT_0073b374 as u32).wrapping_mul(0x93).wrapping_mul(2)
                ) as u32
            );
            uVar28 = l32!(DAT_00732e54, 0).wrapping_add(1);
        } else {
            let k = (l8!(ad!(DAT_0070b216), iVar18) as u32) * 4;
            uVar28 = l32!(ad!(DAT_00709fc4), k).wrapping_add(1);
            s32!(ad!(DAT_00709fc4), k, uVar28);
        }
        s32!(DAT_00732e54, 0, uVar28);
        let iVar5: u32 = (uVar20 as u32).wrapping_mul(0xef);
        sVar21 = l16!(ad!(DAT_0052b15d), iVar5) as i16;
        let mut sVar11: i16 = uVar12.wrapping_add(1) as i16;
        iVar18 = (((uVar13 as u8 as u32).wrapping_add((bVar16 as u32).wrapping_mul(6)))
            .wrapping_mul(2)) as i32;
        let mut sVar31: i16 = sx(l16!(ad!(DAT_0052b159), iVar5))
            .wrapping_mul(sxi(sVar11))
            .wrapping_mul(sx(l16!(ad!(DAT_0052a7b0), iVar18))) as i16;
        let mut uStack_20: u16 = sxi(sVar11)
            .wrapping_mul(sx(l16!(ad!(DAT_0052a948), iVar18)))
            .wrapping_mul(sxi(sVar21)) as u16;
        let mut sVar23: i16 = sxi(sVar11)
            .wrapping_mul(sx(l16!(ad!(DAT_0052aae0), iVar18)))
            .wrapping_mul(sx(l16!(ad!(DAT_0052b15b), iVar5))) as i16;
        if sVar11 == 1 {
            sVar31 = sxi(sVar31).wrapping_add(
                (((l16!(ad!(DAT_0052b159), iVar5) as i16 as i32) / 2) as u32)
                    .wrapping_mul(sx(l16!(ad!(DAT_0052a7b0), iVar18))),
            ) as i16;
            sVar23 = sxi(sVar23).wrapping_add(
                (((l16!(ad!(DAT_0052b15b), iVar5) as i16 as i32) / 2) as u32)
                    .wrapping_mul(sx(l16!(ad!(DAT_0052aae0), iVar18))),
            ) as i16;
            let t: i32 =
                ((sxi(sVar21).wrapping_add((((sVar21 as i32) >> 31) as u32) & 3)) as i32) >> 2;
            uStack_20 = (uStack_20 as u32).wrapping_add(
                (t as i16 as i32 as u32).wrapping_mul(sx(l16!(ad!(DAT_0052a948), iVar18))),
            ) as u16;
        }
        sVar21 = l16!(ad!(DAT_0052ac78), (bVar16 as u32) * 2) as i16;
        if sVar21 != 1 {
            sVar31 = idiv(sVar31 as i32, sVar21 as i32) as i16;
            uStack_20 = idiv(uStack_20 as i16 as i32, sVar21 as i32) as i16 as u16;
            sVar23 = idiv(sVar23 as i32, sVar21 as i32) as i16;
        }
        let puVar36: u32 =
            ((uStack_1d as u32) << 24) | (((cStack_1e as u8) as u32) << 16) | uStack_20 as u32;
        let puVar22: u32;
        let iVar35: i32;
        if _DAT_007394d0 == 0 {
            s32!(DAT_006d4988, 4 + iVar7, DAT_0073abc4);
            s32!(DAT_006d4988, 8 + iVar7, DAT_0073abc8);
            s32!(DAT_006d4988, 0xc + iVar7, DAT_0073abcc);
            s16!(DAT_006d4988, 0x10 + iVar7, DAT_007394c8);
            s16!(DAT_006d4988, 0x12 + iVar7, DAT_007394ce);
            s16!(DAT_006d4988, 0x14 + iVar7, 0);
            s8!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x90, 1);
            iVar18 = l32!(DAT_006d4988, 0x1f + iVar7) as i32;
            s8!(ip!(iVar18), 0x89, l8!(ip!(iVar18), 0x90));
            puVar22 = puVar36;
            iVar35 = DAT_006d4988.wrapping_add(iVar7 as usize) as usize as i32;
        } else {
            iVar18 = ((l8!(DAT_00732e54, 5) as u32)
                .wrapping_mul(0x23)
                .wrapping_add(DAT_006d4988 as usize as u32)) as i32;
            puVar22 = DAT_00732e54 as usize as u32;
            iVar35 = iVar18;
        }
        FUN_0042a230(iVar18, puVar22, iVar35, sVar31, sVar23, puVar36 as i16);
        if (l16!(ad!(DAT_0052b157), iVar5) as i16) != 0 {
            bVar16 = l16!(ad!(DAT_0052b157), iVar5) as u8;
            DAT_0070ab18 = (DAT_0070ab18 as u32).wrapping_shl(bVar16 as u32 & 0x1f) as i32;
            DAT_0070ab14 = (DAT_0070ab14 as u32).wrapping_shl(bVar16 as u32 & 0x1f) as i32;
            DAT_0070ac20 = (DAT_0070ac20 as u32).wrapping_shl(bVar16 as u32 & 0x1f) as i32;
        }
        s32!(
            DAT_006d4988,
            4 + iVar7,
            (DAT_0073abc4 as u32).wrapping_add(DAT_0070ab18 as u32)
        );
        s32!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            6,
            l32!(DAT_006d4988, iVar7 + 4)
        );
        s32!(
            DAT_006d4988,
            8 + iVar7,
            (DAT_0070ab14 as u32).wrapping_add(DAT_0073abc8 as u32)
        );
        s32!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            10,
            l32!(DAT_006d4988, iVar7 + 8)
        );
        s32!(
            DAT_006d4988,
            0xc + iVar7,
            (DAT_0070ac20 as u32).wrapping_add(DAT_0073abcc as u32)
        );
        s32!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0xe,
            l32!(DAT_006d4988, iVar7 + 0xc)
        );
        uVar13 = DAT_0073b374 as u32;
        let puVar9: *mut u8 = if (l8!(ad!(DAT_0070ad40), uVar13.wrapping_mul(0x566)) as i8)
            == (l32!(DAT_00732e54, 0xb0) as i8)
        {
            ad!(DAT_0070ad2c)
        } else {
            ad!(DAT_0070ad18)
        };
        uVar12 = 0;
        loop {
            let uVar17: u16 = uVar12.wrapping_add(1);
            s8!(
                DAT_00732e54,
                0x1fb + uVar12 as u32,
                l8!(
                    puVar9,
                    (uVar12 as u32).wrapping_add(uVar13.wrapping_mul(0x566))
                )
            );
            uVar12 = uVar17;
            if !(uVar17 < 0x10) {
                break;
            }
        }
        s8!(DAT_00732e54, 0x1fa, 0);
        s16!(DAT_00732e54, 0x1dd, 0x3ff);
        s16!(DAT_00732e54, 0x1d9, 0x1fff);
        s16!(DAT_006d4988, 0x10 + iVar7, DAT_007394c8);
        s16!(DAT_00732e54, 0xf, l16!(DAT_006d4988, 0x10 + iVar7));
        s16!(DAT_006d4988, 0x12 + iVar7, DAT_007394ce);
        s16!(DAT_00732e54, 0xd, l16!(DAT_006d4988, 0x12 + iVar7));
        s16!(DAT_006d4988, 0x14 + iVar7, 0);
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x77, 0);
        s8!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x90, 1);
        iVar18 = l32!(DAT_006d4988, 0x1f + iVar7) as i32;
        s8!(ip!(iVar18), 0x89, l8!(ip!(iVar18), 0x90));
        s8!(DAT_00732e54, 0xa4, 0);
        s8!(DAT_00732e54, 0x9c, 0);
        s8!(DAT_00732e54, 0xab, 0);
        s8!(DAT_00732e54, 0x96, 0);
        s8!(DAT_00732e54, 0x97, l8!(DAT_00732e54, 0x96));
        s8!(DAT_00732e54, 0x9d, l8!(DAT_00732e54, 0x97));
        s16!(DAT_00732e54, 0x94, 0xffff);
        s16!(DAT_00732e54, 0xa2, 0xffff);
        s16!(DAT_00732e54, 0x9a, 0xffff);
        s16!(DAT_00732e54, 0xa9, 0xffff);
        s16!(DAT_00732e54, 0xa0, l16!(ad!(DAT_0052b097), iVar5));
        s16!(DAT_00732e54, 0x98, l16!(ad!(DAT_0052b099), iVar5));
        s16!(DAT_00732e54, 0xa7, l16!(ad!(DAT_0052b093), iVar5));
        s16!(DAT_00732e54, 0x92, l16!(ad!(DAT_0052b08d), iVar5));
        s32!(
            puVar2,
            0,
            l16!(ad!(DAT_00524778), (DAT_007394c7 as u32) * 2) as u32
        );
        if DAT_007394c7 < 2 {
            uVar13 = 0;
        } else {
            uVar13 = (DAT_007394c7 as u16).wrapping_sub(2) as u32;
        }
        let mut cVar4: i8 = 0;
        s16!(DAT_00732e54, 9, l16!(ad!(DAT_00524778), uVar13 * 2));
        uStack_20 = 0;
        s8!(DAT_00732e54, 0x211, 0);
        loop {
            uVar28 = uStack_20 as u32;
            s8!(
                DAT_00732e54,
                uVar28 + 0x214,
                l8!(ad!(DAT_0052b0a8), iVar5 + uVar28)
            );
            if bVar8 != 0 {
                s8!(DAT_00732e54, uVar28 + 0x216, 1);
            } else {
                s8!(DAT_00732e54, uVar28 + 0x216, 0);
            }
            s8!(DAT_00732e54, uVar28 + 0x218, 0);
            s8!(DAT_00732e54, uVar28 + 0x21a, 0);
            s16!(DAT_00732e54, uVar28 * 2 + 0x21c, 0);
            s32!(DAT_00732e54, 0x220 + uVar28 * 4, 0);
            if l8!(DAT_00732e54, uVar28 + 0x214) != 0 {
                bVar16 = l8!(ad!(DAT_0052b0ac), iVar5 + uVar28);
                cVar4 = cVar4.wrapping_add(l8!(ad!(DAT_0052b0ae), iVar5 + uVar28) as i8);
                bVar10 = l8!(ad!(DAT_0052b0aa), iVar5 + uVar28);
                if l8!(ad!(DAT_0052b0b0), iVar5 + uVar28) != 2 {
                    s8!(
                        DAT_00732e54,
                        0x211,
                        l8!(DAT_00732e54, 0x211).wrapping_add(1)
                    );
                    s8!(DAT_00732e54, uVar28 + 0x21a, bVar10);
                }
                uVar12 = bVar10 as u16;
                let mut uVar14: u32 = bVar10 as u32;
                while uVar12 <= bVar16 as u16 {
                    if l8!(ad!(DAT_0052b0b0), iVar5 + uVar28) == 2 {
                        s8!(DAT_00732e54, 0x344 + uVar14 * 4, 2);
                    } else {
                        s8!(
                            DAT_00732e54,
                            0x344 + uVar14 * 4,
                            l8!(DAT_00732e54, uVar28 + 0x214)
                        );
                    }
                    uVar12 = (uVar14 as u16).wrapping_add(1);
                    let o = uVar14 * 4;
                    s8!(DAT_00732e54, o + 0x345, 0x7f);
                    s8!(DAT_00732e54, o + 0x346, 0);
                    s16!(DAT_00732e54, o + 0x386, 0xffff);
                    uVar14 = uVar12 as u32;
                }
            }
            uStack_20 = uStack_20.wrapping_add(1);
            if !(uStack_20 < 2) {
                break;
            }
        }
        s8!(DAT_00732e54, 0x212, 2);
        s8!(DAT_00732e54, 0x213, cVar4);
        if l8!(DAT_00732e54, 0x211) == 0 {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 0x10);
        }
        uStack_20 = 0;
        s8!(DAT_00732e54, 0x228, 0);
        loop {
            if uStack_20 == 0 || {
                uVar28 = FUN_004a7aa0(0xc);
                (uVar28 as u16) == uVar20
            } {
                uVar28 = uStack_20 as u32;
                s8!(
                    DAT_00732e54,
                    uVar28 + 0x229,
                    l8!(ad!(DAT_00524870), l8!(ad!(DAT_0070ad45), r566!()))
                );
            } else {
                uVar28 = uStack_20 as u32;
                s8!(DAT_00732e54, uVar28 + 0x229, 0);
            }
            let mut uVar14: u32;
            if uStack_20 == 1
                && {
                    uVar14 = FUN_004a7aa0(0xc);
                    (uVar14 as u16) == uVar20
                }
                && DAT_007092c1 == 0
            {
                s8!(DAT_00732e54, uVar28 + 0x229, 0x95);
            }
            s8!(DAT_00732e54, uVar28 + 0x22b, 1);
            s16!(DAT_00732e54, uVar28 * 2 + 0x22d, 0);
            if l8!(DAT_00732e54, uVar28 + 0x229) != 0 {
                s8!(
                    DAT_00732e54,
                    0x228,
                    (l32!(DAT_00732e54, 0x228) as u8).wrapping_add(1)
                );
                bVar16 = l8!(ad!(DAT_0052b0b6), iVar5 + uVar28);
                let mut local_24: u16 = l8!(ad!(DAT_0052b0b4), iVar5 + uVar28) as u16;
                if local_24 <= bVar16 as u16 {
                    loop {
                        uVar14 = local_24 as u32;
                        let o = uVar14 * 4;
                        s8!(DAT_00732e54, 0x344 + o, l8!(DAT_00732e54, uVar28 + 0x229));
                        s16!(DAT_00732e54, o + 0x386, 0xffff);
                        s8!(DAT_00732e54, o + 0x345, 0x7f);
                        let uVar19: u8 = l8!(ad!(DAT_0052b0ba), iVar5 + uVar28);
                        uVar14 = DAT_0073b374 as u32;
                        let mut uVar25: u32 =
                            l8!(ad!(DAT_0070ad45), uVar14.wrapping_mul(0x566)) as u32;
                        if uStack_20 == 1 && {
                            uVar14 = FUN_004a7aa0(0xc);
                            (uVar14 as u16) == uVar20
                        } {
                            uVar25 = 5;
                        }
                        uVar14 = FUN_00478320(
                            (uVar14 & 0xffff0000) | uVar19 as u32,
                            l16!(ad!(DAT_00524880), uVar25 * 2),
                        );
                        bVar10 = uVar14 as u8;
                        if bVar10 == 0 {
                            bVar10 = 1;
                        }
                        if DAT_007394cb == 1 || DAT_0073abc1 == 1 {
                            bVar10 = bVar10.wrapping_mul(2);
                        } else if DAT_007394cb == 2 || DAT_0073abc1 == 2 {
                            bVar10 = bVar10 >> 1;
                        }
                        if bVar10 == 0 {
                            bVar10 = 1;
                        }
                        if iVar15 != -1 && 9 < bVar10 && l8!(DAT_006d4988, 3 + iVar7) != 0xc {
                            bVar10 = 9;
                        }
                        local_24 = local_24.wrapping_add(1);
                        s8!(DAT_00732e54, o + 0x346, bVar10);
                        if !(local_24 <= bVar16 as u16) {
                            break;
                        }
                    }
                }
            }
            uStack_20 = uStack_20.wrapping_add(1);
            if !(uStack_20 < 2) {
                break;
            }
        }
        s16!(DAT_00732e54, 0x231, 0);
        if (l32!(DAT_00732e54, 0x228) as i8) == 0 {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 8);
        }
        s16!(DAT_00732e54, 0x244, 0);
        s16!(DAT_00732e54, 0x242, l32!(DAT_00732e54, 0x244) as i16);
        s16!(DAT_00732e54, 0x248, 0);
        s16!(DAT_00732e54, 0x246, l32!(DAT_00732e54, 0x248) as i16);
        s8!(DAT_00732e54, 0x24b, 0);
        s8!(DAT_00732e54, 0x24a, l8!(DAT_00732e54, 0x24b));
        s16!(DAT_00732e54, 0x291, 0);
        uVar20 = DAT_0073b374;
        if bVar8 != 0 {
            let iVar15: u32 = (local_14 as u32).wrapping_mul(0x5bd);
            s32!(ad!(DAT_0071b734), iVar15, l16!(DAT_006d4988, iVar7) as u32);
            s16!(ad!(DAT_0071b73e), iVar15, uVar20);
            s16!(ad!(DAT_0071b7b7), iVar15, 0);
            s16!(ad!(DAT_0071b7b5), iVar15, 0);
            s16!(ad!(DAT_0071b7bb), iVar15, 0);
            s16!(ad!(DAT_0071b7b9), iVar15, 0);
            s16!(ad!(DAT_0071b7bf), iVar15, 0);
            s16!(ad!(DAT_0071b7bd), iVar15, 0);
            s32!(ad!(DAT_0071b79d), iVar15, 0);
            s32!(ad!(DAT_0071b7a1), iVar15, 0);
            s32!(ad!(DAT_0071b7a5), iVar15, 0);
            s32!(ad!(DAT_0071b7a9), iVar15, 0);
            s32!(ad!(DAT_0071b7ad), iVar15, 0);
            s32!(ad!(DAT_0071b7b1), iVar15, 0);
            s16!(ad!(DAT_0071b9bf), iVar15, 0);
            s16!(ad!(DAT_0071b7c1), iVar15, 0);
            s16!(ad!(DAT_0071b7c3), iVar15, 0);
            uVar20 = 0;
            loop {
                uVar12 = uVar20.wrapping_add(1);
                s16!(ad!(DAT_0071b7c5), (uVar20 as u32) * 2 + iVar15, 0);
                let o = iVar15.wrapping_add((uVar20 as u32) * 2);
                s16!(ad!(DAT_0071b825), o, 0);
                s16!(ad!(DAT_0071b885), o, 0);
                s16!(ad!(DAT_0071b9e9), o, 0);
                s16!(ad!(DAT_0071ba49), o, 0);
                uVar20 = uVar12;
                if !(uVar12 < 0x30) {
                    break;
                }
            }
            uVar20 = 0;
            loop {
                uVar12 = uVar20.wrapping_add(1);
                s16!(ad!(DAT_0071b8e5), (uVar20 as u32) * 2 + iVar15, 0);
                let o = iVar15.wrapping_add((uVar20 as u32) * 2);
                s16!(ad!(DAT_0071b917), o, 0);
                s16!(ad!(DAT_0071b949), o, 0);
                s16!(ad!(DAT_0071baa9), o, 0);
                uVar20 = uVar12;
                if !(uVar12 < 0x19) {
                    break;
                }
            }
            uVar20 = 0;
            loop {
                uVar12 = uVar20.wrapping_add(1);
                s16!(ad!(DAT_0071b97b), (uVar20 as u32) * 2 + iVar15, 0);
                let o = iVar15.wrapping_add((uVar20 as u32) * 2);
                s16!(ad!(DAT_0071b987), o, 0);
                s16!(ad!(DAT_0071b993), o, 0);
                s16!(ad!(DAT_0071badb), o, 0);
                uVar20 = uVar12;
                if !(uVar12 < 6) {
                    break;
                }
            }
            uVar20 = 0;
            loop {
                uVar12 = uVar20.wrapping_add(1);
                s16!(ad!(DAT_0071b99f), (uVar20 as u32) * 2 + iVar15, 0);
                let o = iVar15.wrapping_add((uVar20 as u32) * 2);
                s16!(ad!(DAT_0071b9af), o, 0);
                s16!(ad!(DAT_0071b9c9), o, 0);
                s16!(ad!(DAT_0071b9d9), o, 0);
                uVar20 = uVar12;
                if !(uVar12 < 8) {
                    break;
                }
            }
            s16!(ad!(DAT_0071b9c1), iVar15, 0);
            s16!(ad!(DAT_0071b9c3), iVar15, 0);
            s16!(ad!(DAT_0071b9c5), iVar15, 0);
            s16!(ad!(DAT_0071b9c7), iVar15, 0);
        }
        s16!(DAT_00732e54, 199, l16!(ad!(DAT_0052b087), iVar5));
        s16!(DAT_00732e54, 0xc5, l16!(ad!(DAT_0052b089), iVar5));
        s16!(DAT_00732e54, 0xc3, 0);
        s16!(DAT_00732e54, 0xcb, 0);
        s16!(DAT_00732e54, 0xc9, 0);
        s32!(DAT_00732e54, 0xcd, 0);
        s32!(DAT_00732e54, 0xd1, 0);
        s32!(DAT_00732e54, 0xd5, 0);
        s32!(DAT_00732e54, 0xd9, 0);
        s32!(DAT_00732e54, 0xdd, 0);
        iVar15 = 0;
        loop {
            iVar18 = iVar15 + 4;
            s32!(DAT_00732e54, iVar15 + 0x1a1, 0);
            iVar15 = iVar18;
            if !(iVar18 < 0x20) {
                break;
            }
        }
        iVar15 = 0;
        loop {
            iVar18 = iVar15 + 1;
            s32!(DAT_00732e54, iVar18 * 4 + 0xdd, 0);
            s8!(DAT_00732e54, iVar15 + 0x1e6, 0);
            iVar15 = iVar18;
            if !(iVar18 < 10) {
                break;
            }
        }
        iVar15 = 0;
        loop {
            iVar18 = iVar15 + 4;
            s32!(DAT_00732e54, iVar15 + 0x1c1, 0);
            iVar15 = iVar18;
            if !(iVar18 < 0x18) {
                break;
            }
        }
        s16!(DAT_00732e54, 0x1e1, 0);
        s8!(DAT_00732e54, 8, 0);
        s8!(DAT_00732e54, 0x1e3, 0);
        s8!(DAT_00732e54, 0x1e4, 0);
        s8!(DAT_00732e54, 0x1e5, 0);
        s8!(DAT_00732e54, 0x29, 0);
        uVar20 = 0;
        loop {
            uVar12 = uVar20.wrapping_add(1);
            s32!(DAT_00732e54, (uVar20 as u32) * 4 + 0x15, 0);
            uVar20 = uVar12;
            if !(uVar12 < 5) {
                break;
            }
        }
        uVar24 = 0;
        s8!(DAT_00732e54, 0x237, 0);
        s16!(DAT_00732e54, 0x238, 0);
        s16!(DAT_00732e54, 0x23a, 0xffff);
        loop {
            if (l8!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x87) as u32) == uVar24 {
                s8!(DAT_00732e54, uVar24 + 0x1f0, 1);
            } else {
                s8!(DAT_00732e54, uVar24 + 0x1f0, 0);
            }
            uVar24 = uVar24.wrapping_add(1);
            if !((uVar24 as i32) < 10) {
                break;
            }
        }
        if l8!(ad!(DAT_0052b081), iVar5) == 0
            && DAT_007394cb != 9
            && DAT_0073abc1 != 9
            && DAT_007394cb != 0x10
            && DAT_0073abc1 != 0x10
        {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 0x80);
        }
        if DAT_007394cb == 6 || DAT_0073abc1 == 6 {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 0x80);
        }
        s16!(DAT_00732e54, 0x20b, l16!(ad!(DAT_0052b083), iVar5));
        if bVar8 != 0 {
            s16!(DAT_00732e54, 0x20d, l16!(ad!(DAT_0052b083), iVar5));
            s8!(DAT_00732e54, 0x210, 1);
        } else {
            s16!(
                DAT_00732e54,
                0x20b,
                (l16!(DAT_00732e54, 0x20b) as i16)
                    .wrapping_add(l16!(ad!(DAT_0052b083), iVar5) as i16)
            );
            s16!(DAT_00732e54, 0x20d, 0);
            s8!(DAT_00732e54, 0x210, 0);
        }
        let mut do_lab = false;
        if DAT_007394cb == 3 || DAT_0073abc1 == 3 {
            s16!(DAT_00732e54, 0x20b, 0);
            s16!(DAT_00732e54, 0x20d, 0);
            do_lab = true;
        } else if DAT_007394cb == 4 || DAT_0073abc1 == 4 {
            s16!(DAT_00732e54, 0x20b, (l16!(DAT_00732e54, 0x20b) as i16) >> 1);
            s16!(DAT_00732e54, 0x20d, (l16!(DAT_00732e54, 0x20d) as i16) >> 1);
            do_lab = true;
        } else if DAT_007394cb == 7 || DAT_0073abc1 == 7 {
            s16!(DAT_00732e54, 0x20b, 0);
            s16!(DAT_00732e54, 0x20d, 0);
        } else if DAT_007394cb == 0x13
            || DAT_0073abc1 == 0x13
            || DAT_007394cb == 0xd
            || DAT_0073abc1 == 0xd
        {
            s16!(DAT_00732e54, 0x20b, (l16!(DAT_00732e54, 0x20b) as i16) >> 1);
            s16!(DAT_00732e54, 0x20d, (l16!(DAT_00732e54, 0x20d) as i16) >> 1);
        } else if DAT_007394cb == 0x12
            || DAT_0073abc1 == 0x12
            || DAT_007394cb == 0xc
            || DAT_0073abc1 == 0xc
        {
            s16!(
                DAT_00732e54,
                0x20b,
                (l16!(DAT_00732e54, 0x20b) as i16 as i32 as u32).wrapping_shl(1)
            );
            s16!(
                DAT_00732e54,
                0x20d,
                (l16!(DAT_00732e54, 0x20d) as i16 as i32 as u32).wrapping_shl(1)
            );
            if (l16!(DAT_00732e54, 0x20b) as i16) < 0 {
                s16!(DAT_00732e54, 0x20b, 0x7fbc);
            }
            if (l16!(DAT_00732e54, 0x20d) as i16) < 0 {
                s16!(DAT_00732e54, 0x20d, 0x7fbc);
            }
        }
        if do_lab {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 1);
        }
        s8!(DAT_00732e54, 0x20f, 2);
        if l8!(ad!(DAT_0052b082), iVar5) == 0
            && DAT_007394cb != 8
            && DAT_0073abc1 != 8
            && DAT_007394cb != 0x10
            && DAT_0073abc1 != 0x10
        {
            s8!(DAT_00732e54, 0x1dd, l8!(DAT_00732e54, 0x1dd) ^ 1);
            s16!(DAT_00732e54, 0x20b, 0);
            s16!(DAT_00732e54, 0x20d, 0);
            s8!(DAT_00732e54, 0x1da, l8!(DAT_00732e54, 0x1da) ^ 8);
        }
        s8!(DAT_00732e54, 0x233, l8!(ad!(DAT_0070ad46), r566!()));
        if DAT_007394d2 == 1
            || DAT_007394d2 == 2
            || DAT_007394d2 == 4
            || DAT_007394d2 == 3
            || DAT_007394d2 == 0xe
        {
            s8!(DAT_00732e54, 0x233, 0);
        }
        s8!(DAT_00732e54, 0x234, 2);
        s16!(DAT_00732e54, 0x235, 9999);
        if l8!(DAT_00732e54, 0x233) == 0 {
            s16!(DAT_00732e54, 0x235, 0);
            s8!(DAT_00732e54, 0x1de, l8!(DAT_00732e54, 0x1de) ^ 1);
            s8!(DAT_00732e54, 0x1da, l8!(DAT_00732e54, 0x1da) ^ 0x10);
            s8!(DAT_00732e54, 0x1d9, l8!(DAT_00732e54, 0x1d9) ^ 0x10);
        }
        s8!(DAT_00732e54, 0x23c, l8!(ad!(DAT_0070b213), r566!()));
        if (l32!(DAT_00732e54, 0x23c) as i8) == 0 {
            s8!(DAT_00732e54, 0x23d, 0);
        } else {
            s8!(DAT_00732e54, 0x23d, l8!(ad!(DAT_0052b13c), iVar5));
            if (l32!(DAT_00732e54, 0x23c) as i8) == 2 {
                let puVar1: *mut u8 = DAT_00732e54.wrapping_add(0x23d);
                uVar24 = FUN_00478320(
                    ((DAT_00732e54 as usize as u32) & 0xffff0000) | puVar1.read() as u32,
                    0xaaac,
                );
                puVar1.write(uVar24 as u8);
            }
        }
        s16!(DAT_00732e54, 0x23e, 0);
        s16!(DAT_00732e54, 0x240, 0);
        s16!(DAT_00732e54, 0x1df, l16!(DAT_00732e54, 0x1dd));
        s16!(DAT_00732e54, 0x1db, l16!(DAT_00732e54, 0x1d9));
        uVar20 = 0;
        loop {
            uVar12 = uVar20.wrapping_add(1);
            s8!(DAT_006d4988, uVar20 as u32 + 0x19 + iVar7, 0);
            uVar20 = uVar12;
            if !(uVar12 < 2) {
                break;
            }
        }
        uVar20 = 0;
        loop {
            uVar12 = uVar20.wrapping_add(1);
            uVar24 = uVar20 as u32;
            s8!(DAT_00732e54, uVar24 + 0x2ca, 0);
            s8!(DAT_00732e54, uVar24 + 0x2f2, 0);
            s8!(DAT_00732e54, uVar24 + 0x31a, 0xff);
            uVar20 = uVar12;
            if !(uVar12 < 0x28) {
                break;
            }
        }
        uVar24 = 0;
        iVar15 = FUN_004490f0(uVar27);
        if 0 < iVar15 {
            loop {
                iVar18 = FUN_00449790(uVar27, uVar24);
                uVar28 = FUN_0044a600(uVar27, uVar24);
                if uVar28 != 0 {
                    s8!(DAT_00732e54, uVar24 + 0x31a, l8!(ad!(DAT_00524898), iVar18));
                }
                if (DAT_007394cb == 5
                    || DAT_0073abc1 == 5
                    || DAT_007394cb == 0xe
                    || DAT_0073abc1 == 0xe)
                    && (iVar18 == 4 || iVar18 == 0x15 || iVar18 == 5)
                {
                    s8!(DAT_00732e54, uVar24 + 0x31a, 0);
                    s8!(DAT_00732e54, uVar24 + 0x2ca, 4);
                }
                uVar24 = (uVar24 as u16).wrapping_add(1) as u32;
                if !((uVar24 as i32) < iVar15) {
                    break;
                }
            }
        }
        if DAT_0073b370 == 5
            && 0x3b < DAT_007394d2
            && DAT_007394d2 < 0x41
            && l8!(ad!(DAT_0070ad46), r566!()) != 0
        {
            uVar20 = ((DAT_007394d2 as u32) * 0xc).wrapping_sub(0x2d0) as u16;
            uVar27 = uVar20 as u32;
            let c: u32 = (l8!(ad!(DAT_0070ad46), r566!()) == 1) as u32;
            uVar24 = ((((0u32.wrapping_sub(c)) & 0xfffa).wrapping_add(0xc)) as u16 as u32)
                .wrapping_add(uVar27);
            while uVar27 < uVar24 {
                bVar16 = l8!(ad!(DAT_005248b8), uVar20);
                if bVar16 != 0xff {
                    s8!(DAT_00732e54, bVar16 as u32 + 0x31a, 0);
                    s8!(DAT_00732e54, bVar16 as u32 + 0x2ca, 4);
                }
                uVar20 = uVar20.wrapping_add(1);
                uVar27 = uVar20 as u32;
            }
        }
        bVar16 = l8!(ad!(DAT_0070ad92), r566!());
        iVar15 = FUN_00481380(ad!(DAT_00524948).cast::<i8>());
        s8!(DAT_00732e54, 0x34, iVar15);
        s8!(DAT_00732e54, 0x35, l8!(DAT_00732e54, 0x34));
        bVar10 = l8!(ad!(DAT_006da3e0), l8!(ad!(DAT_00524798), bVar16));
        s8!(puVar2, 0xf, bVar10);
        let mut uVar19: u8 = DAT_006da414;
        let mut cond = false;
        if DAT_007394ca == 0 {
            uVar19 = DAT_006da412;
            if DAT_007394cc == 0 {
                uVar19 = l8!(ad!(DAT_006da3e0), l8!(ad!(DAT_005247c0), bVar16));
                if (l8!(DAT_00732e54, 5) as i8) == -1 {
                    cond = true;
                }
            }
        }
        if cond {
            s8!(puVar2, 0xe, bVar10);
        } else {
            s8!(puVar2, 0xe, uVar19);
        }
        let mut bVar34: bool;
        let pcVar30: *const i8 = ad!(DAT_00703ca0)
            .wrapping_add((bVar10 as usize) * 0x55)
            .cast::<i8>();
        let r = cmps(8, pcVar30, ad!(DAT_00524948).cast::<i8>(), bVar10 == 0);
        uVar24 = r.0;
        bVar34 = r.1;
        let mut at_003 = false;
        if bVar34 {
            at_003 = true;
        } else {
            let r = cmps(0x11, pcVar30, ad!(DAT_00524934).cast::<i8>(), false);
            uVar24 = r.0;
            bVar34 = r.1;
            if bVar34 {
                at_003 = true;
            } else {
                let r = cmps(0x11, pcVar30, ad!(DAT_00524920).cast::<i8>(), false);
                uVar24 = r.0;
                bVar34 = r.1;
                if bVar34 {
                    at_003 = true;
                } else {
                    let r = cmps(0xc, pcVar30, ad!(DAT_00524914).cast::<i8>(), false);
                    uVar24 = r.0;
                    bVar34 = r.1;
                    if bVar34 {
                        at_003 = true;
                    }
                }
            }
        }
        let mut skip = false;
        if at_003 && bVar8 == 0 {
            uVar24 = uVar24 & 0xffff0000;
            skip = true;
        }
        if !skip {
            let r = cmps(0xe, pcVar30, ad!(DAT_00524904).cast::<i8>(), false);
            iVar15 = r.0 as i32;
            bVar34 = r.1;
            uVar24 = ((iVar15 as u32) & 0xffff0000) | 0x8000;
            if !bVar34 {
                uVar24 = ((DAT_0073b374 as u32).wrapping_mul(0x2b3) & 0xffff0000)
                    | l16!(
                        ad!(DAT_0052a650),
                        (l8!(ad!(DAT_0070ad93), r566!()) as u32) * 2
                    ) as u32;
            }
        }
        s8!(DAT_00732e54, 7, DAT_0073abc0);
        uVar19 = l8!(ad!(DAT_0052b080), iVar5);
        s16!(DAT_00732e54, 0xbd, uVar24 as u16);
        s16!(DAT_00732e54, 0xbf, 0xffff);
        uVar24 = FUN_00478320(
            (uVar24 & 0xffff0000) | l16!(ad!(DAT_0052b08d), iVar5) as u32,
            uVar24 as u16,
        );
        s16!(
            sp!(DAT_006d4988.wrapping_add(iVar7 as usize)),
            0x79,
            uVar24 as i16
        );
        s16!(sp!(DAT_006d4988.wrapping_add(iVar7 as usize)), 0x7b, 0);
        if bVar8 != 0 {
            let b: u32 = (local_14 as u32).wrapping_mul(0x5bd);
            s8!(ad!(DAT_0071b757), b, 0);
            s8!(ad!(DAT_0071b758), b, 0);
            local_14 = b as i32;
            let l = local_14 as u32;
            s8!(ad!(DAT_0071b742), l, uVar19);
            s16!(ad!(DAT_0071b761), l, 0x5555);
            s8!(ad!(DAT_0071b765), l, 2);
            s8!(ad!(DAT_0071b767), l, 2);
            s8!(ad!(DAT_0071b769), l, 2);
            s16!(ad!(DAT_0071b763), l, 0xffff);
            s8!(ad!(DAT_0071b766), l, 3);
            s8!(ad!(DAT_0071b768), l, 2);
            s8!(ad!(DAT_0071b76a), l, 2);
        }
        uVar20 = 0;
        loop {
            uVar12 = uVar20.wrapping_add(1);
            s8!(puVar2, uVar20 as u32 + 5, 0);
            s8!(puVar2, uVar20 as u32 + 9, 0);
            uVar20 = uVar12;
            if !(uVar12 < 4) {
                break;
            }
        }
        s8!(puVar2, 4, 0);
        s8!(puVar2, 0xd, 0);
        s16!(puVar2, 0x1c, 0xffff);
        s16!(puVar2, 0x2f, 0xffff);
        s16!(puVar2, 0x1e, 0);
        s8!(puVar2, 0x22, 0);
        s16!(puVar2, 0x20, 0xffff);
        s8!(puVar2, 0x31, 0xff);
        s32!(puVar2, 0x23, 0);
        s32!(puVar2, 0x27, 0);
        s32!(puVar2, 0x2b, 0);
        s16!(puVar2, 0x32, 0x4000);
        s16!(puVar2, 0x34, 0);
        s16!(puVar2, 0x36, 0);
        s8!(puVar2, 0x10, 4);
        s8!(puVar2, 0x11, 0);
        s32!(
            puVar2,
            0x12,
            l16!(ad!(DAT_00524788), (DAT_007394c7 as u32) * 2) as u32
        );
        uVar20 = FUN_00478470();
        s16!(puVar2, 0x1a, uVar20 ^ 0xbeef);
        s8!(puVar2, 0x38, 0);
        s8!(puVar2, 0x39, 0);
        s32!(puVar2, 0x3a, 0);
        uVar20 = 0;
        loop {
            uVar12 = uVar20.wrapping_add(1);
            s8!(DAT_00732e54, uVar20 as u32 + 0x2b, 0);
            s8!(DAT_00732e54, uVar20 as u32 + 0x2f, 0);
            uVar20 = uVar12;
            if !(uVar12 < 4) {
                break;
            }
        }
        s8!(DAT_00732e54, 0x2a, 0);
        s8!(DAT_00732e54, 0x33, 0);
        s16!(DAT_00732e54, 0x42, 0xffff);
        s16!(DAT_00732e54, 0x55, l16!(DAT_00732e54, 0x42));
        s16!(DAT_00732e54, 0x44, 0);
        s8!(DAT_00732e54, 0x48, 0);
        s16!(DAT_00732e54, 0x46, 0xffff);
        s16!(DAT_00732e54, 0x66, l16!(DAT_00732e54, 0x46));
        s16!(DAT_00732e54, 0x68, l16!(DAT_00732e54, 0x66));
        s16!(DAT_00732e54, 0x70, 0xffff);
        s8!(DAT_00732e54, 0x57, 0xff);
        s8!(DAT_00732e54, 0x72, 0);
        s8!(DAT_00732e54, 0x73, 0);
        s8!(DAT_00732e54, 0x74, 0);
        s8!(DAT_00732e54, 0x75, 0);
        s8!(DAT_00732e54, 0x76, 0);
        s8!(DAT_00732e54, 0x77, 0);
        s8!(DAT_00732e54, 0x79, 0);
        s16!(DAT_00732e54, 0xc1, 0);
        s8!(DAT_00732e54, 0x7a, 0);
        s8!(DAT_00732e54, 0x7b, 0);
        s8!(DAT_00732e54, 0x7c, 0);
        s8!(DAT_00732e54, 0x7d, 0);
        s32!(DAT_00732e54, 0x49, 0);
        uVar24 = 0;
        loop {
            s8!(DAT_00732e54, uVar24 + 0x293, uVar24 as u8);
            uVar20 = (uVar24 as u16).wrapping_add(1);
            s16!(DAT_00732e54, uVar24 * 2 + 0x29e, 100);
            s16!(DAT_00732e54, uVar24 * 2 + 0x2b4, 0);
            uVar24 = uVar20 as u32;
            if !(uVar20 < 10) {
                break;
            }
        }
        s8!(DAT_00732e54, 0x36, 4);
        s32!(
            DAT_00732e54,
            0x38,
            l16!(ad!(DAT_00524788), uVar13 * 2) as u32
        );
        uVar20 = FUN_00478470();
        s16!(DAT_00732e54, 0x40, uVar20 ^ 0xbeef);
        FUN_004296b0(uVar26);
        FUN_00429310(uVar6);
        FUN_004213b0(DAT_00732e54 as usize as i32);
        s16!(DAT_00732e54, 0x342, 0xffff);
        let k: u32 = (DAT_0073b374 as u32).wrapping_mul(0x93).wrapping_mul(2);
        st16(
            ad!(DAT_006d4a68),
            k,
            ld16(ad!(DAT_006d4a68), k).wrapping_add(1),
        );
        uVar26
    }
}
