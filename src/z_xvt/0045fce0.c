/* SPDX-License-Identifier: MIT */
/* z_xvt 0045fce0 */
extern ushort DAT_006d4984;
extern ushort DAT_0071f714;
extern uint DAT_00732a38;
extern int DAT_006d4988;
extern int DAT_00527a00[32];
extern uint DAT_006db7b4;
extern float _DAT_005201b8;
extern float _DAT_00520194;
extern int DAT_005f10b8;
extern int DAT_005f10dc;
extern int DAT_005f1030[34];
extern int DAT_005f10d8;
extern int DAT_005f0fb0[32];
extern uint DAT_005f10d0;
extern uint DAT_005f10d4;
extern int DAT_005f0fa8;
extern uint DAT_005f10e0;
extern char *DAT_005f10b0;
extern int *DAT_005f10c8;
extern float _DAT_005f10e4;
extern float _DAT_005f10b4;
extern int DAT_005279f4;
extern int DAT_0052ae1c;

void __cdecl FUN_0045bbe0(const char *a);
int FUN_004186e0(uint a, ushort b);
int FUN_00418840(uint a, ushort b);
void FUN_0045d7d0(int a, int b, int c);
void FUN_0045dd00(int a, int b, int c);
void FUN_0045e290(int a, int b, int c);
void FUN_0045e7d0(int a, int b, int c);
void FUN_004b5bc2(void);
void FUN_004b5c5c(void *a, uint b);
void FUN_004b5ce0(void);
void FUN_004b5d64(int *a, uint b);
void FUN_004b6054(void);
int *FUN_0045bdc0(int a, int b, int c, byte *d);

void __cdecl FUN_0045fce0(ushort param_1, int param_2, int param_3, uint param_4, int param_5)
{
    float fVar1, fVar3, fVar4, fVar5;
    byte bVar2;
    int iVar6, iVar7, iVar8, iVar9, iVar10, iVar11, iVar12, iVar13, iVar17, iVar18;
    uint uVar14, uVar15, uVar19;
    int *piVar16;
    int *puVar20;
    float local_3d8;
    int local_3cc;
    float local_3c0[24];

    iVar12 = (int)((uint)DAT_006d4984 - (uint)param_3);
    if ((uint)DAT_0071f714 < DAT_00732a38) {
        iVar6 = (int)((uint)DAT_0071f714 * 0x23u + (uint)DAT_006d4988);
        if ((*(char *)(iVar6 + 2) == '\r') && (bVar2 = *(byte *)(iVar6 + 0x19), bVar2 < 0x20)) {
            local_3cc = DAT_00527a00[bVar2];
        } else {
            local_3cc = -1;
        }
    }
    local_3d8 = (float)DAT_006db7b4 * _DAT_005201b8;
    if (_DAT_00520194 < local_3d8) {
        local_3d8 = 0.999984f;
        local_3cc = -1;
    }
    iVar6 = *(int *)(param_5 + 0x10);
    iVar17 = *(int *)(param_5 + 0x14);
    if (0x100 < iVar6) {
        iVar6 = 0x100;
        FUN_0045bbe0("TRUNCATING BITMAP TO 256 WIDE!!!\n");
    }
    if (0x100 < iVar17) {
        iVar17 = 0x100;
        FUN_0045bbe0("TRUNCATING BITMAP TO 256 HIGH!!!\n");
    }
    iVar13 = 1;
    iVar7 = 0;
    do {
        iVar13 = (int)((uint)iVar13 * 2u);
        if (iVar6 <= iVar13) break;
        iVar7 = iVar7 + 1;
    } while (iVar7 < 8);
    iVar7 = 1;
    iVar8 = 0;
    do {
        iVar7 = (int)((uint)iVar7 * 2u);
        if (iVar17 <= iVar7) break;
        iVar8 = iVar8 + 1;
    } while (iVar8 < 8);
    iVar8 = *(int *)(param_5 + 0x10);
    uVar19 = (uint)((int)((uint)*(int *)(param_5 + 0x14) * (param_4 & 0xffff)) >> 9);
    iVar9 = FUN_004186e0(uVar19, param_1);
    uVar14 = (uint)((int)((uint)iVar8 * (param_4 & 0xffff)) >> 9);
    iVar8 = FUN_00418840(uVar14, param_1);
    iVar10 = FUN_00418840(uVar19, param_1);
    iVar11 = FUN_004186e0(uVar14, param_1);
    iVar18 = 0;
    DAT_005f10b8 = 4;
    DAT_005f10dc = 4;
    DAT_005f1030[0] = 0;
    DAT_005f1030[1] = 1;
    DAT_005f1030[2] = 2;
    DAT_005f1030[3] = 3;
    local_3c0[0] = (float)(int)((uint)param_2 + (uint)iVar8 + (uint)iVar9);
    local_3c0[1] = (float)(int)(((uint)iVar10 - (uint)iVar11) + (uint)iVar12);
    local_3c0[3] = 0.0f;
    local_3c0[4] = 0.0f;
    local_3c0[5] = 0.0f;
    local_3c0[2] = local_3d8;
    iVar8 = FUN_004186e0(uVar19, param_1);
    uVar15 = 0u - uVar14;
    iVar9 = FUN_00418840(uVar15, param_1);
    iVar10 = FUN_00418840(uVar19, param_1);
    iVar11 = FUN_004186e0(uVar15, param_1);
    local_3c0[6] = (float)(int)((uint)param_2 + (uint)iVar9 + (uint)iVar8);
    local_3c0[9] = 0.0f;
    local_3c0[7] = (float)(int)(((uint)iVar10 - (uint)iVar11) + (uint)iVar12);
    local_3c0[11] = 0.0f;
    local_3c0[8] = local_3d8;
    uVar19 = 0u - uVar19;
    local_3c0[10] = (float)iVar6 / (float)iVar13;
    iVar8 = FUN_004186e0(uVar19, param_1);
    iVar9 = FUN_00418840(uVar15, param_1);
    iVar10 = FUN_00418840(uVar19, param_1);
    iVar11 = FUN_004186e0(uVar15, param_1);
    local_3c0[12] = (float)(int)((uint)param_2 + (uint)iVar9 + (uint)iVar8);
    local_3c0[15] = 0.0f;
    local_3c0[13] = (float)(int)(((uint)iVar10 - (uint)iVar11) + (uint)iVar12);
    local_3c0[14] = local_3d8;
    local_3c0[16] = (float)iVar6 / (float)iVar13;
    local_3c0[17] = (float)iVar17 / (float)iVar7;
    iVar6 = FUN_004186e0(uVar19, param_1);
    iVar8 = FUN_00418840(uVar14, param_1);
    iVar9 = FUN_00418840(uVar19, param_1);
    iVar10 = FUN_004186e0(uVar14, param_1);
    local_3c0[18] = (float)(int)((uint)param_2 + (uint)iVar8 + (uint)iVar6);
    local_3c0[21] = 0.0f;
    local_3c0[22] = 0.0f;
    local_3c0[23] = (float)iVar17 / (float)iVar7;
    local_3c0[20] = local_3d8;
    DAT_005f10d8 = 0;
    local_3c0[19] = (float)(int)(((uint)iVar9 - (uint)iVar10) + (uint)iVar12);
    if (0 < DAT_005f10b8) {
        piVar16 = &DAT_005f1030[0];
        iVar12 = DAT_005f1030[DAT_005f10b8 - 1];
        do {
            iVar6 = *piVar16;
            piVar16 = piVar16 + 1;
            iVar18 = iVar18 + 1;
            FUN_0045d7d0(iVar12, iVar6, (int)local_3c0);
            iVar12 = iVar6;
        } while (iVar18 < DAT_005f10b8);
    }
    iVar12 = 0;
    DAT_005f10b8 = 0;
    if (0 < DAT_005f10d8) {
        piVar16 = &DAT_005f0fb0[0];
        iVar6 = DAT_005f0fb0[DAT_005f10d8 - 1];
        do {
            iVar17 = *piVar16;
            piVar16 = piVar16 + 1;
            iVar12 = iVar12 + 1;
            FUN_0045dd00(iVar6, iVar17, (int)local_3c0);
            iVar6 = iVar17;
        } while (iVar12 < DAT_005f10d8);
    }
    iVar12 = 0;
    DAT_005f10d8 = 0;
    if (0 < DAT_005f10b8) {
        piVar16 = &DAT_005f1030[0];
        iVar6 = DAT_005f1030[DAT_005f10b8 - 1];
        do {
            iVar17 = *piVar16;
            piVar16 = piVar16 + 1;
            iVar12 = iVar12 + 1;
            FUN_0045e290(iVar6, iVar17, (int)local_3c0);
            iVar6 = iVar17;
        } while (iVar12 < DAT_005f10b8);
    }
    iVar12 = 0;
    DAT_005f10b8 = 0;
    if (0 < DAT_005f10d8) {
        piVar16 = &DAT_005f0fb0[0];
        iVar6 = DAT_005f0fb0[DAT_005f10d8 - 1];
        do {
            iVar17 = *piVar16;
            piVar16 = piVar16 + 1;
            iVar12 = iVar12 + 1;
            FUN_0045e7d0(iVar6, iVar17, (int)local_3c0);
            iVar6 = iVar17;
        } while (iVar12 < DAT_005f10d8);
    }
    if (2 < DAT_005f10b8) {
        if ((DAT_005f10d0 < DAT_005f10d4 + (uint)DAT_005f10b8) ||
            (DAT_005f0fa8 < (int)(DAT_005f10e0 + (uint)DAT_005f10b8))) {
            FUN_004b5bc2();
            FUN_004b5c5c(DAT_005f10b0, DAT_005f10d4);
            FUN_004b5ce0();
            FUN_004b5d64(DAT_005f10c8, DAT_005f10e0);
            FUN_004b6054();
            DAT_005f10e0 = 0;
            DAT_005f10d4 = 0;
        }
        if (DAT_005279f4 != 0) {
            local_3cc = -0x1000001;
            DAT_005279f4 = 0;
        }
        iVar12 = 0;
        if (0 < DAT_005f10b8) {
            piVar16 = &DAT_005f1030[0];
            do {
                iVar6 = *piVar16;
                iVar12 = iVar12 + 1;
                fVar1 = local_3c0[iVar6 * 6 + 1];
                fVar3 = local_3c0[iVar6 * 6 + 4];
                fVar4 = local_3c0[iVar6 * 6 + 2];
                fVar5 = local_3c0[iVar6 * 6 + 5];
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20) = local_3c0[iVar6 * 6] + _DAT_005f10e4;
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 4) = fVar1 + _DAT_005f10b4;
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 8) = fVar4;
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 0xc) = fVar4;
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 0x18) = fVar3;
                *(float *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 0x1c) = fVar5;
                *(int *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 0x10) = local_3cc;
                *(int *)(DAT_005f10b0 + DAT_005f10d4 * 0x20 + 0x14) = 0;
                iVar6 = DAT_005f10b8;
                *piVar16 = (int)DAT_005f10d4;
                DAT_005f10d4 = DAT_005f10d4 + 1;
                piVar16 = piVar16 + 1;
            } while (iVar12 < iVar6);
        }
        iVar12 = 2;
        piVar16 = FUN_0045bdc0(iVar13, iVar7, *(int *)(param_5 + 0xc) + param_5,
                               (byte *)(*(int *)(param_5 + 8) + 0x10 + param_5));
        if (2 < DAT_005f10b8) {
            puVar20 = &DAT_005f1030[1];
            do {
                DAT_005f10c8[DAT_005f10e0 * 5] = DAT_005f1030[0];
                DAT_005f10c8[DAT_005f10e0 * 5 + 1] = puVar20[0];
                DAT_005f10c8[DAT_005f10e0 * 5 + 2] = puVar20[1];
                DAT_005f10c8[DAT_005f10e0 * 5 + 3] = (int)piVar16;
                DAT_005f10c8[DAT_005f10e0 * 5 + 4] = 0x3022;
                if (DAT_0052ae1c != 0) {
                    DAT_005f10c8[DAT_005f10e0 * 5 + 4] = DAT_005f10c8[DAT_005f10e0 * 5 + 4] + 0x300;
                }
                if (local_3cc != -1) {
                    DAT_005f10c8[DAT_005f10e0 * 5 + 4] = DAT_005f10c8[DAT_005f10e0 * 5 + 4] + 0x400;
                }
                iVar12 = iVar12 + 1;
                DAT_005f10e0 = DAT_005f10e0 + 1;
                puVar20 = puVar20 + 1;
            } while (iVar12 < DAT_005f10b8);
        }
    }
    return;
}
