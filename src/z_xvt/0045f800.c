/* SPDX-License-Identifier: MIT */
/* z_xvt 0045f800 */
extern int *DAT_00658020;
extern int DAT_006572c0;
extern int DAT_006572b8;
extern int DAT_006572d4;
extern unsigned int DAT_00525518;
extern unsigned int DAT_00525520;
extern unsigned int DAT_00525528;
extern unsigned short DAT_00527e74;
extern unsigned int DAT_006da9f4;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_007091ae;
extern unsigned int _DAT_006da4fc;
extern unsigned int _DAT_006db7bc;

void __stdcall FUN_0045f800(void)
{
    unsigned char bVar1;
    int iVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned char bVar6;
    unsigned int uVar7;
    unsigned int *puVar8;
    unsigned char *pbVar9;
    unsigned char *pbVar10;
    unsigned int *puVar11;
    unsigned int *puVar12;
    unsigned int *puStack_8c;
    unsigned int uStack_84;
    int iStack_7c;
    unsigned int auStack_6c[27];
    int iStack_5c;
    unsigned int *puStack_48;

    puVar8 = auStack_6c;
    for (iVar4 = 0x1b; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar8 = 0;
        puVar8 = puVar8 + 1;
    }
    auStack_6c[0] = 0x6c;
    do {
        iVar2 = ((int (__stdcall *)(int, int, unsigned int *, int, int))
                 *(int *)(*DAT_00658020 + 100))((int)DAT_00658020, 0, auStack_6c, 0, 0);
        iStack_5c = (int)auStack_6c[4];
        puStack_48 = (unsigned int *)auStack_6c[9];
        puVar8 = puStack_48;
        iVar4 = iStack_5c;
        if (iVar2 == 0) {
            puVar11 = auStack_6c;
            for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
                *puVar11 = 0;
                puVar11 = puVar11 + 1;
            }
            auStack_6c[0] = 0x6c;
            do {
                iVar2 = ((int (__stdcall *)(int, int, unsigned int *, int, int))
                         *(int *)(*DAT_00658020 + 100))(DAT_006572c0, 0, auStack_6c, 0, 0);
                iStack_5c = (int)auStack_6c[4];
                puStack_48 = (unsigned int *)auStack_6c[9];
                if (iVar2 == 0) {
                    puStack_8c = puStack_48;
                    uVar7 = 0;
                    puVar8 = (unsigned int *)((unsigned int)puVar8 +
                             (unsigned int)((((unsigned int)DAT_006572d4 - DAT_00525528) >> 1) * (unsigned int)iVar4) +
                             (((unsigned int)DAT_006572b8 - DAT_00525520) >> 1) * DAT_00525518);
                    if (_DAT_006da4fc != 0) {
                        do {
                            if ((DAT_00525520 & 1) == 0) {
                                uVar3 = DAT_00525520 * DAT_00525518;
                                puVar11 = puStack_8c;
                                puVar12 = puVar8;
                                for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *puVar12 = *puVar11;
                                    puVar11 = puVar11 + 1;
                                    puVar12 = puVar12 + 1;
                                }
                                for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                                    *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                    puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                    puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                }
                            } else {
                                DAT_00525520 = DAT_00525520 - 1;
                                if (DAT_00525520 != 0) {
                                    uVar3 = DAT_00525520 * DAT_00525518;
                                    puVar11 = puStack_8c;
                                    puVar12 = puVar8;
                                    for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                                        *puVar12 = *puVar11;
                                        puVar11 = puVar11 + 1;
                                        puVar12 = puVar12 + 1;
                                    }
                                    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                                        *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                        puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                        puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                    }
                                }
                                DAT_00525520 = DAT_00525520 + 1;
                                *(unsigned short *)puVar8 = *(unsigned short *)puStack_8c;
                            }
                            puStack_8c = (unsigned int *)((unsigned int)puStack_8c + (unsigned int)iStack_5c);
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar4);
                            uVar7 = uVar7 + 1;
                        } while (uVar7 < _DAT_006da4fc);
                    }
                    iStack_7c = 0;
                    pbVar9 = (unsigned char *)((unsigned int)DAT_00527e74 + DAT_006da9f4);
                    if (DAT_006d4984 != 0) {
                        do {
                            if (_DAT_006db7bc != 0) {
                                if ((_DAT_006db7bc & 1) == 0) {
                                    uVar7 = _DAT_006db7bc * DAT_00525518;
                                    puVar11 = puStack_8c;
                                    puVar12 = puVar8;
                                    for (uVar3 = uVar7 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                        *puVar12 = *puVar11;
                                        puVar11 = puVar11 + 1;
                                        puVar12 = puVar12 + 1;
                                    }
                                    for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                        *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                        puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                        puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                    }
                                } else {
                                    _DAT_006db7bc = _DAT_006db7bc - 1;
                                    if (_DAT_006db7bc != 0) {
                                        uVar7 = _DAT_006db7bc * DAT_00525518;
                                        puVar11 = puStack_8c;
                                        puVar12 = puVar8;
                                        for (uVar3 = uVar7 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                            *puVar12 = *puVar11;
                                            puVar11 = puVar11 + 1;
                                            puVar12 = puVar12 + 1;
                                        }
                                        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                            *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                            puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                            puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                        }
                                    }
                                    _DAT_006db7bc = _DAT_006db7bc + 1;
                                    *(unsigned short *)puVar8 = *(unsigned short *)puStack_8c;
                                }
                                puStack_8c = (unsigned int *)((unsigned int)puStack_8c + _DAT_006db7bc * DAT_00525518);
                                puVar8 = (unsigned int *)((unsigned int)puVar8 + _DAT_006db7bc * DAT_00525518);
                            }
                            bVar6 = *pbVar9;
                            pbVar9 = pbVar9 + 1;
                            uStack_84 = 0;
                            pbVar10 = pbVar9;
                            if (DAT_007091ae != 0) {
                                do {
                                    uVar7 = (unsigned int)*pbVar10;
                                    pbVar9 = pbVar10 + 1;
                                    if (uVar7 == 0) {
                                        uVar7 = (unsigned int)*pbVar9;
                                        pbVar9 = pbVar10 + 2;
                                        if (uVar7 == 0) {
                                            bVar1 = *pbVar9;
                                            pbVar9 = pbVar10 + 3;
                                            uVar7 = (unsigned int)bVar1 + 0x100;
                                        }
                                        uVar7 = uVar7 + 0xff;
                                    }
                                    if ((char)bVar6 < 0) {
                                        if ((uVar7 & 1) == 0) {
                                            uVar5 = uVar7 * DAT_00525518;
                                            puVar11 = puStack_8c;
                                            puVar12 = puVar8;
                                            for (uVar3 = uVar5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                                *puVar12 = *puVar11;
                                                puVar11 = puVar11 + 1;
                                                puVar12 = puVar12 + 1;
                                            }
                                            for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                                *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                                puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                                puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                            }
                                        } else {
                                            if (uVar7 - 1 != 0) {
                                                uVar5 = (uVar7 - 1) * DAT_00525518;
                                                puVar11 = puStack_8c;
                                                puVar12 = puVar8;
                                                for (uVar3 = uVar5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                                    *puVar12 = *puVar11;
                                                    puVar11 = puVar11 + 1;
                                                    puVar12 = puVar12 + 1;
                                                }
                                                for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                                    *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                                    puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                                    puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                                }
                                            }
                                            *(unsigned short *)puVar8 = *(unsigned short *)puStack_8c;
                                        }
                                    }
                                    bVar6 = (unsigned char)(-(int)bVar6);
                                    puStack_8c = (unsigned int *)((unsigned int)puStack_8c + uVar7 * DAT_00525518);
                                    puVar8 = (unsigned int *)((unsigned int)puVar8 + uVar7 * DAT_00525518);
                                    uStack_84 = uStack_84 + uVar7;
                                    pbVar10 = pbVar9;
                                } while ((int)uStack_84 < (int)(unsigned int)DAT_007091ae);
                            }
                            uStack_84 = uStack_84 + _DAT_006db7bc;
                            if (uStack_84 < DAT_00525520) {
                                uStack_84 = DAT_00525520 - uStack_84;
                                if ((uStack_84 & 1) == 0) {
                                    uVar3 = uStack_84 * DAT_00525518;
                                    puVar11 = puStack_8c;
                                    puVar12 = puVar8;
                                    for (uVar7 = uVar3 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                        *puVar12 = *puVar11;
                                        puVar11 = puVar11 + 1;
                                        puVar12 = puVar12 + 1;
                                    }
                                    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                                        *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                        puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                        puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                    }
                                } else {
                                    if (uStack_84 - 1 != 0) {
                                        uVar3 = (uStack_84 - 1) * DAT_00525518;
                                        puVar11 = puStack_8c;
                                        puVar12 = puVar8;
                                        for (uVar7 = uVar3 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                            *puVar12 = *puVar11;
                                            puVar11 = puVar11 + 1;
                                            puVar12 = puVar12 + 1;
                                        }
                                        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                                            *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                            puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                            puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                        }
                                    }
                                    *(unsigned short *)puVar8 = *(unsigned short *)puStack_8c;
                                }
                                puVar8 = (unsigned int *)((unsigned int)puVar8 + uStack_84 * DAT_00525518);
                                puStack_8c = (unsigned int *)((unsigned int)puStack_8c + uStack_84 * DAT_00525518);
                                uStack_84 = DAT_00525520;
                            }
                            puVar8 = (unsigned int *)((unsigned int)puVar8 +
                                     ((unsigned int)iVar4 - uStack_84 * DAT_00525518));
                            iStack_7c = iStack_7c + 1;
                            puStack_8c = (unsigned int *)((unsigned int)puStack_8c +
                                         ((unsigned int)iStack_5c - uStack_84 * DAT_00525518));
                        } while (iStack_7c < (int)(unsigned int)DAT_006d4984);
                    }
                    uVar7 = (unsigned int)DAT_006d4984 + _DAT_006da4fc;
                    if (uVar7 < DAT_00525528) {
                        do {
                            if ((DAT_00525520 & 1) == 0) {
                                uVar5 = DAT_00525520 * DAT_00525518;
                                puVar11 = puStack_8c;
                                puVar12 = puVar8;
                                for (uVar3 = uVar5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                    *puVar12 = *puVar11;
                                    puVar11 = puVar11 + 1;
                                    puVar12 = puVar12 + 1;
                                }
                                for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                    puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                    puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                }
                            } else {
                                DAT_00525520 = DAT_00525520 - 1;
                                if (DAT_00525520 != 0) {
                                    uVar5 = DAT_00525520 * DAT_00525518;
                                    puVar11 = puStack_8c;
                                    puVar12 = puVar8;
                                    for (uVar3 = uVar5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                                        *puVar12 = *puVar11;
                                        puVar11 = puVar11 + 1;
                                        puVar12 = puVar12 + 1;
                                    }
                                    for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                        *(unsigned char *)puVar12 = *(unsigned char *)puVar11;
                                        puVar11 = (unsigned int *)((unsigned int)puVar11 + 1);
                                        puVar12 = (unsigned int *)((unsigned int)puVar12 + 1);
                                    }
                                }
                                DAT_00525520 = DAT_00525520 + 1;
                                *(unsigned short *)puVar8 = *(unsigned short *)puStack_8c;
                            }
                            puStack_8c = (unsigned int *)((unsigned int)puStack_8c + (unsigned int)iStack_5c);
                            puVar8 = (unsigned int *)((unsigned int)puVar8 + (unsigned int)iVar4);
                            uVar7 = uVar7 + 1;
                        } while (uVar7 < DAT_00525528);
                    }
                    ((int (__stdcall *)(int, unsigned int *))
                     *(int *)(*DAT_00658020 + 0x80))(DAT_006572c0, puStack_48);
                    ((int (__stdcall *)(int, int))
                     *(int *)(*DAT_00658020 + 0x80))((int)DAT_00658020, iStack_5c);
                    return;
                }
            } while (iVar2 == -0x7789fde4);
            return;
        }
    } while (iVar2 == -0x7789fde4);
    return;
}
