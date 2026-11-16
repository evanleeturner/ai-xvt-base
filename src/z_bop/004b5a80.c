/* SPDX-License-Identifier: MIT */
/* z_bop 004b5a80 */
extern int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13; /* odd address: read as a packed 32-bit int via __builtin_memcpy */

void __cdecl
FUN_004b5a80(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    int bVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    unsigned char *pbVar8;
    unsigned char *pbVar9;
    int iVar10;
    int iVar11;
    unsigned short *puVar12;
    int local_40;
    int local_20;
    int *local_10;
    int local_c;
    int tmp;

    if (*param_1 == param_6) {
        pbVar8 = (unsigned char *)param_1[4];
        __builtin_memcpy(&tmp, &DAT_00aa7b13, 4);
        local_c = (int)((unsigned int)tmp * (unsigned int)param_3 + (unsigned int)param_2 * 2u + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                pbVar8 = pbVar8 + *(int *)pbVar8;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_40 = param_7;
            do {
                iVar5 = 0;
                pbVar9 = pbVar8 + 4;
                while (1) {
                    bVar2 = *pbVar9;
                    pbVar8 = pbVar9 + 1;
                    if (bVar2 == 0x80) break;
                    if ((bVar2 & 0x80) == 0) {
                        if ((bVar2 & 0x40) == 0) {
                            pbVar9 = pbVar9 + 2;
                            uVar3 = (unsigned int)bVar2;
                            iVar11 = param_1[*pbVar8 + 5];
                            puVar12 = (unsigned short *)(local_c + iVar5 * 2);
                            uVar4 = uVar3;
                            if (uVar3 != 0) {
                                for (; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    *puVar12 = (unsigned short)iVar11;
                                    puVar12 = puVar12 + 1;
                                }
                            }
                            iVar5 = iVar5 + uVar3;
                        } else {
                            iVar5 = iVar5 + (int)(unsigned int)(bVar2 & 0x3f);
                            pbVar9 = pbVar8;
                        }
                    } else {
                        bVar2 = bVar2 & 0x7f;
                        iVar11 = 0;
                        if (bVar2 != 0) {
                            do {
                                iVar10 = iVar11 + iVar5;
                                pbVar9 = pbVar8 + iVar11;
                                iVar11 = iVar11 + 1;
                                *(short *)(local_c + iVar10 * 2) = (short)param_1[*pbVar9 + 5];
                            } while (iVar11 < (int)(unsigned int)bVar2);
                        }
                        iVar5 = iVar5 + (int)(unsigned int)bVar2;
                        pbVar9 = pbVar8 + bVar2;
                    }
                }
                __builtin_memcpy(&tmp, &DAT_00aa7b13, 4);
                local_c = local_c + (tmp & 0xfffffffe);
                local_40 = local_40 + -1;
            } while (local_40 != 0);
            return;
        }
    } else {
        local_10 = (int *)param_1[4];
        __builtin_memcpy(&tmp, &DAT_00aa7b13, 4);
        local_c = (int)((unsigned int)tmp * (unsigned int)param_3 + (unsigned int)param_2 * 2u + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                local_10 = (int *)((int)local_10 + *local_10);
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_20 = param_7;
            do {
                bVar1 = 0;
                iVar11 = *local_10;
                iVar10 = 4;
                iVar5 = 0;
                do {
                    for (;;) {
                        for (;;) {
                            do {
                                iVar6 = iVar10 + 1;
                                bVar2 = *(unsigned char *)((int)local_10 + iVar10);
                                uVar4 = (unsigned int)bVar2;
                                iVar7 = iVar5;
                                if (!bVar1) {
                                    if (bVar2 == 0x80) goto LAB_004b5dbb;
                                    if ((bVar2 & 0x80) == 0) {
                                        if ((bVar2 & 0x40) == 0) {
                                            iVar7 = (int)((unsigned int)bVar2 + (unsigned int)iVar5);
                                            if (iVar7 < param_4) {
                                                iVar6 = iVar10 + 2;
                                            } else {
                                                bVar1 = 1;
                                                iVar7 = 0;
                                                bVar2 = (unsigned char)(bVar2 - ((char)param_4 - (char)iVar5));
                                                uVar4 = (unsigned int)bVar2;
                                                if (bVar2 == 0) {
                                                    iVar6 = iVar10 + 2;
                                                }
                                            }
                                        } else {
                                            uVar4 = uVar4 & 0xffffff3f;
                                            iVar7 = (int)((unsigned int)(bVar2 & 0x3f) + (unsigned int)iVar5);
                                            if (param_4 <= iVar7) {
                                                bVar1 = 1;
                                                iVar7 = 0;
                                                bVar2 = (unsigned char)((char)uVar4 - ((char)param_4 - (char)iVar5));
                                                uVar4 = (unsigned int)bVar2;
                                                if (bVar2 != 0) {
                                                    uVar4 = bVar2 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        uVar4 = uVar4 & 0xffffff7f;
                                        iVar7 = (int)((unsigned int)(bVar2 & 0x7f) + (unsigned int)iVar5);
                                        if (iVar7 < param_4) {
                                            iVar6 = iVar6 + (bVar2 & 0x7f);
                                        } else {
                                            bVar1 = 1;
                                            iVar7 = 0;
                                            iVar6 = iVar6 + (param_4 - iVar5);
                                            bVar2 = (unsigned char)((char)uVar4 - (char)(param_4 - iVar5));
                                            uVar4 = (unsigned int)bVar2;
                                            if (bVar2 != 0) {
                                                uVar4 = bVar2 | 0x80;
                                            }
                                        }
                                    }
                                }
                                iVar10 = iVar6;
                                iVar5 = iVar7;
                            } while ((!bVar1) || ((char)uVar4 == '\0'));
                            if ((char)uVar4 == -0x80) goto LAB_004b5dbb;
                            if ((uVar4 & 0x80) == 0) break;
                            uVar3 = uVar4 & 0x7f;
                            if (param_6 <= (int)(uVar3 + (unsigned int)iVar7)) {
                                bVar2 = (unsigned char)((char)param_6 - (char)iVar7);
                                if (bVar2 != 0) {
                                    iVar5 = 0;
                                    do {
                                        iVar10 = iVar5 + 1;
                                        *(short *)(local_c + (iVar5 + iVar7) * 2) =
                                            (short)param_1[*(unsigned char *)(iVar5 + iVar6 + (int)local_10) + 5];
                                        iVar5 = iVar10;
                                    } while (iVar10 < (int)(unsigned int)bVar2);
                                }
                                goto LAB_004b5dbb;
                            }
                            iVar5 = 0;
                            if ((uVar4 & 0x7f) != 0) {
                                do {
                                    iVar10 = iVar5 + 1;
                                    *(short *)(local_c + (iVar5 + iVar7) * 2) =
                                        (short)param_1[*(unsigned char *)(iVar5 + iVar6 + (int)local_10) + 5];
                                    iVar5 = iVar10;
                                } while (iVar10 < (int)uVar3);
                            }
                            iVar5 = (int)((unsigned int)iVar7 + uVar3);
                            iVar10 = (int)((unsigned int)iVar6 + uVar3);
                        }
                        if ((uVar4 & 0x40) != 0) break;
                        iVar10 = iVar6 + 1;
                        iVar5 = (int)((unsigned int)iVar7 + uVar4);
                        if (param_6 <= iVar5) {
                            uVar4 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar7);
                            iVar5 = param_1[*(unsigned char *)((int)local_10 + iVar6) + 5];
                            puVar12 = (unsigned short *)(local_c + iVar7 * 2);
                            if (uVar4 != 0) {
                                for (; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    *puVar12 = (unsigned short)iVar5;
                                    puVar12 = puVar12 + 1;
                                }
                            }
                            goto LAB_004b5dbb;
                        }
                        iVar6 = param_1[*(unsigned char *)((int)local_10 + iVar6) + 5];
                        puVar12 = (unsigned short *)(local_c + iVar7 * 2);
                        if (uVar4 != 0) {
                            for (; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *puVar12 = (unsigned short)iVar6;
                                puVar12 = puVar12 + 1;
                            }
                        }
                    }
                    iVar5 = (int)((unsigned int)iVar7 + (uVar4 & 0x3f));
                } while (iVar5 < param_6);
LAB_004b5dbb:
                local_10 = (int *)((int)local_10 + iVar11);
                __builtin_memcpy(&tmp, &DAT_00aa7b13, 4);
                local_c = local_c + (tmp & 0xfffffffe);
                local_20 = local_20 + -1;
            } while (local_20 != 0);
        }
    }
    return;
}
