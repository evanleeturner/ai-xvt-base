/* SPDX-License-Identifier: MIT */
/* z_bop 00505cd0 */
extern int DAT_00aa6cfc;
extern int DAT_00aa7b13;   /* packed: odd address */
extern int DAT_00aa7b82;   /* packed: address % 4 == 2 */
extern int DAT_00aa7b86;   /* packed */
extern int DAT_00aa7b8a;   /* packed */
extern int DAT_00aa7b8e;   /* packed */

void __cdecl FUN_005063b0(int param_1, int param_2, int param_3, unsigned short param_4);
void __cdecl FUN_00506550(int param_1, int param_2, int param_3, unsigned short param_4);

void __cdecl FUN_00505cd0(int param_1, int param_2, int param_3, int param_4, unsigned short param_5)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned char *puVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned char *puVar7;
    int local_9;
    unsigned int local_8;
    int g_b82, g_b86, g_b8a, g_b8e, g_b13;
    unsigned int fill;

    iVar5 = 0x800;
    if (param_4 == param_2) {
        FUN_005063b0(param_1, param_3, param_2, param_5);
        return;
    }
    if (param_3 == param_1) {
        FUN_00506550(param_2, param_4, param_1, param_5);
        return;
    }

    __builtin_memcpy(&g_b82, &DAT_00aa7b82, 4);
    __builtin_memcpy(&g_b86, &DAT_00aa7b86, 4);
    __builtin_memcpy(&g_b8a, &DAT_00aa7b8a, 4);
    __builtin_memcpy(&g_b8e, &DAT_00aa7b8e, 4);
    __builtin_memcpy(&g_b13, &DAT_00aa7b13, 4);
    fill = (unsigned int)(unsigned char)param_5 * 0x01010101u;

    if (param_2 < param_4) {
        if (param_1 < param_3) {
            local_8 = (unsigned int)((int)((((unsigned int)param_3 - (unsigned int)param_1 + 1u) * 0x1000u)) /
                                     (int)(((unsigned int)param_4 - (unsigned int)param_2 + 1u)));
            local_9 = param_1 < g_b82;
            if (local_9) {
                param_2 = (int)((unsigned int)param_2 +
                          (unsigned int)((int)(((unsigned int)g_b82 - (unsigned int)param_1) * 0x1000u) / (int)local_8));
                param_1 = g_b82;
            }
            iVar4 = param_2;
            if (param_2 < g_b8a) {
                param_1 = (int)((unsigned int)param_1 +
                          (unsigned int)((int)(((unsigned int)g_b8a - (unsigned int)param_2) * local_8) >> 0xc));
                iVar4 = g_b8a;
            }
            local_9 = (param_2 < g_b8a) || local_9;
            if (g_b86 < param_3) {
                local_9 = 1;
                param_4 = (int)((unsigned int)param_4 +
                          (unsigned int)((int)(((unsigned int)g_b86 - (unsigned int)param_3) * 0x1000u) / (int)local_8));
                param_3 = g_b86;
            }
            if (g_b8e < param_4) {
                local_9 = 1;
                param_3 = (int)((unsigned int)param_3 -
                          (unsigned int)((int)(((unsigned int)param_4 - (unsigned int)g_b8e) * local_8) >> 0xc));
                param_4 = g_b8e;
            }
            if (local_9) {
                if (param_3 < param_1) {
                    return;
                }
                if (param_4 < iVar4) {
                    return;
                }
                local_8 = (unsigned int)((int)((((unsigned int)param_3 - (unsigned int)param_1 + 1u) * 0x1000u)) /
                                         (int)(((unsigned int)param_4 - (unsigned int)iVar4 + 1u)));
            }
            uVar1 = (unsigned int)((int)local_8 >> 0xc);
            puVar3 = (unsigned char *)(int)((unsigned int)g_b13 * (unsigned int)iVar4 +
                                            (unsigned int)DAT_00aa6cfc + (unsigned int)param_1);
            if (uVar1 == 0) {
                iVar4 = (int)((unsigned int)param_4 - (unsigned int)iVar4 + 1u);
                if (iVar4 < 1) {
                    return;
                }
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    *puVar3 = (unsigned char)param_5;
                    if (0xfff < iVar5) {
                        iVar5 = iVar5 - 0x1000;
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = puVar3 + g_b13;
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
            iVar4 = (int)((unsigned int)param_4 - (unsigned int)iVar4 + 1u);
            if (iVar4 < 1) {
                return;
            }
            do {
                iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                uVar6 = uVar1;
                if (0xfff < iVar5) {
                    uVar6 = uVar1 + 1;
                    iVar5 = iVar5 - 0x1000;
                }
                puVar7 = puVar3;
                for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                    __builtin_memcpy(puVar7, &fill, 4);
                    puVar7 = puVar7 + 4;
                }
                for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                    *puVar7 = (unsigned char)param_5;
                    puVar7 = puVar7 + 1;
                }
                puVar3 = puVar3 + g_b13 + uVar6;
                iVar4 = iVar4 - 1;
            } while (iVar4 != 0);
            return;
        }
        if ((param_2 < param_4) && (param_3 < param_1)) {
            local_8 = (unsigned int)((int)((((unsigned int)param_1 - (unsigned int)param_3 + 1u) * 0x1000u)) /
                                     (int)(((unsigned int)param_4 - (unsigned int)param_2 + 1u)));
            local_9 = g_b86 < param_1;
            if (local_9) {
                param_2 = (int)((unsigned int)param_2 +
                          (unsigned int)((int)(((unsigned int)param_1 - (unsigned int)g_b86) * 0x1000u) / (int)local_8));
                param_1 = g_b86;
            }
            iVar4 = param_2;
            if (param_2 < g_b8a) {
                param_1 = (int)((unsigned int)param_1 -
                          (unsigned int)((int)(((unsigned int)g_b8a - (unsigned int)param_2) * local_8) >> 0xc));
                iVar4 = g_b8a;
            }
            local_9 = (param_2 < g_b8a) || local_9;
            if (param_3 < g_b82) {
                local_9 = 1;
                param_4 = (int)((unsigned int)param_4 +
                          (unsigned int)((int)(((unsigned int)param_3 - (unsigned int)g_b82) * 0x1000u) / (int)local_8));
                param_3 = g_b82;
            }
            if (g_b8e < param_4) {
                local_9 = 1;
                param_3 = (int)((unsigned int)param_3 +
                          (unsigned int)((int)(((unsigned int)param_4 - (unsigned int)g_b8e) * local_8) >> 0xc));
                param_4 = g_b8e;
            }
            if (local_9) {
                if (param_1 < param_3) {
                    return;
                }
                if (param_4 < iVar4) {
                    return;
                }
                local_8 = (unsigned int)((int)((((unsigned int)param_1 - (unsigned int)param_3 + 1u) * 0x1000u)) /
                                         (int)(((unsigned int)param_4 - (unsigned int)iVar4 + 1u)));
            }
            uVar1 = (unsigned int)((int)local_8 >> 0xc);
            puVar3 = (unsigned char *)(int)((unsigned int)g_b13 * (unsigned int)param_4 +
                                            (unsigned int)param_3 + (unsigned int)DAT_00aa6cfc);
            if (uVar1 == 0) {
                iVar4 = (int)((unsigned int)param_4 - (unsigned int)iVar4 + 1u);
                if (iVar4 < 1) {
                    return;
                }
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    *puVar3 = (unsigned char)param_5;
                    if (0xfff < iVar5) {
                        iVar5 = iVar5 - 0x1000;
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = puVar3 - g_b13;
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
            iVar4 = (int)((unsigned int)param_4 - (unsigned int)iVar4 + 1u);
            if (iVar4 < 1) {
                return;
            }
            do {
                iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                uVar6 = uVar1;
                if (0xfff < iVar5) {
                    uVar6 = uVar1 + 1;
                    iVar5 = iVar5 - 0x1000;
                }
                puVar7 = puVar3;
                for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                    __builtin_memcpy(puVar7, &fill, 4);
                    puVar7 = puVar7 + 4;
                }
                for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                    *puVar7 = (unsigned char)param_5;
                    puVar7 = puVar7 + 1;
                }
                puVar3 = puVar3 + (int)(uVar6 - (unsigned int)g_b13);
                iVar4 = iVar4 - 1;
            } while (iVar4 != 0);
            return;
        }
    }

    if ((param_4 < param_2) && (param_1 < param_3)) {
        local_8 = (unsigned int)((int)((((unsigned int)param_3 - (unsigned int)param_1 + 1u) * 0x1000u)) /
                                 (int)(((unsigned int)param_2 - (unsigned int)param_4 + 1u)));
        local_9 = param_1 < g_b82;
        if (local_9) {
            param_2 = (int)((unsigned int)param_2 +
                      (unsigned int)((int)(((unsigned int)param_1 - (unsigned int)g_b82) * 0x1000u) / (int)local_8));
            param_1 = g_b82;
        }
        iVar4 = param_2;
        if (g_b8e < param_2) {
            param_1 = (int)((unsigned int)param_1 +
                      (unsigned int)((int)(((unsigned int)param_2 - (unsigned int)g_b8e) * local_8) >> 0xc));
            iVar4 = g_b8e;
        }
        local_9 = (g_b8e < param_2) || local_9;
        if (g_b86 < param_3) {
            local_9 = 1;
            param_4 = (int)((unsigned int)param_4 +
                      (unsigned int)((int)(((unsigned int)param_3 - (unsigned int)g_b86) * 0x1000u) / (int)local_8));
            param_3 = g_b86;
        }
        if (param_4 < g_b8a) {
            local_9 = 1;
            param_3 = (int)((unsigned int)param_3 -
                      (unsigned int)((int)(((unsigned int)g_b8a - (unsigned int)param_4) * local_8) >> 0xc));
            param_4 = g_b8a;
        }
        if (local_9) {
            if (param_3 < param_1) {
                return;
            }
            if (iVar4 < param_4) {
                return;
            }
            local_8 = (unsigned int)((int)((((unsigned int)param_3 - (unsigned int)param_1 + 1u) * 0x1000u)) /
                                     (int)(((unsigned int)iVar4 - (unsigned int)param_4 + 1u)));
        }
        uVar1 = (unsigned int)((int)local_8 >> 0xc);
        puVar3 = (unsigned char *)(int)((unsigned int)g_b13 * (unsigned int)iVar4 +
                                        (unsigned int)DAT_00aa6cfc + (unsigned int)param_1);
        if (uVar1 == 0) {
            iVar4 = (int)((unsigned int)iVar4 - (unsigned int)param_4 + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    *puVar3 = (unsigned char)param_5;
                    if (0xfff < iVar5) {
                        iVar5 = iVar5 - 0x1000;
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = puVar3 - g_b13;
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        } else {
            iVar4 = (int)((unsigned int)iVar4 - (unsigned int)param_4 + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    uVar6 = uVar1;
                    if (0xfff < iVar5) {
                        uVar6 = uVar1 + 1;
                        iVar5 = iVar5 - 0x1000;
                    }
                    puVar7 = puVar3;
                    for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                        __builtin_memcpy(puVar7, &fill, 4);
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar7 = (unsigned char)param_5;
                        puVar7 = puVar7 + 1;
                    }
                    puVar3 = puVar3 + (int)(uVar6 - (unsigned int)g_b13);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        }
    } else {
        local_8 = (unsigned int)((int)((((unsigned int)param_1 - (unsigned int)param_3 + 1u) * 0x1000u)) /
                                 (int)(((unsigned int)param_2 - (unsigned int)param_4 + 1u)));
        local_9 = g_b86 < param_1;
        if (local_9) {
            param_2 = (int)((unsigned int)param_2 +
                      (unsigned int)((int)(((unsigned int)g_b86 - (unsigned int)param_1) * 0x1000u) / (int)local_8));
            param_1 = g_b86;
        }
        iVar4 = param_2;
        if (g_b8e < param_2) {
            param_1 = (int)((unsigned int)param_1 -
                      (unsigned int)((int)(((unsigned int)param_2 - (unsigned int)g_b8e) * local_8) >> 0xc));
            iVar4 = g_b8e;
        }
        local_9 = (g_b8e < param_2) || local_9;
        if (param_3 < g_b82) {
            local_9 = 1;
            param_4 = (int)((unsigned int)param_4 +
                      (unsigned int)((int)(((unsigned int)g_b82 - (unsigned int)param_3) * 0x1000u) / (int)local_8));
            param_3 = g_b82;
        }
        if (param_4 < g_b8a) {
            local_9 = 1;
            param_3 = (int)((unsigned int)param_3 +
                      (unsigned int)((int)(((unsigned int)g_b8a - (unsigned int)param_4) * local_8) >> 0xc));
            param_4 = g_b8a;
        }
        if (local_9) {
            if (param_1 < param_3) {
                return;
            }
            if (iVar4 < param_4) {
                return;
            }
            local_8 = (unsigned int)((int)((((unsigned int)param_1 - (unsigned int)param_3 + 1u) * 0x1000u)) /
                                     (int)(((unsigned int)iVar4 - (unsigned int)param_4 + 1u)));
        }
        uVar1 = (unsigned int)((int)local_8 >> 0xc);
        puVar3 = (unsigned char *)(int)((unsigned int)g_b13 * (unsigned int)param_4 +
                                        (unsigned int)param_3 + (unsigned int)DAT_00aa6cfc);
        if (uVar1 == 0) {
            iVar4 = (int)((unsigned int)iVar4 - (unsigned int)param_4 + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    *puVar3 = (unsigned char)param_5;
                    if (0xfff < iVar5) {
                        iVar5 = iVar5 - 0x1000;
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = puVar3 + g_b13;
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
            }
        } else {
            iVar4 = (int)((unsigned int)iVar4 - (unsigned int)param_4 + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned int)iVar5 + (local_8 & 0xfff));
                    uVar6 = uVar1;
                    if (0xfff < iVar5) {
                        uVar6 = uVar1 + 1;
                        iVar5 = iVar5 - 0x1000;
                    }
                    puVar7 = puVar3;
                    for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                        __builtin_memcpy(puVar7, &fill, 4);
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar7 = (unsigned char)param_5;
                        puVar7 = puVar7 + 1;
                    }
                    puVar3 = puVar3 + g_b13 + uVar6;
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        }
    }
    return;
}
