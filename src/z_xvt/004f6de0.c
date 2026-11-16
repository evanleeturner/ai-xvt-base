/* SPDX-License-Identifier: MIT */
/* z_xvt 004f6de0 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
void __cdecl FUN_004f7a80(int param_1, int param_2, int param_3, int param_4, unsigned short param_5);
void __cdecl FUN_004f8160(int param_1, int param_2, int param_3, unsigned short param_4);

extern int DAT_00758830;
extern int DAT_0075964b;   /* unaligned */
extern int DAT_00759657;   /* unaligned */
extern int DAT_007596b7;   /* unaligned */
extern int DAT_007596bb;   /* unaligned */
extern int DAT_007596bf;   /* unaligned */
extern int DAT_007596c3;   /* unaligned */

void __cdecl
FUN_004f6de0(int param_1, int param_2, int param_3, int param_4, unsigned short param_5, int param_6)
{
    int bVar1;
    unsigned char uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned char bVar9;
    int iVar10;
    unsigned int *puVar11;
    unsigned int *puVar12;
    int local_40;
    int local_3c;
    int local_38;
    int local_34;
    int local_30;
    int local_2c;
    int local_14;
    int local_10;
    unsigned int local_c;
    int local_5;
    int local_44;
    int local_48;
    int local_4c;
    int local_50;
    int local_54;
    int local_58;

    local_10 = 0x800;
    if (param_6 == 1) {
        FUN_004f7a80(param_1, param_2, param_3, param_4, param_5);
        return;
    }
    __builtin_memcpy(&local_48, &DAT_00759657, 4);
    __builtin_memcpy(&local_44, &DAT_0075964b, 4);
    __builtin_memcpy(&local_4c, &DAT_007596b7, 4);
    __builtin_memcpy(&local_50, &DAT_007596bb, 4);
    __builtin_memcpy(&local_54, &DAT_007596bf, 4);
    __builtin_memcpy(&local_58, &DAT_007596c3, 4);

    iVar10 = local_48 >> 4;
    iVar5 = param_6 >> 1;
    iVar3 = (param_6 - 1) >> 1;
    bVar9 = (unsigned char)iVar10;
    uVar2 = (unsigned char)param_5;

    if (param_4 == param_2) {
        if ((local_54 <= param_2) && (param_2 <= local_58)) {
            if (param_3 < param_1) {
                if (param_3 < local_4c) {
                    param_3 = local_4c;
                }
                if (local_50 < param_1) {
                    param_1 = local_50;
                }
                param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar5);
                param_4 = (int)((unsigned int)param_4 + (unsigned int)iVar3);
                if (local_58 < param_4) {
                    param_4 = local_58;
                }
                if (param_2 < local_54) {
                    param_2 = local_54;
                }
                puVar11 = (unsigned int *)((unsigned int)DAT_00758830 + ((unsigned int)param_3 << (bVar9 & 0x1f)) + (unsigned int)param_2 * (unsigned int)local_44);
                uVar8 = (unsigned int)(param_1 - param_3) + 1;
                if (param_2 <= param_4) {
                    local_3c = (param_4 - param_2) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar6 = uVar8 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar6 = uVar8, puVar12 = puVar11, 0 < (int)uVar8)) {
                            for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_3c = local_3c - 1;
                    } while (local_3c != 0);
                    return;
                }
            } else {
                if (local_50 < param_3) {
                    param_3 = local_50;
                }
                if (param_1 < local_4c) {
                    param_1 = local_4c;
                }
                param_2 = (int)((unsigned int)param_2 - (unsigned int)iVar5);
                param_4 = (int)((unsigned int)param_4 + (unsigned int)iVar3);
                if (local_58 < param_4) {
                    param_4 = local_58;
                }
                if (param_2 < local_54) {
                    param_2 = local_54;
                }
                puVar11 = (unsigned int *)((unsigned int)DAT_00758830 + ((unsigned int)param_1 << (bVar9 & 0x1f)) + (unsigned int)param_2 * (unsigned int)local_44);
                uVar8 = (unsigned int)(param_3 - param_1) + 1;
                if (param_2 <= param_4) {
                    local_40 = (param_4 - param_2) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar6 = uVar8 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar6 = uVar8, puVar12 = puVar11, 0 < (int)uVar8)) {
                            for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_40 = local_40 - 1;
                    } while (local_40 != 0);
                    return;
                }
            }
        }
    }
    else if ((param_2 < param_4) && (param_1 <= param_3)) {
        local_c = (unsigned int)((int)(((unsigned int)(param_3 - param_1) + 1) * 0x1000u) / ((param_4 - param_2) + 1));
        bVar1 = param_1 < local_4c;
        if (bVar1) {
            iVar4 = local_4c - param_1;
            param_1 = local_4c;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_2 = (int)((unsigned int)param_2 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        local_5 = param_2 < local_54;
        if (local_5) {
            param_1 = (int)((unsigned int)param_1 + (unsigned int)((int)((unsigned int)(local_54 - param_2) * local_c) >> 0xc));
            param_2 = local_54;
        }
        local_5 = local_5 || bVar1;
        if (local_50 < param_3) {
            local_5 = 1;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_4 = (int)((unsigned int)param_4 + (unsigned int)((int)((unsigned int)(local_50 - param_3) * 0x1000u) / (int)local_c));
            param_3 = local_50;
        }
        if (local_58 < param_4) {
            iVar4 = param_4 - local_58;
            param_4 = local_58;
            local_5 = 1;
            param_3 = (int)((unsigned int)param_3 - (unsigned int)((int)((unsigned int)iVar4 * local_c) >> 0xc));
        }
        if (local_5) {
            if (param_3 < param_1) {
                return;
            }
            if (param_4 < param_2) {
                return;
            }
            local_c = (unsigned int)((int)(((unsigned int)(param_3 - param_1) + 1) * 0x1000u) / ((param_4 - param_2) + 1));
        }
        uVar8 = (unsigned int)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_2 <= param_4) {
                do {
                    FUN_004f8160(param_1 - iVar5, iVar3 + param_1, param_2, param_5);
                    local_10 = local_10 + (int)(local_c & 0xfff);
                    if (0xfff < local_10) {
                        local_10 = local_10 - 0x1000;
                        param_1 = param_1 + 1;
                    }
                    param_2 = param_2 + 1;
                } while (param_2 <= param_4);
                return;
            }
        } else if (param_2 <= param_4) {
            do {
                local_10 = local_10 + (int)(local_c & 0xfff);
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    uVar6 = uVar8 + 1;
                    local_10 = local_10 - 0x1000;
                }
                local_14 = param_2 - iVar5;
                iVar4 = iVar3 + param_2;
                if (local_58 < iVar3 + param_2) {
                    iVar4 = local_58;
                }
                if (local_14 < local_54) {
                    local_14 = local_54;
                }
                puVar11 = (unsigned int *)((unsigned int)local_14 * (unsigned int)local_44 + ((unsigned int)param_1 << (bVar9 & 0x1f)) + (unsigned int)DAT_00758830);
                if (local_14 <= iVar4) {
                    local_38 = (iVar4 - local_14) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_38 = local_38 - 1;
                    } while (local_38 != 0);
                }
                param_1 = (int)((unsigned int)param_1 + uVar6);
                param_2 = param_2 + 1;
            } while (param_2 <= param_4);
            return;
        }
    }
    else if ((param_2 < param_4) && (param_3 <= param_1)) {
        local_c = (unsigned int)((int)(((unsigned int)(param_1 - param_3) + 1) * 0x1000u) / ((param_4 - param_2) + 1));
        bVar1 = local_50 < param_1;
        if (bVar1) {
            iVar4 = param_1 - local_50;
            param_1 = local_50;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_2 = (int)((unsigned int)param_2 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        local_5 = param_2 < local_54;
        if (local_5) {
            param_1 = (int)((unsigned int)param_1 - (unsigned int)((int)((unsigned int)(local_54 - param_2) * local_c) >> 0xc));
            param_2 = local_54;
        }
        local_5 = local_5 || bVar1;
        if (param_3 < local_4c) {
            iVar4 = param_3 - local_4c;
            param_3 = local_4c;
            local_5 = 1;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_4 = (int)((unsigned int)param_4 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        if (local_58 < param_4) {
            iVar4 = param_4 - local_58;
            param_4 = local_58;
            local_5 = 1;
            param_3 = (int)((unsigned int)param_3 + (unsigned int)((int)((unsigned int)iVar4 * local_c) >> 0xc));
        }
        if (local_5) {
            if (param_1 < param_3) {
                return;
            }
            if (param_4 < param_2) {
                return;
            }
            local_c = (unsigned int)((int)(((unsigned int)(param_1 - param_3) + 1) * 0x1000u) / ((param_4 - param_2) + 1));
        }
        uVar8 = (unsigned int)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_2 <= param_4) {
                do {
                    FUN_004f8160(param_1 - iVar5, iVar3 + param_1, param_2, param_5);
                    local_10 = local_10 + (int)(local_c & 0xfff);
                    if (0xfff < local_10) {
                        local_10 = local_10 - 0x1000;
                        param_1 = param_1 - 1;
                    }
                    param_2 = param_2 + 1;
                } while (param_2 <= param_4);
                return;
            }
        } else if (param_2 <= param_4) {
            do {
                local_10 = local_10 + (int)(local_c & 0xfff);
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    uVar6 = uVar8 + 1;
                    local_10 = local_10 - 0x1000;
                }
                local_14 = param_4 - iVar5;
                iVar4 = iVar3 + param_4;
                if (local_58 < iVar3 + param_4) {
                    iVar4 = local_58;
                }
                if (local_14 < local_54) {
                    local_14 = local_54;
                }
                puVar11 = (unsigned int *)(((unsigned int)param_3 << (bVar9 & 0x1f)) + (unsigned int)local_14 * (unsigned int)local_44 + (unsigned int)DAT_00758830);
                if (local_14 <= iVar4) {
                    local_34 = (iVar4 - local_14) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_34 = local_34 - 1;
                    } while (local_34 != 0);
                }
                param_3 = (int)((unsigned int)param_3 + uVar6);
                param_4 = param_4 - 1;
            } while (param_2 <= param_4);
            return;
        }
    }
    else if ((param_4 < param_2) && (param_1 <= param_3)) {
        local_c = (unsigned int)((int)(((unsigned int)(param_3 - param_1) + 1) * 0x1000u) / ((param_2 - param_4) + 1));
        bVar1 = param_1 < local_4c;
        if (bVar1) {
            iVar4 = param_1 - local_4c;
            param_1 = local_4c;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_2 = (int)((unsigned int)param_2 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        local_5 = local_58 < param_2;
        if (local_5) {
            iVar4 = param_2 - local_58;
            param_2 = local_58;
            param_1 = (int)((unsigned int)param_1 + (unsigned int)((int)((unsigned int)iVar4 * local_c) >> 0xc));
        }
        local_5 = local_5 || bVar1;
        if (local_50 < param_3) {
            iVar4 = param_3 - local_50;
            param_3 = local_50;
            local_5 = 1;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_4 = (int)((unsigned int)param_4 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        if (param_4 < local_54) {
            local_5 = 1;
            param_3 = (int)((unsigned int)param_3 - (unsigned int)((int)((unsigned int)(local_54 - param_4) * local_c) >> 0xc));
            param_4 = local_54;
        }
        if (local_5) {
            if (param_3 < param_1) {
                return;
            }
            if (param_2 < param_4) {
                return;
            }
            local_c = (unsigned int)((int)(((unsigned int)(param_3 - param_1) + 1) * 0x1000u) / ((param_2 - param_4) + 1));
        }
        uVar8 = (unsigned int)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_4 <= param_2) {
                do {
                    FUN_004f8160(param_1 - iVar5, iVar3 + param_1, param_2, param_5);
                    local_10 = local_10 + (int)(local_c & 0xfff);
                    if (0xfff < local_10) {
                        local_10 = local_10 - 0x1000;
                        param_1 = param_1 + 1;
                    }
                    param_2 = param_2 - 1;
                } while (param_4 <= param_2);
                return;
            }
        } else if (param_4 <= param_2) {
            do {
                local_10 = local_10 + (int)(local_c & 0xfff);
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    uVar6 = uVar8 + 1;
                    local_10 = local_10 - 0x1000;
                }
                local_14 = param_2 - iVar5;
                iVar4 = iVar3 + param_2;
                if (local_58 < iVar3 + param_2) {
                    iVar4 = local_58;
                }
                if (local_14 < local_54) {
                    local_14 = local_54;
                }
                puVar11 = (unsigned int *)((unsigned int)local_14 * (unsigned int)local_44 + ((unsigned int)param_1 << (bVar9 & 0x1f)) + (unsigned int)DAT_00758830);
                if (local_14 <= iVar4) {
                    local_30 = (iVar4 - local_14) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_30 = local_30 - 1;
                    } while (local_30 != 0);
                }
                param_1 = (int)((unsigned int)param_1 + uVar6);
                param_2 = param_2 - 1;
            } while (param_4 <= param_2);
            return;
        }
    }
    else {
        local_c = (unsigned int)((int)(((unsigned int)(param_1 - param_3) + 1) * 0x1000u) / ((param_2 - param_4) + 1));
        bVar1 = local_50 < param_1;
        if (bVar1) {
            iVar4 = local_50 - param_1;
            param_1 = local_50;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_2 = (int)((unsigned int)param_2 + (unsigned int)((int)((unsigned int)iVar4 * 0x1000u) / (int)local_c));
        }
        local_5 = local_58 < param_2;
        if (local_5) {
            iVar4 = param_2 - local_58;
            param_2 = local_58;
            param_1 = (int)((unsigned int)param_1 - (unsigned int)((int)((unsigned int)iVar4 * local_c) >> 0xc));
        }
        local_5 = local_5 || bVar1;
        if (param_3 < local_4c) {
            local_5 = 1;
            if ((int)local_c == 0) RaiseException(0xC0000094UL, 0, 0, NULL);
            param_4 = (int)((unsigned int)param_4 + (unsigned int)((int)((unsigned int)(local_4c - param_3) * 0x1000u) / (int)local_c));
            param_3 = local_4c;
        }
        if (param_4 < local_54) {
            local_5 = 1;
            param_3 = (int)((unsigned int)param_3 + (unsigned int)((int)((unsigned int)(local_54 - param_4) * local_c) >> 0xc));
            param_4 = local_54;
        }
        if (local_5) {
            if (param_1 < param_3) {
                return;
            }
            if (param_2 < param_4) {
                return;
            }
            local_c = (unsigned int)((int)(((unsigned int)(param_1 - param_3) + 1) * 0x1000u) / ((param_2 - param_4) + 1));
        }
        uVar8 = (unsigned int)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            for (; param_4 <= param_2; param_2 = param_2 - 1) {
                FUN_004f8160(param_1 - iVar5, iVar3 + param_1, param_2, param_5);
                local_10 = local_10 + (int)(local_c & 0xfff);
                if (0xfff < local_10) {
                    local_10 = local_10 - 0x1000;
                    param_1 = param_1 - 1;
                }
            }
        } else if (param_4 <= param_2) {
            do {
                local_10 = local_10 + (int)(local_c & 0xfff);
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    uVar6 = uVar8 + 1;
                    local_10 = local_10 - 0x1000;
                }
                local_14 = param_4 - iVar5;
                iVar4 = iVar3 + param_4;
                if (local_58 < iVar3 + param_4) {
                    iVar4 = local_58;
                }
                if (local_14 < local_54) {
                    local_14 = local_54;
                }
                puVar11 = (unsigned int *)(((unsigned int)param_3 << (bVar9 & 0x1f)) + (unsigned int)local_14 * (unsigned int)local_44 + (unsigned int)DAT_00758830);
                if (local_14 <= iVar4) {
                    local_2c = (iVar4 - local_14) + 1;
                    do {
                        if (iVar10 == 0) {
                            puVar12 = puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned int)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned int *)((int)puVar12 + 1);
                            }
                        } else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned int *)((int)puVar12 + 2);
                            }
                        }
                        puVar11 = (unsigned int *)((int)puVar11 + local_44);
                        local_2c = local_2c - 1;
                    } while (local_2c != 0);
                }
                param_3 = (int)((unsigned int)param_3 + uVar6);
                param_4 = param_4 + 1;
            } while (param_4 <= param_2);
            return;
        }
    }
    return;
}
