/* SPDX-License-Identifier: MIT */
/* z_bop 004b69b0 */
extern unsigned char DAT_00aa7b13;   /* unaligned int global (packed) */
extern int DAT_00aa6cfc;

void __cdecl
FUN_004b69b0(int *param_1, int param_2, int param_3, int param_4, int param_5,
             int param_6, int param_7, unsigned char param_8)
{
    int bVar1;
    char cVar2;
    unsigned char bVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char *pbVar7;
    unsigned char *pbVar8;
    int iVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned int *puVar14;
    int *local_c;
    int local_8;
    int local_4;
    int stride;

    __builtin_memcpy(&stride, &DAT_00aa7b13, 4);

    if (*param_1 == param_6) {
        pbVar7 = (unsigned char *)param_1[4];
        iVar10 = (int)((unsigned int)param_2 + (unsigned int)stride * (unsigned int)param_3
                       + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                pbVar7 = pbVar7 + *(int *)pbVar7;
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_8 = param_7;
            do {
                iVar11 = 0;
                pbVar8 = pbVar7 + 4;
                while (1) {
                    bVar3 = *pbVar8;
                    pbVar7 = pbVar8 + 1;
                    if (bVar3 == 0x80) break;
                    if ((bVar3 & 0x80) == 0) {
                        if ((bVar3 & 0x40) == 0) {
                            pbVar8 = pbVar8 + 2;
                            puVar14 = (unsigned int *)(iVar11 + iVar10);
                            for (uVar5 = (unsigned int)(bVar3 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                                *puVar14 = (unsigned int)param_8 * 0x01010101u;
                                puVar14 = puVar14 + 1;
                            }
                            for (uVar5 = bVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned char *)puVar14 = param_8;
                                puVar14 = (unsigned int *)((int)puVar14 + 1);
                            }
                            iVar11 = (int)((unsigned int)iVar11 + (unsigned int)bVar3);
                        } else {
                            iVar11 = (int)((unsigned int)iVar11 + (unsigned int)(bVar3 & 0x3f));
                            pbVar8 = pbVar7;
                        }
                    } else {
                        uVar6 = (unsigned int)(bVar3 & 0x7f);
                        puVar14 = (unsigned int *)(iVar11 + iVar10);
                        for (uVar5 = (unsigned int)((bVar3 & 0x7f) >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar14 = (unsigned int)param_8 * 0x01010101u;
                            puVar14 = puVar14 + 1;
                        }
                        for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned char *)puVar14 = param_8;
                            puVar14 = (unsigned int *)((int)puVar14 + 1);
                        }
                        iVar11 = (int)((unsigned int)iVar11 + uVar6);
                        pbVar8 = pbVar7 + uVar6;
                    }
                }
                iVar10 = (int)((unsigned int)iVar10 + (unsigned int)stride);
                local_8 = local_8 + -1;
            } while (local_8 != 0);
            return;
        }
    } else {
        local_c = (int *)param_1[4];
        iVar10 = (int)((unsigned int)param_2 + (unsigned int)stride * (unsigned int)param_3
                       + (unsigned int)DAT_00aa6cfc);
        if (0 < param_5) {
            do {
                local_c = (int *)((int)local_c + *local_c);
                param_5 = param_5 + -1;
            } while (param_5 != 0);
        }
        if (0 < param_7) {
            local_4 = param_7;
            do {
                iVar9 = 0;
                iVar12 = 4;
                bVar1 = 0;
                iVar11 = *local_c;
LAB_004b6aeb:
                do {
                    iVar13 = iVar12 + 1;
                    bVar3 = *(unsigned char *)(iVar12 + (int)local_c);
                    if (!bVar1) {
                        if (bVar3 == 0x80) goto LAB_004b6c7f;
                        if ((bVar3 & 0x80) == 0) {
                            cVar2 = (char)iVar9;
                            if ((bVar3 & 0x40) == 0) {
                                iVar9 = (int)((unsigned int)bVar3 + (unsigned int)iVar9);
                                if (iVar9 < param_4) {
                                    iVar13 = iVar12 + 2;
                                } else {
                                    bVar1 = 1;
                                    iVar9 = 0;
                                    bVar3 = (unsigned char)(bVar3 - (unsigned char)((char)param_4 - cVar2));
                                    if (bVar3 == 0) {
                                        iVar13 = iVar12 + 2;
                                    }
                                }
                            } else {
                                bVar3 = bVar3 & 0x3f;
                                iVar9 = (int)((unsigned int)bVar3 + (unsigned int)iVar9);
                                if (param_4 <= iVar9) {
                                    bVar1 = 1;
                                    iVar9 = 0;
                                    bVar4 = (unsigned char)(bVar3 - (unsigned char)((char)param_4 - cVar2));
                                    bVar3 = 0;
                                    if (bVar4 != 0) {
                                        bVar3 = bVar4 | 0x40;
                                    }
                                }
                            }
                        } else {
                            bVar3 = bVar3 & 0x7f;
                            iVar12 = (int)((unsigned int)bVar3 + (unsigned int)iVar9);
                            if (iVar12 < param_4) {
                                iVar9 = iVar12;
                                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)bVar3);
                            } else {
                                bVar1 = 1;
                                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)(param_4 - iVar9));
                                bVar4 = (unsigned char)(bVar3 - (char)(param_4 - iVar9));
                                bVar3 = 0;
                                iVar9 = 0;
                                if (bVar4 != 0) {
                                    bVar3 = bVar4 | 0x80;
                                }
                            }
                        }
                    }
                    iVar12 = iVar13;
                } while ((!bVar1) || (bVar3 == 0));

                if (bVar3 == 0x80) goto LAB_004b6c7f;
                if ((bVar3 & 0x80) == 0) {
                    if ((bVar3 & 0x40) == 0) {
                        iVar12 = iVar12 + 1;
                        uVar5 = (unsigned int)bVar3;
                        if (param_6 <= (int)(uVar5 + (unsigned int)iVar9)) goto LAB_004b6c50;
                        puVar14 = (unsigned int *)(iVar9 + iVar10);
                        iVar9 = (int)((unsigned int)iVar9 + uVar5);
                        for (uVar6 = (unsigned int)(bVar3 >> 2); uVar6 != 0; uVar6 = uVar6 - 1) {
                            *puVar14 = (unsigned int)param_8 * 0x01010101u;
                            puVar14 = puVar14 + 1;
                        }
                        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned char *)puVar14 = param_8;
                            puVar14 = (unsigned int *)((int)puVar14 + 1);
                        }
                        goto LAB_004b6aeb;
                    }
                    iVar9 = (int)((unsigned int)iVar9 + (unsigned int)(bVar3 & 0x3f));
                    if (param_6 <= iVar9) goto LAB_004b6c7f;
                    goto LAB_004b6aeb;
                }
                uVar5 = (unsigned int)(bVar3 & 0x7f);
                if ((int)(uVar5 + (unsigned int)iVar9) < param_6) {
                    puVar14 = (unsigned int *)(iVar9 + iVar10);
                    iVar9 = (int)((unsigned int)iVar9 + uVar5);
                    for (uVar6 = (unsigned int)((bVar3 & 0x7f) >> 2); uVar6 != 0; uVar6 = uVar6 - 1) {
                        *puVar14 = (unsigned int)param_8 * 0x01010101u;
                        puVar14 = puVar14 + 1;
                    }
                    for (uVar6 = uVar5 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                        *(unsigned char *)puVar14 = param_8;
                        puVar14 = (unsigned int *)((int)puVar14 + 1);
                    }
                    iVar12 = (int)((unsigned int)iVar12 + uVar5);
                    goto LAB_004b6aeb;
                }
LAB_004b6c50:
                bVar3 = (unsigned char)((char)param_6 - (char)iVar9);
                puVar14 = (unsigned int *)(iVar9 + iVar10);
                for (uVar5 = (unsigned int)(bVar3 >> 2); uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar14 = (unsigned int)param_8 * 0x01010101u;
                    puVar14 = puVar14 + 1;
                }
                for (uVar5 = bVar3 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *(unsigned char *)puVar14 = param_8;
                    puVar14 = (unsigned int *)((int)puVar14 + 1);
                }
LAB_004b6c7f:
                iVar10 = (int)((unsigned int)iVar10 + (unsigned int)stride);
                local_c = (int *)((int)local_c + iVar11);
                local_4 = local_4 + -1;
                if (local_4 == 0) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
