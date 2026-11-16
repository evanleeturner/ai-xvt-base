/* SPDX-License-Identifier: MIT */
/* z_bop 00409c10 */
extern int DAT_0054f99c;
extern int DAT_0054f978;
extern int DAT_0053f8ec[];
extern unsigned short DAT_009d1260;
extern float DAT_00a081ec;

void __cdecl FUN_00409c10(int param_1, int param_2, int param_3)
{
    float *pfVar1;
    float *pfVar2;
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
    float fVar17;
    float fVar18;
    int iVar19;
    int iVar20;
    float *pfVar21;

    iVar19 = DAT_0054f99c;
    fVar3 = *(float *)(param_3 + 4 + (int)((unsigned int)param_1 * 0x18u));
    pfVar1 = (float *)(param_3 + (int)((unsigned int)param_1 * 0x18u));
    fVar4 = *(float *)(param_3 + 4 + (int)((unsigned int)param_2 * 0x18u));
    pfVar2 = (float *)(param_3 + (int)((unsigned int)param_2 * 0x18u));
    fVar13 = (float)DAT_009d1260;
    if (fVar3 <= fVar13) {
        if (fVar13 < fVar4) {
            fVar5 = pfVar1[3];
            fVar6 = pfVar2[3];
            fVar7 = pfVar1[4];
            fVar8 = pfVar2[4];
            fVar9 = pfVar1[5];
            fVar10 = pfVar2[5];
            fVar11 = pfVar1[2];
            fVar14 = *pfVar2 - *pfVar1;
            fVar12 = pfVar2[2];
            fVar15 = fVar6 - fVar5;
            fVar16 = fVar8 - fVar7;
            fVar17 = fVar10 - fVar9;
            fVar18 = fVar12 - fVar11;
            if (fVar4 - fVar13 <= fVar13 - fVar3) {
                fVar3 = (fVar4 - fVar13) / (fVar4 - fVar3);
                pfVar21 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                *pfVar21 = *pfVar2 - fVar3 * fVar14;
                pfVar21[3] = fVar6 - fVar15 * fVar3;
                fVar4 = fVar12 - fVar18 * fVar3;
                if ((fVar18 < 0.0f ? -fVar18 : fVar18) == 0.0f) {
                    pfVar21[2] = fVar4;
                } else {
                    pfVar21[2] = fVar4;
                    fVar3 = DAT_00a081ec;
                    fVar3 = (fVar3 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar19 * 0x18u)) - fVar3 / fVar12) /
                            (fVar3 / fVar11 - fVar3 / fVar12);
                }
                pfVar21[4] = fVar8 - fVar3 * fVar16;
                fVar9 = fVar10 - fVar3 * fVar17;
            } else {
                fVar3 = (fVar13 - fVar3) / (fVar4 - fVar3);
                pfVar21 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                *pfVar21 = fVar3 * fVar14 + *pfVar1;
                pfVar21[3] = fVar15 * fVar3 + fVar5;
                fVar4 = fVar18 * fVar3 + fVar11;
                if ((fVar18 < 0.0f ? -fVar18 : fVar18) == 0.0f) {
                    pfVar21[2] = fVar4;
                    fVar9 = fVar3 * fVar17 + fVar9;
                    pfVar21[4] = fVar3 * fVar16 + fVar7;
                } else {
                    pfVar21[2] = fVar4;
                    fVar3 = DAT_00a081ec;
                    fVar3 = (fVar3 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar19 * 0x18u)) - fVar3 / fVar11) /
                            (fVar3 / fVar12 - fVar3 / fVar11);
                    fVar9 = fVar3 * fVar17 + fVar9;
                    pfVar21[4] = fVar3 * fVar16 + fVar7;
                }
            }
            pfVar21[5] = fVar9;
            pfVar21[1] = (float)DAT_009d1260;
            DAT_0054f978 = (int)((unsigned int)DAT_0054f978 + 1u);
            DAT_0053f8ec[DAT_0054f978] = iVar19;
            return;
        }
        DAT_0054f978 = (int)((unsigned int)DAT_0054f978 + 1u);
        DAT_0053f8ec[DAT_0054f978] = param_2;
    } else if (fVar4 < fVar13) {
        fVar5 = pfVar2[3];
        fVar6 = pfVar1[3];
        fVar7 = pfVar2[4];
        fVar8 = pfVar1[4];
        fVar9 = pfVar2[5];
        fVar10 = pfVar1[5];
        fVar11 = pfVar2[2];
        fVar14 = *pfVar2 - *pfVar1;
        fVar12 = pfVar1[2];
        fVar15 = fVar5 - fVar6;
        fVar16 = fVar7 - fVar8;
        fVar17 = fVar9 - fVar10;
        fVar18 = fVar11 - fVar12;
        if (fVar13 - fVar4 <= fVar3 - fVar13) {
            fVar3 = (fVar13 - fVar4) / (fVar4 - fVar3);
            pfVar21 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            *pfVar21 = fVar3 * fVar14 + *pfVar2;
            pfVar21[3] = fVar15 * fVar3 + fVar5;
            fVar4 = fVar18 * fVar3 + fVar11;
            if ((fVar18 < 0.0f ? -fVar18 : fVar18) == 0.0f) {
                pfVar21[2] = fVar4;
                fVar8 = fVar3 * fVar16 + fVar7;
                fVar9 = fVar3 * fVar17 + fVar9;
            } else {
                pfVar21[2] = fVar4;
                fVar3 = DAT_00a081ec;
                fVar3 = (fVar3 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar19 * 0x18u)) - fVar3 / fVar11) /
                        (fVar3 / fVar12 - fVar3 / fVar11);
                fVar8 = fVar7 - fVar3 * fVar16;
                fVar9 = fVar9 - fVar3 * fVar17;
            }
        } else {
            fVar3 = (fVar3 - fVar13) / (fVar4 - fVar3);
            pfVar21 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            *pfVar21 = *pfVar1 - fVar3 * fVar14;
            pfVar21[3] = fVar6 - fVar15 * fVar3;
            fVar4 = fVar12 - fVar18 * fVar3;
            if ((fVar18 < 0.0f ? -fVar18 : fVar18) == 0.0f) {
                pfVar21[2] = fVar4;
                fVar8 = fVar8 - fVar3 * fVar16;
                fVar9 = fVar10 - fVar3 * fVar17;
            } else {
                pfVar21[2] = fVar4;
                fVar3 = DAT_00a081ec;
                fVar3 = (fVar3 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar19 * 0x18u)) - fVar3 / fVar12) /
                        (fVar3 / fVar11 - fVar3 / fVar12);
                fVar8 = fVar3 * fVar16 + fVar8;
                fVar9 = fVar3 * fVar17 + fVar10;
            }
        }
        pfVar21[4] = fVar8;
        pfVar21[5] = fVar9;
        pfVar21[1] = (float)DAT_009d1260;
        iVar20 = (int)((unsigned int)DAT_0054f978 + 2u);
        DAT_0054f978 = iVar20;
        DAT_0053f8ec[iVar20 - 1] = iVar19;
        DAT_0053f8ec[iVar20] = param_2;
        return;
    }
    return;
}
