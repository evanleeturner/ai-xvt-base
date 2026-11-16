/* SPDX-License-Identifier: MIT */
/* z_xvt 004d2700 */
extern unsigned char DAT_0075964b[4];
extern int DAT_00758830;

void __cdecl
FUN_004d2700(int *param_1, int param_2, int param_3, int param_4, int param_5,
             int param_6, int param_7, unsigned short param_8)
{
    char cVar1;
    _Bool bVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned char *pbVar8;
    unsigned char *pbVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned short *puVar14;
    int local_20;
    int local_14;
    int *local_10;
    int local_c;

    __builtin_memcpy(&uVar8, DAT_0075964b, 4);

    if (*param_1 == param_6) {
        pbVar8 = (unsigned char *)param_1[4];
        local_c = (int)((unsigned int)param_3 * uVar8 + (unsigned int)param_2 * 2u
                        + (unsigned int)DAT_00758830);
        if (0 < param_5) {
            do {
                pbVar8 = (unsigned char *)((unsigned int)pbVar8 + (unsigned int)*(int *)pbVar8);
                param_5 = (int)((unsigned int)param_5 - 1u);
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_20 = param_7;
            uVar5 = uVar8 & 0xfffffffe;
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
                            puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar11 * 2u);
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
                        puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar11 * 2u);
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
                local_20 = (int)((unsigned int)local_20 - 1u);
            } while (local_20 != 0);
            return;
        }
    } else {
        local_10 = (int *)param_1[4];
        local_c = (int)((unsigned int)param_3 * uVar8 + (unsigned int)param_2 * 2u
                        + (unsigned int)DAT_00758830);
        if (0 < param_5) {
            do {
                local_10 = (int *)((unsigned int)local_10 + (unsigned int)*local_10);
                param_5 = (int)((unsigned int)param_5 - 1u);
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_14 = param_7;
            uVar5 = uVar8 & 0xfffffffe;
            do {
                iVar10 = 0;
                iVar12 = 4;
                bVar2 = 0;
                iVar11 = *local_10;
                do {
                    for (;;) {
                        for (;;) {
                            do {
                                iVar13 = (int)((unsigned int)iVar12 + 1u);
                                bVar3 = *(unsigned char *)((unsigned int)iVar12 + (unsigned int)local_10);
                                if (!bVar2) {
                                    if (bVar3 == 0x80) goto LAB_004d2a07;
                                    if ((bVar3 & 0x80) == 0) {
                                        cVar1 = (char)iVar10;
                                        if ((bVar3 & 0x40) == 0) {
                                            iVar10 = (int)((unsigned int)bVar3 + (unsigned int)iVar10);
                                            if (iVar10 < param_4) {
                                                iVar13 = (int)((unsigned int)iVar12 + 2u);
                                            } else {
                                                bVar2 = 1;
                                                iVar10 = 0;
                                                bVar3 = (unsigned char)(bVar3 - (unsigned char)((char)param_4 - cVar1));
                                                if (bVar3 == 0) {
                                                    iVar13 = (int)((unsigned int)iVar12 + 2u);
                                                }
                                            }
                                        } else {
                                            bVar3 = bVar3 & 0x3f;
                                            iVar10 = (int)((unsigned int)bVar3 + (unsigned int)iVar10);
                                            if (param_4 <= iVar10) {
                                                bVar2 = 1;
                                                iVar10 = 0;
                                                bVar4 = (unsigned char)(bVar3 - (unsigned char)((char)param_4 - cVar1));
                                                bVar3 = 0;
                                                if (bVar4 != 0) {
                                                    bVar3 = bVar4 | 0x40;
                                                }
                                            }
                                        }
                                    } else {
                                        bVar3 = bVar3 & 0x7f;
                                        iVar12 = (int)((unsigned int)iVar10 + (unsigned int)bVar3);
                                        if (iVar12 < param_4) {
                                            iVar10 = iVar12;
                                            iVar13 = (int)((unsigned int)iVar13 + (unsigned int)bVar3);
                                        } else {
                                            bVar2 = 1;
                                            iVar13 = (int)((unsigned int)iVar13
                                                           + ((unsigned int)param_4 - (unsigned int)iVar10));
                                            bVar4 = (unsigned char)(bVar3
                                                    - (unsigned char)((unsigned int)param_4 - (unsigned int)iVar10));
                                            bVar3 = 0;
                                            iVar10 = 0;
                                            if (bVar4 != 0) {
                                                bVar3 = bVar4 | 0x80;
                                            }
                                        }
                                    }
                                }
                                iVar12 = iVar13;
                            } while (!bVar2 || bVar3 == 0);
                            if (bVar3 == 0x80) goto LAB_004d2a07;
                            if ((bVar3 & 0x80) == 0) break;
                            uVar7 = (unsigned int)(bVar3 & 0x7f);
                            if (param_6 <= (int)((unsigned int)iVar10 + uVar7)) {
                                uVar7 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar10);
                                puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar10 * 2u);
                                if (uVar7 != 0) {
                                    for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                        *puVar14 = param_8;
                                        puVar14 = puVar14 + 1;
                                    }
                                }
                                goto LAB_004d2a07;
                            }
                            puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar10 * 2u);
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
                        iVar12 = (int)((unsigned int)iVar12 + 1u);
                        uVar7 = (unsigned int)bVar3;
                        if (param_6 <= (int)((unsigned int)iVar10 + uVar7)) {
                            uVar7 = (unsigned int)(unsigned char)((char)param_6 - (char)iVar10);
                            puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar10 * 2u);
                            if (uVar7 != 0) {
                                for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar14 = param_8;
                                    puVar14 = puVar14 + 1;
                                }
                            }
                            goto LAB_004d2a07;
                        }
                        puVar14 = (unsigned short *)((unsigned int)local_c + (unsigned int)iVar10 * 2u);
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
LAB_004d2a07:
                local_10 = (int *)((unsigned int)local_10 + (unsigned int)iVar11);
                local_c = (int)((unsigned int)local_c + uVar5);
                local_14 = (int)((unsigned int)local_14 - 1u);
            } while (local_14 != 0);
        }
    }
    return;
}
