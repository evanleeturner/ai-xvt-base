/* SPDX-License-Identifier: MIT */
/* z_xvt 004774e0 */
extern int DAT_00525518;
extern int _DAT_006da4fc;
extern int DAT_0070acc8;
extern int _DAT_006db7bc;
extern int DAT_005f1d44;
extern int DAT_006a888e;   /* at 0x006a888e, not 4-aligned: read via memcpy */
extern int DAT_0052ae50;

/* The real __ftol takes its operand in ST0 (left there by the preceding x87
   code) and returns the truncated value in EDX:EAX. C cannot express the ST0
   operand, so the call is written exactly as Ghidra shows it. */
long long __cdecl __ftol(void);

#define IADD(a, b) ((int)((unsigned int)(a) + (unsigned int)(b)))
#define ISUB(a, b) ((int)((unsigned int)(a) - (unsigned int)(b)))
#define IMUL(a, b) ((int)((unsigned int)(a) * (unsigned int)(b)))

void __cdecl FUN_004774e0(int param_1, int param_2, int param_3, int param_4, float param_5)
{
    undefined4 *piVar1;
    undefined1 uVar2;
    undefined4 *puVar3;
    int iVar4;
    float fVar5;
    float fVar6;
    float *pfVar7;
    int iVar8;
    int iVar9;
    undefined1 *puVar10;
    undefined1 *puVar11;
    long long lVar12;
    int base;
    unsigned int uTmp;

    if (DAT_00525518 == 2) {
        iVar8 = IMUL(param_2, -2);
    } else {
        iVar8 = ISUB(0, param_2);
    }
    param_1 = IADD(param_1, iVar8);
    DAT_005f1d44 = IADD(IMUL(IADD(_DAT_006da4fc, param_4), DAT_0070acc8),
                        IMUL(_DAT_006db7bc, DAT_00525518));
    __builtin_memcpy(&base, &DAT_006a888e, 4);
    for (puVar3 = *(undefined4 **)IADD(base, IMUL(param_4, 4)); puVar3 != (undefined4 *)0;
         puVar3 = (undefined4 *)*puVar3) {
        iVar8 = (int)puVar3[2];
        iVar9 = param_2;
        if (param_2 < iVar8) {
            if (param_2 < (int)puVar3[1]) break;
            iVar4 = (int)puVar3[5];
            iVar9 = iVar8;
            if (*(float *)(iVar4 + 0x4c) < param_5) {
                iVar9 = param_2;
                if (param_5 < *(float *)(iVar4 + 0x48)) {
                    pfVar7 = (float *)(iVar4 + 0x28);
                    fVar5 = (float)param_4 * *(float *)(iVar4 + 0x2c) + *(float *)(iVar4 + 0x30) +
                            (float)param_2 * *pfVar7;
                    __builtin_memcpy(&uTmp, pfVar7, 4);
                    if (param_5 <= fVar5) {
                        iVar9 = iVar8;
                        if (0x80000000u < uTmp) {
                            if (iVar8 < param_3) {
                                if (param_5 <= (float)ISUB(iVar8, param_2) * *pfVar7 + fVar5) goto LAB_00477716;
                            } else if (param_5 <= (float)ISUB(param_3, param_2) * *pfVar7 + fVar5) {
                                return;
                            }
                            lVar12 = __ftol();
                            iVar9 = IADD(param_2, (int)lVar12);
                        }
                        goto joined_r0x00477691;
                    }
                    if (0 < (int)uTmp) {
                        if (param_3 < iVar8) {
                            fVar6 = (float)ISUB(param_3, param_2) * *pfVar7;
                        } else {
                            fVar6 = (float)ISUB(iVar8, param_2) * *pfVar7;
                        }
                        if (param_5 < fVar6 + fVar5) {
                            lVar12 = __ftol();
                            param_3 = IADD(param_2, (int)lVar12);
                            if (param_3 <= param_2) {
                                return;
                            }
                        }
                    }
                }
            } else {
joined_r0x00477691:
                if (param_3 <= iVar9) {
                    return;
                }
            }
        }
LAB_00477716:
        param_2 = iVar9;
    }
    do {
        if (puVar3 == (undefined4 *)0) {
LAB_00477aa4:
            iVar8 = ISUB(param_3, param_2);
            if (0 < iVar8) {
                if (DAT_00525518 == 2) {
                    puVar10 = (undefined1 *)IADD(param_1, IMUL(param_2, 2));
                    puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, DAT_005f1d44), IMUL(param_2, 2));
                    DAT_00525518 = 2;
                    do {
                        uVar2 = puVar10[1];
                        *puVar11 = *puVar10;
                        puVar11[1] = uVar2;
                        puVar10 = puVar10 + 2;
                        puVar11 = puVar11 + 2;
                        iVar8 = ISUB(iVar8, 1);
                    } while (iVar8 != 0);
                    return;
                }
                puVar10 = (undefined1 *)IADD(param_1, param_2);
                puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, param_2), DAT_005f1d44);
                do {
                    uVar2 = *puVar10;
                    puVar10 = puVar10 + 1;
                    *puVar11 = uVar2;
                    puVar11 = puVar11 + 1;
                    iVar8 = ISUB(iVar8, 1);
                } while (iVar8 != 0);
            }
            return;
        }
        piVar1 = puVar3 + 1;
        iVar8 = (int)*piVar1;
        if (param_3 <= iVar8) goto LAB_00477aa4;
        iVar9 = (int)puVar3[5];
        if (param_5 <= *(float *)(iVar9 + 0x4c)) {
            iVar8 = ISUB(iVar8, param_2);
            if (0 < iVar8) {
                if (DAT_00525518 == 2) {
                    puVar10 = (undefined1 *)IADD(param_1, IMUL(param_2, 2));
                    puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, DAT_005f1d44), IMUL(param_2, 2));
                    do {
                        uVar2 = puVar10[1];
                        *puVar11 = *puVar10;
                        puVar11[1] = uVar2;
                        puVar10 = puVar10 + 2;
                        puVar11 = puVar11 + 2;
                        iVar8 = ISUB(iVar8, 1);
                    } while (iVar8 != 0);
                } else {
                    puVar10 = (undefined1 *)IADD(param_1, param_2);
                    puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, param_2), DAT_005f1d44);
                    do {
                        uVar2 = *puVar10;
                        puVar10 = puVar10 + 1;
                        *puVar11 = uVar2;
                        puVar11 = puVar11 + 1;
                        iVar8 = ISUB(iVar8, 1);
                    } while (iVar8 != 0);
                }
            }
            param_2 = (int)puVar3[2];
            goto joined_r0x0047797c;
        }
        if (param_5 < *(float *)(iVar9 + 0x48)) {
            pfVar7 = (float *)(iVar9 + 0x28);
            fVar5 = (float)param_4 * *(float *)(iVar9 + 0x2c) + *(float *)(iVar9 + 0x30) +
                    (float)iVar8 * *pfVar7;
            if (param_5 <= fVar5) {
                iVar8 = ISUB(iVar8, param_2);
                if (0 < iVar8) {
                    if (DAT_00525518 == 2) {
                        puVar10 = (undefined1 *)IADD(param_1, IMUL(param_2, 2));
                        puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, DAT_005f1d44), IMUL(param_2, 2));
                        do {
                            uVar2 = puVar10[1];
                            *puVar11 = *puVar10;
                            puVar11[1] = uVar2;
                            puVar10 = puVar10 + 2;
                            puVar11 = puVar11 + 2;
                            iVar8 = ISUB(iVar8, 1);
                        } while (iVar8 != 0);
                    } else {
                        puVar10 = (undefined1 *)IADD(param_1, param_2);
                        puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, param_2), DAT_005f1d44);
                        do {
                            uVar2 = *puVar10;
                            puVar10 = puVar10 + 1;
                            *puVar11 = uVar2;
                            puVar11 = puVar11 + 1;
                            iVar8 = ISUB(iVar8, 1);
                        } while (iVar8 != 0);
                    }
                }
                param_2 = (int)puVar3[2];
                pfVar7 = (float *)((int)puVar3[5] + 0x28);
                __builtin_memcpy(&uTmp, pfVar7, 4);
                if (0x80000000u < uTmp) {
                    if (param_2 < param_3) {
                        if ((float)ISUB(param_2, (int)*piVar1) * *pfVar7 + fVar5 < param_5) {
                            lVar12 = __ftol();
                            param_2 = ISUB(param_2, (int)lVar12);
                        }
                        goto LAB_00477a9a;
                    }
                    if (param_5 <= (float)ISUB(param_3, (int)*piVar1) * *pfVar7 + fVar5) {
                        return;
                    }
                    lVar12 = __ftol();
                    param_2 = ISUB(param_3, (int)lVar12);
                }
joined_r0x0047797c:
                if (param_3 <= param_2) {
                    return;
                }
            } else {
                __builtin_memcpy(&uTmp, pfVar7, 4);
                if (0 < (int)uTmp) {
                    iVar9 = (int)puVar3[2];
                    if (iVar9 < param_3) {
                        if (param_5 < (float)ISUB(iVar9, iVar8) * *pfVar7 + fVar5) {
                            lVar12 = __ftol();
                            iVar8 = ISUB(ISUB(iVar9, (int)lVar12), param_2);
                            if (0 < iVar8) {
                                if (DAT_00525518 == 2) {
                                    puVar10 = (undefined1 *)IADD(param_1, IMUL(param_2, 2));
                                    puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, DAT_005f1d44), IMUL(param_2, 2));
                                    do {
                                        uVar2 = puVar10[1];
                                        *puVar11 = *puVar10;
                                        puVar11[1] = uVar2;
                                        puVar10 = puVar10 + 2;
                                        puVar11 = puVar11 + 2;
                                        iVar8 = ISUB(iVar8, 1);
                                    } while (iVar8 != 0);
                                } else {
                                    puVar10 = (undefined1 *)IADD(param_1, param_2);
                                    puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, param_2), DAT_005f1d44);
                                    do {
                                        uVar2 = *puVar10;
                                        puVar10 = puVar10 + 1;
                                        *puVar11 = uVar2;
                                        puVar11 = puVar11 + 1;
                                        iVar8 = ISUB(iVar8, 1);
                                    } while (iVar8 != 0);
                                }
                            }
                            param_2 = (int)puVar3[2];
                        }
                    } else if (param_5 < (float)ISUB(param_3, iVar8) * *pfVar7 + fVar5) {
                        lVar12 = __ftol();
                        iVar8 = ISUB(IADD((int)puVar3[1], (int)lVar12), param_2);
                        if (iVar8 < 1) {
                            return;
                        }
                        if (DAT_00525518 == 2) {
                            puVar10 = (undefined1 *)IADD(param_1, IMUL(param_2, 2));
                            puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, DAT_005f1d44), IMUL(param_2, 2));
                            do {
                                uVar2 = puVar10[1];
                                *puVar11 = *puVar10;
                                puVar11[1] = uVar2;
                                puVar10 = puVar10 + 2;
                                puVar11 = puVar11 + 2;
                                iVar8 = ISUB(iVar8, 1);
                            } while (iVar8 != 0);
                            return;
                        }
                        puVar10 = (undefined1 *)IADD(param_1, param_2);
                        puVar11 = (undefined1 *)IADD(IADD(DAT_0052ae50, param_2), DAT_005f1d44);
                        do {
                            uVar2 = *puVar10;
                            puVar10 = puVar10 + 1;
                            *puVar11 = uVar2;
                            puVar11 = puVar11 + 1;
                            iVar8 = ISUB(iVar8, 1);
                        } while (iVar8 != 0);
                        return;
                    }
                }
            }
        }
LAB_00477a9a:
        puVar3 = (undefined4 *)*puVar3;
    } while (1);
}
