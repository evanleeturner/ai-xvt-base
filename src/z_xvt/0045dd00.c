/* SPDX-License-Identifier: MIT */
/* z_xvt 0045dd00 */
extern unsigned short DAT_00703be4;
extern int DAT_00732f3c; /* UNRESOLVED: width/type unconfirmed; the (float) cast suggests an integer type */
extern int DAT_005f10dc;
extern int DAT_005f10b8;
extern int DAT_005f102c[];

void __cdecl FUN_0045dd00(int param_1, int param_2, int param_3)
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

    iVar19 = DAT_005f10dc;
    fVar3 = *(float *)((unsigned int)param_3 + 4u + (unsigned int)param_1 * 0x18u);
    pfVar1 = (float *)((unsigned int)param_3 + (unsigned int)param_1 * 0x18u);
    fVar4 = *(float *)((unsigned int)param_3 + 4u + (unsigned int)param_2 * 0x18u);
    pfVar2 = (float *)((unsigned int)param_3 + (unsigned int)param_2 * 0x18u);
    fVar13 = (float)DAT_00703be4;
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
                pfVar21 = (float *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
                DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
                *pfVar21 = *pfVar2 - fVar3 * fVar14;
                pfVar21[3] = fVar6 - fVar15 * fVar3;
                fVar4 = fVar12 - fVar3 * fVar18;
                if (fVar18 == 0.0f) {
                    pfVar21[2] = fVar4;
                }
                else {
                    pfVar21[2] = fVar4;
                    fVar3 = (float)DAT_00732f3c;
                    fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar19 * 0x18u) - fVar3 / fVar12) /
                            (fVar3 / fVar11 - fVar3 / fVar12);
                }
                pfVar21[4] = fVar8 - fVar3 * fVar16;
                fVar9 = fVar10 - fVar3 * fVar17;
            }
            else {
                fVar3 = (fVar13 - fVar3) / (fVar4 - fVar3);
                pfVar21 = (float *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
                DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
                *pfVar21 = fVar3 * fVar14 + *pfVar1;
                pfVar21[3] = fVar15 * fVar3 + fVar5;
                fVar4 = fVar3 * fVar18 + fVar11;
                if (fVar18 == 0.0f) {
                    pfVar21[2] = fVar4;
                    fVar9 = fVar3 * fVar17 + fVar9;
                    pfVar21[4] = fVar3 * fVar16 + fVar7;
                }
                else {
                    pfVar21[2] = fVar4;
                    fVar3 = (float)DAT_00732f3c;
                    fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar19 * 0x18u) - fVar3 / fVar11) /
                            (fVar3 / fVar12 - fVar3 / fVar11);
                    fVar9 = fVar3 * fVar17 + fVar9;
                    pfVar21[4] = fVar3 * fVar16 + fVar7;
                }
            }
            pfVar21[5] = fVar9;
            pfVar21[1] = (float)DAT_00703be4;
            DAT_005f10b8 = (int)((unsigned int)DAT_005f10b8 + 1u);
            DAT_005f102c[DAT_005f10b8] = iVar19;
            return;
        }
        DAT_005f10b8 = (int)((unsigned int)DAT_005f10b8 + 1u);
        DAT_005f102c[DAT_005f10b8] = param_2;
    }
    else if (fVar4 < fVar13) {
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
            pfVar21 = (float *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
            DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
            *pfVar21 = fVar3 * fVar14 + *pfVar2;
            pfVar21[3] = fVar15 * fVar3 + fVar5;
            fVar4 = fVar3 * fVar18 + fVar11;
            if (fVar18 == 0.0f) {
                pfVar21[2] = fVar4;
                fVar8 = fVar3 * fVar16 + fVar7;
                fVar9 = fVar3 * fVar17 + fVar9;
            }
            else {
                pfVar21[2] = fVar4;
                fVar3 = (float)DAT_00732f3c;
                fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar19 * 0x18u) - fVar3 / fVar11) /
                        (fVar3 / fVar12 - fVar3 / fVar11);
                fVar8 = fVar7 - fVar3 * fVar16;
                fVar9 = fVar9 - fVar3 * fVar17;
            }
        }
        else {
            fVar3 = (fVar3 - fVar13) / (fVar4 - fVar3);
            pfVar21 = (float *)((unsigned int)param_3 + (unsigned int)DAT_005f10dc * 0x18u);
            DAT_005f10dc = (int)((unsigned int)DAT_005f10dc + 1u);
            *pfVar21 = *pfVar1 - fVar3 * fVar14;
            pfVar21[3] = fVar6 - fVar15 * fVar3;
            fVar4 = fVar12 - fVar3 * fVar18;
            if (fVar18 == 0.0f) {
                pfVar21[2] = fVar4;
                fVar8 = fVar8 - fVar3 * fVar16;
                fVar9 = fVar10 - fVar3 * fVar17;
            }
            else {
                pfVar21[2] = fVar4;
                fVar3 = (float)DAT_00732f3c;
                fVar3 = (fVar3 / *(float *)((unsigned int)param_3 + 8u + (unsigned int)iVar19 * 0x18u) - fVar3 / fVar12) /
                        (fVar3 / fVar11 - fVar3 / fVar12);
                fVar8 = fVar3 * fVar16 + fVar8;
                fVar9 = fVar3 * fVar17 + fVar10;
            }
        }
        pfVar21[4] = fVar8;
        pfVar21[5] = fVar9;
        pfVar21[1] = (float)DAT_00703be4;
        iVar20 = (int)((unsigned int)DAT_005f10b8 + 2u);
        DAT_005f10b8 = iVar20;
        /* Ghidra: *(int *)(&DAT_005f1028 + iVar20 * 4), i.e. the int slot at byte address 0x5f1028 + iVar20 * 4 */
        DAT_005f102c[(int)((unsigned int)iVar20 - 1u)] = iVar19;
        DAT_005f102c[iVar20] = param_2;
        return;
    }
    return;
}
