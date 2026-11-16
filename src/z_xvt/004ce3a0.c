/* SPDX-License-Identifier: MIT */
/* z_xvt 004ce3a0 */
extern undefined4 DAT_007806fa;
extern undefined4 DAT_00780706;
extern undefined4 DAT_0078070a;
extern undefined4 DAT_0078070e;
extern undefined4 DAT_00780712;
extern undefined4 DAT_00780716;
extern undefined4 DAT_0078071a;
extern undefined4 DAT_0078071e;
extern undefined4 DAT_00780722;
extern undefined4 DAT_00780726;
extern undefined4 DAT_0078072a;
extern undefined4 DAT_00780c5a;
extern undefined4 DAT_00780c82;
extern undefined4 DAT_00780c86;
extern undefined4 DAT_00780e86;
extern undefined4 DAT_00780e8a;
extern undefined4 DAT_00780e8e;
extern undefined4 DAT_00780e92;
extern undefined4 DAT_00781092;
extern undefined4 DAT_00781096;
extern undefined4 DAT_007810a6;
extern undefined4 DAT_008050a6;
extern int DAT_008050aa;
extern int DAT_008050ae;
extern undefined4 DAT_008050b2;
extern undefined4 DAT_008158b2;
extern undefined4 DAT_00815ac6;
extern int DAT_0081b5be;

undefined4 __cdecl
FUN_004ce3a0(undefined4 *param_1, undefined4 *param_2, undefined4 *param_3, undefined4 *param_4,
             undefined4 *param_5, undefined4 *param_6, int param_7, int *param_8, int *param_9,
             undefined4 *param_10, undefined4 *param_11, int *param_12, undefined4 *param_13,
             undefined4 *param_14, undefined4 *param_15, undefined4 *param_16, undefined4 *param_17,
             undefined4 *param_18, undefined4 *param_19, undefined4 *param_20, undefined4 *param_21,
             undefined4 *param_22)
{
    int iVar1;
    int iVar2;
    int iVar3;
    undefined4 *puVar4;
    int iVar5;
    undefined4 *puVar6;
    undefined4 *puVar7;
    undefined4 uVar8;
    int iVar9;

    __builtin_memcpy(&uVar8, &DAT_007806fa, 4);
    *param_1 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780706, 4);
    *param_2 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_0078070a, 4);
    param_2[1] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_0078070e, 4);
    param_2[2] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780712, 4);
    param_2[3] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780716, 4);
    *param_3 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_0078071a, 4);
    param_3[1] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_0078071e, 4);
    param_3[2] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780722, 4);
    param_3[3] = uVar8;
    __builtin_memcpy(&uVar8, &DAT_0078072a, 4);
    *param_4 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780726, 4);
    *param_5 = uVar8;
    puVar6 = &DAT_00780c5a;
    for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
        __builtin_memcpy(&uVar8, puVar6, 4);
        *param_6 = uVar8;
        puVar6 = puVar6 + 1;
        param_6 = param_6 + 1;
    }
    __builtin_memcpy(&iVar9, &DAT_008050ae, 4);
    *param_9 = iVar9;
    __builtin_memcpy(&iVar9, &DAT_008050aa, 4);
    *param_8 = iVar9;
    __builtin_memcpy(&uVar8, &DAT_008050a6, 4);
    *param_10 = uVar8;
    iVar3 = 0;
    __builtin_memcpy(&iVar1, &DAT_008050aa, 4);
    __builtin_memcpy(&iVar9, &DAT_008050ae, 4);
    if (0 < iVar9) {
        do {
            iVar5 = (int)((unsigned int)iVar1 + 1u);
            puVar6 = &DAT_007810a6 + (unsigned int)iVar1 * 0x84u;
            puVar4 = (undefined4 *)((unsigned int)iVar1 * 0x210u + (unsigned int)param_7);
            for (iVar2 = 0x84; iVar2 != 0; iVar2 = iVar2 + -1) {
                __builtin_memcpy(&uVar8, puVar6, 4);
                *puVar4 = uVar8;
                puVar6 = puVar6 + 1;
                puVar4 = puVar4 + 1;
            }
            if (0x3ff < iVar5) {
                iVar5 = 0;
            }
            iVar3 = (int)((unsigned int)iVar3 + 1u);
            iVar1 = iVar5;
            __builtin_memcpy(&iVar9, &DAT_008050ae, 4);
        } while (iVar3 < iVar9);
    }
    iVar1 = 0;
    __builtin_memcpy(&iVar9, &DAT_0081b5be, 4);
    *param_12 = iVar9;
    __builtin_memcpy(&iVar9, &DAT_0081b5be, 4);
    if (0 < iVar9) {
        puVar6 = &DAT_00815ac6;
        do {
            puVar4 = puVar6;
            puVar7 = param_11;
            for (iVar3 = 0x8e; iVar3 != 0; iVar3 = iVar3 + -1) {
                __builtin_memcpy(&uVar8, puVar4, 4);
                *puVar7 = uVar8;
                puVar4 = puVar4 + 1;
                puVar7 = puVar7 + 1;
            }
            param_11 = param_11 + 0x8e;
            iVar1 = (int)((unsigned int)iVar1 + 1u);
            puVar6 = puVar6 + 0x8e;
            __builtin_memcpy(&iVar9, &DAT_0081b5be, 4);
        } while (iVar1 < iVar9);
    }
    __builtin_memcpy(&uVar8, &DAT_00780c82, 4);
    *param_13 = uVar8;
    puVar6 = &DAT_00780c86;
    for (iVar1 = 0x80; iVar1 != 0; iVar1 = iVar1 + -1) {
        __builtin_memcpy(&uVar8, puVar6, 4);
        *param_14 = uVar8;
        puVar6 = puVar6 + 1;
        param_14 = param_14 + 1;
    }
    __builtin_memcpy(&uVar8, &DAT_00780e86, 4);
    *param_15 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780e8a, 4);
    *param_16 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00780e8e, 4);
    *param_17 = uVar8;
    puVar6 = &DAT_00780e92;
    for (iVar1 = 0x80; iVar1 != 0; iVar1 = iVar1 + -1) {
        __builtin_memcpy(&uVar8, puVar6, 4);
        *param_18 = uVar8;
        puVar6 = puVar6 + 1;
        param_18 = param_18 + 1;
    }
    __builtin_memcpy(&uVar8, &DAT_00781092, 4);
    *param_19 = uVar8;
    __builtin_memcpy(&uVar8, &DAT_00781096, 4);
    *param_20 = uVar8;
    puVar6 = param_21;
    for (iVar1 = 0x4200; puVar4 = &DAT_008050b2, iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar6 = 0;
        puVar6 = puVar6 + 1;
    }
    do {
        puVar7 = puVar4 + 0x84;
        puVar6 = param_21;
        for (iVar1 = 0x84; iVar1 != 0; iVar1 = iVar1 + -1) {
            __builtin_memcpy(&uVar8, puVar4, 4);
            *puVar6 = uVar8;
            puVar4 = puVar4 + 1;
            puVar6 = puVar6 + 1;
        }
        param_21 = param_21 + 0x84;
        puVar4 = puVar7;
    } while (puVar7 < &DAT_008158b2);
    __builtin_memcpy(&uVar8, &DAT_008158b2, 4);
    *param_22 = uVar8;
    return 1;
}
