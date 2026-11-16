/* SPDX-License-Identifier: MIT */
/* z_xvt 0049a2f0 */
extern char *DAT_00645778;
extern int DAT_00645a84;
extern int DAT_00703c88;
extern int DAT_00732a38;
extern int DAT_006db7a4;
extern int DAT_00703ac0;

void * __cdecl FID_conflict__memcpy(void *dst, const void *src, size_t n);

void __cdecl FUN_0049a2f0(byte *param_1)
{
    int iVar1;
    byte *pbVar2;
    size_t sVar3;
    unsigned int uVar4;
    byte bVar5;
    unsigned short uVar6;
    char *pcVar7;
    char *pcVar8;
    char *pcVar9;
    char *pcVar10;
    char *pcVar11;
    int local_10;
    int local_c;
    int tmp;

    pcVar7 = DAT_00645778 + DAT_00645a84;
    iVar1 = *(int *)param_1;
    local_c = 0;
    local_10 = 0;
    pcVar8 = DAT_00645778;
    param_1 = param_1 + 4;
    if (0 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
        do {
            pcVar10 = pcVar8;
            if ((local_10 < DAT_006db7a4) || (DAT_00703ac0 <= local_10)) {
                if (local_c == 0) {
                    if (iVar1 <= local_10) break;
                    pbVar2 = param_1 + 1;
                    bVar5 = *param_1;
                    param_1 = pbVar2;
                    if ((bVar5 & 0x80) != 0) {
                        uVar6 = (unsigned short)bVar5;
                        bVar5 = 0;
                        local_c = (int)(uVar6 & 0x7f) - 1;
                    }
                } else {
                    local_c = local_c + -1;
                    bVar5 = 0;
                }
                pcVar9 = pcVar8 + 1;
                pcVar10 = pcVar9;
                if (*pcVar8 == '\0') {
                    if ((bVar5 & 1) != 0) {
                        pcVar10 = pcVar8 + 0x24;
                        sVar3 = (size_t)((unsigned int)pcVar7 - (unsigned int)pcVar9);
                        pcVar7 = pcVar7 + 0x23;
                        FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                        pcVar8 = pcVar9;
                        for (uVar4 = (unsigned int)((unsigned int)pcVar10 - (unsigned int)pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                            pcVar8[0] = '\0';
                            pcVar8[1] = '\0';
                            pcVar8[2] = '\0';
                            pcVar8[3] = '\0';
                            pcVar8 = pcVar8 + 4;
                        }
                        for (uVar4 = ((unsigned int)pcVar10 - (unsigned int)pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                            *pcVar8 = '\0';
                            pcVar8 = pcVar8 + 1;
                        }
                    }
                } else if ((bVar5 & 1) == 0) {
                    FID_conflict__memcpy(pcVar9, pcVar8 + 0x24, (size_t)((unsigned int)pcVar7 - (unsigned int)(pcVar8 + 0x24)));
                    pcVar7 = pcVar7 + -0x23;
                } else {
                    pcVar9 = pcVar8 + 0x24;
                    pcVar10 = pcVar9;
                    if (*(int *)(pcVar8 + 0x20) == 0) {
                        if ((bVar5 & 2) != 0) {
                            pcVar10 = pcVar8 + 0xd3;
                            sVar3 = (size_t)((unsigned int)pcVar7 - (unsigned int)pcVar9);
                            pcVar7 = pcVar7 + 0xaf;
                            FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                            pcVar8 = pcVar9;
                            for (uVar4 = (unsigned int)((unsigned int)pcVar10 - (unsigned int)pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                pcVar8[0] = '\0';
                                pcVar8[1] = '\0';
                                pcVar8[2] = '\0';
                                pcVar8[3] = '\0';
                                pcVar8 = pcVar8 + 4;
                            }
                            for (uVar4 = ((unsigned int)pcVar10 - (unsigned int)pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                *pcVar8 = '\0';
                                pcVar8 = pcVar8 + 1;
                            }
                        }
                    } else if ((bVar5 & 2) == 0) {
                        FID_conflict__memcpy(pcVar9, pcVar8 + 0xd3, (size_t)((unsigned int)pcVar7 - (unsigned int)(pcVar8 + 0xd3)));
                        pcVar7 = pcVar7 + -0xaf;
                    } else {
                        pcVar9 = pcVar8 + 0xd3;
                        pcVar10 = pcVar9;
                        __builtin_memcpy(&tmp, pcVar8 + 0xcb, 4);
                        if (tmp == 0) {
                            if ((bVar5 & 4) != 0) {
                                pcVar10 = pcVar8 + 0x509;
                                sVar3 = (size_t)((unsigned int)pcVar7 - (unsigned int)pcVar9);
                                pcVar7 = pcVar7 + 0x436;
                                FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                                pcVar11 = pcVar9;
                                for (uVar4 = (unsigned int)((unsigned int)pcVar10 - (unsigned int)pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    pcVar11[0] = '\0';
                                    pcVar11[1] = '\0';
                                    pcVar11[2] = '\0';
                                    pcVar11[3] = '\0';
                                    pcVar11 = pcVar11 + 4;
                                }
                                for (uVar4 = ((unsigned int)pcVar10 - (unsigned int)pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    *pcVar11 = '\0';
                                    pcVar11 = pcVar11 + 1;
                                }
                            }
                        } else if ((bVar5 & 4) == 0) {
                            FID_conflict__memcpy(pcVar9, pcVar8 + 0x509, (size_t)((unsigned int)pcVar7 - (unsigned int)(pcVar8 + 0x509)));
                            pcVar7 = pcVar7 + -0x436;
                        } else {
                            pcVar10 = pcVar8 + 0x509;
                        }
                        pcVar9 = pcVar10;
                        __builtin_memcpy(&tmp, pcVar8 + 199, 4);
                        if (tmp == 0) {
                            if ((bVar5 & 8) != 0) {
                                pcVar9 = pcVar10 + 8;
                                sVar3 = (size_t)((unsigned int)pcVar7 - (unsigned int)pcVar10);
                                pcVar7 = pcVar7 + 8;
                                FID_conflict__memcpy(pcVar9, pcVar10, sVar3);
                                pcVar11 = pcVar10;
                                for (uVar4 = (unsigned int)((unsigned int)pcVar9 - (unsigned int)pcVar10) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    pcVar11[0] = '\0';
                                    pcVar11[1] = '\0';
                                    pcVar11[2] = '\0';
                                    pcVar11[3] = '\0';
                                    pcVar11 = pcVar11 + 4;
                                }
                                for (uVar4 = ((unsigned int)pcVar9 - (unsigned int)pcVar10) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    *pcVar11 = '\0';
                                    pcVar11 = pcVar11 + 1;
                                }
                            }
                        } else if ((bVar5 & 8) == 0) {
                            FID_conflict__memcpy(pcVar10, pcVar10 + 8, (size_t)((unsigned int)pcVar7 - (unsigned int)(pcVar10 + 8)));
                            pcVar7 = pcVar7 + -8;
                        } else {
                            pcVar9 = pcVar10 + 8;
                        }
                        pcVar10 = pcVar9;
                        __builtin_memcpy(&tmp, pcVar8 + 0xcf, 4);
                        if (tmp == 0) {
                            if ((bVar5 & 0x10) != 0) {
                                pcVar10 = pcVar9 + 0x4c;
                                sVar3 = (size_t)((unsigned int)pcVar7 - (unsigned int)pcVar9);
                                pcVar7 = pcVar7 + 0x4c;
                                FID_conflict__memcpy(pcVar10, pcVar9, sVar3);
                                pcVar8 = pcVar9;
                                for (uVar4 = (unsigned int)((unsigned int)pcVar10 - (unsigned int)pcVar9) >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    pcVar8[0] = '\0';
                                    pcVar8[1] = '\0';
                                    pcVar8[2] = '\0';
                                    pcVar8[3] = '\0';
                                    pcVar8 = pcVar8 + 4;
                                }
                                for (uVar4 = ((unsigned int)pcVar10 - (unsigned int)pcVar9) & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
                                    *pcVar8 = '\0';
                                    pcVar8 = pcVar8 + 1;
                                }
                            }
                        } else if ((bVar5 & 0x10) == 0) {
                            FID_conflict__memcpy(pcVar9, pcVar9 + 0x4c, (size_t)((unsigned int)pcVar7 - (unsigned int)(pcVar9 + 0x4c)));
                            pcVar7 = pcVar7 + -0x4c;
                        } else {
                            pcVar10 = pcVar9 + 0x4c;
                        }
                    }
                }
            }
            local_10 = local_10 + 1;
            pcVar8 = pcVar10;
        } while (local_10 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
    }
    DAT_00645a84 = (int)((unsigned int)pcVar7 - (unsigned int)DAT_00645778);
    return;
}
