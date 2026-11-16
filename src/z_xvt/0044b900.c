/* SPDX-License-Identifier: MIT */
/* z_xvt 0044b900 */
ushort FUN_0049f010(uint, undefined2, ushort);
uint FUN_00429a20(uint);
ushort FUN_00429c40(uint);
ushort FUN_0044c560(ushort, short);
uint FUN_00412810(int, int, int);
void FUN_0042a1a0(uint, uint);
uint FUN_004440e0(ushort);

extern ushort DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern byte DAT_006d9d60;
extern int DAT_00732e54;
extern char DAT_006d9d78;
extern byte DAT_006d9d7a;
extern ushort DAT_006d9d50;
extern byte DAT_006d9d7b[4];
extern byte DAT_006d9d7f[4];
extern byte DAT_006d9d83[4];
extern uint DAT_006d499c;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern byte DAT_0052687a[];

undefined4 __cdecl
FUN_0044b900(undefined2 param_1, ushort param_2, short param_3, undefined2 param_4, ushort param_5)
{
    byte bVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar7;
    int iVar8;
    uint bVar4;
    ushort uVar5;
    int iVar6;
    uint uVar7;
    undefined2 uVar9;
    uint uVar8;
    ushort uVar10;
    ushort uVar11;
    ushort local_a;
    int local_8;
    uint local_4;

    uVar9 = 0;
    local_8 = 0;
    uVar7 = (uint)DAT_00732a3c;
    if ((int)(uint)DAT_00732a3c < DAT_006d4980) {
        do {
            uVar8 = uVar7 & 0xffff;
            iVar6 = (int)((uint)DAT_006d4988 + uVar8 * 0x23);
            if ((*(char *)(iVar6 + 3) != '\0') && (*(byte *)(iVar6 + 0x16) != DAT_006d9d60)) {
                bVar4 = FUN_0049f010(uVar7, param_1, param_2);
                uVar10 = (ushort)bVar4;
                bVar4 = FUN_0049f010(uVar7, param_4, param_5);
                uVar11 = (ushort)bVar4;
                if (param_3 == 1) {
                    uVar10 = uVar10 | uVar11;
                } else {
                    uVar10 = uVar10 & uVar11;
                }
                if ((uVar10 != 0) && (*(short *)(DAT_00732e54 + 0x342) != (short)uVar7)) {
                    bVar4 = FUN_00429a20(uVar8);
                    if (bVar4 != 0) {
                        __builtin_memcpy(&iVar6, (void *)((uint)DAT_006d4988 + 0x1f + uVar8 * 0x23), 4);
                        __builtin_memcpy(&iVar2, (void *)(iVar6 + 0xa7), 4);
                        __builtin_memcpy(&uVar5, (void *)(iVar2 + 0x1df), 2);
                        if ((DAT_006d9d78 == '\0') ||
                            ((uVar5 != 0) &&
                             (((DAT_006d9d78 == '\0') || (*(char *)(iVar2 + 0x1e5) == '\0')) ||
                              (__builtin_memcpy(&uVar8, (void *)((uint)DAT_006d9d50 * 0x23 + 0x1f + (uint)DAT_006d4988), 4),
                               *(char *)(uVar8 + 0x87) != *(char *)(iVar6 + 0x87))))) {
                            if ((DAT_006d9d7a & 4) != 0) {
                                bVar4 = FUN_00429c40(uVar7);
                                if ((short)bVar4 == 0) goto LAB_0044ba45;
                            }
                            local_8 = local_8 + 1;
                        }
                    }
                }
            }
LAB_0044ba45:
            uVar9 = (undefined2)(uVar8 >> 0x10);
            uVar10 = (short)uVar7 + 1;
            uVar7 = (uVar7 & 0xffff0000) | uVar10;
        } while ((int)(uint)uVar10 < DAT_006d4980);
    }
    uVar7 = ((uint)uVar9 << 0x10) | (ushort)DAT_00732a38;
    if ((int)(DAT_00732a38 & 0xffff) < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38)) {
        do {
            bVar1 = *(byte *)((uVar7 & 0xffff) * 0x23 + 3 + (uint)DAT_006d4988);
            if ((bVar1 != 0) && ((DAT_0052687a[(uint)bVar1 * 0x16] & 2) != 0)) {
                bVar4 = FUN_0049f010(uVar7, param_1, param_2);
                uVar10 = (ushort)bVar4;
                bVar4 = FUN_0049f010(uVar7, param_4, param_5);
                uVar11 = (ushort)bVar4;
                if (param_3 == 1) {
                    uVar10 = uVar10 | uVar11;
                } else {
                    uVar10 = uVar10 & uVar11;
                }
                if ((uVar10 != 0) && (*(short *)(DAT_00732e54 + 0x342) != (short)uVar7)) {
                    bVar4 = FUN_00429a20(uVar7 & 0xffff);
                    if (bVar4 != 0) {
                        if ((DAT_006d9d7a & 4) != 0) {
                            bVar4 = FUN_00429c40(uVar7);
                            if ((short)bVar4 == 0) goto LAB_0044bb37;
                        }
                        local_8 = local_8 + 1;
                    }
                }
            }
LAB_0044bb37:
            uVar10 = (short)uVar7 + 1;
            uVar7 = (uVar7 & 0xffff0000) | uVar10;
        } while ((int)(uint)uVar10 < (int)((uint)DAT_00703c88 + (uint)DAT_00732a38));
    }
    if (local_8 == 0) {
        return 0xffff;
    }
    uVar7 = (uVar7 & 0xffff0000) | DAT_00732a3c;
    local_4 = 0xffffffff;
    local_a = 0xffff;
    if ((int)(uint)DAT_00732a3c < DAT_006d4980) {
        do {
            uVar8 = uVar7 & 0xffff;
            iVar6 = (int)(uVar8 * 0x23);
            uVar10 = (ushort)uVar7;
            if ((*(char *)((uint)DAT_006d4988 + iVar6 + 3) != '\0') &&
                (*(byte *)((uint)DAT_006d4988 + iVar6 + 0x16) != DAT_006d9d60)) {
                bVar4 = FUN_0049f010(uVar7, param_1, param_2);
                uVar11 = (ushort)bVar4;
                bVar4 = FUN_0049f010(uVar7, param_4, param_5);
                uVar5 = (ushort)bVar4;
                if (param_3 == 1) {
                    uVar11 = uVar11 | uVar5;
                } else {
                    uVar11 = uVar11 & uVar5;
                }
                if ((uVar11 != 0) && (*(ushort *)(DAT_00732e54 + 0x342) != uVar10)) {
                    bVar4 = FUN_00429a20(uVar8);
                    if (bVar4 != 0) {
                        __builtin_memcpy(&iVar2, (void *)((uint)DAT_006d4988 + 0x1f + iVar6), 4);
                        __builtin_memcpy(&iVar3, (void *)(iVar2 + 0xa7), 4);
                        __builtin_memcpy(&uVar5, (void *)(iVar3 + 0x1df), 2);
                        if ((DAT_006d9d78 == '\0') ||
                            ((uVar5 != 0) &&
                             (((DAT_006d9d78 == '\0') || (*(char *)(iVar3 + 0x1e5) == '\0')) ||
                              (__builtin_memcpy(&iVar4, (void *)((uint)DAT_006d9d50 * 0x23 + 0x1f + (uint)DAT_006d4988), 4),
                               *(char *)(iVar4 + 0x87) != *(char *)(iVar2 + 0x87))))) {
                            if ((DAT_006d9d7a & 4) != 0) {
                                bVar4 = FUN_00429c40(uVar7);
                                if ((short)bVar4 == 0) goto LAB_0044bd5e;
                            }
                            if ((DAT_006d9d7a & 1) != 0) {
                                bVar4 = FUN_0044c560(uVar10, (short)local_8);
                                if ((short)bVar4 == 0) goto LAB_0044bd5e;
                            }
                            if ((DAT_006d9d7a & 0x20) == 0) {
                                FUN_0042a1a0((uint)DAT_006d9d50, uVar8);
                            } else {
                                iVar6 = (int)((uint)DAT_006d4988 + iVar6);
                                __builtin_memcpy(&iVar2, (void *)(iVar6 + 4), 4);
                                __builtin_memcpy(&iVar3, (void *)(iVar6 + 8), 4);
                                __builtin_memcpy(&iVar4, (void *)(iVar6 + 0xc), 4);
                                __builtin_memcpy(&iVar5, DAT_006d9d7b, 4);
                                __builtin_memcpy(&iVar7, DAT_006d9d7f, 4);
                                __builtin_memcpy(&iVar8, DAT_006d9d83, 4);
                                DAT_006d499c = FUN_00412810((int)((uint)iVar2 - (uint)iVar5),
                                                            (int)((uint)iVar3 - (uint)iVar7),
                                                            (int)((uint)iVar4 - (uint)iVar8));
                            }
                            if (0x4000 < (int)DAT_006d499c) {
                                uVar8 = FUN_004440e0(uVar10);
                                if ((char)uVar8 == '\x01') goto LAB_0044bd5e;
                            }
                            if (DAT_006d499c < local_4) {
                                local_4 = DAT_006d499c;
                                local_a = uVar10;
                            }
                        }
                    }
                }
            }
LAB_0044bd5e:
            uVar7 = (uVar7 & 0xffff0000) | (ushort)(uVar10 + 1);
        } while ((int)(uint)(ushort)(uVar10 + 1) < DAT_006d4980);
    }
    uVar7 = (uVar7 & 0xffff0000) | (ushort)DAT_00732a38;
    iVar6 = (int)((uint)DAT_00703c88 + (uint)DAT_00732a38);
    if ((int)(DAT_00732a38 & 0xffff) < iVar6) {
        do {
            uVar8 = uVar7 & 0xffff;
            bVar1 = *(byte *)(uVar8 * 0x23 + 3 + (uint)DAT_006d4988);
            uVar10 = (ushort)uVar7;
            if ((bVar1 != 0) && ((DAT_0052687a[(uint)bVar1 * 0x16] & 2) != 0)) {
                bVar4 = FUN_0049f010(uVar7, param_1, param_2);
                uVar11 = (ushort)bVar4;
                bVar4 = FUN_0049f010(uVar7, param_4, param_5);
                uVar5 = (ushort)bVar4;
                if (param_3 == 1) {
                    uVar11 = uVar11 | uVar5;
                } else {
                    uVar11 = uVar11 & uVar5;
                }
                if ((uVar11 != 0) && (*(ushort *)(DAT_00732e54 + 0x342) != uVar10)) {
                    bVar4 = FUN_00429a20(uVar8);
                    if (bVar4 != 0) {
                        if ((DAT_006d9d7a & 4) != 0) {
                            bVar4 = FUN_00429c40(uVar7);
                            if ((short)bVar4 == 0) goto LAB_0044be95;
                        }
                        if ((DAT_006d9d7a & 1) != 0) {
                            bVar4 = FUN_0044c560(uVar10, (short)local_8);
                            if ((short)bVar4 == 0) goto LAB_0044be95;
                        }
                        FUN_0042a1a0((uint)DAT_006d9d50, uVar8);
                        if (DAT_006d499c < local_4) {
                            local_4 = DAT_006d499c;
                            local_a = uVar10;
                        }
                    }
                }
            }
LAB_0044be95:
            uVar7 = (uVar7 & 0xffff0000) | (ushort)(uVar10 + 1);
            iVar6 = (int)((uint)DAT_00703c88 + (uint)DAT_00732a38);
        } while ((int)(uint)(ushort)(uVar10 + 1) < iVar6);
    }
    return (((uint)iVar6 >> 0x10) << 0x10) | local_a;
}
