/* SPDX-License-Identifier: MIT */
/* z_xvt 004d13b0 */
extern int DAT_0075964b;   /* unaligned address (odd): read via memcpy at each use */
extern int DAT_00758830;

void __cdecl FUN_004d13b0(int *param_1, int param_2, int param_3, int param_4,
                          int param_5, int param_6, int param_7)
{
    _Bool bVar1;
    char cVar2;
    int iVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned char *pbVar8;
    unsigned char *pbVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned short *puVar14;
    int local_24;
    int local_1c;
    int *local_10;
    int local_c;
    int tmpStride;

    if (*param_1 == param_6) {
        pbVar8 = (unsigned char *)param_1[4];
        __builtin_memcpy(&tmpStride, &DAT_0075964b, 4);
        local_c = (int)(((unsigned int)param_3 * (unsigned int)tmpStride
                         + (unsigned int)param_2 * 2u) + (unsigned int)DAT_00758830);
        if (0 < param_5) {
            do {
                pbVar8 = (unsigned char *)((unsigned int)pbVar8 + (unsigned int)*(int *)pbVar8);
                param_5 = (int)((unsigned int)param_5 - 1u);
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_24 = param_7;
            do {
                iVar10 = 0;
                pbVar9 = pbVar8 + 4;
                while (1) {
                    bVar4 = *pbVar9;
                    pbVar8 = pbVar9 + 1;
                    if (bVar4 == 0x80) break;
                    if ((bVar4 & 0x80) == 0) {
                        if ((bVar4 & 0x40) == 0) {
                            pbVar9 = pbVar9 + 2;
                            uVar5 = (unsigned int)bVar4;
                            iVar12 = param_1[*pbVar8 + 5];
                            puVar14 = (unsigned short *)((unsigned int)local_c
                                                         + (unsigned int)iVar10 * 2u);
                            uVar7 = uVar5;
                            if (uVar5 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar14 = (unsigned short)iVar12;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            iVar10 = (int)((unsigned int)iVar10 + uVar5);
                        } else {
                            iVar10 = (int)((unsigned int)iVar10 + (unsigned int)(bVar4 & 0x3f));
                            pbVar9 = pbVar8;
                        }
                    } else {
                        iVar12 = 0;
                        bVar4 = bVar4 & 0x7f;
                        if (bVar4 != 0) {
                            do {
                                iVar13 = (int)((unsigned int)iVar10 + (unsigned int)iVar12);
                                pbVar9 = pbVar8 + iVar12;
                                iVar12 = (int)((unsigned int)iVar12 + 1u);
                                *(short *)((unsigned int)local_c + (unsigned int)iVar13 * 2u) =
                                    (short)param_1[*pbVar9 + 5];
                            } while (iVar12 < (int)(unsigned int)bVar4);
                        }
                        iVar10 = (int)((unsigned int)iVar10 + (unsigned int)bVar4);
                        pbVar9 = pbVar8 + bVar4;
                    }
                }
                __builtin_memcpy(&tmpStride, &DAT_0075964b, 4);
                local_c = (int)((unsigned int)local_c + ((unsigned int)tmpStride & 0xfffffffeu));
                local_24 = (int)((unsigned int)local_24 - 1u);
            } while (local_24 != 0);
            return;
        }
    } else {
        local_10 = (int *)param_1[4];
        __builtin_memcpy(&tmpStride, &DAT_0075964b, 4);
        local_c = (int)(((unsigned int)param_3 * (unsigned int)tmpStride
                         + (unsigned int)param_2 * 2u) + (unsigned int)DAT_00758830);
        if (0 < param_5) {
            do {
                local_10 = (int *)((unsigned int)local_10 + (unsigned int)*local_10);
                param_5 = (int)((unsigned int)param_5 - 1u);
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_1c = param_7;
            do {
                iVar12 = 0;
                bVar1 = 0;
                iVar10 = *local_10;
                iVar13 = 4;
                do {
                    for (;;) {
                        do {
                            iVar11 = (int)((unsigned int)iVar13 + 1u);
                            bVar4 = *(unsigned char *)((unsigned int)iVar13 + (unsigned int)local_10);
                            uVar7 = (unsigned int)bVar4;
                            if (!bVar1) {
                                if (bVar4 == 0x80) goto LAB_004d1715;
                                if ((bVar4 & 0x80) == 0) {
                                    cVar2 = (char)iVar12;
                                    if ((bVar4 & 0x40) == 0) {
                                        iVar12 = (int)((unsigned int)bVar4 + (unsigned int)iVar12);
                                        if (iVar12 < param_4) {
                                            iVar11 = (int)((unsigned int)iVar13 + 2u);
                                        } else {
                                            bVar1 = 1;
                                            iVar12 = 0;
                                            bVar4 = (unsigned char)(bVar4 - ((char)param_4 - cVar2));
                                            uVar7 = (unsigned int)bVar4;
                                            if (bVar4 == 0) {
                                                iVar11 = (int)((unsigned int)iVar13 + 2u);
                                            }
                                        }
                                    } else {
                                        uVar7 = uVar7 & 0xffffff3f;
                                        iVar12 = (int)((unsigned int)(bVar4 & 0x3f) + (unsigned int)iVar12);
                                        if (param_4 <= iVar12) {
                                            bVar1 = 1;
                                            iVar12 = 0;
                                            bVar4 = (unsigned char)((char)uVar7 - ((char)param_4 - cVar2));
                                            uVar7 = (unsigned int)bVar4;
                                            if (bVar4 != 0) {
                                                uVar7 = bVar4 | 0x40;
                                            }
                                        }
                                    }
                                } else {
                                    uVar7 = uVar7 & 0xffffff7f;
                                    iVar13 = (int)((unsigned int)(bVar4 & 0x7f) + (unsigned int)iVar12);
                                    if (iVar13 < param_4) {
                                        iVar11 = (int)((unsigned int)iVar11 + (unsigned int)(bVar4 & 0x7f));
                                        iVar12 = iVar13;
                                    } else {
                                        bVar1 = 1;
                                        iVar11 = (int)((unsigned int)iVar11
                                                       + ((unsigned int)param_4 - (unsigned int)iVar12));
                                        bVar4 = (unsigned char)((char)uVar7
                                                - (char)((unsigned int)param_4 - (unsigned int)iVar12));
                                        uVar7 = (unsigned int)bVar4;
                                        iVar12 = 0;
                                        if (bVar4 != 0) {
                                            uVar7 = bVar4 | 0x80;
                                        }
                                    }
                                }
                            }
                            iVar13 = iVar11;
                        } while (!bVar1 || (bVar4 = (unsigned char)uVar7, bVar4 == 0));

                        if (bVar4 == 0x80) goto LAB_004d1715;

                        if ((uVar7 & 0x80) != 0) {
                            uVar5 = uVar7 & 0x7f;
                            if (param_6 <= (int)(uVar5 + (unsigned int)iVar12)) {
                                bVar4 = (unsigned char)((char)param_6 - (char)iVar12);
                                if (bVar4 != 0) {
                                    iVar13 = 0;
                                    do {
                                        iVar6 = (int)((unsigned int)iVar13 + 1u);
                                        *(short *)((unsigned int)local_c
                                                   + ((unsigned int)iVar13 + (unsigned int)iVar12) * 2u) =
                                            (short)param_1[*(unsigned char *)
                                                ((unsigned int)local_10 + (unsigned int)iVar13
                                                 + (unsigned int)iVar11) + 5];
                                        iVar13 = iVar6;
                                    } while (iVar6 < (int)(unsigned int)bVar4);
                                }
                                goto LAB_004d1715;
                            }
                            iVar13 = 0;
                            if ((uVar7 & 0x7f) != 0) {
                                do {
                                    iVar6 = (int)((unsigned int)iVar12 + (unsigned int)iVar13);
                                    iVar3 = (int)((unsigned int)iVar11 + (unsigned int)iVar13);
                                    iVar13 = (int)((unsigned int)iVar13 + 1u);
                                    *(short *)((unsigned int)local_c + (unsigned int)iVar6 * 2u) =
                                        (short)param_1[*(unsigned char *)
                                            ((unsigned int)local_10 + (unsigned int)iVar3) + 5];
                                } while (iVar13 < (int)uVar5);
                            }
                            iVar12 = (int)((unsigned int)iVar12 + uVar5);
                            iVar13 = (int)((unsigned int)iVar11 + uVar5);
                            continue;
                        }

                        if ((uVar7 & 0x40) != 0) break;

                        iVar13 = (int)((unsigned int)iVar11 + 1u);
                        if (param_6 <= (int)(uVar7 + (unsigned int)iVar12)) {
                            uVar7 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar12);
                            iVar13 = param_1[*(unsigned char *)((unsigned int)iVar11
                                                                + (unsigned int)local_10) + 5];
                            puVar14 = (unsigned short *)((unsigned int)local_c
                                                         + (unsigned int)iVar12 * 2u);
                            if (uVar7 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar14 = (unsigned short)iVar13;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            goto LAB_004d1715;
                        }
                        iVar11 = param_1[*(unsigned char *)((unsigned int)iVar11
                                                            + (unsigned int)local_10) + 5];
                        puVar14 = (unsigned short *)((unsigned int)local_c
                                                     + (unsigned int)iVar12 * 2u);
                        uVar5 = uVar7;
                        if (uVar7 != 0) {
                            for (; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *puVar14 = (unsigned short)iVar11;
                                puVar14 = puVar14 + 1;
                            }
                        }
                        iVar12 = (int)((unsigned int)iVar12 + uVar7);
                    }
                    iVar12 = (int)((unsigned int)iVar12 + (unsigned int)(bVar4 & 0x3f));
                } while (iVar12 < param_6);
LAB_004d1715:
                local_10 = (int *)((unsigned int)local_10 + (unsigned int)iVar10);
                __builtin_memcpy(&tmpStride, &DAT_0075964b, 4);
                local_c = (int)((unsigned int)local_c + ((unsigned int)tmpStride & 0xfffffffeu));
                local_1c = (int)((unsigned int)local_1c - 1u);
            } while (local_1c != 0);
        }
    }
    return;
}
