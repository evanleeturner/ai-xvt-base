/* SPDX-License-Identifier: MIT */
/* z_xvt 004cb8b0 */
extern int  __cdecl _sprintf(char *, const char *, ...);
extern int  __cdecl FUN_00433150(void);
extern unsigned int __cdecl FUN_004cee90(int);

extern char s__SB__u__00532468[];
extern char s__SG__u__0053245c[];
extern char s__SS__u__00532450[];
extern char s_Send_Returned____8x_00532438[];

/* Packed globals (addresses not multiples of 4): raw byte storage, accessed via memcpy. */
extern unsigned char DAT_00780c82[4];
extern unsigned char DAT_00780e86[4];
extern unsigned char DAT_00780e8a[4];
extern unsigned char DAT_00780e8e[4];
extern unsigned char DAT_00781092[4];
extern unsigned char DAT_00781096[4];
extern unsigned char DAT_0078072a[4];
extern unsigned char DAT_00780c7a[4];
extern unsigned char DAT_007806fa[4];
extern unsigned char DAT_0081b5be[4];
extern unsigned char DAT_008158b2[4];
extern unsigned char DAT_008050a6[4];
extern unsigned char DAT_008050ae[4];

extern unsigned char DAT_00780c86[];
extern unsigned char DAT_00780e92[];
extern unsigned char DAT_00815ace[];
extern unsigned char DAT_00815ad6[];
extern unsigned char DAT_00815ade[];
extern unsigned char DAT_00815ae2[];
extern unsigned char DAT_00815ae6[];
extern unsigned char DAT_00815ae7[];
extern unsigned char DAT_00815ce6[];
extern unsigned char DAT_008050b2[];
extern unsigned char DAT_008050b6[];
extern unsigned char DAT_008050ba[];
extern unsigned char DAT_008050be[];
extern unsigned char DAT_008050bf[];
extern unsigned char DAT_008050c0[];
extern unsigned char DAT_008050c2[];
extern unsigned char DAT_007810a6[];
extern unsigned char DAT_007810aa[];
extern unsigned char DAT_007810ae[];
extern unsigned char DAT_007810b2[];
extern unsigned char DAT_007810b3[];
extern unsigned char DAT_007810b4[];
extern unsigned char DAT_007810b5[];
extern unsigned char DAT_007810b6[];

bool __cdecl FUN_004cb8b0(int param_1, unsigned int *param_2, unsigned int param_3)
{
    short sVar1;
    int iVar2;
    int iVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned short uVar7;
    unsigned char *puVar8;
    unsigned char *puVar9;
    unsigned char *puVar10;
    unsigned char *puVar11;
    unsigned char *puVar12;
    int local_564;
    unsigned char *local_55c;
    int local_554;
    unsigned int local_550[20];
    unsigned char local_500[0x400];
    char local_100[256];
    unsigned int uTmpA;
    unsigned int uTmpB;
    unsigned int uTmpC;
    short sTmp2;
    int *pIface;
    unsigned char *pSrc;

    local_554 = 0;
    uVar6 = *param_2;
    bVar4 = (unsigned char)uVar6 & 0x7f;
    uVar7 = (unsigned short)bVar4;
    if ((uVar6 < 0x3c) || (0x3f < uVar6)) {
        local_550[0] = 1;
    } else {
        local_550[0] = 0;
    }
    sVar1 = (short)param_3;
    pSrc = (unsigned char *)(param_2 + 1);
    if (param_1 == 0) {
        __builtin_memcpy(&uTmpA, DAT_00780c82, 4);
        _sprintf(local_100, s__SB__u__00532468, uTmpA);
        uVar7 = (unsigned short)((unsigned short)bVar4 | (((unsigned short)uTmpA & 0x7f) << 8));
        uTmpA = uTmpA + 1;
        __builtin_memcpy(DAT_00780c82, &uTmpA, 4);
        if (0x7f < uTmpA) {
            uTmpA = 0;
            __builtin_memcpy(DAT_00780c82, &uTmpA, 4);
        }
        puVar8 = local_500 + 2;
        local_564 = 2;
        __builtin_memcpy(local_500, &uVar7, 2);
        if ((local_550[0] != 0) && (iVar2 = FUN_00433150(), iVar2 == 0)) {
            sTmp2 = (short)((unsigned int)sVar1 - 4);
            __builtin_memcpy(local_500 + 2, &sTmp2, 2);
            puVar8 = local_500 + 4;
            local_564 = 4;
        }
        uVar6 = param_3 - 4;
        puVar9 = pSrc;
        puVar12 = puVar8;
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            puVar12[0] = puVar9[0];
            puVar12[1] = puVar9[1];
            puVar12[2] = puVar9[2];
            puVar12[3] = puVar9[3];
            puVar9 = puVar9 + 4;
            puVar12 = puVar12 + 4;
        }
        puVar11 = puVar8 + (param_3 - 4);
        for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
            *puVar12 = *puVar9;
            puVar9 = puVar9 + 1;
            puVar12 = puVar12 + 1;
        }
        local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
        if (local_550[0] != 0) {
            __builtin_memcpy(&uTmpA, DAT_00780e8a, 4);
            if (uTmpA == 0) {
                __builtin_memcpy(&uTmpB, DAT_00780e86, 4);
                local_550[0] = uTmpB;
                puVar10 = DAT_00780c86;
                for (uVar5 = uTmpB >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                    puVar11[0] = puVar10[0];
                    puVar11[1] = puVar10[1];
                    puVar11[2] = puVar10[2];
                    puVar11[3] = puVar10[3];
                    puVar10 = puVar10 + 4;
                    puVar11 = puVar11 + 4;
                }
                for (uVar5 = uTmpB & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar11 = *puVar10;
                    puVar10 = puVar10 + 1;
                    puVar11 = puVar11 + 1;
                }
                __builtin_memcpy(&uTmpB, DAT_00780e86, 4);
                local_564 = (int)((unsigned int)local_564 + uTmpB);
            } else {
                local_564 = (int)((unsigned int)local_564 + 1);
                *puVar11 = 0x39;
                uTmpA = 0;
                __builtin_memcpy(DAT_00780e8a, &uTmpA, 4);
            }
        }
        puVar8 = pSrc;
        puVar9 = DAT_00780c86 + 1;
        DAT_00780c86[0] = bVar4;
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
            puVar9[0] = puVar8[0];
            puVar9[1] = puVar8[1];
            puVar9[2] = puVar8[2];
            puVar9[3] = puVar8[3];
            puVar8 = puVar8 + 4;
            puVar9 = puVar9 + 4;
        }
        uTmpA = param_3 - 3;
        __builtin_memcpy(DAT_00780e86, &uTmpA, 4);
        for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar9 = *puVar8;
            puVar8 = puVar8 + 1;
            puVar9 = puVar9 + 1;
        }
    } else {
        __builtin_memcpy(&uTmpA, DAT_0078072a, 4);
        if ((int)uTmpA == param_1) {
            __builtin_memcpy(&uTmpA, DAT_00780c82, 4);
            _sprintf(local_100, s__SG__u__0053245c, uTmpA);
            __builtin_memcpy(&uTmpA, DAT_00780e8e, 4);
            uVar7 = (unsigned short)((unsigned short)bVar4 | (((unsigned short)uTmpA & 0x7f) << 8) | 0x8080);
            uTmpA = uTmpA + 1;
            __builtin_memcpy(DAT_00780e8e, &uTmpA, 4);
            if (0x7f < uTmpA) {
                uTmpA = 0;
                __builtin_memcpy(DAT_00780e8e, &uTmpA, 4);
            }
            puVar8 = local_500 + 2;
            local_564 = 2;
            __builtin_memcpy(local_500, &uVar7, 2);
            if ((local_550[0] != 0) && (iVar2 = FUN_00433150(), iVar2 == 0)) {
                sTmp2 = (short)((unsigned int)sVar1 - 4);
                __builtin_memcpy(local_500 + 2, &sTmp2, 2);
                puVar8 = local_500 + 4;
                local_564 = 4;
            }
            uVar6 = param_3 - 4;
            puVar9 = pSrc;
            puVar12 = puVar8;
            for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                puVar12[0] = puVar9[0];
                puVar12[1] = puVar9[1];
                puVar12[2] = puVar9[2];
                puVar12[3] = puVar9[3];
                puVar9 = puVar9 + 4;
                puVar12 = puVar12 + 4;
            }
            puVar11 = puVar8 + (param_3 - 4);
            for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar12 = *puVar9;
                puVar9 = puVar9 + 1;
                puVar12 = puVar12 + 1;
            }
            local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
            if (local_550[0] != 0) {
                __builtin_memcpy(&uTmpA, DAT_00781096, 4);
                if (uTmpA == 0) {
                    __builtin_memcpy(&uTmpB, DAT_00781092, 4);
                    local_550[0] = uTmpB;
                    puVar10 = DAT_00780e92;
                    for (uVar5 = uTmpB >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                        puVar11[0] = puVar10[0];
                        puVar11[1] = puVar10[1];
                        puVar11[2] = puVar10[2];
                        puVar11[3] = puVar10[3];
                        puVar10 = puVar10 + 4;
                        puVar11 = puVar11 + 4;
                    }
                    for (uVar5 = uTmpB & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                        *puVar11 = *puVar10;
                        puVar10 = puVar10 + 1;
                        puVar11 = puVar11 + 1;
                    }
                    __builtin_memcpy(&uTmpB, DAT_00781092, 4);
                    local_564 = (int)((unsigned int)local_564 + uTmpB);
                } else {
                    local_564 = (int)((unsigned int)local_564 + 1);
                    *puVar11 = 0x39;
                    uTmpA = 0;
                    __builtin_memcpy(DAT_00781096, &uTmpA, 4);
                }
            }
            puVar8 = pSrc;
            puVar9 = DAT_00780e92 + 1;
            DAT_00780e92[0] = bVar4;
            for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                puVar9[0] = puVar8[0];
                puVar9[1] = puVar8[1];
                puVar9[2] = puVar8[2];
                puVar9[3] = puVar8[3];
                puVar8 = puVar8 + 4;
                puVar9 = puVar9 + 4;
            }
            uTmpA = param_3 - 3;
            __builtin_memcpy(DAT_00781092, &uTmpA, 4);
            for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *puVar9 = *puVar8;
                puVar8 = puVar8 + 1;
                puVar9 = puVar9 + 1;
            }
        } else {
            uVar6 = FUN_004cee90(param_1);
            __builtin_memcpy(&uTmpB, DAT_0081b5be, 4);
            if ((uVar6 < uTmpB) && (uVar6 < 0x28)) {
                __builtin_memcpy(&iVar2, DAT_00815ae2 + uVar6 * 0x238, 4);
                _sprintf(local_100, s__SS__u__00532450, (unsigned int)iVar2);
                __builtin_memcpy(&iVar2, DAT_00815ae2 + uVar6 * 0x238, 4);
                iVar3 = (int)((unsigned int)iVar2 + 1);
                __builtin_memcpy(DAT_00815ae2 + uVar6 * 0x238, &iVar3, 4);
                uVar7 = (unsigned short)((unsigned short)bVar4 | (((unsigned short)iVar2 & 0x7f) << 8));
                if (0x7f < iVar3) {
                    iVar3 = 0;
                    __builtin_memcpy(DAT_00815ae2 + uVar6 * 0x238, &iVar3, 4);
                }
            }
            uVar7 = uVar7 | 0x8000;
            local_55c = local_500 + 2;
            local_564 = 2;
            __builtin_memcpy(local_500, &uVar7, 2);
            if ((local_550[0] != 0) && (iVar2 = FUN_00433150(), iVar2 == 0)) {
                sTmp2 = (short)((unsigned int)sVar1 - 4);
                __builtin_memcpy(local_500 + 2, &sTmp2, 2);
                local_55c = local_500 + 4;
                local_564 = 4;
            }
            puVar8 = pSrc;
            puVar9 = local_55c;
            for (uVar5 = (param_3 - 4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                puVar9[0] = puVar8[0];
                puVar9[1] = puVar8[1];
                puVar9[2] = puVar8[2];
                puVar9[3] = puVar8[3];
                puVar8 = puVar8 + 4;
                puVar9 = puVar9 + 4;
            }
            for (uVar5 = (param_3 - 4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar9 = *puVar8;
                puVar8 = puVar8 + 1;
                puVar9 = puVar9 + 1;
            }
            local_564 = (int)((param_3 - 4) + (unsigned int)local_564);
            if (local_550[0] != 0) {
                __builtin_memcpy(&uTmpA, DAT_00815ce6 + uVar6 * 0x238, 4);
                local_550[0] = uTmpA;
                puVar11 = DAT_00815ae6 + uVar6 * 0x238;
                puVar10 = local_55c + (param_3 - 4);
                for (uVar5 = local_550[0] >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                    puVar10[0] = puVar11[0];
                    puVar10[1] = puVar11[1];
                    puVar10[2] = puVar11[2];
                    puVar10[3] = puVar11[3];
                    puVar11 = puVar11 + 4;
                    puVar10 = puVar10 + 4;
                }
                for (uVar5 = local_550[0] & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                    *puVar10 = *puVar11;
                    puVar11 = puVar11 + 1;
                    puVar10 = puVar10 + 1;
                }
                __builtin_memcpy(&uTmpA, DAT_00815ce6 + uVar6 * 0x238, 4);
                local_564 = (int)((unsigned int)local_564 + uTmpA);
            }
            DAT_00815ae6[uVar6 * 0x238] = bVar4;
            puVar8 = pSrc;
            puVar9 = DAT_00815ae7 + uVar6 * 0x238;
            for (uVar5 = (param_3 - 4) >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
                puVar9[0] = puVar8[0];
                puVar9[1] = puVar8[1];
                puVar9[2] = puVar8[2];
                puVar9[3] = puVar8[3];
                puVar8 = puVar8 + 4;
                puVar9 = puVar9 + 4;
            }
            for (uVar5 = (param_3 - 4) & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
                *puVar9 = *puVar8;
                puVar8 = puVar8 + 1;
                puVar9 = puVar9 + 1;
            }
            uTmpA = param_3 - 3;
            __builtin_memcpy(DAT_00815ce6 + uVar6 * 0x238, &uTmpA, 4);
        }
    }
    __builtin_memcpy(&uTmpA, DAT_00780c7a, 4);
    if ((int)uTmpA != param_1) {
        __builtin_memcpy(&uTmpB, DAT_008158b2, 4);
        puVar8 = (unsigned char *)param_2;
        puVar9 = DAT_008050c2 + uTmpB * 0x210;
        for (uVar6 = param_3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            puVar9[0] = puVar8[0];
            puVar9[1] = puVar8[1];
            puVar9[2] = puVar8[2];
            puVar9[3] = puVar8[3];
            puVar8 = puVar8 + 4;
            puVar9 = puVar9 + 4;
        }
        for (uVar6 = param_3 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *puVar9 = *puVar8;
            puVar8 = puVar8 + 1;
            puVar9 = puVar9 + 1;
        }
        __builtin_memcpy(DAT_008050b2 + uTmpB * 0x210, &param_1, 4);
        __builtin_memcpy(DAT_008050b6 + uTmpB * 0x210, &param_3, 4);
        uTmpA = 0;
        __builtin_memcpy(DAT_008050ba + uTmpB * 0x210, &uTmpA, 4);
        DAT_008050be[uTmpB * 0x210] = 0;
        if (param_1 == 0) {
            DAT_008050bf[uTmpB * 0x210] = 0;
        } else {
            __builtin_memcpy(&uTmpA, DAT_0078072a, 4);
            if ((int)uTmpA == param_1) {
                DAT_008050bf[uTmpB * 0x210] = 2;
            } else {
                DAT_008050bf[uTmpB * 0x210] = 1;
            }
        }
        DAT_008050c0[uTmpB * 0x210] = (unsigned char)(local_500[1] & 0x7f);
        uTmpB = uTmpB + 1;
        __builtin_memcpy(DAT_008158b2, &uTmpB, 4);
        __builtin_memcpy(&uVar7, local_500, 2);
        if (0x7f < uTmpB) {
            uTmpB = 0;
            __builtin_memcpy(DAT_008158b2, &uTmpB, 4);
        }
    }
    __builtin_memcpy(&uTmpA, DAT_00780c7a, 4);
    __builtin_memcpy(&pIface, DAT_007806fa, 4);
    if (((uTmpA == (unsigned int)param_1) || (param_1 == 0) || (pIface == (int *)0)) ||
        (__builtin_memcpy(&uTmpC, DAT_0078072a, 4), (int)uTmpC == param_1)) {
        __builtin_memcpy(&uTmpB, DAT_008050ae, 4);
        if (uTmpB < 0x400) {
            __builtin_memcpy(&uTmpC, DAT_008050a6, 4);
            puVar8 = DAT_007810b6 + uTmpC * 0x210;
            puVar9 = (unsigned char *)param_2;
            for (uVar6 = param_3 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
                puVar8[0] = puVar9[0];
                puVar8[1] = puVar9[1];
                puVar8[2] = puVar9[2];
                puVar8[3] = puVar9[3];
                puVar9 = puVar9 + 4;
                puVar8 = puVar8 + 4;
            }
            for (uVar6 = param_3 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
                *puVar8 = *puVar9;
                puVar9 = puVar9 + 1;
                puVar8 = puVar8 + 1;
            }
            __builtin_memcpy(&uTmpA, DAT_00780c7a, 4);
            __builtin_memcpy(DAT_007810a6 + uTmpC * 0x210, &uTmpA, 4);
            __builtin_memcpy(DAT_007810aa + uTmpC * 0x210, &param_3, 4);
            uTmpA = 0;
            __builtin_memcpy(DAT_007810ae + uTmpC * 0x210, &uTmpA, 4);
            DAT_007810b2[uTmpC * 0x210] = 0;
            DAT_007810b5[uTmpC * 0x210] = 0;
            __builtin_memcpy(&uTmpA, DAT_00780c7a, 4);
            uVar6 = FUN_004cee90((int)uTmpA);
            __builtin_memcpy(&uTmpB, DAT_0081b5be, 4);
            if (param_1 == 0) {
                DAT_007810b3[uTmpC * 0x210] = 0;
                if ((uVar6 < uTmpB) && (uVar6 < 0x28)) {
                    uTmpA = (unsigned int)((uVar7 & 0x7f00) >> 8);
                    __builtin_memcpy(DAT_00815ad6 + uVar6 * 0x238, &uTmpA, 4);
                }
            } else {
                __builtin_memcpy(&uTmpA, DAT_0078072a, 4);
                if ((int)uTmpA == param_1) {
                    DAT_007810b3[uTmpC * 0x210] = 2;
                    if ((uVar6 < uTmpB) && (uVar6 < 0x28)) {
                        uTmpA = (unsigned int)((uVar7 & 0x7f00) >> 8);
                        __builtin_memcpy(DAT_00815ade + uVar6 * 0x238, &uTmpA, 4);
                    }
                } else {
                    DAT_007810b3[uTmpC * 0x210] = 1;
                    if ((uVar6 < uTmpB) && (uVar6 < 0x28)) {
                        uTmpA = (unsigned int)((uVar7 & 0x7f00) >> 8);
                        __builtin_memcpy(DAT_00815ace + uVar6 * 0x238, &uTmpA, 4);
                    }
                }
            }
            DAT_007810b4[uTmpC * 0x210] = (unsigned char)(local_500[1] & 0x7f);
            uTmpC = uTmpC + 1;
            __builtin_memcpy(DAT_008050a6, &uTmpC, 4);
            __builtin_memcpy(&uTmpB, DAT_008050ae, 4);
            uTmpB = uTmpB + 1;
            __builtin_memcpy(DAT_008050ae, &uTmpB, 4);
            if (0x3ff < uTmpC) {
                uTmpC = 0;
                __builtin_memcpy(DAT_008050a6, &uTmpC, 4);
            }
        }
    }
    __builtin_memcpy(&pIface, DAT_007806fa, 4);
    if (pIface != (int *)0) {
        __builtin_memcpy(&uTmpA, DAT_00780c7a, 4);
        if ((int)uTmpA != param_1) {
            local_554 = (**(int (__stdcall **)(int *, unsigned int, int, int, void *, int))(*pIface + 0x68))
                            (pIface, uTmpA, param_1, 0, local_500, local_564);
        }
        if (local_554 != 0) {
            _sprintf((char *)local_550, s_Send_Returned____8x_00532438, local_554);
        }
        return local_554 == 0;
    }
    return 1;
}
