/* SPDX-License-Identifier: MIT */
/* z_xvt 004d1cf0 */
extern unsigned char DAT_0075964b[4];
extern int DAT_00758830;

void __cdecl
FUN_004d1cf0(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    unsigned char *pbVar1;
    int iVar2;
    int bVar3;
    unsigned char bVar4;
    unsigned char bVar5;
    char cVar6;
    unsigned char *pbVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned int uVar10;
    char cVar11;
    int iVar12;
    int iVar13;
    int iVar14;
    unsigned short *puVar15;
    int local_24;
    int local_18;
    unsigned int local_14;
    int local_10;
    unsigned char *local_c;
    int stride;

    if (*param_1 == param_6) {
        local_c = (unsigned char *)param_1[4];
        __builtin_memcpy(&stride, DAT_0075964b, 4);
        local_14 = (unsigned int)stride * (unsigned int)param_3 + (unsigned int)param_2 * 2u + (unsigned int)DAT_00758830;
        if (0 < param_5) {
            do {
                local_c = local_c + *(int *)local_c;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_24 = param_7;
            do {
                iVar9 = 0;
                local_c = local_c + 4;
                while (1) {
                    pbVar7 = local_c + 1;
                    bVar5 = *local_c;
                    if (bVar5 == 0x80) break;
                    if ((bVar5 & 0x80) == 0) {
                        if ((bVar5 & 0x40) == 0) {
                            local_c = local_c + 2;
                            uVar10 = (unsigned int)bVar5;
                            iVar13 = param_1[*pbVar7 + 5];
                            puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar9 * 2u);
                            uVar8 = uVar10;
                            if (uVar10 != 0) {
                                for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                                    *puVar15 = (unsigned short)iVar13;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            iVar9 = (int)((unsigned int)iVar9 + uVar10);
                        } else {
                            uVar10 = (unsigned int)(bVar5 & 0x3f);
                            iVar13 = param_1[5];
                            puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar9 * 2u);
                            uVar8 = uVar10;
                            if (uVar10 != 0) {
                                for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                                    *puVar15 = (unsigned short)iVar13;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            iVar9 = (int)((unsigned int)iVar9 + uVar10);
                            local_c = pbVar7;
                        }
                    } else {
                        bVar4 = bVar5 & 0x7f;
                        iVar13 = 0;
                        if (bVar4 != 0) {
                            do {
                                pbVar1 = pbVar7 + iVar13;
                                iVar13 = iVar13 + 1;
                                *(unsigned short *)(local_14 + (unsigned int)(iVar9 + -1 + iVar13) * 2u) =
                                    (unsigned short)param_1[*pbVar1 + 5];
                            } while (iVar13 < (int)(bVar5 & 0xffffff7f));
                        }
                        iVar9 = (int)((unsigned int)iVar9 + (unsigned int)bVar4);
                        local_c = pbVar7 + bVar4;
                    }
                }
                __builtin_memcpy(&stride, DAT_0075964b, 4);
                local_14 = local_14 + ((unsigned int)stride & 0xfffffffeu);
                local_24 = local_24 + -1;
                local_c = pbVar7;
            } while (local_24 != 0);
            return;
        }
    } else {
        local_c = (unsigned char *)param_1[4];
        __builtin_memcpy(&stride, DAT_0075964b, 4);
        local_14 = (unsigned int)stride * (unsigned int)param_3 + (unsigned int)param_2 * 2u + (unsigned int)DAT_00758830;
        if (0 < param_5) {
            do {
                local_c = (unsigned char *)((int)local_c + *(int *)local_c);
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_18 = param_7;
            do {
                local_10 = 4;
                iVar13 = *(int *)local_c;
                bVar3 = 0;
                iVar9 = 0;
                while (1) {
                    while (1) {
                        while (1) {
                            do {
                                iVar14 = local_10 + 1;
                                bVar5 = *(unsigned char *)((int)local_c + local_10);
                                uVar8 = (unsigned int)bVar5;
                                iVar12 = iVar9;
                                if (!bVar3) {
                                    if (bVar5 == 0x80) goto LAB_004d210d;
                                    if ((bVar5 & 0x80) == 0) {
                                        if ((bVar5 & 0x40) == 0) {
                                            iVar12 = (int)((unsigned int)bVar5 + (unsigned int)iVar9);
                                            if (iVar12 < param_4) {
                                                iVar14 = local_10 + 2;
                                            } else {
                                                bVar3 = 1;
                                                iVar12 = 0;
                                                bVar5 = bVar5 - ((char)param_4 - (char)iVar9);
                                                uVar8 = (unsigned int)bVar5;
                                                if (bVar5 == 0) {
                                                    iVar14 = local_10 + 2;
                                                }
                                            }
                                        } else {
                                            uVar8 = uVar8 & 0xffffff3fu;
                                            iVar12 = (int)((unsigned int)(bVar5 & 0x3f) + (unsigned int)iVar9);
                                            if (param_4 <= iVar12) {
                                                bVar3 = 1;
                                                iVar12 = 0;
                                                bVar5 = (char)uVar8 - ((char)param_4 - (char)iVar9);
                                                uVar8 = (unsigned int)bVar5;
                                                if (bVar5 != 0) {
                                                    uVar8 = bVar5 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        uVar8 = uVar8 & 0xffffff7fu;
                                        iVar12 = (int)((unsigned int)(bVar5 & 0x7f) + (unsigned int)iVar9);
                                        if (iVar12 < param_4) {
                                            iVar14 = iVar14 + (bVar5 & 0x7f);
                                        } else {
                                            bVar3 = 1;
                                            iVar12 = 0;
                                            bVar5 = (char)uVar8 - (char)(param_4 - iVar9);
                                            uVar8 = (unsigned int)bVar5;
                                            iVar14 = (param_4 - iVar9) + iVar14;
                                            if (bVar5 != 0) {
                                                uVar8 = bVar5 | 0x80;
                                            }
                                        }
                                    }
                                }
                                local_10 = iVar14;
                                iVar9 = iVar12;
                            } while ((!bVar3) || (bVar5 = (unsigned char)uVar8, bVar5 == 0));
                            if (bVar5 == 0x80) goto LAB_004d210d;
                            cVar11 = (char)iVar12;
                            cVar6 = (char)param_6;
                            if ((uVar8 & 0x80) == 0) break;
                            uVar10 = (unsigned int)(bVar5 & 0x7f);
                            if (param_6 <= (int)(uVar10 + (unsigned int)iVar12)) {
                                iVar9 = 0;
                                if ((unsigned char)(cVar6 - cVar11) != 0) {
                                    do {
                                        iVar14 = iVar9 + iVar12;
                                        iVar2 = iVar9 + local_10;
                                        iVar9 = iVar9 + 1;
                                        *(unsigned short *)(local_14 + (unsigned int)iVar14 * 2u) =
                                            (unsigned short)param_1[*(unsigned char *)((int)local_c + iVar2) + 5];
                                    } while (iVar9 < (int)(unsigned int)(unsigned char)(cVar6 - cVar11));
                                }
                                goto LAB_004d210d;
                            }
                            iVar9 = 0;
                            if ((uVar8 & 0x7f) != 0) {
                                do {
                                    iVar14 = iVar9 + 1;
                                    *(unsigned short *)(local_14 + (unsigned int)(iVar9 + iVar12) * 2u) =
                                        (unsigned short)param_1[*(unsigned char *)((int)local_c + iVar14 + local_10 + -1) + 5];
                                    iVar9 = iVar14;
                                } while (iVar14 < (int)uVar10);
                            }
                            iVar9 = (int)((unsigned int)iVar12 + uVar10);
                            local_10 = (int)((unsigned int)local_10 + uVar10);
                        }
                        if ((uVar8 & 0x40) != 0) break;
                        iVar14 = local_10 + 1;
                        iVar9 = (int)(uVar8 + (unsigned int)iVar12);
                        if (param_6 <= iVar9) {
                            uVar8 = (unsigned int)(unsigned char)(cVar6 - cVar11);
                            iVar9 = param_1[*(unsigned char *)((int)local_c + local_10) + 5];
                            puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar12 * 2u);
                            if (uVar8 != 0) {
                                for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                                    *puVar15 = (unsigned short)iVar9;
                                    puVar15 = puVar15 + 1;
                                }
                            }
                            goto LAB_004d210d;
                        }
                        iVar2 = param_1[*(unsigned char *)((int)local_c + local_10) + 5];
                        puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar12 * 2u);
                        local_10 = iVar14;
                        if (uVar8 != 0) {
                            for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                                *puVar15 = (unsigned short)iVar2;
                                puVar15 = puVar15 + 1;
                            }
                        }
                    }
                    uVar8 = (unsigned int)(bVar5 & 0x3f);
                    iVar9 = (int)(uVar8 + (unsigned int)iVar12);
                    if (param_6 <= iVar9) break;
                    iVar14 = param_1[5];
                    puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar12 * 2u);
                    if (uVar8 != 0) {
                        for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                            *puVar15 = (unsigned short)iVar14;
                            puVar15 = puVar15 + 1;
                        }
                    }
                }
                uVar8 = (unsigned int)(unsigned char)(cVar6 - cVar11);
                iVar9 = param_1[5];
                puVar15 = (unsigned short *)(local_14 + (unsigned int)iVar12 * 2u);
                if (uVar8 != 0) {
                    for (; uVar8 != 0; uVar8 = uVar8 - 1) {
                        *puVar15 = (unsigned short)iVar9;
                        puVar15 = puVar15 + 1;
                    }
                }
LAB_004d210d:
                local_c = (unsigned char *)((int)local_c + iVar13);
                __builtin_memcpy(&stride, DAT_0075964b, 4);
                local_14 = local_14 + ((unsigned int)stride & 0xfffffffeu);
                local_18 = local_18 + -1;
            } while (local_18 != 0);
        }
    }
    return;
}
