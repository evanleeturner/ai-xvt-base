/* SPDX-License-Identifier: MIT */
/* z_xvt 00414070 */
extern int DAT_0052ae18;
extern int DAT_00525500;
extern int DAT_005254fc;
extern int DAT_00525730;
extern float _DAT_0052005c;
extern float _DAT_00520060;
extern float _DAT_00520064;
extern float _DAT_0052006c;
extern float _DAT_00520070;
extern double _DAT_00520078;
extern float _DAT_00520080;
extern float _DAT_00520084;
extern float _DAT_00520088;
extern float _DAT_00732ea0;
extern float DAT_00732f30;
extern float _DAT_00732f34;
extern float DAT_0072f460[];

int __cdecl FUN_00487680(int param_1, float *param_2, float *param_3);

void __cdecl FUN_00414070(int *param_1, int param_2, float *param_3, float *param_4, float *param_5)
{
    float fVar1, fVar2, fVar3, fVar4, fVar5;
    int iVar6;
    float *piVar7;
    float fStack_30, fStack_2c, fStack_28, fStack_24, fStack_14;
    int iStack_10;
    float fStack_c, fStack_8, fStack_4;
    unsigned int ub;
    int ib;
    float *pOut = (float *)(param_2 + 0xc);

    if ((DAT_0052ae18 != 0) &&
        (*(char *)(*param_1 + 2) == '\a' || *(char *)(*param_1 + 2) == '\x06')) {
        *(unsigned int *)pOut = 0x3f800000;
        return;
    }
    if (DAT_00525500 == 0) {
        *(unsigned int *)pOut = 0x3ecccccd;
    } else {
        *pOut = (_DAT_00732ea0 * _DAT_0052005c * param_3[2] +
                 DAT_00732f30 * _DAT_0052005c * *param_3 +
                 _DAT_00732f34 * _DAT_0052005c * param_3[1]) * _DAT_00520060;
        __builtin_memcpy(&ub, pOut, 4);
        if (ub < 0x80000001u) {
            fStack_4 = _DAT_00732ea0 + param_4[2];
            fStack_c = DAT_00732f30 + *param_4;
            fStack_8 = _DAT_00732f34 + param_4[1];
            iVar6 = FUN_00487680(*param_1, param_4, &fStack_c);
            if (iVar6 != 0) {
                *(unsigned int *)pOut = 0;
            }
        } else {
            *(unsigned int *)pOut = 0;
        }
    }
    iStack_10 = 0;
    if (DAT_00525730 < 1) {
        return;
    }
    piVar7 = DAT_0072f460;
    do {
        fStack_c = piVar7[0];
        fStack_8 = piVar7[1];
        fStack_4 = piVar7[2];
        fVar1 = fStack_c - *param_4;
        fVar2 = fStack_8 - param_4[1];
        fVar3 = fStack_4 - param_4[2];
        fStack_14 = param_3[2] * fVar3 + param_3[1] * fVar2 + *param_3 * fVar1;
        __builtin_memcpy(&ub, &fStack_14, 4);
        if (((DAT_0052ae18 != 0) || (ub < 0x80000001u)) &&
            (iVar6 = FUN_00487680(*param_1, param_4, &fStack_c), iVar6 == 0)) {
            fStack_24 = fVar1;
            __builtin_memcpy(&ub, &fVar1, 4);
            if (0x80000000u < ub) fStack_24 = -fVar1;
            fStack_28 = fVar2;
            __builtin_memcpy(&ub, &fVar2, 4);
            if (0x80000000u < ub) fStack_28 = -fVar2;
            fStack_2c = fVar3;
            __builtin_memcpy(&ub, &fVar3, 4);
            if (0x80000000u < ub) fStack_2c = -fVar3;
            if ((fStack_24 < fStack_28) || (fStack_24 < fStack_2c)) {
                if ((fStack_28 < fStack_24) || (fStack_28 < fStack_2c)) {
                    fVar4 = (fStack_28 + fStack_24) * _DAT_0052006c + fStack_2c;
                } else {
                    fVar4 = (fStack_2c + fStack_24) * _DAT_0052006c + fStack_28;
                }
            } else {
                fVar4 = (fStack_2c + fStack_28) * _DAT_0052006c + fStack_24;
            }
            if (DAT_0052ae18 != 0) {
                if (fStack_14 / fVar4 < _DAT_00520070) goto LAB_004144e2;
                fStack_14 = fVar4 * (float)_DAT_00520078;
            }
            if (DAT_005254fc == 0) {
LAB_004144a9:
                fStack_30 = 0.0f;
            } else {
                fStack_24 = (*param_5 - *param_4) + fVar1;
                fStack_28 = (param_5[1] - param_4[1]) + fVar2;
                fStack_2c = (param_5[2] - param_4[2]) + fVar3;
                fVar1 = *param_3 * fStack_24;
                fVar3 = param_3[1] * fStack_28;
                fVar2 = param_3[2] * fStack_2c;
                __builtin_memcpy(&ub, &fStack_24, 4);
                if (0x80000000u < ub) fStack_24 = -fStack_24;
                __builtin_memcpy(&ub, &fStack_28, 4);
                if (0x80000000u < ub) fStack_28 = -fStack_28;
                __builtin_memcpy(&ub, &fStack_2c, 4);
                if (0x80000000u < ub) fStack_2c = -fStack_2c;
                if ((fStack_24 < fStack_28) || (fStack_24 < fStack_2c)) {
                    if ((fStack_28 < fStack_24) || (fStack_28 < fStack_2c)) {
                        fVar5 = (fStack_28 + fStack_24) * _DAT_00520084;
                        fStack_24 = fStack_2c;
                    } else {
                        fVar5 = (fStack_2c + fStack_24) * _DAT_00520084;
                        fStack_24 = fStack_28;
                    }
                } else {
                    fVar5 = (fStack_2c + fStack_28) * _DAT_00520084;
                }
                fStack_30 = ((fVar2 + fVar1 + fVar3) * _DAT_00520080) /
                            (fStack_24 * _DAT_00520088 + fVar5);
                if (fStack_30 < _DAT_00520080) goto LAB_004144a9;
                fStack_30 = fStack_30 * fStack_30 * fStack_30;
                fStack_30 = fStack_30 * fStack_30;
                fStack_30 = fStack_30 * fStack_30;
                fStack_30 = fStack_30 * fStack_30;
                fStack_30 = fStack_30 * fStack_30;
            }
            fStack_30 = fStack_14 / (fVar4 * fVar4) + fStack_30;
            if (_DAT_00520064 < fStack_30) {
                *pOut = piVar7[3] * fStack_30 + *pOut;
                __builtin_memcpy(&ib, pOut, 4);
                if (0x3f7fffff < ib) {
                    *(unsigned int *)pOut = 0x3f800000;
                    return;
                }
            }
        }
LAB_004144e2:
        piVar7 = piVar7 + 4;
        iStack_10 = iStack_10 + 1;
    } while (iStack_10 < DAT_00525730);
}
