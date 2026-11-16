/* SPDX-License-Identifier: MIT */
/* z_bop 004b5770 */
extern int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13[4]; /* unaligned global int at 0xaa7b13 */

void __cdecl
FUN_004b5770(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    int iVar1;
    int bVar2;
    unsigned char bVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned char bVar9;
    int iVar10;
    unsigned char *pbVar11;
    unsigned char *pbVar12;
    unsigned char *pbVar13;
    int local_14;
    int local_c;
    int local_4;
    int stride;
    unsigned int fill;

    pbVar11 = (unsigned char *)param_1[4];
    if (*param_1 == param_6) {
        __builtin_memcpy(&stride, DAT_00aa7b13, 4);
        local_14 = (int)((unsigned int)stride * (unsigned int)param_3 + (unsigned int)param_2 + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                int off;
                __builtin_memcpy(&off, pbVar11, 4);
                pbVar11 = pbVar11 + off;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_4 = param_7;
            do {
                iVar10 = 0;
                pbVar12 = pbVar11 + 4;
                while (1) {
                    bVar9 = *pbVar12;
                    pbVar11 = pbVar12 + 1;
                    if (bVar9 == 0x80) break;
                    if ((bVar9 & 0x80) == 0) {
                        if ((bVar9 & 0x40) == 0) {
                            bVar3 = *pbVar11;
                            pbVar12 = pbVar12 + 2;
                            pbVar11 = (unsigned char *)(local_14 + iVar10);
                            iVar10 = iVar10 + (unsigned int)bVar9;
                            fill = (unsigned int)bVar3 * 0x01010101u;
                            for (uVar5 = (unsigned int)(bVar9 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                                __builtin_memcpy(pbVar11, &fill, 4);
                                pbVar11 = pbVar11 + 4;
                            }
                            for (uVar5 = bVar9 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *pbVar11 = bVar3;
                                pbVar11 = pbVar11 + 1;
                            }
                        } else {
                            iVar10 = iVar10 + (bVar9 & 0x3f);
                            pbVar12 = pbVar11;
                        }
                    } else {
                        uVar5 = (unsigned int)(bVar9 & 0x7f);
                        pbVar13 = (unsigned char *)(local_14 + iVar10);
                        iVar10 = iVar10 + uVar5;
                        pbVar12 = pbVar11 + uVar5;
                        for (uVar4 = (unsigned int)((bVar9 & 0x7f) >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                            __builtin_memcpy(pbVar13, pbVar11, 4);
                            pbVar11 = pbVar11 + 4;
                            pbVar13 = pbVar13 + 4;
                        }
                        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *pbVar13 = *pbVar11;
                            pbVar11 = pbVar11 + 1;
                            pbVar13 = pbVar13 + 1;
                        }
                    }
                }
                __builtin_memcpy(&stride, DAT_00aa7b13, 4);
                local_14 = local_14 + stride;
                local_4 = local_4 + -1;
            } while (local_4 != 0);
            return;
        }
    } else {
        __builtin_memcpy(&stride, DAT_00aa7b13, 4);
        local_14 = (int)((unsigned int)stride * (unsigned int)param_3 + (unsigned int)param_2 + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                int off;
                __builtin_memcpy(&off, pbVar11, 4);
                pbVar11 = pbVar11 + off;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_c = param_7;
            do {
                iVar7 = 4;
                __builtin_memcpy(&iVar1, pbVar11, 4);
                bVar2 = 0;
                iVar10 = 0;
LAB_004b5892:
                do {
                    bVar9 = pbVar11[iVar7];
                    uVar5 = (unsigned int)bVar9;
                    iVar8 = iVar7 + 1;
                    iVar6 = iVar10;
                    if (!bVar2) {
                        if (bVar9 == 0x80) goto LAB_004b5a54;
                        if ((bVar9 & 0x80) == 0) {
                            if ((bVar9 & 0x40) == 0) {
                                iVar6 = (int)((unsigned int)bVar9 + (unsigned int)iVar10);
                                if (iVar6 < param_4) {
                                    iVar8 = iVar7 + 2;
                                } else {
                                    bVar2 = 1;
                                    iVar6 = 0;
                                    bVar9 = (unsigned char)(bVar9 - (unsigned char)(param_4 - iVar10));
                                    uVar5 = (unsigned int)bVar9;
                                    if (bVar9 == 0) {
                                        iVar8 = iVar7 + 2;
                                    }
                                }
                            } else {
                                uVar5 = uVar5 & 0xffffff3f;
                                iVar6 = (int)((unsigned int)(bVar9 & 0x3f) + (unsigned int)iVar10);
                                if (param_4 <= iVar6) {
                                    bVar2 = 1;
                                    iVar6 = 0;
                                    bVar9 = (unsigned char)((unsigned char)uVar5 - (unsigned char)(param_4 - iVar10));
                                    uVar5 = (unsigned int)bVar9;
                                    if (bVar9 != 0) {
                                        uVar5 = bVar9 | 0x40;
                                    }
                                }
                            }
                        } else {
                            uVar5 = uVar5 & 0xffffff7f;
                            iVar6 = (int)((unsigned int)(bVar9 & 0x7f) + (unsigned int)iVar10);
                            if (iVar6 < param_4) {
                                iVar8 = iVar8 + (bVar9 & 0x7f);
                            } else {
                                bVar2 = 1;
                                iVar6 = 0;
                                iVar8 = iVar8 + (param_4 - iVar10);
                                bVar9 = (unsigned char)((unsigned char)uVar5 - (unsigned char)(param_4 - iVar10));
                                uVar5 = (unsigned int)bVar9;
                                if (bVar9 != 0) {
                                    uVar5 = bVar9 | 0x80;
                                }
                            }
                        }
                    }
                    iVar7 = iVar8;
                    iVar10 = iVar6;
                } while (!bVar2 || ((bVar9 = (unsigned char)uVar5), bVar9 == 0));
                if (bVar9 == 0x80) goto LAB_004b5a54;
                if ((uVar5 & 0x80) == 0) {
                    if ((uVar5 & 0x40) == 0) {
                        bVar9 = pbVar11[iVar7];
                        iVar7 = iVar7 + 1;
                        iVar10 = (int)((unsigned int)iVar6 + uVar5);
                        fill = (unsigned int)bVar9 * 0x01010101u;
                        if (param_6 <= iVar10) {
                            bVar3 = (unsigned char)((unsigned char)param_6 - (unsigned char)iVar6);
                            pbVar12 = (unsigned char *)(local_14 + iVar6);
                            for (uVar5 = (unsigned int)(bVar3 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                                __builtin_memcpy(pbVar12, &fill, 4);
                                pbVar12 = pbVar12 + 4;
                            }
                            for (uVar5 = bVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *pbVar12 = bVar9;
                                pbVar12 = pbVar12 + 1;
                            }
                            goto LAB_004b5a54;
                        }
                        pbVar12 = (unsigned char *)(local_14 + iVar6);
                        for (uVar4 = uVar5 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                            __builtin_memcpy(pbVar12, &fill, 4);
                            pbVar12 = pbVar12 + 4;
                        }
                        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *pbVar12 = bVar9;
                            pbVar12 = pbVar12 + 1;
                        }
                        goto LAB_004b5892;
                    }
                    iVar10 = iVar6 + (int)(uVar5 & 0x3f);
                    if (param_6 <= iVar10) goto LAB_004b5a54;
                    goto LAB_004b5892;
                }
                uVar5 = (unsigned int)(bVar9 & 0x7f);
                iVar10 = (int)(uVar5 + (unsigned int)iVar6);
                if (iVar10 < param_6) {
                    pbVar12 = pbVar11 + iVar7;
                    iVar7 = iVar7 + (int)uVar5;
                    pbVar13 = (unsigned char *)(local_14 + iVar6);
                    for (uVar4 = (unsigned int)((bVar9 & 0x7f) >> 2); uVar4 != 0; uVar4 = uVar4 - 1) {
                        __builtin_memcpy(pbVar13, pbVar12, 4);
                        pbVar12 = pbVar12 + 4;
                        pbVar13 = pbVar13 + 4;
                    }
                    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pbVar13 = *pbVar12;
                        pbVar12 = pbVar12 + 1;
                        pbVar13 = pbVar13 + 1;
                    }
                    goto LAB_004b5892;
                }
                bVar9 = (unsigned char)((unsigned char)param_6 - (unsigned char)iVar6);
                pbVar12 = pbVar11 + iVar7;
                pbVar13 = (unsigned char *)(local_14 + iVar6);
                for (uVar5 = (unsigned int)(bVar9 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                    __builtin_memcpy(pbVar13, pbVar12, 4);
                    pbVar12 = pbVar12 + 4;
                    pbVar13 = pbVar13 + 4;
                }
                for (uVar5 = bVar9 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *pbVar13 = *pbVar12;
                    pbVar12 = pbVar12 + 1;
                    pbVar13 = pbVar13 + 1;
                }
LAB_004b5a54:
                pbVar11 = pbVar11 + iVar1;
                __builtin_memcpy(&stride, DAT_00aa7b13, 4);
                local_14 = local_14 + stride;
                local_c = local_c + -1;
                if (local_c == 0) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
