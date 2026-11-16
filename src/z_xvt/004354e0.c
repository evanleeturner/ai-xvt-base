/* SPDX-License-Identifier: MIT */
/* z_xvt 004354e0 */
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern short          DAT_006d9980;
extern int            DAT_00732e8c;
extern int            DAT_0070acc8;
extern int            DAT_00525534;
extern int            DAT_00525518;
extern unsigned short DAT_007325e0[];

void __cdecl FUN_004a6a80(void);
void __cdecl FUN_004a6f90(void);

void __cdecl FUN_004354e0(int param_1, int param_2, int param_3, int param_4, unsigned char param_5)
{
    unsigned short uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned int *puVar9;
    unsigned char *pbVar10;
    int local_8;

    if ((((0 < (int)((unsigned int)param_2 + (unsigned int)param_4)) &&
          (uVar3 = (unsigned int)param_3 + (unsigned int)param_1, 0 < (int)uVar3)) &&
         (param_1 < (int)(unsigned int)DAT_007091ae)) &&
        (((param_2 < (int)(unsigned int)DAT_006d4984) && (0 < param_4)) && (0 < param_3))) {
        iVar7 = param_3 >> 3;
        local_8 = param_4 >> 3;
        if (iVar7 < 3) {
            iVar7 = 3;
        }
        if (local_8 < 3) {
            local_8 = 3;
        }
        if (param_3 < iVar7) {
            iVar7 = param_3;
        }
        if (param_4 < local_8) {
            local_8 = param_4;
        }
        FUN_004a6a80();
        if (-1 < param_2) {
            uVar5 = (unsigned int)iVar7 + (unsigned int)param_1;
            if ((0 < (int)uVar5) && (uVar2 = (unsigned int)DAT_007091ae, param_1 < (int)uVar2)) {
                iVar4 = param_1;
                if (param_1 < 0) {
                    iVar4 = 0;
                }
                if ((int)uVar2 < (int)uVar5) {
                    uVar5 = uVar2;
                }
                iVar4 = (int)((unsigned int)(int)DAT_006d9980 + (unsigned int)iVar4);
                iVar8 = (int)((unsigned int)(int)DAT_006d9980 + uVar5);
                if (DAT_00525518 == 2) {
                    uVar1 = DAT_007325e0[(unsigned int)param_5];
                    if (iVar4 < iVar8) {
                        puVar9 = (unsigned int *)((unsigned int)DAT_0070acc8 * ((unsigned int)param_2 + (unsigned int)DAT_00732e8c) +
                                                  (unsigned int)DAT_00525534 + (unsigned int)iVar4 * 2u);
                        for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                            puVar9 = puVar9 + 1;
                        }
                        for (uVar5 = (unsigned int)((((unsigned int)iVar8 - (unsigned int)iVar4) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned short *)puVar9 = uVar1;
                            puVar9 = (unsigned int *)((int)puVar9 + 2);
                        }
                    }
                } else if (iVar4 < iVar8) {
                    pbVar10 = (unsigned char *)((unsigned int)DAT_0070acc8 * ((unsigned int)param_2 + (unsigned int)DAT_00732e8c) +
                                                (unsigned int)DAT_00525534 + (unsigned int)iVar4);
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                        pbVar10 = pbVar10 + 4;
                    }
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pbVar10 = param_5;
                        pbVar10 = pbVar10 + 1;
                    }
                }
            }
            iVar4 = (int)((unsigned int)param_3 + ((unsigned int)param_1 - (unsigned int)iVar7));
            if ((0 < (int)uVar3) && (uVar5 = (unsigned int)DAT_007091ae, iVar4 < (int)uVar5)) {
                if (iVar4 < 0) {
                    iVar4 = 0;
                }
                uVar2 = uVar3;
                if ((int)uVar5 < (int)uVar3) {
                    uVar2 = uVar5;
                }
                iVar4 = (int)((unsigned int)(int)DAT_006d9980 + (unsigned int)iVar4);
                iVar8 = (int)((unsigned int)(int)DAT_006d9980 + uVar2);
                if (DAT_00525518 == 2) {
                    uVar1 = DAT_007325e0[(unsigned int)param_5];
                    if (iVar4 < iVar8) {
                        puVar9 = (unsigned int *)((unsigned int)DAT_0070acc8 * ((unsigned int)param_2 + (unsigned int)DAT_00732e8c) +
                                                  (unsigned int)DAT_00525534 + (unsigned int)iVar4 * 2u);
                        for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                            puVar9 = puVar9 + 1;
                        }
                        for (uVar5 = (unsigned int)((((unsigned int)iVar8 - (unsigned int)iVar4) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned short *)puVar9 = uVar1;
                            puVar9 = (unsigned int *)((int)puVar9 + 2);
                        }
                    }
                } else if (iVar4 < iVar8) {
                    pbVar10 = (unsigned char *)((unsigned int)DAT_0070acc8 * ((unsigned int)param_2 + (unsigned int)DAT_00732e8c) +
                                                (unsigned int)DAT_00525534 + (unsigned int)iVar4);
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                        pbVar10 = pbVar10 + 4;
                    }
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pbVar10 = param_5;
                        pbVar10 = pbVar10 + 1;
                    }
                }
            }
        }
        if ((int)((unsigned int)param_2 + (unsigned int)param_4) <= (int)(unsigned int)DAT_006d4984) {
            uVar5 = (unsigned int)iVar7 + (unsigned int)param_1;
            if ((0 < (int)uVar5) && (uVar2 = (unsigned int)DAT_007091ae, param_1 < (int)uVar2)) {
                iVar4 = param_1;
                if (param_1 < 0) {
                    iVar4 = 0;
                }
                if ((int)uVar2 < (int)uVar5) {
                    uVar5 = uVar2;
                }
                iVar6 = (int)(uVar5 + (unsigned int)(int)DAT_006d9980);
                iVar4 = (int)((unsigned int)(int)DAT_006d9980 + (unsigned int)iVar4);
                iVar8 = (int)((unsigned int)DAT_00732e8c + (unsigned int)param_4 - 1u + (unsigned int)param_2);
                if (DAT_00525518 == 2) {
                    uVar1 = DAT_007325e0[(unsigned int)param_5];
                    if (iVar4 < iVar6) {
                        puVar9 = (unsigned int *)((unsigned int)DAT_0070acc8 * (unsigned int)iVar8 +
                                                  (unsigned int)DAT_00525534 + (unsigned int)iVar4 * 2u);
                        for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                            puVar9 = puVar9 + 1;
                        }
                        for (uVar5 = (unsigned int)((((unsigned int)iVar6 - (unsigned int)iVar4) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned short *)puVar9 = uVar1;
                            puVar9 = (unsigned int *)((int)puVar9 + 2);
                        }
                    }
                } else if (iVar4 < iVar6) {
                    pbVar10 = (unsigned char *)((unsigned int)DAT_0070acc8 * (unsigned int)iVar8 +
                                                (unsigned int)DAT_00525534 + (unsigned int)iVar4);
                    for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                        pbVar10 = pbVar10 + 4;
                    }
                    for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pbVar10 = param_5;
                        pbVar10 = pbVar10 + 1;
                    }
                }
            }
            iVar7 = (int)((unsigned int)param_3 + ((unsigned int)param_1 - (unsigned int)iVar7));
            if ((0 < (int)uVar3) && (uVar5 = (unsigned int)DAT_007091ae, iVar7 < (int)uVar5)) {
                if (iVar7 < 0) {
                    iVar7 = 0;
                }
                uVar2 = uVar3;
                if ((int)uVar5 < (int)uVar3) {
                    uVar2 = uVar5;
                }
                iVar8 = (int)(uVar2 + (unsigned int)(int)DAT_006d9980);
                iVar7 = (int)((unsigned int)(int)DAT_006d9980 + (unsigned int)iVar7);
                iVar4 = (int)((unsigned int)DAT_00732e8c + (unsigned int)param_4 - 1u + (unsigned int)param_2);
                if (DAT_00525518 == 2) {
                    uVar1 = DAT_007325e0[(unsigned int)param_5];
                    if (iVar7 < iVar8) {
                        puVar9 = (unsigned int *)((unsigned int)iVar4 * (unsigned int)DAT_0070acc8 +
                                                  (unsigned int)DAT_00525534 + (unsigned int)iVar7 * 2u);
                        for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar7) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                            puVar9 = puVar9 + 1;
                        }
                        for (uVar5 = (unsigned int)((((unsigned int)iVar8 - (unsigned int)iVar7) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned short *)puVar9 = uVar1;
                            puVar9 = (unsigned int *)((int)puVar9 + 2);
                        }
                    }
                } else if (iVar7 < iVar8) {
                    pbVar10 = (unsigned char *)((unsigned int)iVar4 * (unsigned int)DAT_0070acc8 +
                                                (unsigned int)DAT_00525534 + (unsigned int)iVar7);
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar7) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                        pbVar10 = pbVar10 + 4;
                    }
                    for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar7) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *pbVar10 = param_5;
                        pbVar10 = pbVar10 + 1;
                    }
                }
            }
        }
        iVar7 = 1;
        if (1 < local_8) {
            do {
                if ((-1 < (int)((unsigned int)param_2 + (unsigned int)iVar7)) &&
                    ((int)((unsigned int)param_2 + (unsigned int)iVar7) < (int)(unsigned int)DAT_006d4984)) {
                    if (-1 < param_1) {
                        iVar8 = (int)((unsigned int)param_1 + (unsigned int)(int)DAT_006d9980);
                        iVar6 = (int)((unsigned int)DAT_00732e8c + (unsigned int)iVar7 + (unsigned int)param_2);
                        iVar4 = (int)((unsigned int)iVar8 + 1u);
                        if (DAT_00525518 == 2) {
                            uVar1 = DAT_007325e0[(unsigned int)param_5];
                            if (iVar8 < iVar4) {
                                puVar9 = (unsigned int *)((unsigned int)iVar6 * (unsigned int)DAT_0070acc8 +
                                                          (unsigned int)DAT_00525534 + (unsigned int)iVar8 * 2u);
                                for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar8) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                                    puVar9 = puVar9 + 1;
                                }
                                for (uVar5 = (unsigned int)((((unsigned int)iVar4 - (unsigned int)iVar8) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *(unsigned short *)puVar9 = uVar1;
                                    puVar9 = (unsigned int *)((int)puVar9 + 2);
                                }
                            }
                        } else if (iVar8 < iVar4) {
                            pbVar10 = (unsigned char *)((unsigned int)iVar6 * (unsigned int)DAT_0070acc8 +
                                                        (unsigned int)DAT_00525534 + (unsigned int)iVar8);
                            for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar8) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                                pbVar10 = pbVar10 + 4;
                            }
                            for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar8) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *pbVar10 = param_5;
                                pbVar10 = pbVar10 + 1;
                            }
                        }
                    }
                    if ((int)uVar3 <= (int)(unsigned int)DAT_007091ae) {
                        iVar6 = (int)((unsigned int)DAT_00732e8c + (unsigned int)iVar7 + (unsigned int)param_2);
                        iVar8 = (int)((unsigned int)param_3 + (unsigned int)(int)DAT_006d9980 + (unsigned int)param_1);
                        iVar4 = (int)((unsigned int)iVar8 - 1u);
                        if (DAT_00525518 == 2) {
                            uVar1 = DAT_007325e0[(unsigned int)param_5];
                            if (iVar4 < iVar8) {
                                puVar9 = (unsigned int *)((unsigned int)iVar6 * (unsigned int)DAT_0070acc8 +
                                                          (unsigned int)DAT_00525534 + (unsigned int)iVar4 * 2u);
                                for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                                    puVar9 = puVar9 + 1;
                                }
                                for (uVar5 = (unsigned int)((((unsigned int)iVar8 - (unsigned int)iVar4) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                                    *(unsigned short *)puVar9 = uVar1;
                                    puVar9 = (unsigned int *)((int)puVar9 + 2);
                                }
                            }
                        } else if (iVar4 < iVar8) {
                            pbVar10 = (unsigned char *)((unsigned int)iVar6 * (unsigned int)DAT_0070acc8 +
                                                        (unsigned int)DAT_00525534 + (unsigned int)iVar4);
                            for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                                pbVar10 = pbVar10 + 4;
                            }
                            for (uVar5 = ((unsigned int)iVar8 - (unsigned int)iVar4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *pbVar10 = param_5;
                                pbVar10 = pbVar10 + 1;
                            }
                        }
                    }
                }
                iVar7 = iVar7 + 1;
            } while (iVar7 < local_8);
        }
        for (iVar7 = param_4 - local_8; iVar7 < param_4 + -1; iVar7 = iVar7 + 1) {
            if (((local_8 <= iVar7) && (-1 < (int)((unsigned int)param_2 + (unsigned int)iVar7))) &&
                ((int)((unsigned int)param_2 + (unsigned int)iVar7) < (int)(unsigned int)DAT_006d4984)) {
                if (-1 < param_1) {
                    iVar6 = (int)((unsigned int)param_1 + (unsigned int)(int)DAT_006d9980);
                    iVar8 = (int)((unsigned int)DAT_00732e8c + (unsigned int)iVar7 + (unsigned int)param_2);
                    iVar4 = (int)((unsigned int)iVar6 + 1u);
                    if (DAT_00525518 == 2) {
                        uVar1 = DAT_007325e0[(unsigned int)param_5];
                        if (iVar6 < iVar4) {
                            puVar9 = (unsigned int *)((unsigned int)iVar8 * (unsigned int)DAT_0070acc8 +
                                                      (unsigned int)DAT_00525534 + (unsigned int)iVar6 * 2u);
                            for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar6) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                                puVar9 = puVar9 + 1;
                            }
                            for (uVar5 = (unsigned int)((((unsigned int)iVar4 - (unsigned int)iVar6) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned short *)puVar9 = uVar1;
                                puVar9 = (unsigned int *)((int)puVar9 + 2);
                            }
                        }
                    } else if (iVar6 < iVar4) {
                        pbVar10 = (unsigned char *)((unsigned int)iVar8 * (unsigned int)DAT_0070acc8 +
                                                    (unsigned int)DAT_00525534 + (unsigned int)iVar6);
                        for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar6) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                            pbVar10 = pbVar10 + 4;
                        }
                        for (uVar5 = ((unsigned int)iVar4 - (unsigned int)iVar6) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *pbVar10 = param_5;
                            pbVar10 = pbVar10 + 1;
                        }
                    }
                }
                if ((int)uVar3 <= (int)(unsigned int)DAT_007091ae) {
                    iVar6 = (int)((unsigned int)param_3 + (unsigned int)(int)DAT_006d9980 + (unsigned int)param_1);
                    iVar8 = (int)((unsigned int)DAT_00732e8c + (unsigned int)iVar7 + (unsigned int)param_2);
                    iVar4 = (int)((unsigned int)iVar6 - 1u);
                    if (DAT_00525518 == 2) {
                        uVar1 = DAT_007325e0[(unsigned int)param_5];
                        if (iVar4 < iVar6) {
                            puVar9 = (unsigned int *)((unsigned int)iVar8 * (unsigned int)DAT_0070acc8 +
                                                      (unsigned int)DAT_00525534 + (unsigned int)iVar4 * 2u);
                            for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) >> 1; uVar5 != 0; uVar5 = uVar5 - 1) {
                                *puVar9 = ((unsigned int)uVar1 << 16) | (unsigned int)uVar1;
                                puVar9 = puVar9 + 1;
                            }
                            for (uVar5 = (unsigned int)((((unsigned int)iVar6 - (unsigned int)iVar4) & 1U) != 0); uVar5 != 0; uVar5 = uVar5 - 1) {
                                *(unsigned short *)puVar9 = uVar1;
                                puVar9 = (unsigned int *)((int)puVar9 + 2);
                            }
                        }
                    } else if (iVar4 < iVar6) {
                        pbVar10 = (unsigned char *)((unsigned int)iVar8 * (unsigned int)DAT_0070acc8 +
                                                    (unsigned int)DAT_00525534 + (unsigned int)iVar4);
                        for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *(unsigned int *)pbVar10 = (unsigned int)param_5 * 0x01010101u;
                            pbVar10 = pbVar10 + 4;
                        }
                        for (uVar5 = ((unsigned int)iVar6 - (unsigned int)iVar4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                            *pbVar10 = param_5;
                            pbVar10 = pbVar10 + 1;
                        }
                    }
                }
            }
        }
        FUN_004a6f90();
    }
    return;
}
