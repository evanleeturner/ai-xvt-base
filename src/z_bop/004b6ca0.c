/* SPDX-License-Identifier: MIT */
/* z_bop 004b6ca0 */
extern unsigned char DAT_00aa7b13;   /* unaligned 32-bit global, read via memcpy */
extern int DAT_00aa6cfc;

void __cdecl FUN_004b6ca0(int *param_1, int param_2, int param_3, int param_4,
                          int param_5, int param_6, int param_7, unsigned short param_8)
{
    char cVar1;
    int bVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned char *pbVar8;
    unsigned char *pbVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned short *puVar14;
    int local_50;
    int local_1c;
    int *local_10;
    int local_c;
    unsigned int g_7b13;

    __builtin_memcpy(&g_7b13, &DAT_00aa7b13, 4);

    if (*param_1 == param_6) {
        pbVar8 = (unsigned char *)param_1[4];
        local_c = (int)(g_7b13 * (unsigned int)param_3 + (unsigned int)param_2 * 2u
                        + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                pbVar8 = pbVar8 + *(int *)pbVar8;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_50 = param_7;
            uVar5 = g_7b13 & 0xfffffffe;
            do {
                iVar11 = 0;
                pbVar9 = pbVar8 + 4;
                while (1) {
                    bVar3 = *pbVar9;
                    pbVar8 = pbVar9 + 1;
                    if (bVar3 == 0x80) break;
                    if ((bVar3 & 0x80) == 0) {
                        if ((bVar3 & 0x40) == 0) {
                            pbVar9 = pbVar9 + 2;
                            uVar6 = (unsigned int)bVar3;
                            puVar14 = (unsigned short *)(local_c + iVar11 * 2);
                            uVar7 = uVar6;
                            if (uVar6 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar14 = param_8;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            iVar11 = (int)((unsigned int)iVar11 + uVar6);
                        } else {
                            iVar11 = (int)((unsigned int)iVar11 + (unsigned int)(bVar3 & 0x3f));
                            pbVar9 = pbVar8;
                        }
                    } else {
                        uVar6 = (unsigned int)(bVar3 & 0x7f);
                        puVar14 = (unsigned short *)(local_c + iVar11 * 2);
                        uVar7 = uVar6;
                        if (uVar6 != 0) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar14 = param_8;
                                puVar14 = puVar14 + 1;
                            }
                        }
                        iVar11 = (int)((unsigned int)iVar11 + uVar6);
                        pbVar9 = pbVar8 + uVar6;
                    }
                }
                local_c = (int)((unsigned int)local_c + uVar5);
                local_50 = local_50 + -1;
            } while (local_50 != 0);
            return;
        }
    } else {
        local_10 = (int *)param_1[4];
        local_c = (int)(g_7b13 * (unsigned int)param_3 + (unsigned int)param_2 * 2u
                        + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                local_10 = (int *)((int)local_10 + *local_10);
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_1c = param_7;
            uVar5 = g_7b13 & 0xfffffffe;
            do {
                iVar10 = 0;
                iVar12 = 4;
                bVar2 = 0;
                iVar11 = *local_10;
                do {
                    while (1) {
                        while (1) {
                            do {
                                iVar13 = iVar12 + 1;
                                bVar3 = *(unsigned char *)(iVar12 + (int)local_10);
                                if (!bVar2) {
                                    if (bVar3 == 0x80) goto LAB_004b6f85;
                                    if ((bVar3 & 0x80) == 0) {
                                        cVar1 = (char)iVar10;
                                        if ((bVar3 & 0x40) == 0) {
                                            iVar10 = (int)((unsigned int)bVar3 + (unsigned int)iVar10);
                                            if (iVar10 < param_4) {
                                                iVar13 = iVar12 + 2;
                                            } else {
                                                bVar2 = 1;
                                                iVar10 = 0;
                                                bVar3 = (unsigned char)(bVar3 - ((char)param_4 - cVar1));
                                                if (bVar3 == 0) {
                                                    iVar13 = iVar12 + 2;
                                                }
                                            }
                                        } else {
                                            bVar3 = bVar3 & 0x3f;
                                            iVar10 = (int)((unsigned int)bVar3 + (unsigned int)iVar10);
                                            if (param_4 <= iVar10) {
                                                bVar2 = 1;
                                                iVar10 = 0;
                                                bVar4 = (unsigned char)(bVar3 - ((char)param_4 - cVar1));
                                                bVar3 = 0;
                                                if (bVar4 != 0) {
                                                    bVar3 = bVar4 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        bVar3 = bVar3 & 0x7f;
                                        iVar12 = (int)((unsigned int)bVar3 + (unsigned int)iVar10);
                                        if (iVar12 < param_4) {
                                            iVar10 = iVar12;
                                            iVar13 = (int)((unsigned int)iVar13 + (unsigned int)bVar3);
                                        } else {
                                            bVar2 = 1;
                                            iVar13 = iVar13 + (param_4 - iVar10);
                                            bVar4 = (unsigned char)(bVar3 - (char)(param_4 - iVar10));
                                            bVar3 = 0;
                                            iVar10 = 0;
                                            if (bVar4 != 0) {
                                                bVar3 = bVar4 | 0x80;
                                            }
                                        }
                                    }
                                }
                                iVar12 = iVar13;
                            } while ((!bVar2) || (bVar3 == 0));
                            if (bVar3 == 0x80) goto LAB_004b6f85;
                            if ((bVar3 & 0x80) == 0) break;
                            uVar7 = (unsigned int)(bVar3 & 0x7f);
                            if (param_6 <= (int)(uVar7 + (unsigned int)iVar10)) {
                                uVar7 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar10);
                                puVar14 = (unsigned short *)(local_c + iVar10 * 2);
                                if (uVar7 != 0) {
                                    for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                        *puVar14 = param_8;
                                        puVar14 = puVar14 + 1;
                                    }
                                }
                                goto LAB_004b6f85;
                            }
                            puVar14 = (unsigned short *)(local_c + iVar10 * 2);
                            uVar6 = uVar7;
                            if (uVar7 != 0) {
                                for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                    *puVar14 = param_8;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            iVar10 = (int)((unsigned int)iVar10 + uVar7);
                            iVar12 = (int)((unsigned int)iVar12 + uVar7);
                        }
                        if ((bVar3 & 0x40) != 0) break;
                        iVar12 = iVar12 + 1;
                        uVar7 = (unsigned int)bVar3;
                        if (param_6 <= (int)(uVar7 + (unsigned int)iVar10)) {
                            uVar7 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar10);
                            puVar14 = (unsigned short *)(local_c + iVar10 * 2);
                            if (uVar7 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar14 = param_8;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            goto LAB_004b6f85;
                        }
                        puVar14 = (unsigned short *)(local_c + iVar10 * 2);
                        uVar6 = uVar7;
                        if (uVar7 != 0) {
                            for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *puVar14 = param_8;
                                puVar14 = puVar14 + 1;
                            }
                        }
                        iVar10 = (int)((unsigned int)iVar10 + uVar7);
                    }
                    iVar10 = (int)((unsigned int)iVar10 + (unsigned int)(bVar3 & 0x3f));
                } while (iVar10 < param_6);
LAB_004b6f85:
                local_10 = (int *)((int)local_10 + iVar11);
                local_c = (int)((unsigned int)local_c + uVar5);
                local_1c = local_1c + -1;
            } while (local_1c != 0);
        }
    }
    return;
}
