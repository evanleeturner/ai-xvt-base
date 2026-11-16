/* SPDX-License-Identifier: MIT */
/* z_bop 004cefe0 */
int __cdecl _sprintf(char *, const char *, ...);
int __cdecl FUN_0046fad0(void);
unsigned int __cdecl FUN_004d23b0(int param_1);

extern char s__SB__u__0052b960[];
extern char s__SG__u__0052b954[];
extern char s__SS__u__0052b948[];
extern char s_Send_Returned____8x_0052b930[];

extern unsigned char DAT_00acf14d[];
extern unsigned char DAT_00acf151[];
extern unsigned char DAT_00acf351[];
extern unsigned char DAT_00acf355[];
extern unsigned char DAT_00acf359[];
extern unsigned char DAT_00acf35d[];
extern unsigned char DAT_00acf55d[];
extern unsigned char DAT_00acf561[];
extern unsigned char DAT_00acebf5[];
extern unsigned char DAT_00acebc5[];
extern unsigned char DAT_00acf145[];
extern unsigned char DAT_00b53571[];
extern unsigned char DAT_00b53579[];
extern unsigned char DAT_00b63d7d[];
extern unsigned char DAT_00b69a89[];
extern unsigned char DAT_00b5357d[];
extern unsigned char DAT_00b53581[];
extern unsigned char DAT_00b53585[];
extern unsigned char DAT_00b53589[];
extern unsigned char DAT_00b5358a[];
extern unsigned char DAT_00b5358b[];
extern unsigned char DAT_00b5358d[];
extern unsigned char DAT_00acf571[];
extern unsigned char DAT_00acf575[];
extern unsigned char DAT_00acf579[];
extern unsigned char DAT_00acf57d[];
extern unsigned char DAT_00acf57e[];
extern unsigned char DAT_00acf57f[];
extern unsigned char DAT_00acf580[];
extern unsigned char DAT_00acf581[];
extern unsigned char DAT_00b63f99[];
extern unsigned char DAT_00b63fa1[];
extern unsigned char DAT_00b63fa9[];
extern unsigned char DAT_00b63fad[];
extern unsigned char DAT_00b63fb1[];
extern unsigned char DAT_00b641b1[];

bool __cdecl FUN_004cefe0(int param_1, unsigned int *param_2, unsigned int param_3)
{
    short sVar1;
    int iVar2;
    int iVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned short uVar8;
    unsigned char *puVar7;
    unsigned char *puVar9;
    unsigned char *puVar10;
    unsigned char *puVar11;
    unsigned char *puVar12;
    unsigned char *local_55c;
    int local_564;
    int local_554;
    unsigned short local_500[512];
    unsigned int local_550[20];
    char local_100[256];
    unsigned int t;
    unsigned int t2;
    unsigned int idx;
    unsigned int zero;
    unsigned short us;

    zero = 0;
    local_554 = 0;
    uVar6 = *param_2;
    bVar4 = (unsigned char)uVar6 & 0x7f;
    uVar8 = (unsigned short)bVar4;
    if ((uVar6 < 0x3c) || (0x3f < uVar6)) {
        local_550[0] = 1;
    } else {
        local_550[0] = 0;
    }
    sVar1 = (short)param_3;
    if (param_1 == 0) {
        __builtin_memcpy(&t, DAT_00acf14d, 4);
        _sprintf(local_100, s__SB__u__0052b960, t);
        uVar8 = (unsigned short)t;
        t = t + 1;
        __builtin_memcpy(DAT_00acf14d, &t, 4);
        uVar8 = (unsigned short)((unsigned short)bVar4 | (unsigned short)((uVar8 & 0x7f) << 8));
        if (0x7f < t) {
            __builtin_memcpy(DAT_00acf14d, &zero, 4);
        }
        puVar7 = (unsigned char *)&local_500[1];
        local_564 = 2;
        local_500[0] = uVar8;
        if ((local_550[0] != 0) && (iVar2 = FUN_0046fad0(), iVar2 == 0)) {
            local_500[1] = (unsigned short)(short)(sVar1 + -4);
            puVar7 = (unsigned char *)&local_500[2];
            local_564 = 4;
        }
        uVar6 = param_3 - 4;
        puVar9 = (unsigned char *)(param_2 + 1);
        puVar12 = puVar7;
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            __builtin_memcpy(puVar12, puVar9, 4);
            puVar9 = puVar9 + 4;
            puVar12 = puVar12 + 4;
        }
        puVar11 = puVar7 + (param_3 - 4);
        for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *puVar12 = *puVar9;
            puVar9 = puVar9 + 1;
            puVar12 = puVar12 + 1;
        }
        local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
        if (local_550[0] != 0) {
            __builtin_memcpy(&t2, DAT_00acf355, 4);
            if (t2 == 0) {
                __builtin_memcpy(&local_550[0], DAT_00acf351, 4);
                puVar10 = DAT_00acf151;
                __builtin_memcpy(&t2, DAT_00acf351, 4);
                for (uVar5 = t2 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                    __builtin_memcpy(puVar11, puVar10, 4);
                    puVar10 = puVar10 + 4;
                    puVar11 = puVar11 + 4;
                }
                __builtin_memcpy(&t2, DAT_00acf351, 4);
                for (uVar5 = t2 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar11 = *puVar10;
                    puVar10 = puVar10 + 1;
                    puVar11 = puVar11 + 1;
                }
                __builtin_memcpy(&t2, DAT_00acf351, 4);
                local_564 = (int)((unsigned int)local_564 + t2);
            } else {
                *puVar11 = 0x39;
                local_564 = (int)((unsigned int)local_564 + 1);
                __builtin_memcpy(DAT_00acf355, &zero, 4);
            }
        }
        puVar7 = (unsigned char *)(param_2 + 1);
        puVar9 = DAT_00acf151 + 1;
        DAT_00acf151[0] = bVar4;
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            __builtin_memcpy(puVar9, puVar7, 4);
            puVar7 = puVar7 + 4;
            puVar9 = puVar9 + 4;
        }
        t2 = param_3 - 3;
        __builtin_memcpy(DAT_00acf351, &t2, 4);
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar9 = *puVar7;
            puVar7 = puVar7 + 1;
            puVar9 = puVar9 + 1;
        }
    } else {
        __builtin_memcpy(&t, DAT_00acebf5, 4);
        if ((int)t == param_1) {
            __builtin_memcpy(&t, DAT_00acf14d, 4);
            _sprintf(local_100, s__SG__u__0052b954, t);
            __builtin_memcpy(&t, DAT_00acf359, 4);
            uVar8 = (unsigned short)t;
            t = t + 1;
            __builtin_memcpy(DAT_00acf359, &t, 4);
            uVar8 = (unsigned short)((unsigned short)bVar4 | (unsigned short)((uVar8 & 0x7f) << 8) | 0x8080);
            if (0x7f < t) {
                __builtin_memcpy(DAT_00acf359, &zero, 4);
            }
            puVar7 = (unsigned char *)&local_500[1];
            local_564 = 2;
            local_500[0] = uVar8;
            if ((local_550[0] != 0) && (iVar2 = FUN_0046fad0(), iVar2 == 0)) {
                local_500[1] = (unsigned short)(short)(sVar1 + -4);
                puVar7 = (unsigned char *)&local_500[2];
                local_564 = 4;
            }
            uVar6 = param_3 - 4;
            puVar9 = (unsigned char *)(param_2 + 1);
            puVar12 = puVar7;
            for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(puVar12, puVar9, 4);
                puVar9 = puVar9 + 4;
                puVar12 = puVar12 + 4;
            }
            puVar11 = puVar7 + (param_3 - 4);
            for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar12 = *puVar9;
                puVar9 = puVar9 + 1;
                puVar12 = puVar12 + 1;
            }
            local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
            if (local_550[0] != 0) {
                __builtin_memcpy(&t2, DAT_00acf561, 4);
                if (t2 == 0) {
                    __builtin_memcpy(&local_550[0], DAT_00acf55d, 4);
                    puVar10 = DAT_00acf35d;
                    __builtin_memcpy(&t2, DAT_00acf55d, 4);
                    for (uVar5 = t2 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        __builtin_memcpy(puVar11, puVar10, 4);
                        puVar10 = puVar10 + 4;
                        puVar11 = puVar11 + 4;
                    }
                    __builtin_memcpy(&t2, DAT_00acf55d, 4);
                    for (uVar5 = t2 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *puVar11 = *puVar10;
                        puVar10 = puVar10 + 1;
                        puVar11 = puVar11 + 1;
                    }
                    __builtin_memcpy(&t2, DAT_00acf55d, 4);
                    local_564 = (int)((unsigned int)local_564 + t2);
                } else {
                    *puVar11 = 0x39;
                    local_564 = (int)((unsigned int)local_564 + 1);
                    __builtin_memcpy(DAT_00acf561, &zero, 4);
                }
            }
            puVar7 = (unsigned char *)(param_2 + 1);
            puVar9 = DAT_00acf35d + 1;
            DAT_00acf35d[0] = bVar4;
            for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(puVar9, puVar7, 4);
                puVar7 = puVar7 + 4;
                puVar9 = puVar9 + 4;
            }
            t2 = param_3 - 3;
            __builtin_memcpy(DAT_00acf55d, &t2, 4);
            for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *puVar9 = *puVar7;
                puVar7 = puVar7 + 1;
                puVar9 = puVar9 + 1;
            }
        } else {
            uVar6 = FUN_004d23b0(param_1);
            __builtin_memcpy(&t, DAT_00b69a89, 4);
            if ((uVar6 < t) && (uVar6 < 0x28)) {
                __builtin_memcpy(&t, DAT_00b63fad + uVar6 * 0x238, 4);
                _sprintf(local_100, s__SS__u__0052b948, t);
                iVar2 = (int)t;
                iVar3 = (int)((unsigned int)iVar2 + 1);
                t2 = (unsigned int)iVar3;
                __builtin_memcpy(DAT_00b63fad + uVar6 * 0x238, &t2, 4);
                us = (unsigned short)iVar2;
                uVar8 = (unsigned short)((unsigned short)bVar4 | (unsigned short)((us & 0x7f) << 8));
                if (0x7f < iVar3) {
                    __builtin_memcpy(DAT_00b63fad + uVar6 * 0x238, &zero, 4);
                }
            }
            uVar8 = (unsigned short)(uVar8 | 0x8000);
            local_55c = (unsigned char *)&local_500[1];
            local_564 = 2;
            local_500[0] = uVar8;
            if ((local_550[0] != 0) && (iVar2 = FUN_0046fad0(), iVar2 == 0)) {
                local_500[1] = (unsigned short)(short)(sVar1 + -4);
                local_55c = (unsigned char *)&local_500[2];
                local_564 = 4;
            }
            puVar7 = (unsigned char *)(param_2 + 1);
            puVar9 = local_55c;
            for (uVar5 = (param_3 - 4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(puVar9, puVar7, 4);
                puVar7 = puVar7 + 4;
                puVar9 = puVar9 + 4;
            }
            for (uVar5 = (param_3 - 4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar9 = *puVar7;
                puVar7 = puVar7 + 1;
                puVar9 = puVar9 + 1;
            }
            local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
            if (local_550[0] != 0) {
                __builtin_memcpy(&local_550[0], DAT_00b641b1 + uVar6 * 0x238, 4);
                puVar11 = DAT_00b63fb1 + uVar6 * 0x238;
                puVar10 = local_55c + (param_3 - 4);
                for (uVar5 = local_550[0] >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                    __builtin_memcpy(puVar10, puVar11, 4);
                    puVar11 = puVar11 + 4;
                    puVar10 = puVar10 + 4;
                }
                for (uVar5 = local_550[0] & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar10 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar10 = puVar10 + 1;
                }
                __builtin_memcpy(&t2, DAT_00b641b1 + uVar6 * 0x238, 4);
                local_564 = (int)((unsigned int)local_564 + t2);
            }
            DAT_00b63fb1[uVar6 * 0x238] = bVar4;
            puVar7 = (unsigned char *)(param_2 + 1);
            puVar9 = DAT_00b63fb1 + 1 + uVar6 * 0x238;
            for (uVar5 = (param_3 - 4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                __builtin_memcpy(puVar9, puVar7, 4);
                puVar7 = puVar7 + 4;
                puVar9 = puVar9 + 4;
            }
            for (uVar5 = (param_3 - 4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar9 = *puVar7;
                puVar7 = puVar7 + 1;
                puVar9 = puVar9 + 1;
            }
            t2 = param_3 - 3;
            __builtin_memcpy(DAT_00b641b1 + uVar6 * 0x238, &t2, 4);
        }
    }
    __builtin_memcpy(&t, DAT_00acf145, 4);
    if ((int)t != param_1) {
        puVar7 = (unsigned char *)param_2;
        __builtin_memcpy(&idx, DAT_00b63d7d, 4);
        puVar9 = DAT_00b5358d + idx * 0x210;
        for (uVar6 = param_3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            __builtin_memcpy(puVar9, puVar7, 4);
            puVar7 = puVar7 + 4;
            puVar9 = puVar9 + 4;
        }
        for (uVar6 = param_3 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar9 = *puVar7;
            puVar7 = puVar7 + 1;
            puVar9 = puVar9 + 1;
        }
        t2 = (unsigned int)param_1;
        __builtin_memcpy(DAT_00b5357d + idx * 0x210, &t2, 4);
        __builtin_memcpy(DAT_00b53581 + idx * 0x210, &param_3, 4);
        __builtin_memcpy(DAT_00b53585 + idx * 0x210, &zero, 4);
        DAT_00b53589[idx * 0x210] = 0;
        __builtin_memcpy(&t2, DAT_00acebf5, 4);
        if (param_1 == 0) {
            DAT_00b5358a[idx * 0x210] = 0;
        } else if ((int)t2 == param_1) {
            DAT_00b5358a[idx * 0x210] = 2;
        } else {
            DAT_00b5358a[idx * 0x210] = 1;
        }
        DAT_00b5358b[idx * 0x210] = (unsigned char)((unsigned char)(local_500[0] >> 8) & 0x7f);
        idx = idx + 1;
        __builtin_memcpy(DAT_00b63d7d, &idx, 4);
        uVar8 = local_500[0];
        if (0x7f < idx) {
            __builtin_memcpy(DAT_00b63d7d, &zero, 4);
        }
    }
    __builtin_memcpy(&t, DAT_00acf145, 4);
    __builtin_memcpy(&t2, DAT_00acebc5, 4);
    if (((((int)t == param_1) || (param_1 == 0)) || (t2 == 0)) ||
        (__builtin_memcpy(&t2, DAT_00acebf5, 4), (int)t2 == param_1)) {
        __builtin_memcpy(&t2, DAT_00b53579, 4);
        if (t2 < 0x400) {
            __builtin_memcpy(&idx, DAT_00b53571, 4);
            puVar7 = DAT_00acf581 + idx * 0x210;
            puVar9 = (unsigned char *)param_2;
            for (uVar6 = param_3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                __builtin_memcpy(puVar7, puVar9, 4);
                puVar9 = puVar9 + 4;
                puVar7 = puVar7 + 4;
            }
            for (uVar6 = param_3 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *puVar7 = *puVar9;
                puVar9 = puVar9 + 1;
                puVar7 = puVar7 + 1;
            }
            __builtin_memcpy(&t, DAT_00acf145, 4);
            __builtin_memcpy(DAT_00acf571 + idx * 0x210, &t, 4);
            __builtin_memcpy(DAT_00acf575 + idx * 0x210, &param_3, 4);
            __builtin_memcpy(DAT_00acf579 + idx * 0x210, &zero, 4);
            DAT_00acf57d[idx * 0x210] = 0;
            DAT_00acf580[idx * 0x210] = 0;
            __builtin_memcpy(&t, DAT_00acf145, 4);
            uVar6 = FUN_004d23b0((int)t);
            __builtin_memcpy(&t, DAT_00b69a89, 4);
            __builtin_memcpy(&t2, DAT_00acebf5, 4);
            if (param_1 == 0) {
                DAT_00acf57e[idx * 0x210] = 0;
                if ((uVar6 < t) && (uVar6 < 0x28)) {
                    us = (unsigned short)((unsigned int)((uVar8 & 0x7f00) >> 8));
                    t2 = (unsigned int)us;
                    __builtin_memcpy(DAT_00b63fa1 + uVar6 * 0x238, &t2, 4);
                }
            } else if ((int)t2 == param_1) {
                DAT_00acf57e[idx * 0x210] = 2;
                if ((uVar6 < t) && (uVar6 < 0x28)) {
                    t2 = (unsigned int)((uVar8 & 0x7f00) >> 8);
                    __builtin_memcpy(DAT_00b63fa9 + uVar6 * 0x238, &t2, 4);
                }
            } else {
                DAT_00acf57e[idx * 0x210] = 1;
                if ((uVar6 < t) && (uVar6 < 0x28)) {
                    t2 = (unsigned int)((uVar8 & 0x7f00) >> 8);
                    __builtin_memcpy(DAT_00b63f99 + uVar6 * 0x238, &t2, 4);
                }
            }
            DAT_00acf57f[idx * 0x210] = (unsigned char)((unsigned char)(local_500[0] >> 8) & 0x7f);
            __builtin_memcpy(&t2, DAT_00b53579, 4);
            t2 = t2 + 1;
            __builtin_memcpy(DAT_00b53579, &t2, 4);
            idx = idx + 1;
            __builtin_memcpy(DAT_00b53571, &idx, 4);
            if (0x3ff < idx) {
                __builtin_memcpy(DAT_00b53571, &zero, 4);
            }
        }
    }
    __builtin_memcpy(&t2, DAT_00acebc5, 4);
    if (t2 != 0) {
        __builtin_memcpy(&t, DAT_00acf145, 4);
        if ((int)t != param_1) {
            unsigned int vt;
            unsigned int fn;
            __builtin_memcpy(&vt, (void *)t2, 4);
            __builtin_memcpy(&fn, (void *)(vt + 0x68), 4);
            local_554 = ((int (__stdcall *)(unsigned int, unsigned int, int, unsigned int, void *, int))fn)
                            (t2, t, param_1, 0, local_500, local_564);
        }
        if (local_554 != 0) {
            _sprintf((char *)local_550, s_Send_Returned____8x_0052b930, local_554);
        }
        return local_554 == 0;
    }
    return 1;
}
