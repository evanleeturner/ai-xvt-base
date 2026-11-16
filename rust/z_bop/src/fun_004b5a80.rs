// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_00aa6cfc: i32;
    static mut DAT_00aa7b13: u8;
}

#[inline(always)]
unsafe fn rd_tmp() -> i32 {
    // SAFETY: the game's own global, read as a packed 32-bit value.
    unsafe { (&raw mut DAT_00aa7b13).cast::<i32>().read_unaligned() }
}

#[inline(always)]
unsafe fn fill(base: i32, idx: i32, mut n: u32, v: i32) {
    // SAFETY: caller guarantees the destination is the game's surface memory.
    unsafe {
        let mut p = base.wrapping_add(idx.wrapping_mul(2)) as usize as *mut u16;
        while n != 0 {
            p.write_unaligned(v as u16);
            p = p.byte_add(2);
            n -= 1;
        }
    }
}

#[inline(always)]
unsafe fn put(base: i32, idx: i32, v: i32) {
    // SAFETY: caller guarantees the destination is the game's surface memory.
    unsafe {
        (base.wrapping_add(idx.wrapping_mul(2)) as usize as *mut u16).write_unaligned(v as u16);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_004b5a80(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    mut param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: all pointers and globals are the game's own memory, accessed
    // exactly where the original does.
    unsafe {
        if param_1.read() == param_6 {
            let mut pbVar8 = param_1.add(4).read() as usize as *const u8;
            let mut local_c: i32 = rd_tmp()
                .wrapping_mul(param_3)
                .wrapping_add(param_2.wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc);
            if 0 < param_5 {
                loop {
                    let off = (pbVar8 as *const i32).read_unaligned();
                    pbVar8 = pbVar8.offset(off as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_40 = param_7;
                loop {
                    let mut iVar5: i32 = 0;
                    let mut pbVar9 = pbVar8.add(4);
                    loop {
                        let mut bVar2 = pbVar9.read();
                        pbVar8 = pbVar9.add(1);
                        if bVar2 == 0x80 {
                            break;
                        }
                        if bVar2 & 0x80 == 0 {
                            if bVar2 & 0x40 == 0 {
                                pbVar9 = pbVar9.add(2);
                                let uVar3 = bVar2 as u32;
                                let iVar11 = param_1.add(pbVar8.read() as usize + 5).read();
                                fill(local_c, iVar5, uVar3, iVar11);
                                iVar5 = iVar5.wrapping_add(uVar3 as i32);
                            } else {
                                iVar5 = iVar5.wrapping_add((bVar2 & 0x3f) as i32);
                                pbVar9 = pbVar8;
                            }
                        } else {
                            bVar2 &= 0x7f;
                            let mut iVar11: i32 = 0;
                            if bVar2 != 0 {
                                loop {
                                    let iVar10 = iVar11.wrapping_add(iVar5);
                                    let p = pbVar8.offset(iVar11 as isize);
                                    iVar11 += 1;
                                    put(local_c, iVar10, param_1.add(p.read() as usize + 5).read());
                                    if iVar11 >= bVar2 as i32 {
                                        break;
                                    }
                                }
                            }
                            iVar5 = iVar5.wrapping_add(bVar2 as i32);
                            pbVar9 = pbVar8.add(bVar2 as usize);
                        }
                    }
                    local_c = local_c.wrapping_add(rd_tmp() & 0xfffffffe_u32 as i32);
                    local_40 -= 1;
                    if local_40 == 0 {
                        break;
                    }
                }
                return;
            }
        } else {
            let mut local_10 = param_1.add(4).read() as usize as *const u8;
            let mut local_c: i32 = rd_tmp()
                .wrapping_mul(param_3)
                .wrapping_add(param_2.wrapping_mul(2))
                .wrapping_add(DAT_00aa6cfc);
            if 0 < param_5 {
                loop {
                    let off = (local_10 as *const i32).read_unaligned();
                    local_10 = local_10.offset(off as isize);
                    param_5 -= 1;
                    if param_5 == 0 {
                        break;
                    }
                }
            }
            if 0 < param_7 {
                let mut local_20 = param_7;
                loop {
                    let mut bVar1: u8 = 0;
                    let iVar11 = (local_10 as *const i32).read_unaligned();
                    let mut iVar10: i32 = 4;
                    let mut iVar5: i32 = 0;
                    let mut iVar6: i32;
                    let mut iVar7: i32;
                    let mut bVar2: u8;
                    let mut uVar4: u32 = 0;
                    'row: {
                        loop {
                            loop {
                                loop {
                                    loop {
                                        iVar6 = iVar10.wrapping_add(1);
                                        bVar2 = local_10.offset(iVar10 as isize).read();
                                        uVar4 = bVar2 as u32;
                                        iVar7 = iVar5;
                                        if bVar1 == 0 {
                                            if bVar2 == 0x80 {
                                                break 'row;
                                            }
                                            if bVar2 & 0x80 == 0 {
                                                if bVar2 & 0x40 == 0 {
                                                    iVar7 = (bVar2 as i32).wrapping_add(iVar5);
                                                    if iVar7 < param_4 {
                                                        iVar6 = iVar10.wrapping_add(2);
                                                    } else {
                                                        bVar1 = 1;
                                                        iVar7 = 0;
                                                        bVar2 = bVar2.wrapping_sub(
                                                            (param_4 as i8)
                                                                .wrapping_sub(iVar5 as i8)
                                                                as u8,
                                                        );
                                                        uVar4 = bVar2 as u32;
                                                        if bVar2 == 0 {
                                                            iVar6 = iVar10.wrapping_add(2);
                                                        }
                                                    }
                                                } else {
                                                    uVar4 &= 0xffffff3f;
                                                    iVar7 =
                                                        ((bVar2 & 0x3f) as i32).wrapping_add(iVar5);
                                                    if param_4 <= iVar7 {
                                                        bVar1 = 1;
                                                        iVar7 = 0;
                                                        bVar2 = (uVar4 as i8).wrapping_sub(
                                                            (param_4 as i8)
                                                                .wrapping_sub(iVar5 as i8),
                                                        )
                                                            as u8;
                                                        uVar4 = bVar2 as u32;
                                                        if bVar2 != 0 {
                                                            uVar4 = (bVar2 | 0x40) as u32;
                                                        }
                                                    }
                                                }
                                            } else {
                                                uVar4 &= 0xffffff7f;
                                                iVar7 = ((bVar2 & 0x7f) as i32).wrapping_add(iVar5);
                                                if iVar7 < param_4 {
                                                    iVar6 =
                                                        iVar6.wrapping_add((bVar2 & 0x7f) as i32);
                                                } else {
                                                    bVar1 = 1;
                                                    iVar7 = 0;
                                                    iVar6 = iVar6
                                                        .wrapping_add(param_4.wrapping_sub(iVar5));
                                                    bVar2 = (uVar4 as i8).wrapping_sub(
                                                        param_4.wrapping_sub(iVar5) as i8,
                                                    )
                                                        as u8;
                                                    uVar4 = bVar2 as u32;
                                                    if bVar2 != 0 {
                                                        uVar4 = (bVar2 | 0x80) as u32;
                                                    }
                                                }
                                            }
                                        }
                                        iVar10 = iVar6;
                                        iVar5 = iVar7;
                                        if !(bVar1 == 0 || (uVar4 as i8) == 0) {
                                            break;
                                        }
                                    }
                                    if (uVar4 as i8) == -128 {
                                        break 'row;
                                    }
                                    if uVar4 & 0x80 == 0 {
                                        break;
                                    }
                                    let uVar3 = uVar4 & 0x7f;
                                    if param_6 <= (uVar3.wrapping_add(iVar7 as u32)) as i32 {
                                        bVar2 = (param_6 as i8).wrapping_sub(iVar7 as i8) as u8;
                                        if bVar2 != 0 {
                                            iVar5 = 0;
                                            loop {
                                                iVar10 = iVar5 + 1;
                                                let b = local_10
                                                    .offset(iVar5.wrapping_add(iVar6) as isize)
                                                    .read();
                                                put(
                                                    local_c,
                                                    iVar5.wrapping_add(iVar7),
                                                    param_1.add(b as usize + 5).read(),
                                                );
                                                iVar5 = iVar10;
                                                if iVar10 >= bVar2 as i32 {
                                                    break;
                                                }
                                            }
                                        }
                                        break 'row;
                                    }
                                    iVar5 = 0;
                                    if uVar4 & 0x7f != 0 {
                                        loop {
                                            iVar10 = iVar5 + 1;
                                            let b = local_10
                                                .offset(iVar5.wrapping_add(iVar6) as isize)
                                                .read();
                                            put(
                                                local_c,
                                                iVar5.wrapping_add(iVar7),
                                                param_1.add(b as usize + 5).read(),
                                            );
                                            iVar5 = iVar10;
                                            if iVar10 >= uVar3 as i32 {
                                                break;
                                            }
                                        }
                                    }
                                    iVar5 = (iVar7 as u32).wrapping_add(uVar3) as i32;
                                    iVar10 = (iVar6 as u32).wrapping_add(uVar3) as i32;
                                }
                                if uVar4 & 0x40 != 0 {
                                    break;
                                }
                                iVar10 = iVar6.wrapping_add(1);
                                iVar5 = (iVar7 as u32).wrapping_add(uVar4) as i32;
                                if param_6 <= iVar5 {
                                    let n = (param_6 as i8).wrapping_sub(iVar7 as i8) as u8 as u32;
                                    let v = param_1
                                        .add(local_10.offset(iVar6 as isize).read() as usize + 5)
                                        .read();
                                    fill(local_c, iVar7, n, v);
                                    break 'row;
                                }
                                let v = param_1
                                    .add(local_10.offset(iVar6 as isize).read() as usize + 5)
                                    .read();
                                fill(local_c, iVar7, uVar4, v);
                            }
                            iVar5 = (iVar7 as u32).wrapping_add(uVar4 & 0x3f) as i32;
                            if !(iVar5 < param_6) {
                                break;
                            }
                        }
                    }
                    local_10 = local_10.offset(iVar11 as isize);
                    local_c = local_c.wrapping_add(rd_tmp() & 0xfffffffe_u32 as i32);
                    local_20 -= 1;
                    if local_20 == 0 {
                        break;
                    }
                }
            }
        }
    }
}
