/* SPDX-License-Identifier: MIT */
/* z_bop 004b6340 */
extern int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13; /* unaligned int at 0xaa7b13, read via memcpy */

void __cdecl
FUN_004b6340(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    int bVar1;
    char cVar2;
    int iVar3;
    unsigned char bVar4;
    unsigned char bVar5;
    char cVar6;
    unsigned int uVar7;
    int iVar8;
    unsigned int uVar9;
    unsigned char *pbVar10;
    unsigned char *pbVar11;
    int iVar12;
    int iVar13;
    int iVar14;
    unsigned short *puVar15;
    int local_5c;
    int local_28;
    int *local_10;
    int local_c;
    int dat_a7b13;

    __builtin_memcpy(&dat_a7b13, &DAT_00aa7b13, 4);

    if (*param_1 == param_6) {
        pbVar10 = (unsigned char *)param_1[4];
        local_c = (int)((unsigned int)dat_a7b13 * (unsigned int)param_3
                        + (unsigned int)param_2 * 2u + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                pbVar10 = pbVar10 + *(int *)pbVar10;
                param_5 = param_5 - 1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_5c = param_7;
            do {
                iVar12 = 0;
                pbVar11 = pbVar10 + 4;
                while (1) {
                    bVar4 = *pbVar11;
                    pbVar10 = pbVar11 + 1;
                    if (bVar4 == 0x80) break;
                    if ((bVar4 & 0x80) == 0) {
                        if ((bVar4 & 0x40) == 0) {
                            pbVar11 = pbVar11 + 2;
                            uVar9 = (unsigned int)bVar4;
                            iVar8 = param_1[*pbVar10 + 5];
                            puVar15 = (unsigned short *)(local_c + iVar12 * 2);
                            uVar7 = uVar9;
                            if (uVar9 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar15 = (unsigned short)iVar8;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            iVar12 = iVar12 + (int)uVar9;
                        } else {
                            uVar9 = (unsigned int)(bVar4 & 0x3f);
                            iVar8 = param_1[5];
                            puVar15 = (unsigned short *)(local_c + iVar12 * 2);
                            uVar7 = uVar9;
                            if (uVar9 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar15 = (unsigned short)iVar8;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            iVar12 = iVar12 + (int)uVar9;
                            pbVar11 = pbVar10;
                        }
                    } else {
                        bVar4 = bVar4 & 0x7f;
                        iVar8 = 0;
                        if (bVar4 != 0) {
                            do {
                                iVar14 = iVar12 + iVar8;
                                pbVar11 = pbVar10 + iVar8;
                                iVar8 = iVar8 + 1;
                                *(short *)(local_c + iVar14 * 2) = (short)param_1[*pbVar11 + 5];
                            } while (iVar8 < (int)(unsigned int)bVar4);
                        }
                        iVar12 = iVar12 + (int)(unsigned int)bVar4;
                        pbVar11 = pbVar10 + bVar4;
                    }
                }
                local_c = (int)((unsigned int)local_c + ((unsigned int)dat_a7b13 & 0xfffffffeu));
                local_5c = local_5c - 1;
            } while (local_5c != 0);
            return;
        }
    } else {
        local_10 = (int *)param_1[4];
        local_c = (int)((unsigned int)dat_a7b13 * (unsigned int)param_3
                        + (unsigned int)param_2 * 2u + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                local_10 = (int *)((int)local_10 + *local_10);
                param_5 = param_5 - 1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_28 = param_7;
            do {
                bVar1 = 0;
                iVar8 = *local_10;
                iVar12 = 0;
                iVar14 = 4;
                while (1) {
                    while (1) {
                        while (1) {
                            do {
                                iVar13 = iVar14 + 1;
                                bVar4 = *(unsigned char *)(iVar14 + (int)local_10);
                                iVar3 = iVar12;
                                if (!bVar1) {
                                    if (bVar4 == 0x80) goto LAB_004b6716;
                                    if ((bVar4 & 0x80) == 0) {
                                        if ((bVar4 & 0x40) == 0) {
                                            iVar3 = (int)(unsigned int)bVar4 + iVar12;
                                            if (iVar3 < param_4) {
                                                iVar13 = iVar14 + 2;
                                            } else {
                                                bVar1 = 1;
                                                iVar3 = 0;
                                                bVar4 = (unsigned char)(bVar4 - ((char)param_4 - (char)iVar12));
                                                if (bVar4 == 0) {
                                                    iVar13 = iVar14 + 2;
                                                }
                                            }
                                        } else {
                                            bVar4 = bVar4 & 0x3f;
                                            iVar3 = (int)(unsigned int)bVar4 + iVar12;
                                            if (param_4 <= iVar3) {
                                                bVar1 = 1;
                                                iVar3 = 0;
                                                bVar5 = (unsigned char)(bVar4 - ((char)param_4 - (char)iVar12));
                                                bVar4 = 0;
                                                if (bVar5 != 0) {
                                                    bVar4 = bVar5 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        bVar4 = bVar4 & 0x7f;
                                        iVar3 = iVar12 + (int)(unsigned int)bVar4;
                                        if (iVar3 < param_4) {
                                            iVar13 = iVar13 + (int)(unsigned int)bVar4;
                                        } else {
                                            bVar1 = 1;
                                            iVar3 = 0;
                                            iVar13 = iVar13 + (param_4 - iVar12);
                                            bVar5 = (unsigned char)(bVar4 - (char)(param_4 - iVar12));
                                            bVar4 = 0;
                                            if (bVar5 != 0) {
                                                bVar4 = bVar5 | 0x80;
                                            }
                                        }
                                    }
                                }
                                iVar12 = iVar3;
                                iVar14 = iVar13;
                            } while ((!bVar1) || (bVar4 == 0));
                            if (bVar4 == 0x80) goto LAB_004b6716;
                            cVar2 = (char)iVar3;
                            cVar6 = (char)param_6;
                            if ((bVar4 & 0x80) == 0) break;
                            uVar7 = (unsigned int)(bVar4 & 0x7f);
                            if (param_6 <= (int)(uVar7 + (unsigned int)iVar3)) {
                                if ((unsigned char)(cVar6 - cVar2) != 0) {
                                    iVar12 = 0;
                                    do {
                                        iVar14 = iVar12 + 1;
                                        *(short *)(local_c + (iVar3 + iVar12) * 2) =
                                            (short)param_1[*(unsigned char *)((int)local_10 + iVar13 + iVar12) + 5];
                                        iVar12 = iVar14;
                                    } while (iVar14 < (int)(unsigned int)(unsigned char)(cVar6 - cVar2));
                                }
                                goto LAB_004b6716;
                            }
                            iVar12 = 0;
                            if ((bVar4 & 0x7f) != 0) {
                                do {
                                    iVar14 = iVar12 + 1;
                                    *(short *)(local_c + (iVar3 + iVar12) * 2) =
                                        (short)param_1[*(unsigned char *)((int)local_10 + iVar13 + iVar12) + 5];
                                    iVar12 = iVar14;
                                } while (iVar14 < (int)uVar7);
                            }
                            iVar12 = iVar3 + (int)uVar7;
                            iVar14 = iVar13 + (int)uVar7;
                        }
                        if ((bVar4 & 0x40) != 0) break;
                        iVar14 = iVar13 + 1;
                        uVar7 = (unsigned int)bVar4;
                        iVar12 = iVar3 + (int)uVar7;
                        if (param_6 <= iVar12) {
                            uVar7 = (unsigned int)(unsigned char)(cVar6 - cVar2);
                            iVar12 = param_1[*(unsigned char *)(iVar13 + (int)local_10) + 5];
                            puVar15 = (unsigned short *)(local_c + iVar3 * 2);
                            if (uVar7 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar15 = (unsigned short)iVar12;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            goto LAB_004b6716;
                        }
                        iVar13 = param_1[*(unsigned char *)(iVar13 + (int)local_10) + 5];
                        puVar15 = (unsigned short *)(local_c + iVar3 * 2);
                        if (uVar7 != 0) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar15 = (unsigned short)iVar13;
                                puVar15 = puVar15 + 1;
                            }
                        }
                    }
                    uVar7 = (unsigned int)(bVar4 & 0x3f);
                    iVar12 = (int)uVar7 + iVar3;
                    if (param_6 <= iVar12) break;
                    iVar13 = param_1[5];
                    puVar15 = (unsigned short *)(local_c + iVar3 * 2);
                    if (uVar7 != 0) {
                        for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                            *puVar15 = (unsigned short)iVar13;
                            puVar15 = puVar15 + 1;
                        }
                    }
                }
                uVar7 = (unsigned int)(unsigned char)(cVar6 - cVar2);
                iVar12 = param_1[5];
                puVar15 = (unsigned short *)(local_c + iVar3 * 2);
                if (uVar7 != 0) {
                    for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                        *puVar15 = (unsigned short)iVar12;
                        puVar15 = puVar15 + 1;
                    }
                }
LAB_004b6716:
                local_10 = (int *)((int)local_10 + iVar8);
                local_c = (int)((unsigned int)local_c + ((unsigned int)dat_a7b13 & 0xfffffffeu));
                local_28 = local_28 - 1;
            } while (local_28 != 0);
        }
    }
    return;
}
