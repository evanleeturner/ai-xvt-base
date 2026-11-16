/* SPDX-License-Identifier: MIT */
/* z_bop 004285a0 */
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;

int FUN_004af000(int param_1, int param_2);
long long __ftol(void); /* operand is on the x87 stack; source unresolved */
long long FUN_0046a6d0(short param_1);
long long FUN_0046a3c0(short param_1);

void __cdecl FUN_004285a0(short param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    long long lVar7;
    long long lVar8;
    long long lVar9;
    int local_b4;
    int local_b0;
    int local_ac;
    int local_a8;
    int local_a4;
    int local_a0;
    int local_9c;

    DAT_009d8a58 = param_4;
    DAT_009d8a54 = param_5;
    DAT_009d8b60 = param_6;
    iVar1 = FUN_004af000(param_2, param_3);
    if (iVar1 != 0) {
        lVar7 = __ftol();
        iVar1 = (int)lVar7;
        lVar7 = __ftol();
        iVar3 = (int)lVar7;
        lVar7 = __ftol();
        iVar4 = (int)lVar7;
        lVar7 = FUN_0046a6d0(param_1);
        iVar2 = (int)(short)lVar7;
        lVar7 = FUN_0046a3c0(param_1);
        iVar5 = (int)(short)lVar7;
        uVar1 = (unsigned int)iVar1;
        uVar2 = (unsigned int)iVar2;
        uVar3 = (unsigned int)iVar3;
        uVar4 = (unsigned int)iVar4;
        uVar5 = (unsigned int)iVar5;
        if (iVar2 < 0) {
            uVar6 = 0U - uVar2;
            iVar6 = (int)uVar6;
            local_ac = (int)((unsigned int)((int)(uVar1 * uVar1) >> 0xf) * uVar6 + uVar1 * uVar1 + uVar2 * 0x8000U);
            if (0x3fffffff < local_ac) {
                local_ac = 0x3fffffff;
            }
            if (local_ac < -0x3fffffff) {
                local_ac = -0x3fff0000;
            }
            local_a4 = (int)((unsigned int)((int)(uVar1 * uVar3) >> 0xf) * uVar6 + uVar1 * uVar3 + ((uVar5 * uVar4) & 0xffff8000U));
            if (0x3fffffff < local_a4) {
                local_a4 = 0x3fffffff;
            }
            if (local_a4 < -0x3fffffff) {
                local_a4 = -0x3fff0000;
            }
            local_9c = (int)((unsigned int)((int)(uVar1 * uVar4) >> 0xf) * uVar6 + uVar1 * uVar4 + ((uVar5 * uVar3) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_9c) {
                local_9c = 0x3fffffff;
            }
            if (local_9c < -0x3fffffff) {
                local_9c = -0x3fff0000;
            }
            local_b0 = (int)((unsigned int)((int)(uVar1 * uVar3) >> 0xf) * uVar6 + uVar1 * uVar3 + ((uVar5 * uVar4) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_b0) {
                local_b0 = 0x3fffffff;
            }
            if (local_b0 < -0x3fffffff) {
                local_b0 = -0x3fff0000;
            }
            local_a8 = (int)((unsigned int)((int)(uVar3 * uVar3) >> 0xf) * uVar6 + uVar3 * uVar3 + uVar2 * 0x8000U);
            if (0x3fffffff < local_a8) {
                local_a8 = 0x3fffffff;
            }
            if (local_a8 < -0x3fffffff) {
                local_a8 = -0x3fff0000;
            }
            local_a0 = (int)((unsigned int)((int)(uVar3 * uVar4) >> 0xf) * uVar6 + uVar3 * uVar4 + ((uVar5 * uVar1) & 0xffff8000U));
            if (0x3fffffff < local_a0) {
                local_a0 = 0x3fffffff;
            }
            if (local_a0 < -0x3fffffff) {
                local_a0 = -0x3fff0000;
            }
            local_b4 = (int)((unsigned int)((int)(uVar1 * uVar4) >> 0xf) * uVar6 + uVar1 * uVar4 + ((uVar5 * uVar3) & 0xffff8000U));
            if (0x3fffffff < local_b4) {
                local_b4 = 0x3fffffff;
            }
            if (local_b4 < -0x3fffffff) {
                local_b4 = -0x3fff0000;
            }
            iVar1 = (int)((unsigned int)((int)(uVar3 * uVar4) >> 0xf) * uVar6 + uVar3 * uVar4 + ((uVar5 * uVar1) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < iVar1) {
                iVar1 = 0x3fffffff;
            }
            if (iVar1 < -0x3fffffff) {
                iVar1 = -0x3fff0000;
            }
            iVar3 = (int)((unsigned int)((int)(uVar4 * uVar4) >> 0xf) * uVar6 + uVar4 * uVar4 + uVar2 * 0x8000U);
            if (0x3fffffff < iVar3) {
                iVar3 = 0x3fffffff;
            }
            if (iVar3 < -0x3fffffff) {
                iVar3 = -0x3fff0000;
            }
        }
        else {
            uVar6 = 0x7fffU - uVar2;
            iVar6 = (int)uVar6;
            local_ac = (int)((unsigned int)((int)(uVar1 * uVar1) >> 0xf) * uVar6 + uVar2 * 0x8000U);
            if (0x3fffffff < local_ac) {
                local_ac = 0x3fffffff;
            }
            if (local_ac < -0x3fffffff) {
                local_ac = -0x3fff0000;
            }
            local_a4 = (int)((unsigned int)((int)(uVar1 * uVar3) >> 0xf) * uVar6 + ((uVar5 * uVar4) & 0xffff8000U));
            if (0x3fffffff < local_a4) {
                local_a4 = 0x3fffffff;
            }
            if (local_a4 < -0x3fffffff) {
                local_a4 = -0x3fff0000;
            }
            local_9c = (int)((unsigned int)((int)(uVar1 * uVar4) >> 0xf) * uVar6 + ((uVar5 * uVar3) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_9c) {
                local_9c = 0x3fffffff;
            }
            if (local_9c < -0x3fffffff) {
                local_9c = -0x3fff0000;
            }
            local_b0 = (int)((unsigned int)((int)(uVar1 * uVar3) >> 0xf) * uVar6 + ((uVar5 * uVar4) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_b0) {
                local_b0 = 0x3fffffff;
            }
            if (local_b0 < -0x3fffffff) {
                local_b0 = -0x3fff0000;
            }
            local_a8 = (int)((unsigned int)((int)(uVar3 * uVar3) >> 0xf) * uVar6 + uVar2 * 0x8000U);
            if (0x3fffffff < local_a8) {
                local_a8 = 0x3fffffff;
            }
            if (local_a8 < -0x3fffffff) {
                local_a8 = -0x3fff0000;
            }
            local_a0 = (int)((unsigned int)((int)(uVar3 * uVar4) >> 0xf) * uVar6 + ((uVar5 * uVar1) & 0xffff8000U));
            if (0x3fffffff < local_a0) {
                local_a0 = 0x3fffffff;
            }
            if (local_a0 < -0x3fffffff) {
                local_a0 = -0x3fff0000;
            }
            local_b4 = (int)((unsigned int)((int)(uVar1 * uVar4) >> 0xf) * uVar6 + ((uVar5 * uVar3) & 0xffff8000U));
            if (0x3fffffff < local_b4) {
                local_b4 = 0x3fffffff;
            }
            if (local_b4 < -0x3fffffff) {
                local_b4 = -0x3fff0000;
            }
            iVar1 = (int)((unsigned int)((int)(uVar3 * uVar4) >> 0xf) * uVar6 + ((uVar5 * uVar1) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < iVar1) {
                iVar1 = 0x3fffffff;
            }
            if (iVar1 < -0x3fffffff) {
                iVar1 = -0x3fff0000;
            }
            iVar3 = (int)((unsigned int)((int)(uVar4 * uVar4) >> 0xf) * uVar6 + uVar2 * 0x8000U);
            if (0x3fffffff < iVar3) {
                iVar3 = 0x3fffffff;
            }
            if (iVar3 < -0x3fffffff) {
                iVar3 = -0x3fff0000;
            }
        }
        local_9c = local_9c >> 0xf;
        local_a0 = local_a0 >> 0xf;
        local_a4 = local_a4 >> 0xf;
        local_a8 = local_a8 >> 0xf;
        local_ac = local_ac >> 0xf;
        local_b0 = local_b0 >> 0xf;
        local_b4 = local_b4 >> 0xf;
        lVar7 = __ftol();
        iVar2 = (int)((unsigned int)param_4 - (unsigned int)(int)lVar7);
        lVar7 = __ftol();
        iVar5 = (int)((unsigned int)param_5 + (unsigned int)(int)lVar7);
        lVar7 = __ftol();
        iVar6 = (int)((unsigned int)param_6 - (unsigned int)(int)lVar7);
        iVar4 = (int)((unsigned int)iVar2 * (unsigned int)local_ac + (unsigned int)iVar5 * (unsigned int)local_b0 + (unsigned int)iVar6 * (unsigned int)local_b4);
        if (0x3fffffff < iVar4) {
            iVar4 = 0x3fffffff;
        }
        if (iVar4 < -0x3fffffff) {
            iVar4 = -0x3fff0000;
        }
        iVar1 = (int)((unsigned int)iVar2 * (unsigned int)local_a4 + (unsigned int)iVar5 * (unsigned int)local_a8 + (unsigned int)iVar6 * (unsigned int)(iVar1 >> 0xf));
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        iVar3 = (int)((unsigned int)iVar2 * (unsigned int)local_9c + (unsigned int)iVar5 * (unsigned int)local_a0 + (unsigned int)iVar6 * (unsigned int)(iVar3 >> 0xf));
        if (0x3fffffff < iVar3) {
            iVar3 = 0x3fffffff;
        }
        if (iVar3 < -0x3fffffff) {
            iVar3 = -0x3fff0000;
        }
        lVar7 = __ftol();
        lVar8 = __ftol();
        lVar9 = __ftol();
        DAT_009d8a58 = (int)((unsigned int)(iVar4 >> 0xf) + (unsigned int)(int)lVar9);
        DAT_009d8a54 = (int)((unsigned int)(iVar1 >> 0xf) - (unsigned int)(int)lVar7);
        DAT_009d8b60 = (int)((unsigned int)(iVar3 >> 0xf) + (unsigned int)(int)lVar8);
    }
    return;
}
