/* SPDX-License-Identifier: MIT */
/* z_xvt 004d1960 */
extern int DAT_0075964b;   /* odd address (0x75964b): read via memcpy */
extern int DAT_00758830;

void __cdecl
FUN_004d1960(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6, int param_7)
{
    int iVar2;
    int iVar8;
    int iVar10;
    int iVar13;
    int iVar14;
    int local_14;
    int local_8;
    int local_4;
    int stride;
    unsigned char *pbVar11;
    unsigned char *pbVar12;
    unsigned char *pbVar15;
    unsigned char *puVar16;
    unsigned char *puVar17;
    unsigned char *local_10;
    unsigned char bVar4;
    unsigned char bVar5;
    unsigned char uVar1;
    unsigned int uVar7;
    unsigned int uVar9;
    unsigned int uVar6;
    int bVar3;
    int tmp;

    __builtin_memcpy(&stride, (unsigned char *)&DAT_0075964b, 4);

    if (*param_1 == param_6) {
        pbVar11 = (unsigned char *)param_1[4];
        local_14 = (int)((unsigned int)param_3 * (unsigned int)stride + (unsigned int)DAT_00758830 + (unsigned int)param_2);
        if (0 < param_5) {
            do {
                __builtin_memcpy(&tmp, pbVar11, 4);
                pbVar11 = pbVar11 + tmp;
                param_5 = param_5 - 1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_8 = param_7;
            do {
                iVar10 = 0;
                pbVar12 = pbVar11 + 4;
                while (1) {
                    bVar4 = *pbVar12;
                    pbVar11 = pbVar12 + 1;
                    if (bVar4 == 0x80) break;
                    if ((bVar4 & 0x80) == 0) {
                        if ((bVar4 & 0x40) == 0) {
                            bVar5 = *pbVar11;
                            pbVar12 = pbVar12 + 2;
                            pbVar11 = (unsigned char *)(local_14 + iVar10);
                            iVar10 = iVar10 + (int)(unsigned int)bVar4;
                            uVar6 = (unsigned int)bVar5 * 0x01010101u;
                            for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                                __builtin_memcpy(pbVar11, &uVar6, 4);
                                pbVar11 = pbVar11 + 4;
                            }
                            for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *pbVar11 = bVar5;
                                pbVar11 = pbVar11 + 1;
                            }
                        } else {
                            bVar4 = bVar4 & 0x3f;
                            puVar16 = (unsigned char *)(local_14 + iVar10);
                            iVar10 = iVar10 + (int)(unsigned int)bVar4;
                            uVar6 = 0;
                            for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                                __builtin_memcpy(puVar16, &uVar6, 4);
                                puVar16 = puVar16 + 4;
                            }
                            for (uVar7 = bVar4 & 3; pbVar12 = pbVar11, uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar16 = 0;
                                puVar16 = puVar16 + 1;
                            }
                        }
                    } else {
                        uVar7 = bVar4 & 0x7f;
                        pbVar15 = (unsigned char *)(local_14 + iVar10);
                        iVar10 = iVar10 + (int)uVar7;
                        pbVar12 = pbVar11 + uVar7;
                        for (uVar7 = uVar7 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                            __builtin_memcpy(pbVar15, pbVar11, 4);
                            pbVar11 = pbVar11 + 4;
                            pbVar15 = pbVar15 + 4;
                        }
                        for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                            *pbVar15 = *pbVar11;
                            pbVar11 = pbVar11 + 1;
                            pbVar15 = pbVar15 + 1;
                        }
                    }
                }
                local_14 = (int)((unsigned int)local_14 + (unsigned int)stride);
                local_8 = local_8 - 1;
            } while (local_8 != 0);
            return;
        }
    } else {
        local_10 = (unsigned char *)param_1[4];
        local_14 = (int)((unsigned int)param_3 * (unsigned int)stride + (unsigned int)DAT_00758830 + (unsigned int)param_2);
        if (0 < param_5) {
            do {
                __builtin_memcpy(&tmp, local_10, 4);
                local_10 = local_10 + tmp;
                param_5 = param_5 - 1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_4 = param_7;
            do {
                bVar3 = 0;
                __builtin_memcpy(&iVar2, local_10, 4);
                iVar10 = 0;
                iVar14 = 4;
LAB_004d1ac1:
                do {
                    iVar13 = iVar14 + 1;
                    bVar4 = *(local_10 + iVar14);
                    iVar8 = iVar10;
                    if (!bVar3) {
                        if (bVar4 == 0x80) goto LAB_004d1cb1;
                        if ((bVar4 & 0x80) == 0) {
                            if ((bVar4 & 0x40) == 0) {
                                iVar8 = (int)(unsigned int)bVar4 + iVar10;
                                if (iVar8 < param_4) {
                                    iVar13 = iVar14 + 2;
                                } else {
                                    bVar3 = 1;
                                    iVar8 = 0;
                                    bVar4 = (unsigned char)(bVar4 - (param_4 - iVar10));
                                    if (bVar4 == 0) {
                                        iVar13 = iVar14 + 2;
                                    }
                                }
                            } else {
                                bVar4 = bVar4 & 0x3f;
                                iVar8 = (int)(unsigned int)bVar4 + iVar10;
                                if (param_4 <= iVar8) {
                                    bVar3 = 1;
                                    iVar8 = 0;
                                    bVar5 = (unsigned char)(bVar4 - (param_4 - iVar10));
                                    bVar4 = 0;
                                    if (bVar5 != 0) {
                                        bVar4 = bVar5 | 0x40;
                                    }
                                }
                            }
                        } else {
                            bVar4 = bVar4 & 0x7f;
                            iVar8 = (int)(unsigned int)bVar4 + iVar10;
                            if (iVar8 < param_4) {
                                iVar13 = iVar13 + (int)(unsigned int)bVar4;
                            } else {
                                bVar3 = 1;
                                iVar8 = 0;
                                iVar13 = iVar13 + (param_4 - iVar10);
                                bVar5 = (unsigned char)(bVar4 - (param_4 - iVar10));
                                bVar4 = 0;
                                if (bVar5 != 0) {
                                    bVar4 = bVar5 | 0x80;
                                }
                            }
                        }
                    }
                    iVar14 = iVar13;
                    iVar10 = iVar8;
                } while ((!bVar3) || (bVar4 == 0));
                if (bVar4 != 0x80) {
                    if ((bVar4 & 0x80) == 0) {
                        if ((bVar4 & 0x40) == 0) {
                            iVar13 = iVar14 + 1;
                            uVar1 = *(local_10 + iVar14);
                            iVar10 = (int)(unsigned int)bVar4 + iVar8;
                            uVar6 = (unsigned int)uVar1 * 0x01010101u;
                            if (iVar10 < param_6) {
                                puVar16 = (unsigned char *)(local_14 + iVar8);
                                for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                                    __builtin_memcpy(puVar16, &uVar6, 4);
                                    puVar16 = puVar16 + 4;
                                }
                                for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar16 = uVar1;
                                    puVar16 = puVar16 + 1;
                                }
                                iVar14 = iVar13;
                                goto LAB_004d1ac1;
                            }
                        } else {
                            bVar4 = bVar4 & 0x3f;
                            iVar10 = (int)(unsigned int)bVar4 + iVar8;
                            uVar6 = 0;
                            if (iVar10 < param_6) {
                                puVar16 = (unsigned char *)(local_14 + iVar8);
                                for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                                    __builtin_memcpy(puVar16, &uVar6, 4);
                                    puVar16 = puVar16 + 4;
                                }
                                for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                    *puVar16 = 0;
                                    puVar16 = puVar16 + 1;
                                }
                                goto LAB_004d1ac1;
                            }
                        }
                        bVar4 = (unsigned char)(param_6 - iVar8);
                        puVar16 = (unsigned char *)(local_14 + iVar8);
                        for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                            __builtin_memcpy(puVar16, &uVar6, 4);
                            puVar16 = puVar16 + 4;
                        }
                        for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                            *puVar16 = (unsigned char)uVar6;
                            puVar16 = puVar16 + 1;
                        }
                    } else {
                        uVar7 = (unsigned int)(bVar4 & 0x7f);
                        iVar10 = (int)uVar7 + iVar8;
                        if (iVar10 < param_6) {
                            puVar16 = local_10 + iVar14;
                            iVar14 = iVar14 + (int)uVar7;
                            puVar17 = (unsigned char *)(local_14 + iVar8);
                            for (uVar9 = (unsigned int)((bVar4 & 0x7f) >> 2); uVar9 != 0; uVar9 = uVar9 - 1) {
                                __builtin_memcpy(puVar17, puVar16, 4);
                                puVar16 = puVar16 + 4;
                                puVar17 = puVar17 + 4;
                            }
                            for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar17 = *puVar16;
                                puVar16 = puVar16 + 1;
                                puVar17 = puVar17 + 1;
                            }
                            goto LAB_004d1ac1;
                        }
                        bVar4 = (unsigned char)(param_6 - iVar8);
                        puVar16 = local_10 + iVar14;
                        puVar17 = (unsigned char *)(local_14 + iVar8);
                        for (uVar7 = (unsigned int)(bVar4 >> 2); uVar7 != 0; uVar7 = uVar7 - 1) {
                            __builtin_memcpy(puVar17, puVar16, 4);
                            puVar16 = puVar16 + 4;
                            puVar17 = puVar17 + 4;
                        }
                        for (uVar7 = bVar4 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                            *puVar17 = *puVar16;
                            puVar16 = puVar16 + 1;
                            puVar17 = puVar17 + 1;
                        }
                    }
                }
LAB_004d1cb1:
                local_10 = local_10 + iVar2;
                local_14 = (int)((unsigned int)local_14 + (unsigned int)stride);
                local_4 = local_4 - 1;
                if (local_4 == 0) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
