/* SPDX-License-Identifier: MIT */
/* z_bop 005050f0 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
void __cdecl FUN_00505cd0(int param_1, int param_2, int param_3, int param_4, unsigned short param_5);
void __cdecl FUN_005063b0(int param_1, int param_2, int param_3, unsigned short param_4);

/* Packed globals: none of these addresses is a multiple of 4, so each is read via memcpy. */
extern unsigned char DAT_00aa7b1f[4];
extern unsigned char DAT_00aa7b13[4];
extern unsigned char DAT_00aa7b82[4];
extern unsigned char DAT_00aa7b86[4];
extern unsigned char DAT_00aa7b8a[4];
extern unsigned char DAT_00aa7b8e[4];
extern int DAT_00aa6cfc;

/* 64-bit IDIV of a sign-extended 32-bit dividend: faults on divisor 0 or INT_MIN / -1 */
#define DIV_FAULT(a, b) (((b) == 0) || (((a) == (int)0x80000000) && ((b) == -1)))

void __cdecl
FUN_005050f0(int param_1, int param_2, int param_3, int param_4, unsigned short param_5, int param_6)
{
    int bVar1;
    unsigned char uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    unsigned uVar6;
    unsigned uVar7;
    unsigned uVar8;
    unsigned char bVar9;
    int iVar10;
    unsigned char *puVar11;
    unsigned *puVar12;
    int local_7c;
    int local_6c;
    int local_5c;
    int local_4c;
    int local_3c;
    int local_2c;
    int local_14;
    int local_10;
    unsigned local_c;
    int local_5;
    int dv_a;
    int dv_b;
    int v_b1f;
    int v_b13;
    int v_b82;
    int v_b86;
    int v_b8a;
    int v_b8e;

    local_10 = 0x800;
    if (param_6 == 1) {
        FUN_00505cd0(param_1, param_2, param_3, param_4, param_5);
        return;
    }
    __builtin_memcpy(&v_b1f, DAT_00aa7b1f, 4);
    __builtin_memcpy(&v_b13, DAT_00aa7b13, 4);
    __builtin_memcpy(&v_b82, DAT_00aa7b82, 4);
    __builtin_memcpy(&v_b86, DAT_00aa7b86, 4);
    __builtin_memcpy(&v_b8a, DAT_00aa7b8a, 4);
    __builtin_memcpy(&v_b8e, DAT_00aa7b8e, 4);
    iVar10 = v_b1f >> 4;
    iVar5 = param_6 >> 1;
    iVar3 = (int)((unsigned)param_6 - 1u) >> 1;
    bVar9 = (unsigned char)iVar10;
    uVar2 = (unsigned char)param_5;
    if (param_4 == param_2) {
        if ((v_b8a <= param_2) && (param_2 <= v_b8e)) {
            if (param_3 < param_1) {
                if (param_3 < v_b82) {
                    param_3 = v_b82;
                }
                if (v_b86 < param_1) {
                    param_1 = v_b86;
                }
                param_2 = (int)((unsigned)param_2 - (unsigned)iVar5);
                param_4 = (int)((unsigned)param_4 + (unsigned)iVar3);
                if (v_b8e < param_4) {
                    param_4 = v_b8e;
                }
                if (param_2 < v_b8a) {
                    param_2 = v_b8a;
                }
                puVar11 = (unsigned char *)((unsigned)DAT_00aa6cfc + (unsigned)v_b13 * (unsigned)param_2
                                            + ((unsigned)param_3 << (bVar9 & 0x1f)));
                uVar8 = ((unsigned)param_1 - (unsigned)param_3) + 1u;
                if (param_2 <= param_4) {
                    local_6c = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar6 = uVar8 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar6 = uVar8, puVar12 = (unsigned *)puVar11, 0 < (int)uVar8)) {
                            for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_6c = (int)((unsigned)local_6c - 1u);
                    } while (local_6c != 0);
                    return;
                }
            } else {
                if (v_b86 < param_3) {
                    param_3 = v_b86;
                }
                if (param_1 < v_b82) {
                    param_1 = v_b82;
                }
                param_2 = (int)((unsigned)param_2 - (unsigned)iVar5);
                param_4 = (int)((unsigned)param_4 + (unsigned)iVar3);
                if (v_b8e < param_4) {
                    param_4 = v_b8e;
                }
                if (param_2 < v_b8a) {
                    param_2 = v_b8a;
                }
                puVar11 = (unsigned char *)((unsigned)DAT_00aa6cfc + (unsigned)v_b13 * (unsigned)param_2
                                            + ((unsigned)param_1 << (bVar9 & 0x1f)));
                uVar8 = ((unsigned)param_3 - (unsigned)param_1) + 1u;
                if (param_2 <= param_4) {
                    local_7c = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar6 = uVar8 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar6 = uVar8 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar6 = uVar8, puVar12 = (unsigned *)puVar11, 0 < (int)uVar8)) {
                            for (; uVar6 != 0; uVar6 = uVar6 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_7c = (int)((unsigned)local_7c - 1u);
                    } while (local_7c != 0);
                    return;
                }
            }
        }
    }
    else if ((param_2 < param_4) && (param_1 <= param_3)) {
        dv_a = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
        dv_b = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
        if (DIV_FAULT(dv_a, dv_b)) goto fault;
        local_c = (unsigned)(dv_a / dv_b);
        bVar1 = param_1 < v_b82;
        if (bVar1) {
            dv_a = (int)(((unsigned)v_b82 - (unsigned)param_1) * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(dv_a / dv_b));
            param_1 = v_b82;
        }
        local_5 = param_2 < v_b8a;
        if (local_5) {
            param_1 = (int)((unsigned)param_1 + (unsigned)((int)(((unsigned)v_b8a - (unsigned)param_2) * local_c) >> 0xc));
            param_2 = v_b8a;
        }
        local_5 = local_5 || bVar1;
        if (v_b86 < param_3) {
            local_5 = 1;
            dv_a = (int)(((unsigned)v_b86 - (unsigned)param_3) * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(dv_a / dv_b));
            param_3 = v_b86;
        }
        if (v_b8e < param_4) {
            iVar4 = (int)((unsigned)param_4 - (unsigned)v_b8e);
            param_4 = v_b8e;
            local_5 = 1;
            param_3 = (int)((unsigned)param_3 - (unsigned)((int)((unsigned)iVar4 * local_c) >> 0xc));
        }
        if (local_5) {
            if (param_3 < param_1) {
                return;
            }
            if (param_4 < param_2) {
                return;
            }
            dv_a = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
            dv_b = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            local_c = (unsigned)(dv_a / dv_b);
        }
        uVar8 = (unsigned)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_2 <= param_4) {
                do {
                    FUN_005063b0((int)((unsigned)param_1 - (unsigned)iVar5), (int)((unsigned)iVar3 + (unsigned)param_1), param_2, param_5);
                    local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                    if (0xfff < local_10) {
                        local_10 = (int)((unsigned)local_10 - 0x1000u);
                        param_1 = (int)((unsigned)param_1 + 1u);
                    }
                    param_2 = (int)((unsigned)param_2 + 1u);
                } while (param_2 <= param_4);
                return;
            }
        }
        else if (param_2 <= param_4) {
            do {
                local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    local_10 = (int)((unsigned)local_10 - 0x1000u);
                    uVar6 = uVar8 + 1;
                }
                local_14 = (int)((unsigned)param_2 - (unsigned)iVar5);
                iVar4 = (int)((unsigned)iVar3 + (unsigned)param_2);
                if (v_b8e < (int)((unsigned)iVar3 + (unsigned)param_2)) {
                    iVar4 = v_b8e;
                }
                if (local_14 < v_b8a) {
                    local_14 = v_b8a;
                }
                puVar11 = (unsigned char *)(((unsigned)param_1 << (bVar9 & 0x1f)) + (unsigned)v_b13 * (unsigned)local_14
                                            + (unsigned)DAT_00aa6cfc);
                if (local_14 <= iVar4) {
                    local_5c = (int)(((unsigned)iVar4 - (unsigned)local_14) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = (unsigned *)puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_5c = (int)((unsigned)local_5c - 1u);
                    } while (local_5c != 0);
                }
                param_1 = (int)((unsigned)param_1 + uVar6);
                param_2 = (int)((unsigned)param_2 + 1u);
            } while (param_2 <= param_4);
            return;
        }
    }
    else if ((param_2 < param_4) && (param_3 <= param_1)) {
        dv_a = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
        dv_b = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
        if (DIV_FAULT(dv_a, dv_b)) goto fault;
        local_c = (unsigned)(dv_a / dv_b);
        bVar1 = v_b86 < param_1;
        if (bVar1) {
            iVar4 = (int)((unsigned)param_1 - (unsigned)v_b86);
            param_1 = v_b86;
            dv_a = (int)((unsigned)iVar4 * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(dv_a / dv_b));
        }
        local_5 = param_2 < v_b8a;
        if (local_5) {
            param_1 = (int)((unsigned)param_1 - (unsigned)((int)(((unsigned)v_b8a - (unsigned)param_2) * local_c) >> 0xc));
            param_2 = v_b8a;
        }
        local_5 = local_5 || bVar1;
        if (param_3 < v_b82) {
            iVar4 = (int)((unsigned)param_3 - (unsigned)v_b82);
            param_3 = v_b82;
            local_5 = 1;
            dv_a = (int)((unsigned)iVar4 * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(dv_a / dv_b));
        }
        if (v_b8e < param_4) {
            iVar4 = (int)((unsigned)param_4 - (unsigned)v_b8e);
            param_4 = v_b8e;
            local_5 = 1;
            param_3 = (int)((unsigned)param_3 + (unsigned)((int)((unsigned)iVar4 * local_c) >> 0xc));
        }
        if (local_5) {
            if (param_1 < param_3) {
                return;
            }
            if (param_4 < param_2) {
                return;
            }
            dv_a = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
            dv_b = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            local_c = (unsigned)(dv_a / dv_b);
        }
        uVar8 = (unsigned)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_2 <= param_4) {
                do {
                    FUN_005063b0((int)((unsigned)param_1 - (unsigned)iVar5), (int)((unsigned)iVar3 + (unsigned)param_1), param_2, param_5);
                    local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                    if (0xfff < local_10) {
                        local_10 = (int)((unsigned)local_10 - 0x1000u);
                        param_1 = (int)((unsigned)param_1 - 1u);
                    }
                    param_2 = (int)((unsigned)param_2 + 1u);
                } while (param_2 <= param_4);
                return;
            }
        }
        else if (param_2 <= param_4) {
            do {
                local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    local_10 = (int)((unsigned)local_10 - 0x1000u);
                    uVar6 = uVar8 + 1;
                }
                local_14 = (int)((unsigned)param_4 - (unsigned)iVar5);
                iVar4 = (int)((unsigned)iVar3 + (unsigned)param_4);
                if (v_b8e < (int)((unsigned)iVar3 + (unsigned)param_4)) {
                    iVar4 = v_b8e;
                }
                if (local_14 < v_b8a) {
                    local_14 = v_b8a;
                }
                puVar11 = (unsigned char *)((unsigned)v_b13 * (unsigned)local_14 + ((unsigned)param_3 << (bVar9 & 0x1f))
                                            + (unsigned)DAT_00aa6cfc);
                if (local_14 <= iVar4) {
                    local_4c = (int)(((unsigned)iVar4 - (unsigned)local_14) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = (unsigned *)puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_4c = (int)((unsigned)local_4c - 1u);
                    } while (local_4c != 0);
                }
                param_3 = (int)((unsigned)param_3 + uVar6);
                param_4 = (int)((unsigned)param_4 - 1u);
            } while (param_2 <= param_4);
            return;
        }
    }
    else if ((param_4 < param_2) && (param_1 <= param_3)) {
        dv_a = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
        dv_b = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
        if (DIV_FAULT(dv_a, dv_b)) goto fault;
        local_c = (unsigned)(dv_a / dv_b);
        bVar1 = param_1 < v_b82;
        if (bVar1) {
            iVar4 = (int)((unsigned)param_1 - (unsigned)v_b82);
            param_1 = v_b82;
            dv_a = (int)((unsigned)iVar4 * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(dv_a / dv_b));
        }
        local_5 = v_b8e < param_2;
        if (local_5) {
            iVar4 = (int)((unsigned)param_2 - (unsigned)v_b8e);
            param_2 = v_b8e;
            param_1 = (int)((unsigned)param_1 + (unsigned)((int)((unsigned)iVar4 * local_c) >> 0xc));
        }
        local_5 = local_5 || bVar1;
        if (v_b86 < param_3) {
            iVar4 = (int)((unsigned)param_3 - (unsigned)v_b86);
            param_3 = v_b86;
            local_5 = 1;
            dv_a = (int)((unsigned)iVar4 * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(dv_a / dv_b));
        }
        if (param_4 < v_b8a) {
            local_5 = 1;
            param_3 = (int)((unsigned)param_3 - (unsigned)((int)(((unsigned)v_b8a - (unsigned)param_4) * local_c) >> 0xc));
            param_4 = v_b8a;
        }
        if (local_5) {
            if (param_3 < param_1) {
                return;
            }
            if (param_2 < param_4) {
                return;
            }
            dv_a = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
            dv_b = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            local_c = (unsigned)(dv_a / dv_b);
        }
        uVar8 = (unsigned)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            if (param_4 <= param_2) {
                do {
                    FUN_005063b0((int)((unsigned)param_1 - (unsigned)iVar5), (int)((unsigned)iVar3 + (unsigned)param_1), param_2, param_5);
                    local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                    if (0xfff < local_10) {
                        local_10 = (int)((unsigned)local_10 - 0x1000u);
                        param_1 = (int)((unsigned)param_1 + 1u);
                    }
                    param_2 = (int)((unsigned)param_2 - 1u);
                } while (param_4 <= param_2);
                return;
            }
        }
        else if (param_4 <= param_2) {
            do {
                local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    local_10 = (int)((unsigned)local_10 - 0x1000u);
                    uVar6 = uVar8 + 1;
                }
                local_14 = (int)((unsigned)param_2 - (unsigned)iVar5);
                iVar4 = (int)((unsigned)iVar3 + (unsigned)param_2);
                if (v_b8e < (int)((unsigned)iVar3 + (unsigned)param_2)) {
                    iVar4 = v_b8e;
                }
                if (local_14 < v_b8a) {
                    local_14 = v_b8a;
                }
                puVar11 = (unsigned char *)(((unsigned)param_1 << (bVar9 & 0x1f)) + (unsigned)v_b13 * (unsigned)local_14
                                            + (unsigned)DAT_00aa6cfc);
                if (local_14 <= iVar4) {
                    local_3c = (int)(((unsigned)iVar4 - (unsigned)local_14) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = (unsigned *)puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_3c = (int)((unsigned)local_3c - 1u);
                    } while (local_3c != 0);
                }
                param_1 = (int)((unsigned)param_1 + uVar6);
                param_2 = (int)((unsigned)param_2 - 1u);
            } while (param_4 <= param_2);
            return;
        }
    }
    else {
        dv_a = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
        dv_b = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
        if (DIV_FAULT(dv_a, dv_b)) goto fault;
        local_c = (unsigned)(dv_a / dv_b);
        bVar1 = v_b86 < param_1;
        if (bVar1) {
            dv_a = (int)(((unsigned)v_b86 - (unsigned)param_1) * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(dv_a / dv_b));
            param_1 = v_b86;
        }
        local_5 = v_b8e < param_2;
        if (local_5) {
            iVar4 = (int)((unsigned)param_2 - (unsigned)v_b8e);
            param_2 = v_b8e;
            param_1 = (int)((unsigned)param_1 - (unsigned)((int)((unsigned)iVar4 * local_c) >> 0xc));
        }
        local_5 = local_5 || bVar1;
        if (param_3 < v_b82) {
            local_5 = 1;
            dv_a = (int)(((unsigned)v_b82 - (unsigned)param_3) * 0x1000u);
            dv_b = (int)local_c;
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(dv_a / dv_b));
            param_3 = v_b82;
        }
        if (param_4 < v_b8a) {
            local_5 = 1;
            param_3 = (int)((unsigned)param_3 + (unsigned)((int)(((unsigned)v_b8a - (unsigned)param_4) * local_c) >> 0xc));
            param_4 = v_b8a;
        }
        if (local_5) {
            if (param_1 < param_3) {
                return;
            }
            if (param_2 < param_4) {
                return;
            }
            dv_a = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
            dv_b = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
            if (DIV_FAULT(dv_a, dv_b)) goto fault;
            local_c = (unsigned)(dv_a / dv_b);
        }
        uVar8 = (unsigned)((int)local_c >> 0xc);
        if (uVar8 == 0) {
            for (; param_4 <= param_2; param_2 = (int)((unsigned)param_2 - 1u)) {
                FUN_005063b0((int)((unsigned)param_1 - (unsigned)iVar5), (int)((unsigned)iVar3 + (unsigned)param_1), param_2, param_5);
                local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                if (0xfff < local_10) {
                    local_10 = (int)((unsigned)local_10 - 0x1000u);
                    param_1 = (int)((unsigned)param_1 - 1u);
                }
            }
        }
        else if (param_4 <= param_2) {
            do {
                local_10 = (int)((unsigned)local_10 + (local_c & 0xfff));
                uVar6 = uVar8;
                if (0xfff < local_10) {
                    local_10 = (int)((unsigned)local_10 - 0x1000u);
                    uVar6 = uVar8 + 1;
                }
                local_14 = (int)((unsigned)param_4 - (unsigned)iVar5);
                iVar4 = (int)((unsigned)iVar3 + (unsigned)param_4);
                if (v_b8e < (int)((unsigned)iVar3 + (unsigned)param_4)) {
                    iVar4 = v_b8e;
                }
                if (local_14 < v_b8a) {
                    local_14 = v_b8a;
                }
                puVar11 = (unsigned char *)((unsigned)v_b13 * (unsigned)local_14 + ((unsigned)param_3 << (bVar9 & 0x1f))
                                            + (unsigned)DAT_00aa6cfc);
                if (local_14 <= iVar4) {
                    local_2c = (int)(((unsigned)iVar4 - (unsigned)local_14) + 1u);
                    do {
                        if (iVar10 == 0) {
                            puVar12 = (unsigned *)puVar11;
                            for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *puVar12 = (unsigned)uVar2 * 0x01010101u;
                                puVar12 = puVar12 + 1;
                            }
                            for (uVar7 = uVar6 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned char *)puVar12 = uVar2;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 1);
                            }
                        }
                        else if ((iVar10 == 1) && (uVar7 = uVar6, puVar12 = (unsigned *)puVar11, 0 < (int)uVar6)) {
                            for (; uVar7 != 0; uVar7 = uVar7 - 1) {
                                *(unsigned short *)puVar12 = param_5;
                                puVar12 = (unsigned *)((unsigned char *)puVar12 + 2);
                            }
                        }
                        puVar11 = puVar11 + v_b13;
                        local_2c = (int)((unsigned)local_2c - 1u);
                    } while (local_2c != 0);
                }
                param_3 = (int)((unsigned)param_3 + uVar6);
                param_4 = (int)((unsigned)param_4 + 1u);
            } while (param_4 <= param_2);
            return;
        }
    }
    return;
fault:
    RaiseException(0xC0000094UL, 0, 0, NULL);
    return;
}
