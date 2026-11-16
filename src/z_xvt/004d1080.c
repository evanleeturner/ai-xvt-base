/* SPDX-License-Identifier: MIT */
/* z_xvt 004d1080 */
typedef unsigned char byte;
typedef unsigned int uint;

extern int DAT_00758830;
extern byte DAT_0075964b[4]; /* unaligned global at 0x75964b, read via memcpy */

void __cdecl
FUN_004d1080(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    byte uVar1;
    int iVar2;
    int bVar3;
    uint uVar4;
    uint uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    byte bVar9;
    byte bVar10;
    byte *pbVar11;
    byte *pbVar12;
    int iVar13;
    byte *piVar14;
    byte *puVar15;
    byte *pbVar16;
    byte *puVar17;
    int local_14;
    int local_8;
    int local_4;
    int tmp;
    uint fill;
    int stride;

    if (*param_1 == param_6) {
        pbVar11 = (byte *)param_1[4];
        __builtin_memcpy(&stride, DAT_0075964b, 4);
        local_14 = (int)((uint)param_3 * (uint)stride + (uint)DAT_00758830 + (uint)param_2);
        if (0 < param_5) {
            do {
                __builtin_memcpy(&tmp, pbVar11, 4);
                pbVar11 = pbVar11 + tmp;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_8 = param_7;
            do {
                iVar13 = 0;
                pbVar12 = pbVar11 + 4;
                while (1) {
                    bVar9 = *pbVar12;
                    pbVar11 = pbVar12 + 1;
                    if (bVar9 == 0x80) break;
                    if ((bVar9 & 0x80) == 0) {
                        if ((bVar9 & 0x40) == 0) {
                            bVar10 = *pbVar11;
                            pbVar12 = pbVar12 + 2;
                            pbVar11 = (byte *)(local_14 + iVar13);
                            iVar13 = iVar13 + (uint)bVar9;
                            fill = (uint)bVar10 * 0x01010101u;
                            for (uVar4 = (uint)(bVar9 >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                                __builtin_memcpy(pbVar11, &fill, 4);
                                pbVar11 = pbVar11 + 4;
                            }
                            for (uVar4 = bVar9 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *pbVar11 = bVar10;
                                pbVar11 = pbVar11 + 1;
                            }
                        } else {
                            iVar13 = iVar13 + (uint)(bVar9 & 0x3f);
                            pbVar12 = pbVar11;
                        }
                    } else {
                        uVar5 = (uint)(bVar9 & 0x7f);
                        pbVar16 = (byte *)(local_14 + iVar13);
                        iVar13 = iVar13 + (int)uVar5;
                        pbVar12 = pbVar11 + uVar5;
                        for (uVar4 = (uint)((bVar9 & 0x7f) >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                            __builtin_memcpy(pbVar16, pbVar11, 4);
                            pbVar11 = pbVar11 + 4;
                            pbVar16 = pbVar16 + 4;
                        }
                        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *pbVar16 = *pbVar11;
                            pbVar11 = pbVar11 + 1;
                            pbVar16 = pbVar16 + 1;
                        }
                    }
                }
                __builtin_memcpy(&stride, DAT_0075964b, 4);
                local_14 = (int)((uint)local_14 + (uint)stride);
                local_8 = local_8 + -1;
            } while (local_8 != 0);
            return;
        }
    } else {
        piVar14 = (byte *)param_1[4];
        __builtin_memcpy(&stride, DAT_0075964b, 4);
        local_14 = (int)((uint)param_3 * (uint)stride + (uint)DAT_00758830 + (uint)param_2);
        if (0 < param_5) {
            do {
                __builtin_memcpy(&tmp, piVar14, 4);
                piVar14 = piVar14 + tmp;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_4 = param_7;
            do {
                iVar7 = 4;
                __builtin_memcpy(&iVar2, piVar14, 4);
                bVar3 = 0;
                iVar13 = 0;
LAB_004d11b7:
                do {
                    bVar9 = *(byte *)(iVar7 + (int)piVar14);
                    iVar8 = iVar7 + 1;
                    iVar6 = iVar13;
                    if (!bVar3) {
                        if (bVar9 == 0x80) goto LAB_004d137d;
                        if ((bVar9 & 0x80) == 0) {
                            if ((bVar9 & 0x40) == 0) {
                                iVar6 = (int)((uint)bVar9 + (uint)iVar13);
                                if (iVar6 < param_4) {
                                    iVar8 = iVar7 + 2;
                                } else {
                                    bVar3 = 1;
                                    iVar6 = 0;
                                    bVar9 = (byte)(bVar9 - (byte)((char)param_4 - (char)iVar13));
                                    if (bVar9 == 0) {
                                        iVar8 = iVar7 + 2;
                                    }
                                }
                            } else {
                                bVar9 = bVar9 & 0x3f;
                                iVar6 = (int)((uint)bVar9 + (uint)iVar13);
                                if (param_4 <= iVar6) {
                                    bVar3 = 1;
                                    iVar6 = 0;
                                    bVar10 = (byte)(bVar9 - (byte)((char)param_4 - (char)iVar13));
                                    bVar9 = 0;
                                    if (bVar10 != 0) {
                                        bVar9 = bVar10 | 0x40;
                                    }
                                }
                            }
                        } else {
                            bVar9 = bVar9 & 0x7f;
                            iVar6 = (int)((uint)bVar9 + (uint)iVar13);
                            if (iVar6 < param_4) {
                                iVar8 = iVar8 + (int)(uint)bVar9;
                            } else {
                                bVar3 = 1;
                                iVar6 = 0;
                                iVar8 = iVar8 + (param_4 - iVar13);
                                bVar10 = (byte)(bVar9 - (byte)(param_4 - iVar13));
                                bVar9 = 0;
                                if (bVar10 != 0) {
                                    bVar9 = bVar10 | 0x80;
                                }
                            }
                        }
                    }
                    iVar7 = iVar8;
                    iVar13 = iVar6;
                } while ((!bVar3) || (bVar9 == 0));
                if (bVar9 == 0x80) goto LAB_004d137d;
                if ((bVar9 & 0x80) == 0) {
                    if ((bVar9 & 0x40) == 0) {
                        uVar1 = *(byte *)(iVar7 + (int)piVar14);
                        iVar7 = iVar7 + 1;
                        iVar13 = (int)((uint)iVar6 + (uint)bVar9);
                        fill = (uint)uVar1 * 0x01010101u;
                        if (param_6 <= iVar13) {
                            bVar9 = (byte)((char)param_6 - (char)iVar6);
                            puVar15 = (byte *)(local_14 + iVar6);
                            for (uVar4 = (uint)(bVar9 >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                                __builtin_memcpy(puVar15, &fill, 4);
                                puVar15 = puVar15 + 4;
                            }
                            for (uVar4 = bVar9 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar15 = uVar1;
                                puVar15 = puVar15 + 1;
                            }
                            goto LAB_004d137d;
                        }
                        puVar15 = (byte *)(local_14 + iVar6);
                        for (uVar4 = (uint)(bVar9 >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                            __builtin_memcpy(puVar15, &fill, 4);
                            puVar15 = puVar15 + 4;
                        }
                        for (uVar4 = bVar9 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                            *puVar15 = uVar1;
                            puVar15 = puVar15 + 1;
                        }
                        goto LAB_004d11b7;
                    }
                    iVar13 = (int)((uint)iVar6 + (uint)(bVar9 & 0x3f));
                    if (param_6 <= iVar13) goto LAB_004d137d;
                    goto LAB_004d11b7;
                }
                uVar4 = (uint)(bVar9 & 0x7f);
                iVar13 = (int)(uVar4 + (uint)iVar6);
                if (iVar13 < param_6) {
                    puVar15 = (byte *)(iVar7 + (int)piVar14);
                    iVar7 = (int)((uint)iVar7 + uVar4);
                    puVar17 = (byte *)(local_14 + iVar6);
                    for (uVar5 = (uint)((bVar9 & 0x7f) >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                        __builtin_memcpy(puVar17, puVar15, 4);
                        puVar15 = puVar15 + 4;
                        puVar17 = puVar17 + 4;
                    }
                    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                        *puVar17 = *puVar15;
                        puVar15 = puVar15 + 1;
                        puVar17 = puVar17 + 1;
                    }
                    goto LAB_004d11b7;
                }
                bVar9 = (byte)((char)param_6 - (char)iVar6);
                puVar15 = (byte *)(iVar7 + (int)piVar14);
                puVar17 = (byte *)(local_14 + iVar6);
                for (uVar4 = (uint)(bVar9 >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                    __builtin_memcpy(puVar17, puVar15, 4);
                    puVar15 = puVar15 + 4;
                    puVar17 = puVar17 + 4;
                }
                for (uVar4 = bVar9 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                    *puVar17 = *puVar15;
                    puVar15 = puVar15 + 1;
                    puVar17 = puVar17 + 1;
                }
LAB_004d137d:
                piVar14 = piVar14 + iVar2;
                __builtin_memcpy(&stride, DAT_0075964b, 4);
                local_14 = (int)((uint)local_14 + (uint)stride);
                local_4 = local_4 + -1;
                if (local_4 == 0) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
