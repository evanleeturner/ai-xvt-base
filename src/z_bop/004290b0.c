/* SPDX-License-Identifier: MIT */
/* z_bop 004290b0 */
extern int DAT_00a0048c;
extern int DAT_00a00494;
extern int DAT_00a0049c;
extern int DAT_00a004ac;
extern int DAT_00a004b4;
extern int DAT_00a004d0;
extern int DAT_00a07c68;
extern int DAT_00a07c6c;
extern int DAT_00a07cc4;

long long __cdecl FUN_0046a6d0(short param_1);
long long __cdecl FUN_0046a3c0(short param_1);

void __cdecl FUN_004290b0(int param_1, int param_2, int param_3, short param_4)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    long long lVar8;
    long long lVar9;
    int local_30;
    int local_2c;
    int local_28;
    int local_24;
    int local_20;
    int local_1c;
    int local_14;
    int local_10;

    if (param_4 != 0) {
        lVar8 = FUN_0046a6d0(param_4);
        lVar9 = FUN_0046a3c0(param_4);
        iVar1 = (int)(short)lVar8;
        if ((short)lVar8 < 0) {
            iVar6 = (int)(0U - (unsigned int)iVar1);
            local_28 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_1) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_1 * (unsigned int)param_1 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < local_28) {
                local_28 = 0x3fffffff;
            }
            if (local_28 < -0x3fffffff) {
                local_28 = -0x3fff0000;
            }
            iVar7 = (int)(short)lVar9;
            local_1c = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_1 * (unsigned int)param_2 + ((unsigned int)iVar7 * (unsigned int)param_3 & 0xffff8000U));
            if (0x3fffffff < local_1c) {
                local_1c = 0x3fffffff;
            }
            if (local_1c < -0x3fffffff) {
                local_1c = -0x3fff0000;
            }
            local_10 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_1 * (unsigned int)param_3 + (((unsigned int)iVar7 * (unsigned int)param_2) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_10) {
                local_10 = 0x3fffffff;
            }
            if (local_10 < -0x3fffffff) {
                local_10 = -0x3fff0000;
            }
            local_2c = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_1 * (unsigned int)param_2 + (((unsigned int)iVar7 * (unsigned int)param_3) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_2c) {
                local_2c = 0x3fffffff;
            }
            if (local_2c < -0x3fffffff) {
                local_2c = -0x3fff0000;
            }
            local_20 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_2 * (unsigned int)param_2 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < local_20) {
                local_20 = 0x3fffffff;
            }
            if (local_20 < -0x3fffffff) {
                local_20 = -0x3fff0000;
            }
            local_14 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_2 * (unsigned int)param_3 + ((unsigned int)iVar7 * (unsigned int)param_1 & 0xffff8000U));
            if (0x3fffffff < local_14) {
                local_14 = 0x3fffffff;
            }
            if (local_14 < -0x3fffffff) {
                local_14 = -0x3fff0000;
            }
            local_30 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_1 * (unsigned int)param_3 + ((unsigned int)iVar7 * (unsigned int)param_2 & 0xffff8000U));
            if (0x3fffffff < local_30) {
                local_30 = 0x3fffffff;
            }
            if (local_30 < -0x3fffffff) {
                local_30 = -0x3fff0000;
            }
            local_24 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_2 * (unsigned int)param_3 + (((unsigned int)iVar7 * (unsigned int)param_1) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_24) {
                local_24 = 0x3fffffff;
            }
            if (local_24 < -0x3fffffff) {
                local_24 = -0x3fff0000;
            }
            iVar1 = (int)((unsigned int)((int)((unsigned int)param_3 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)param_3 * (unsigned int)param_3 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < iVar1) {
                iVar1 = 0x3fffffff;
            }
            if (iVar1 < -0x3fffffff) {
                iVar1 = -0x3fff0000;
            }
        }
        else {
            iVar6 = 0x7fff - iVar1;
            local_28 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_1) >> 0xf) * (unsigned int)iVar6 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < local_28) {
                local_28 = 0x3fffffff;
            }
            if (local_28 < -0x3fffffff) {
                local_28 = -0x3fff0000;
            }
            iVar7 = (int)(short)lVar9;
            local_1c = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + ((unsigned int)iVar7 * (unsigned int)param_3 & 0xffff8000U));
            if (0x3fffffff < local_1c) {
                local_1c = 0x3fffffff;
            }
            if (local_1c < -0x3fffffff) {
                local_1c = -0x3fff0000;
            }
            local_10 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (((unsigned int)iVar7 * (unsigned int)param_2) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_10) {
                local_10 = 0x3fffffff;
            }
            if (local_10 < -0x3fffffff) {
                local_10 = -0x3fff0000;
            }
            local_2c = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + (((unsigned int)iVar7 * (unsigned int)param_3) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_2c) {
                local_2c = 0x3fffffff;
            }
            if (local_2c < -0x3fffffff) {
                local_2c = -0x3fff0000;
            }
            local_20 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_2) >> 0xf) * (unsigned int)iVar6 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < local_20) {
                local_20 = 0x3fffffff;
            }
            if (local_20 < -0x3fffffff) {
                local_20 = -0x3fff0000;
            }
            local_14 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + ((unsigned int)iVar7 * (unsigned int)param_1 & 0xffff8000U));
            if (0x3fffffff < local_14) {
                local_14 = 0x3fffffff;
            }
            if (local_14 < -0x3fffffff) {
                local_14 = -0x3fff0000;
            }
            local_30 = (int)((unsigned int)((int)((unsigned int)param_1 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + ((unsigned int)iVar7 * (unsigned int)param_2 & 0xffff8000U));
            if (0x3fffffff < local_30) {
                local_30 = 0x3fffffff;
            }
            if (local_30 < -0x3fffffff) {
                local_30 = -0x3fff0000;
            }
            local_24 = (int)((unsigned int)((int)((unsigned int)param_2 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (((unsigned int)iVar7 * (unsigned int)param_1) >> 0xf) * 0xffff8000U);
            if (0x3fffffff < local_24) {
                local_24 = 0x3fffffff;
            }
            if (local_24 < -0x3fffffff) {
                local_24 = -0x3fff0000;
            }
            iVar1 = (int)((unsigned int)((int)((unsigned int)param_3 * (unsigned int)param_3) >> 0xf) * (unsigned int)iVar6 + (unsigned int)iVar1 * 0x8000U);
            if (0x3fffffff < iVar1) {
                iVar1 = 0x3fffffff;
            }
            if (iVar1 < -0x3fffffff) {
                iVar1 = -0x3fff0000;
            }
        }
        local_10 = local_10 >> 0xf;
        local_14 = local_14 >> 0xf;
        local_1c = local_1c >> 0xf;
        local_20 = local_20 >> 0xf;
        local_24 = local_24 >> 0xf;
        local_28 = local_28 >> 0xf;
        local_2c = local_2c >> 0xf;
        local_30 = local_30 >> 0xf;
        iVar1 = iVar1 >> 0xf;

        iVar6 = (int)((unsigned int)DAT_00a0049c * (unsigned int)local_28 + (unsigned int)DAT_00a0048c * (unsigned int)local_2c + (unsigned int)DAT_00a00494 * (unsigned int)local_30);
        if (0x3fffffff < iVar6) {
            iVar6 = 0x3fffffff;
        }
        if (iVar6 < -0x3fffffff) {
            iVar6 = -0x3fff0000;
        }
        iVar7 = (int)((unsigned int)DAT_00a0049c * (unsigned int)local_1c + (unsigned int)DAT_00a0048c * (unsigned int)local_20 + (unsigned int)DAT_00a00494 * (unsigned int)local_24);
        if (0x3fffffff < iVar7) {
            iVar7 = 0x3fffffff;
        }
        if (iVar7 < -0x3fffffff) {
            iVar7 = -0x3fff0000;
        }
        iVar2 = (int)((unsigned int)DAT_00a0049c * (unsigned int)local_10 + (unsigned int)DAT_00a0048c * (unsigned int)local_14 + (unsigned int)DAT_00a00494 * (unsigned int)iVar1);
        if (0x3fffffff < iVar2) {
            iVar2 = 0x3fffffff;
        }
        if (iVar2 < -0x3fffffff) {
            iVar2 = -0x3fff0000;
        }
        DAT_00a00494 = iVar2 >> 0xf;

        iVar2 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)local_28 + (unsigned int)DAT_00a004ac * (unsigned int)local_2c + (unsigned int)DAT_00a004b4 * (unsigned int)local_30);
        if (0x3fffffff < iVar2) {
            iVar2 = 0x3fffffff;
        }
        if (iVar2 < -0x3fffffff) {
            iVar2 = -0x3fff0000;
        }
        iVar3 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)local_1c + (unsigned int)DAT_00a004ac * (unsigned int)local_20 + (unsigned int)DAT_00a004b4 * (unsigned int)local_24);
        if (0x3fffffff < iVar3) {
            iVar3 = 0x3fffffff;
        }
        if (iVar3 < -0x3fffffff) {
            iVar3 = -0x3fff0000;
        }
        iVar4 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)local_10 + (unsigned int)DAT_00a004ac * (unsigned int)local_14 + (unsigned int)DAT_00a004b4 * (unsigned int)iVar1);
        if (0x3fffffff < iVar4) {
            iVar4 = 0x3fffffff;
        }
        if (iVar4 < -0x3fffffff) {
            iVar4 = -0x3fff0000;
        }
        DAT_00a004b4 = iVar4 >> 0xf;

        iVar4 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)local_28 + (unsigned int)DAT_00a07c6c * (unsigned int)local_2c + (unsigned int)DAT_00a07cc4 * (unsigned int)local_30);
        if (0x3fffffff < iVar4) {
            iVar4 = 0x3fffffff;
        }
        if (iVar4 < -0x3fffffff) {
            iVar4 = -0x3fff0000;
        }
        iVar5 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)local_1c + (unsigned int)DAT_00a07c6c * (unsigned int)local_20 + (unsigned int)DAT_00a07cc4 * (unsigned int)local_24);
        if (0x3fffffff < iVar5) {
            iVar5 = 0x3fffffff;
        }
        if (iVar5 < -0x3fffffff) {
            iVar5 = -0x3fff0000;
        }
        iVar1 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)local_10 + (unsigned int)DAT_00a07c6c * (unsigned int)local_14 + (unsigned int)DAT_00a07cc4 * (unsigned int)iVar1);
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        DAT_00a07cc4 = iVar1 >> 0xf;

        DAT_00a0048c = iVar7 >> 0xf;
        DAT_00a0049c = iVar6 >> 0xf;
        DAT_00a004ac = iVar3 >> 0xf;
        DAT_00a004d0 = iVar2 >> 0xf;
        DAT_00a07c68 = iVar4 >> 0xf;
        DAT_00a07c6c = iVar5 >> 0xf;
    }
    return;
}
