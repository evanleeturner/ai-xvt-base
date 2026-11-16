/* SPDX-License-Identifier: MIT */
/* z_bop 0040a6e0 */
extern int DAT_0054f99c;
extern int DAT_0054f978;
extern int DAT_0053f8ec[];
extern unsigned short DAT_009d682c;   /* P-code: INT_ZEXT of 2 bytes, then INT2FLOAT */
extern int DAT_00a081ec;              /* UNRESOLVED: width/signedness unconfirmed; global lookup returned "not found"; 4-byte signed int assumed */

void __cdecl FUN_0040a6e0(int param_1, int param_2, int param_3)
{
    int iVar1;
    float fVar2, fVar3, fVar4, fVar5, fVar6, fVar7, fVar8, fVar9;
    float fVar10, fVar11, fVar12, fVar13, fVar14, fVar15, fVar16, fVar17;
    int iVar18;
    int iVar19;
    float *pfVar20;

    iVar18 = DAT_0054f99c;
    fVar2 = *(float *)(param_3 + (int)((unsigned int)param_1 * 0x18u));
    iVar19 = (int)((unsigned int)param_3 + (unsigned int)param_1 * 0x18u);
    fVar3 = *(float *)(param_3 + (int)((unsigned int)param_2 * 0x18u));
    iVar1 = (int)((unsigned int)param_3 + (unsigned int)param_2 * 0x18u);
    fVar12 = (float)DAT_009d682c;

    if (fVar2 <= fVar12) {
        if (fVar3 > fVar12) {
            fVar4 = *(float *)(iVar19 + 0xc);
            fVar5 = *(float *)(iVar1 + 0xc);
            fVar6 = *(float *)(iVar19 + 0x10);
            fVar7 = *(float *)(iVar1 + 0x10);
            fVar8 = *(float *)(iVar19 + 0x14);
            fVar9 = *(float *)(iVar1 + 0x14);
            fVar10 = *(float *)(iVar19 + 8);
            fVar11 = *(float *)(iVar1 + 8);
            fVar13 = *(float *)(iVar1 + 4) - *(float *)(iVar19 + 4);
            fVar14 = fVar5 - fVar4;
            fVar15 = fVar7 - fVar6;
            fVar16 = fVar9 - fVar8;
            fVar17 = fVar11 - fVar10;
            if (fVar3 - fVar12 <= fVar12 - fVar2) {
                fVar2 = (fVar3 - fVar12) / (fVar3 - fVar2);
                pfVar20 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                pfVar20[1] = *(float *)(iVar1 + 4) - fVar2 * fVar13;
                pfVar20[3] = fVar5 - fVar14 * fVar2;
                fVar3 = fVar11 - fVar17 * fVar2;
                if ((fVar17 < 0.0f ? -fVar17 : fVar17) == 0.0f) {
                    pfVar20[2] = fVar3;
                } else {
                    pfVar20[2] = fVar3;
                    fVar2 = (float)DAT_00a081ec;
                    fVar2 = (fVar2 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar18 * 0x18u)) - fVar2 / fVar11) /
                            (fVar2 / fVar10 - fVar2 / fVar11);
                }
                pfVar20[4] = fVar7 - fVar15 * fVar2;
                fVar8 = fVar9 - fVar2 * fVar16;
            } else {
                fVar2 = (fVar12 - fVar2) / (fVar3 - fVar2);
                pfVar20 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
                DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
                pfVar20[1] = fVar2 * fVar13 + *(float *)(iVar19 + 4);
                pfVar20[3] = fVar2 * fVar14 + fVar4;
                fVar3 = fVar2 * fVar17 + fVar10;
                if ((fVar17 < 0.0f ? -fVar17 : fVar17) == 0.0f) {
                    pfVar20[2] = fVar3;
                    fVar8 = fVar2 * fVar16 + fVar8;
                    pfVar20[4] = fVar2 * fVar15 + fVar6;
                } else {
                    pfVar20[2] = fVar3;
                    fVar2 = (float)DAT_00a081ec;
                    fVar2 = (fVar2 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar18 * 0x18u)) - fVar2 / fVar10) /
                            (fVar2 / fVar11 - fVar2 / fVar10);
                    fVar8 = fVar2 * fVar16 + fVar8;
                    pfVar20[4] = fVar15 * fVar2 + fVar6;
                }
            }
            pfVar20[5] = fVar8;
            *pfVar20 = (float)DAT_009d682c;
            DAT_0054f978 = (int)((unsigned int)DAT_0054f978 + 1u);
            DAT_0053f8ec[DAT_0054f978] = iVar18;
            return;
        }
        DAT_0054f978 = (int)((unsigned int)DAT_0054f978 + 1u);
        DAT_0053f8ec[DAT_0054f978] = param_2;
    } else if (fVar3 <= fVar12) {
        fVar4 = *(float *)(iVar1 + 0xc);
        fVar5 = *(float *)(iVar19 + 0xc);
        fVar6 = *(float *)(iVar1 + 0x10);
        fVar7 = *(float *)(iVar19 + 0x10);
        fVar8 = *(float *)(iVar1 + 0x14);
        fVar9 = *(float *)(iVar19 + 0x14);
        fVar10 = *(float *)(iVar1 + 8);
        fVar11 = *(float *)(iVar19 + 8);
        fVar13 = *(float *)(iVar1 + 4) - *(float *)(iVar19 + 4);
        fVar14 = fVar4 - fVar5;
        fVar15 = fVar6 - fVar7;
        fVar16 = fVar8 - fVar9;
        fVar17 = fVar10 - fVar11;
        if (fVar12 - fVar3 <= fVar2 - fVar12) {
            fVar2 = (fVar12 - fVar3) / (fVar3 - fVar2);
            pfVar20 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            pfVar20[1] = fVar2 * fVar13 + *(float *)(iVar1 + 4);
            pfVar20[3] = fVar14 * fVar2 + fVar4;
            fVar3 = fVar17 * fVar2 + fVar10;
            if ((fVar17 < 0.0f ? -fVar17 : fVar17) == 0.0f) {
                pfVar20[2] = fVar3;
                fVar6 = fVar15 * fVar2 + fVar6;
                fVar9 = fVar16 * fVar2 + fVar8;
            } else {
                pfVar20[2] = fVar3;
                fVar2 = (float)DAT_00a081ec;
                fVar2 = (fVar2 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar18 * 0x18u)) - fVar2 / fVar10) /
                        (fVar2 / fVar11 - fVar2 / fVar10);
                fVar6 = fVar6 - fVar15 * fVar2;
                fVar9 = fVar8 - fVar2 * fVar16;
            }
        } else {
            fVar2 = (fVar2 - fVar12) / (fVar3 - fVar2);
            pfVar20 = (float *)(param_3 + (int)((unsigned int)DAT_0054f99c * 0x18u));
            DAT_0054f99c = (int)((unsigned int)DAT_0054f99c + 1u);
            pfVar20[1] = *(float *)(iVar19 + 4) - fVar2 * fVar13;
            pfVar20[3] = fVar5 - fVar2 * fVar14;
            fVar3 = fVar11 - fVar2 * fVar17;
            if ((fVar17 < 0.0f ? -fVar17 : fVar17) == 0.0f) {
                pfVar20[2] = fVar3;
                fVar6 = fVar7 - fVar2 * fVar15;
                fVar9 = fVar9 - fVar2 * fVar16;
            } else {
                pfVar20[2] = fVar3;
                fVar2 = (float)DAT_00a081ec;
                fVar2 = (fVar2 / *(float *)(param_3 + 8 + (int)((unsigned int)iVar18 * 0x18u)) - fVar2 / fVar11) /
                        (fVar2 / fVar10 - fVar2 / fVar11);
                fVar6 = fVar15 * fVar2 + fVar7;
                fVar9 = fVar2 * fVar16 + fVar9;
            }
        }
        pfVar20[4] = fVar6;
        pfVar20[5] = fVar9;
        *pfVar20 = (float)DAT_009d682c;
        iVar19 = (int)((unsigned int)DAT_0054f978 + 2u);
        DAT_0054f978 = iVar19;
        DAT_0053f8ec[(int)((unsigned int)iVar19 - 1u)] = iVar18;
        DAT_0053f8ec[iVar19] = param_2;
        return;
    }
    return;
}
