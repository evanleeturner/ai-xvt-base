/* SPDX-License-Identifier: MIT */
/* z_bop 004201f0 */
extern int DAT_005233c0;
extern int _DAT_00a08150;
extern int _DAT_00a081e0;
extern int _DAT_00a081e4;
extern float _DAT_00518098;
extern float _DAT_0051809c;
extern float _DAT_005180a0;
extern float _DAT_005180a8;
extern float _DAT_005180ac;
extern double _DAT_005180b0;
extern float _DAT_005180b8;
extern float _DAT_005180bc;
extern float _DAT_005180c0;
extern int DAT_005235f0;
extern int DAT_009fd3b0[];
extern int DAT_00527e90;
extern int DAT_005233bc;

int __cdecl FUN_00473580(int param_1, float *param_2, float *param_3);

void __cdecl FUN_004201f0(int *param_1, int param_2, float *param_3, float *param_4, float *in_stack_00000014)
{
    float fVar1, fVar2, fVar3, fVar4, fVar5;
    int iVar6;
    int *piVar7;
    float local_30, local_2c, local_28, fStack_24;
    float local_14;
    int local_10;
    float local_c, local_8, local_4;
    unsigned int bits;

    if ((*(char *)(*param_1 + 2) == '\a') || (*(char *)(*param_1 + 2) == '\x06')) {
        *(unsigned int *)(param_2 + 0xc) = 0x3f800000;
    }
    else {
        if (DAT_005233c0 == 0) {
            *(unsigned int *)(param_2 + 0xc) = 0x3ecccccd;
        }
        else {
            *(float *)(param_2 + 0xc) =
                ((float)_DAT_00a08150 * _DAT_00518098 * param_3[2] +
                 (float)_DAT_00a081e0 * _DAT_00518098 * *param_3 +
                 (float)_DAT_00a081e4 * _DAT_00518098 * param_3[1]) * _DAT_0051809c;
            if (*(unsigned int *)(param_2 + 0xc) < 0x80000001) {
                local_4 = (float)_DAT_00a08150 + param_4[2];
                local_c = (float)_DAT_00a081e0 + *param_4;
                local_8 = (float)_DAT_00a081e4 + param_4[1];
                iVar6 = FUN_00473580(*param_1, param_4, &local_c);
                if (iVar6 != 0) {
                    *(unsigned int *)(param_2 + 0xc) = 0;
                }
            }
            else {
                *(unsigned int *)(param_2 + 0xc) = 0;
            }
        }
        local_10 = 0;
        if (0 < DAT_005235f0) {
            piVar7 = DAT_009fd3b0;
            do {
                local_c = (float)*piVar7;
                local_8 = (float)piVar7[1];
                local_4 = (float)piVar7[2];
                fVar1 = local_c - *param_4;
                fVar2 = local_8 - param_4[1];
                fVar3 = local_4 - param_4[2];
                local_14 = param_3[2] * fVar3 + param_3[1] * fVar2 + *param_3 * fVar1;
                __builtin_memcpy(&bits, &local_14, 4);
                if (((DAT_00527e90 != 0) || (bits < 0x80000001)) &&
                    (iVar6 = FUN_00473580(*param_1, param_4, &local_c), iVar6 == 0)) {
                    fStack_24 = fVar1;
                    __builtin_memcpy(&bits, &fVar1, 4);
                    if (0x80000000 < bits) {
                        fStack_24 = -fVar1;
                    }
                    local_28 = fVar2;
                    __builtin_memcpy(&bits, &fVar2, 4);
                    if (0x80000000 < bits) {
                        local_28 = -fVar2;
                    }
                    local_2c = fVar3;
                    __builtin_memcpy(&bits, &fVar3, 4);
                    if (0x80000000 < bits) {
                        local_2c = -fVar3;
                    }
                    if ((fStack_24 < local_28) || (fStack_24 < local_2c)) {
                        if ((local_28 < fStack_24) || (local_28 < local_2c)) {
                            fVar4 = (local_28 + fStack_24) * _DAT_005180a8 + local_2c;
                        }
                        else {
                            fVar4 = (local_2c + fStack_24) * _DAT_005180a8 + local_28;
                        }
                    }
                    else {
                        fVar4 = (local_28 + local_2c) * _DAT_005180a8 + fStack_24;
                    }
                    if (DAT_00527e90 != 0) {
                        if (local_14 / fVar4 < _DAT_005180ac) goto LAB_0042064e;
                        local_14 = fVar4 * (float)_DAT_005180b0;
                    }
                    if (DAT_005233bc == 0) {
LAB_00420615:
                        local_30 = 0.0f;
                    }
                    else {
                        fStack_24 = (*in_stack_00000014 - *param_4) + fVar1;
                        local_28 = (in_stack_00000014[1] - param_4[1]) + fVar2;
                        local_2c = (in_stack_00000014[2] - param_4[2]) + fVar3;
                        fVar1 = *param_3 * fStack_24;
                        fVar3 = param_3[1] * local_28;
                        fVar2 = param_3[2] * local_2c;
                        __builtin_memcpy(&bits, &fStack_24, 4);
                        if (0x80000000 < bits) {
                            fStack_24 = -fStack_24;
                        }
                        __builtin_memcpy(&bits, &local_28, 4);
                        if (0x80000000 < bits) {
                            local_28 = -local_28;
                        }
                        __builtin_memcpy(&bits, &local_2c, 4);
                        if (0x80000000 < bits) {
                            local_2c = -local_2c;
                        }
                        if ((fStack_24 < local_28) || (fStack_24 < local_2c)) {
                            if ((local_28 < fStack_24) || (local_28 < local_2c)) {
                                fVar5 = (local_28 + fStack_24) * _DAT_005180bc;
                                fStack_24 = local_2c;
                            }
                            else {
                                fVar5 = (local_2c + fStack_24) * _DAT_005180bc;
                                fStack_24 = local_28;
                            }
                        }
                        else {
                            fVar5 = (local_28 + local_2c) * _DAT_005180bc;
                        }
                        local_30 = ((fVar2 + fVar1 + fVar3) * _DAT_005180b8) /
                                   (fStack_24 * _DAT_005180c0 + fVar5);
                        if (local_30 < _DAT_005180b8) goto LAB_00420615;
                        local_30 = local_30 * local_30 * local_30;
                        local_30 = local_30 * local_30;
                        local_30 = local_30 * local_30;
                        local_30 = local_30 * local_30;
                        local_30 = local_30 * local_30;
                    }
                    local_30 = local_30 + local_14 / (fVar4 * fVar4);
                    if (_DAT_005180a0 < local_30) {
                        *(float *)(param_2 + 0xc) = (float)piVar7[3] * local_30 + *(float *)(param_2 + 0xc);
                        if (0x3f7fffff < *(int *)(param_2 + 0xc)) {
                            *(unsigned int *)(param_2 + 0xc) = 0x3f800000;
                            return;
                        }
                    }
                }
LAB_0042064e:
                piVar7 = piVar7 + 4;
                local_10 = (int)((unsigned int)local_10 + 1u);
                if (DAT_005235f0 <= local_10) {
                    return;
                }
            } while (1);
        }
    }
    return;
}
