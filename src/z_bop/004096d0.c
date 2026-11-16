/* SPDX-License-Identifier: MIT */
/* z_bop 004096d0 */
extern int DAT_0054f99c;
extern int DAT_0054f998;
extern int DAT_0052f868[];
extern int DAT_0052f86c[];
extern float DAT_00a081ec;

void __cdecl FUN_004096d0(int param_1, int param_2, int param_3)
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
    int iVar18;
    int iVar19;
    float *pfVar20;
    unsigned int uVar3;
    unsigned int uVar4;

    iVar18 = DAT_0054f99c;
    fVar3 = *(float *)((unsigned int)param_3 + 4u + (unsigned int)param_1 * 0x18u);
    pfVar1 = (float *)((unsigned int)param_3 + (unsigned int)param_1 * 0x18u);
    fVar4 = *(float *)((unsigned int)param_3 + 4u + (unsigned int)param_2 * 0x18u);
    pfVar2 = (float *)((unsigned int)param_3 + (unsigned int)param_2 * 0x18u);
    __builtin_memcpy(&uVar3, &fVar3, 4);
    __builtin_memcpy(&uVar4, &fVar4, 4);
    if (uVar3 < 0x80000001u) {
        if (0x80000000u < uVar4) {
            fVar5 = pfVar1[3];
            fVar6 = pfVar2[3];
            fVar7 = pfVar1[4];
            fVar8 = pfVar2[4];
            fVar9 = pfVar1[5];
            fVar10 = pfVar2[5];
            fVar11 = pfVar1[2];
            fVar13 = *pfVar2 - *pfVar1;
            fVar12 = pfVar2[2];
            fVar14 = fVar6 - fVar5;
            fVar15 = fVar8 - fVar7;
            fVar16 = fVar10 - fVar9;
            fVar17 = fVar12 - fVar11;
            if (-fVar4 <= fVar3) {
                fVar3 = -fVar4 / (fVar4 - fVar3);
                pfVar20 = (float *)((unsigned int)param_3 + (unsigned int)DAT_0054f99c * 0x18u);
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                *pfVar20 = fVar3 * fVar13 + *pfVar2;
                pfVar20[3] = fVar14 * fVar3 + fVar6;
                fVar4 = fVar17 * fVar3 + fVar12;
                if (fVar17 == 0.0f) {
                    pfVar20[2] = fVar4;
                    fVar8 = fVar15 * fVar3 + fVar8;
                    fVar9 = fVar16 * fVar3 + fVar10;
                } else {
                    pfVar20[2] = fVar4;
                    fVar3 = DAT_00a081ec;
                    fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar18 * 0x18u) - fVar3 / fVar12) /
                            (fVar3 / fVar11 - fVar3 / fVar12);
                    fVar8 = fVar8 - fVar15 * fVar3;
                    fVar9 = fVar10 - fVar3 * fVar16;
                }
            } else {
                fVar3 = fVar3 / (fVar4 - fVar3);
                pfVar20 = (float *)((unsigned int)param_3 + (unsigned int)DAT_0054f99c * 0x18u);
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                *pfVar20 = *pfVar1 - fVar3 * fVar13;
                pfVar20[3] = fVar5 - fVar3 * fVar14;
                fVar4 = fVar11 - fVar3 * fVar17;
                if (fVar17 == 0.0f) {
                    pfVar20[2] = fVar4;
                    fVar8 = fVar7 - fVar3 * fVar15;
                    fVar9 = fVar9 - fVar3 * fVar16;
                } else {
                    pfVar20[2] = fVar4;
                    fVar3 = DAT_00a081ec;
                    fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar18 * 0x18u) - fVar3 / fVar11) /
                            (fVar3 / fVar12 - fVar3 / fVar11);
                    fVar8 = fVar15 * fVar3 + fVar7;
                    fVar9 = fVar3 * fVar16 + fVar9;
                }
            }
            pfVar20[4] = fVar8;
            pfVar20[5] = fVar9;
            pfVar20[1] = 0.0f;
            DAT_0054f998 = (int)((unsigned int)DAT_0054f998 + 1u);
            DAT_0052f86c[DAT_0054f998] = iVar18;
            return;
        }
        DAT_0054f998 = (int)((unsigned int)DAT_0054f998 + 1u);
        DAT_0052f86c[DAT_0054f998] = param_2;
    } else if (uVar4 < 0x80000001u) {
        fVar5 = pfVar2[3];
        fVar6 = pfVar1[3];
        fVar7 = pfVar2[4];
        fVar8 = pfVar1[4];
        fVar9 = pfVar2[5];
        fVar10 = pfVar1[5];
        fVar11 = pfVar2[2];
        fVar13 = *pfVar2 - *pfVar1;
        fVar12 = pfVar1[2];
        fVar14 = fVar5 - fVar6;
        fVar15 = fVar7 - fVar8;
        fVar16 = fVar9 - fVar10;
        fVar17 = fVar11 - fVar12;
        if (fVar4 <= -fVar3) {
            fVar4 = fVar4 / (fVar4 - fVar3);
            pfVar20 = (float *)((unsigned int)param_3 + (unsigned int)DAT_0054f99c * 0x18u);
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            *pfVar20 = *pfVar2 - fVar4 * fVar13;
            pfVar20[3] = fVar5 - fVar14 * fVar4;
            fVar3 = fVar11 - fVar17 * fVar4;
            if (fVar17 == 0.0f) {
                pfVar20[2] = fVar3;
            } else {
                pfVar20[2] = fVar3;
                fVar3 = DAT_00a081ec;
                fVar4 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar18 * 0x18u) - fVar3 / fVar11) /
                        (fVar3 / fVar12 - fVar3 / fVar11);
            }
            pfVar20[4] = fVar7 - fVar15 * fVar4;
            fVar10 = fVar9 - fVar4 * fVar16;
        } else {
            fVar3 = -fVar3 / (fVar4 - fVar3);
            pfVar20 = (float *)((unsigned int)param_3 + (unsigned int)DAT_0054f99c * 0x18u);
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            *pfVar20 = fVar3 * fVar13 + *pfVar1;
            pfVar20[3] = fVar3 * fVar14 + fVar6;
            fVar4 = fVar3 * fVar17 + fVar12;
            if (fVar17 == 0.0f) {
                pfVar20[2] = fVar4;
                fVar10 = fVar3 * fVar16 + fVar10;
                pfVar20[4] = fVar3 * fVar15 + fVar8;
            } else {
                pfVar20[2] = fVar4;
                fVar3 = DAT_00a081ec;
                fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar18 * 0x18u) - fVar3 / fVar12) /
                        (fVar3 / fVar11 - fVar3 / fVar12);
                fVar10 = fVar3 * fVar16 + fVar10;
                pfVar20[4] = fVar15 * fVar3 + fVar8;
            }
        }
        pfVar20[5] = fVar10;
        pfVar20[1] = 0.0f;
        iVar19 = (int)((unsigned int)DAT_0054f998 + 2u);
        DAT_0054f998 = iVar19;
        DAT_0052f868[iVar19] = iVar18;
        DAT_0052f86c[iVar19] = param_2;
        return;
    }
    return;
}
