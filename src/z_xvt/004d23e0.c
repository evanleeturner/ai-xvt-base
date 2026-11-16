/* SPDX-License-Identifier: MIT */
/* z_xvt 004d23e0 */
extern int DAT_00758830;
extern unsigned char DAT_0075964b[4]; /* unaligned int at 0x75964b */

void __cdecl
FUN_004d23e0(int *param_1, int param_2, int param_3, int param_4, int param_5, int param_6,
             int param_7, unsigned char param_8)
{
    unsigned int stride;
    unsigned int fillw;

    stride = (unsigned int)DAT_0075964b[0]
           | ((unsigned int)DAT_0075964b[1] << 8)
           | ((unsigned int)DAT_0075964b[2] << 16)
           | ((unsigned int)DAT_0075964b[3] << 24);
    fillw = (unsigned int)param_8 * 0x01010101u;

    if (*param_1 == param_6) {
        unsigned char *pbVar10;
        unsigned char *pbVar11;
        unsigned int iVar9;
        unsigned int iVar7;
        unsigned char bVar3;
        unsigned int uVar5;
        unsigned int uVar6;
        unsigned int *puVar14;
        int local_8;

        pbVar10 = (unsigned char *)param_1[4];
        iVar9 = (unsigned int)param_2 + (unsigned int)param_3 * stride
                + (unsigned int)DAT_00758830;
        if (0 < param_5) {
            do {
                pbVar10 = pbVar10 + *(int *)pbVar10;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_8 = param_7;
            do {
                iVar7 = 0;
                pbVar11 = pbVar10 + 4;
                while (1) {
                    bVar3 = *pbVar11;
                    pbVar10 = pbVar11 + 1;
                    if (bVar3 == 0x80) break;
                    if ((bVar3 & 0x80) == 0) {
                        if ((bVar3 & 0x40) == 0) {
                            pbVar11 = pbVar11 + 2;
                            puVar14 = (unsigned int *)(iVar7 + iVar9);
                            iVar7 = iVar7 + (unsigned int)bVar3;
                            for (uVar5 = (unsigned int)(bVar3 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                                *puVar14 = fillw;
                                puVar14 = puVar14 + 1;
                            }
                            for (uVar5 = bVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned char *)puVar14 = param_8;
                                puVar14 = (unsigned int *)((int)puVar14 + 1);
                            }
                        } else {
                            iVar7 = iVar7 + (unsigned int)(bVar3 & 0x3f);
                            pbVar11 = pbVar10;
                        }
                    } else {
                        uVar6 = (unsigned int)(bVar3 & 0x7f);
                        puVar14 = (unsigned int *)(iVar7 + iVar9);
                        iVar7 = iVar7 + uVar6;
                        for (uVar5 = (unsigned int)((bVar3 & 0x7f) >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar14 = fillw;
                            puVar14 = puVar14 + 1;
                        }
                        for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned char *)puVar14 = param_8;
                            puVar14 = (unsigned int *)((int)puVar14 + 1);
                        }
                        pbVar11 = pbVar10 + uVar6;
                    }
                }
                iVar9 = iVar9 + stride;
                local_8 = local_8 + -1;
            } while (local_8 != 0);
            return;
        }
    } else {
        unsigned char *local_c;
        int local_4;
        int bVar1;
        unsigned int iVar8;
        unsigned int iVar12;
        unsigned int iVar13;
        unsigned int iVar9;
        int iVar7;
        unsigned char bVar3;
        unsigned char bVar4;
        char cVar2;
        unsigned int uVar5;
        unsigned int uVar6;
        unsigned int *puVar14;

        local_c = (unsigned char *)param_1[4];
        iVar9 = (unsigned int)param_2 + (unsigned int)param_3 * stride
                + (unsigned int)DAT_00758830;
        if (0 < param_5) {
            do {
                local_c = local_c + *(int *)local_c;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_4 = param_7;
            do {
                iVar8 = 0;
                iVar12 = 4;
                bVar1 = 0;
                iVar7 = *(int *)local_c;
LAB_004d2527:
                do {
                    iVar13 = iVar12 + 1;
                    bVar3 = local_c[iVar12];
                    if (!bVar1) {
                        if (bVar3 == 0x80) goto LAB_004d26c4;
                        if ((bVar3 & 0x80) == 0) {
                            cVar2 = (char)iVar8;
                            if ((bVar3 & 0x40) == 0) {
                                iVar8 = (unsigned int)bVar3 + iVar8;
                                if ((int)iVar8 < param_4) {
                                    iVar13 = iVar12 + 2;
                                } else {
                                    bVar1 = 1;
                                    iVar8 = 0;
                                    bVar3 = (unsigned char)(bVar3 - (char)((char)param_4 - cVar2));
                                    if (bVar3 == 0) {
                                        iVar13 = iVar12 + 2;
                                    }
                                }
                            } else {
                                bVar3 = bVar3 & 0x3f;
                                iVar8 = (unsigned int)bVar3 + iVar8;
                                if (param_4 <= (int)iVar8) {
                                    bVar1 = 1;
                                    iVar8 = 0;
                                    bVar4 = (unsigned char)(bVar3 - (char)((char)param_4 - cVar2));
                                    bVar3 = 0;
                                    if (bVar4 != 0) {
                                        bVar3 = bVar4 | 0x40;
                                    }
                                }
                            }
                        } else {
                            bVar3 = bVar3 & 0x7f;
                            iVar12 = iVar8 + (unsigned int)bVar3;
                            if ((int)iVar12 < param_4) {
                                iVar8 = iVar12;
                                iVar13 = iVar13 + (unsigned int)bVar3;
                            } else {
                                bVar1 = 1;
                                iVar13 = iVar13 + ((unsigned int)param_4 - iVar8);
                                bVar4 = (unsigned char)(bVar3 - (char)((unsigned int)param_4 - iVar8));
                                bVar3 = 0;
                                iVar8 = 0;
                                if (bVar4 != 0) {
                                    bVar3 = bVar4 | 0x80;
                                }
                            }
                        }
                    }
                    iVar12 = iVar13;
                } while (!bVar1 || bVar3 == 0);
                if (bVar3 == 0x80) goto LAB_004d26c4;
                if ((bVar3 & 0x80) == 0) {
                    if ((bVar3 & 0x40) == 0) {
                        iVar12 = iVar12 + 1;
                        uVar5 = (unsigned int)bVar3;
                        if (param_6 <= (int)(iVar8 + uVar5)) goto LAB_004d2695;
                        puVar14 = (unsigned int *)(iVar8 + iVar9);
                        iVar8 = iVar8 + uVar5;
                        for (uVar6 = (unsigned int)(bVar3 >> 2); uVar6 != 0; uVar6 = uVar6 - 1) {
                            *puVar14 = fillw;
                            puVar14 = puVar14 + 1;
                        }
                        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned char *)puVar14 = param_8;
                            puVar14 = (unsigned int *)((int)puVar14 + 1);
                        }
                        goto LAB_004d2527;
                    }
                    iVar8 = iVar8 + (unsigned int)(bVar3 & 0x3f);
                    if (param_6 <= (int)iVar8) goto LAB_004d26c4;
                    goto LAB_004d2527;
                }
                uVar5 = (unsigned int)(bVar3 & 0x7f);
                if ((int)(iVar8 + uVar5) < param_6) {
                    puVar14 = (unsigned int *)(iVar8 + iVar9);
                    iVar8 = iVar8 + uVar5;
                    for (uVar6 = (unsigned int)((bVar3 & 0x7f) >> 2); uVar6 != 0; uVar6 = uVar6 - 1) {
                        *puVar14 = fillw;
                        puVar14 = puVar14 + 1;
                    }
                    for (uVar6 = uVar5 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                        *(unsigned char *)puVar14 = param_8;
                        puVar14 = (unsigned int *)((int)puVar14 + 1);
                    }
                    iVar12 = iVar12 + uVar5;
                    goto LAB_004d2527;
                }
LAB_004d2695:
                bVar3 = (unsigned char)((char)param_6 - (char)iVar8);
                puVar14 = (unsigned int *)(iVar8 + iVar9);
                for (uVar5 = (unsigned int)(bVar3 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar14 = fillw;
                    puVar14 = puVar14 + 1;
                }
                for (uVar5 = bVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *(unsigned char *)puVar14 = param_8;
                    puVar14 = (unsigned int *)((int)puVar14 + 1);
                }
LAB_004d26c4:
                local_c = local_c + iVar7;
                iVar9 = iVar9 + stride;
                local_4 = local_4 + -1;
                if (local_4 == 0) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
