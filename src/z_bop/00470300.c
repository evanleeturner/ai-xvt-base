/* SPDX-License-Identifier: MIT */
/* z_bop 00470300 */
extern int DAT_0099944a;
extern int DAT_00999462;
extern int DAT_0099945c;
extern int DAT_00999478;
extern int DAT_00a081ec;
extern int DAT_009d7680;
extern unsigned short DAT_009d682c;
extern unsigned short DAT_009a1fe4;
extern float _DAT_005181b8;
extern float _DAT_005181c0;
extern float _DAT_005181c4;
extern float _DAT_005181c8;
extern float _DAT_0099940c;

void __cdecl FUN_00420db0(int param_1, float *param_2, int param_3);
void __cdecl FUN_00420fe0(float *param_1, float *param_2);
void __cdecl FUN_004201f0(int *param_1, int param_2, float *param_3, float *param_4);

#define ADD(a, b) ((int)((unsigned)(a) + (unsigned)(b)))
#define MUL(a, b) ((int)((unsigned)(a) * (unsigned)(b)))
#define F10(k) (((float *)piVar10)[k])

void __cdecl FUN_00470300(int *param_1)
{
    float fVar1;
    float fVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    float *pfVar7;
    float *pfVar8;
    int *piVar9;
    int *piVar10;
    float local_4c;
    float local_44;
    float local_40;
    float local_3c;
    float local_38;
    float local_34;
    float local_30;
    float local_2c;
    float local_28;
    int local_24;
    int local_20;
    int local_1c;
    float local_18;
    float local_14;
    float local_10;
    float local_c;
    float local_8;
    float local_4;
    int t99944a;
    int t999462;

    __builtin_memcpy(&t999462, &DAT_00999462, 4);
    __builtin_memcpy(&t99944a, &DAT_0099944a, 4);
    iVar5 = t999462;
    piVar10 = (int *)ADD(MUL(param_1[0x2e], 0x70), t99944a);
    param_1[0x2f] = t999462;
    iVar6 = DAT_0099945c;
    iVar4 = 0;
    param_1[0x32] = 0;
    pfVar8 = (float *)ADD(iVar6, MUL(iVar5, 0x18));
    iVar6 = 0;
    if (0 < param_1[0x1e]) {
        do {
            iVar4 = ADD(iVar4, 4);
            iVar6 = ADD(iVar6, 1);
            *(int *)ADD(ADD(DAT_00999478, -4), iVar4) = -1;
        } while (iVar6 < param_1[0x1e]);
    }
    local_20 = 0;
    if (0 < param_1[0x31]) {
        do {
            FUN_00420db0((int)piVar10, (float *)ADD(param_1[0x26], MUL(*piVar10, 0x18)), (int)(param_1 + 2));
            iVar6 = param_1[0x28];
            piVar10[0x12] = 0;
            local_38 = 0.0f;
            local_24 = 0;
            F10(0x13) = (float)DAT_00a081ec;
            piVar3 = (int *)ADD(MUL(*piVar10, 0x40), iVar6);
            do {
                piVar9 = piVar3;
                iVar6 = *piVar9;
                __builtin_memcpy(&local_34, &piVar9[0xc], 4);
                if (iVar6 == -1) break;
                iVar5 = *(int *)ADD(DAT_00999478, MUL(iVar6, 4));
                if (iVar5 == -1) {
                    local_1c = MUL(iVar6, 0xc);
                    *(int *)ADD(DAT_00999478, MUL(iVar6, 4)) = param_1[0x32];
                    param_1[0x32] = ADD(param_1[0x32], 1);
                    pfVar7 = (float *)ADD(param_1[0x1f], local_1c);
                    local_44 = *pfVar7;
                    local_40 = pfVar7[1];
                    local_3c = pfVar7[2];
                    FUN_00420fe0(&local_44, (float *)(param_1 + 5));
                    local_44 = (float)param_1[2] + local_44;
                    local_40 = (float)param_1[3] + local_40;
                    local_3c = (float)param_1[4] + local_3c;
                    if (_DAT_005181c0 <= local_3c) {
                        pfVar8[2] = (float)DAT_00a081ec / local_3c;
                        *pfVar8 = pfVar8[2] * local_44;
                        pfVar8[1] = pfVar8[2] * local_40;
                        *pfVar8 = (float)(DAT_009d682c >> 1) + *pfVar8;
                        pfVar8[1] = (float)ADD((unsigned)(DAT_009a1fe4 >> 1), DAT_009d7680) + pfVar8[1];
                        local_4c = pfVar8[2];
                    } else {
                        pfVar8[2] = local_3c - _DAT_005181c0;
                        *pfVar8 = local_44;
                        pfVar8[1] = local_40;
                        piVar10[3] = -1;
                        local_4c = (float)DAT_00a081ec;
                    }
                    FUN_004201f0(param_1, (int)pfVar8,
                                 (float *)ADD(param_1[0x21], MUL(*(int *)&local_34, 0xc)),
                                 (float *)ADD(param_1[0x1f], local_1c));
                    pfVar8 = pfVar8 + 6;
                } else {
                    iVar5 = ADD(param_1[0x2f], iVar5);
                    if (*(unsigned *)ADD(ADD(DAT_0099945c, 8), MUL(iVar5, 0x18)) < 0x80000001u) {
                        local_4c = *(float *)ADD(ADD(DAT_0099945c, MUL(iVar5, 0x18)), 8);
                    } else {
                        piVar10[3] = -1;
                        local_4c = (float)DAT_00a081ec;
                    }
                }
                local_38 = local_38 + local_4c;
                if (F10(0x12) < local_4c) {
                    F10(0x12) = local_4c;
                }
                if (local_4c < F10(0x13)) {
                    F10(0x13) = local_4c;
                }
                local_24 = ADD(local_24, 1);
                piVar3 = piVar9 + 1;
            } while (local_24 < 4);
            if (param_1[0x20] != 0) {
                iVar6 = piVar9[5];
                pfVar7 = (float *)ADD(param_1[0x1f], MUL(piVar9[-3], 0xc));
                local_44 = *pfVar7;
                local_40 = pfVar7[1];
                local_3c = pfVar7[2];
                FUN_00420fe0(&local_44, (float *)(param_1 + 5));
                local_44 = (float)param_1[2] + local_44;
                local_40 = (float)param_1[3] + local_40;
                local_3c = (float)param_1[4] + local_3c;
                F10(10) = (local_44 - *(float *)ADD(ADD(param_1[0x20], 4), MUL(iVar6, 8)) * F10(7))
                          - *(float *)ADD(param_1[0x20], MUL(iVar6, 8)) * F10(4);
                F10(0xb) = (local_40 - *(float *)ADD(ADD(param_1[0x20], 4), MUL(iVar6, 8)) * F10(8))
                           - *(float *)ADD(param_1[0x20], MUL(iVar6, 8)) * F10(5);
                F10(0xc) = (local_3c - *(float *)ADD(ADD(param_1[0x20], 4), MUL(iVar6, 8)) * F10(9))
                           - *(float *)ADD(param_1[0x20], MUL(iVar6, 8)) * F10(6);
                local_18 = F10(0xc) * F10(8) - F10(9) * F10(0xb);
                local_14 = F10(9) * F10(10) - F10(0xc) * F10(7);
                local_10 = F10(0xb) * F10(7) - F10(10) * F10(8);
                local_c = F10(6) * F10(0xb) - F10(0xc) * F10(5);
                local_8 = F10(0xc) * F10(4) - F10(10) * F10(6);
                local_4 = F10(5) * F10(10) - F10(0xb) * F10(4);
                local_30 = F10(9) * F10(5) - F10(6) * F10(8);
                local_2c = F10(6) * F10(7) - F10(9) * F10(4);
                local_28 = F10(8) * F10(4) - F10(5) * F10(7);
                if (((local_30 < 0.0f ? -local_30 : local_30) == 0.0f) &&
                    ((local_2c < 0.0f ? -local_2c : local_2c) == 0.0f) &&
                    ((local_28 < 0.0f ? -local_28 : local_28) == 0.0f)) {
                    local_28 = 1.0f;
                }
                fVar1 = _DAT_005181c0 /
                        (F10(0xb) * local_2c + F10(10) * local_30 + F10(0xc) * local_28);
                fVar2 = _DAT_0099940c * fVar1;
                F10(4) = local_18 * fVar2;
                F10(5) = local_14 * fVar2;
                F10(6) = local_10 * fVar1;
                F10(7) = local_c * fVar2;
                F10(8) = local_8 * fVar2;
                F10(9) = local_4 * fVar1;
                F10(10) = local_30 * fVar2;
                F10(0xb) = fVar2 * local_2c;
                F10(0xc) = fVar1 * local_28;
                F10(6) = F10(6) - (float)(DAT_009d682c >> 1) * F10(4);
                F10(6) = F10(6) -
                         (float)ADD((unsigned)(DAT_009a1fe4 >> 1), DAT_009d7680) * F10(5);
                F10(9) = F10(9) - (float)(DAT_009d682c >> 1) * F10(7);
                F10(9) = F10(9) -
                         (float)ADD((unsigned)(DAT_009a1fe4 >> 1), DAT_009d7680) * F10(8);
                F10(0xc) = F10(0xc) - (float)(DAT_009d682c >> 1) * F10(10);
                F10(0xc) = F10(0xc) -
                           (float)ADD((unsigned)(DAT_009a1fe4 >> 1), DAT_009d7680) * F10(0xb);
                local_34 = F10(8) * F10(4) - F10(5) * F10(7);
                if (local_34 < _DAT_005181b8) {
                    local_34 = -local_34;
                }
                fVar1 = _DAT_005181c8;
                if (*piVar9 == -1) {
                    fVar1 = _DAT_005181c4;
                }
                local_38 = fVar1 / local_38;
                piVar10[0x1b] = (int)local_38; /* compiles to __ftol */
            }
            piVar10 = piVar10 + 0x1c;
            local_20 = ADD(local_20, 1);
        } while (local_20 < param_1[0x31]);
    }
    __builtin_memcpy(&t999462, &DAT_00999462, 4);
    t999462 = ADD(t999462, param_1[0x32]);
    __builtin_memcpy(&DAT_00999462, &t999462, 4);
    return;
}
