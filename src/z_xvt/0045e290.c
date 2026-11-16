/* SPDX-License-Identifier: MIT */
/* z_xvt 0045e290 */
extern int DAT_005f10dc;
extern int DAT_005f10d8;
extern undefined4 DAT_005f0fac[];
extern undefined4 DAT_005f0fa8[];
extern undefined4 DAT_00732f3c;

void __cdecl FUN_0045e290(int param_1, int param_2, int param_3)
{
    unsigned int iVar1;
    float fVar2;
    float fVar3;
    float fVar4;
    float fVar5;
    float fVar6;
    float fVar7;
    float fVar8;
    float fVar9;
    float fVar10;
    float fVar11;
    float fVar12;
    float fVar13;
    float fVar14;
    float fVar15;
    float fVar16;
    int iVar17;
    unsigned int iVar18;
    undefined4 *puVar19;

    iVar17 = DAT_005f10dc;
    fVar2 = *(float *)((unsigned int)param_3 + (unsigned int)param_1 * 0x18u);
    iVar18 = (unsigned int)param_3 + (unsigned int)param_1 * 0x18u;
    fVar3 = *(float *)((unsigned int)param_3 + (unsigned int)param_2 * 0x18u);
    iVar1 = (unsigned int)param_3 + (unsigned int)param_2 * 0x18u;
    if (*(unsigned int *)iVar18 < 0x80000001u) {
        if (0x80000000u < *(unsigned int *)iVar1) {
            fVar4 = *(float *)(iVar18 + 0xcu);
            fVar5 = *(float *)(iVar1 + 0xcu);
            fVar6 = *(float *)(iVar18 + 0x10u);
            fVar7 = *(float *)(iVar1 + 0x10u);
            fVar8 = *(float *)(iVar18 + 0x14u);
            fVar9 = *(float *)(iVar1 + 0x14u);
            fVar10 = *(float *)(iVar18 + 8u);
            fVar11 = *(float *)(iVar1 + 8u);
            fVar12 = *(float *)(iVar1 + 4u) - *(float *)(iVar18 + 4u);
            fVar13 = fVar5 - fVar4;
            fVar14 = fVar7 - fVar6;
            fVar15 = fVar9 - fVar8;
            fVar16 = fVar11 - fVar10;
            if (-fVar3 <= fVar2) {
                fVar2 = -fVar3 / (fVar3 - fVar2);
                puVar19 = (undefined4 *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
                DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
                *(float *)&puVar19[1] = fVar2 * fVar12 + *(float *)(iVar1 + 4u);
                *(float *)&puVar19[3] = fVar2 * fVar13 + fVar5;
                fVar3 = fVar2 * fVar16 + fVar11;
                if ((fVar16 < 0.0f ? -fVar16 : fVar16) == 0.0f) {
                    *(float *)&puVar19[2] = fVar3;
                    fVar7 = fVar2 * fVar14 + fVar7;
                    fVar8 = fVar2 * fVar15 + fVar9;
                } else {
                    *(float *)&puVar19[2] = fVar3;
                    fVar2 = *(float *)&DAT_00732f3c;
                    fVar2 = (fVar2 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar17 * 0x18u) - fVar2 / fVar11) /
                            (fVar2 / fVar10 - fVar2 / fVar11);
                    fVar7 = fVar7 - fVar14 * fVar2;
                    fVar8 = fVar9 - fVar2 * fVar15;
                }
            } else {
                fVar2 = fVar2 / (fVar3 - fVar2);
                puVar19 = (undefined4 *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
                DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
                *(float *)&puVar19[1] = *(float *)(iVar18 + 4u) - fVar2 * fVar12;
                *(float *)&puVar19[3] = fVar4 - fVar2 * fVar13;
                fVar3 = fVar10 - fVar2 * fVar16;
                if ((fVar16 < 0.0f ? -fVar16 : fVar16) == 0.0f) {
                    *(float *)&puVar19[2] = fVar3;
                    fVar7 = fVar6 - fVar2 * fVar14;
                    fVar8 = fVar8 - fVar2 * fVar15;
                } else {
                    *(float *)&puVar19[2] = fVar3;
                    fVar2 = *(float *)&DAT_00732f3c;
                    fVar2 = (fVar2 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar17 * 0x18u) - fVar2 / fVar10) /
                            (fVar2 / fVar11 - fVar2 / fVar10);
                    fVar7 = fVar14 * fVar2 + fVar6;
                    fVar8 = fVar2 * fVar15 + fVar8;
                }
            }
            *(float *)&puVar19[4] = fVar7;
            *(float *)&puVar19[5] = fVar8;
            *puVar19 = 0;
            DAT_005f10d8 = (int)((unsigned int)DAT_005f10d8 + 1u);
            DAT_005f0fac[DAT_005f10d8] = iVar17;
            return;
        }
        DAT_005f10d8 = (int)((unsigned int)DAT_005f10d8 + 1u);
        DAT_005f0fac[DAT_005f10d8] = param_2;
    } else if (*(unsigned int *)iVar1 < 0x80000001u) {
        fVar4 = *(float *)(iVar1 + 0xcu);
        fVar5 = *(float *)(iVar18 + 0xcu);
        fVar6 = *(float *)(iVar1 + 0x10u);
        fVar7 = *(float *)(iVar18 + 0x10u);
        fVar8 = *(float *)(iVar1 + 0x14u);
        fVar9 = *(float *)(iVar18 + 0x14u);
        fVar10 = *(float *)(iVar1 + 8u);
        fVar11 = *(float *)(iVar18 + 8u);
        fVar12 = *(float *)(iVar1 + 4u) - *(float *)(iVar18 + 4u);
        fVar13 = fVar4 - fVar5;
        fVar14 = fVar6 - fVar7;
        fVar15 = fVar8 - fVar9;
        fVar16 = fVar10 - fVar11;
        if (fVar3 <= -fVar2) {
            fVar3 = fVar3 / (fVar3 - fVar2);
            puVar19 = (undefined4 *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
            DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
            *(float *)&puVar19[1] = *(float *)(iVar1 + 4u) - fVar3 * fVar12;
            *(float *)&puVar19[3] = fVar4 - fVar3 * fVar13;
            fVar2 = fVar10 - fVar3 * fVar16;
            if ((fVar16 < 0.0f ? -fVar16 : fVar16) == 0.0f) {
                *(float *)&puVar19[2] = fVar2;
            } else {
                *(float *)&puVar19[2] = fVar2;
                fVar2 = *(float *)&DAT_00732f3c;
                fVar3 = (fVar2 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar17 * 0x18u) - fVar2 / fVar10) /
                        (fVar2 / fVar11 - fVar2 / fVar10);
            }
            *(float *)&puVar19[4] = fVar6 - fVar14 * fVar3;
            fVar9 = fVar8 - fVar3 * fVar15;
        } else {
            fVar2 = -fVar2 / (fVar3 - fVar2);
            puVar19 = (undefined4 *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
            DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
            *(float *)&puVar19[1] = fVar2 * fVar12 + *(float *)(iVar18 + 4u);
            *(float *)&puVar19[3] = fVar2 * fVar13 + fVar5;
            fVar3 = fVar2 * fVar16 + fVar11;
            if ((fVar16 < 0.0f ? -fVar16 : fVar16) == 0.0f) {
                *(float *)&puVar19[2] = fVar3;
                fVar9 = fVar2 * fVar15 + fVar9;
                *(float *)&puVar19[4] = fVar2 * fVar14 + fVar7;
            } else {
                *(float *)&puVar19[2] = fVar3;
                fVar2 = *(float *)&DAT_00732f3c;
                fVar2 = (fVar2 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar17 * 0x18u) - fVar2 / fVar11) /
                        (fVar2 / fVar10 - fVar2 / fVar11);
                fVar9 = fVar2 * fVar15 + fVar9;
                *(float *)&puVar19[4] = fVar14 * fVar2 + fVar7;
            }
        }
        *(float *)&puVar19[5] = fVar9;
        *puVar19 = 0;
        iVar18 = (unsigned int)DAT_005f10d8 + 2u;
        DAT_005f10d8 = (int)iVar18;
        DAT_005f0fa8[iVar18] = iVar17;
        DAT_005f0fac[iVar18] = param_2;
        return;
    }
    return;
}
