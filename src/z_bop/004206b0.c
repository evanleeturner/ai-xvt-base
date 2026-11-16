/* SPDX-License-Identifier: MIT */
/* z_bop 004206b0 */
extern float  _DAT_005180a0, _DAT_00518098, _DAT_005180a8, _DAT_005180b8;
extern float  _DAT_005180bc, _DAT_005180c0, _DAT_005180c4, _DAT_005180c8;
extern float  _DAT_005180cc, _DAT_005180d0;
extern int    DAT_0051c00c, DAT_0051c010;
extern int   *DAT_0051c014;
extern int    DAT_005235f0, DAT_005233bc;
extern float  DAT_00550be0[];
extern float  DAT_00550c10[];
extern int    DAT_009fd3b0[];
extern float  _DAT_00550c00, _DAT_00550c04, _DAT_00550c08;
extern float  DAT_00550c70, DAT_00550c74, DAT_00550c78;
extern int    _DAT_00a081e4, _DAT_00a08150, _DAT_00a081e0;
extern unsigned short DAT_009d682c, DAT_009a1fe4;
extern int    DAT_009d7680;
extern float  _DAT_0099940c;

void FUN_0044f880(int param_1);
void FUN_00420fe0(float *param_1, float *param_2);

/* Unresolved: _DAT_00a081e0/e4 and _DAT_00a08150 are assumed int (the decompiler casts them to float).
   The float10 return is written as long double, which is x87-width on i386 GCC. */
long double __cdecl FUN_004206b0(int *param_1, int param_2, int param_3, float param_4)
{
    int *piVar1;
    float fVar2, fVar3, fVar4, fVar5, fVar6, fVar7;
    float *pfVar8;
    float *pfVar9;
    int iVar10;
    int *piVar11;
    float local_48, local_44, local_40;
    int local_3c;
    float local_38;
    float local_28, local_24, local_20;
    float local_1c, local_18, local_14, local_10;
    float local_c, local_8, local_4;
    int bits;
    unsigned int u;

    piVar1 = (int *)param_1[1];
    if (piVar1 == (int *)0) {
        return (long double)_DAT_005180a0;
    }
    if (*piVar1 != DAT_0051c00c) {
        FUN_0044f880(*piVar1);
        DAT_0051c010 = DAT_005235f0;
        DAT_0051c00c = *piVar1;
        local_3c = 0;
        if (0 < DAT_005235f0) {
            pfVar9 = DAT_00550be0;
            piVar11 = DAT_009fd3b0;
            pfVar8 = DAT_00550c10;
            while (1) {
                *pfVar8 = (float)*piVar11;
                pfVar8[1] = (float)piVar11[1];
                pfVar8[2] = (float)piVar11[2];
                FUN_00420fe0(pfVar8, (float *)(piVar1 + 5));
                local_3c = local_3c + 1;
                *pfVar8 = (float)piVar1[2] + *pfVar8;
                pfVar8[1] = (float)piVar1[3] + pfVar8[1];
                pfVar8[2] = (float)piVar1[4] + pfVar8[2];
                if (DAT_005235f0 <= local_3c) break;
                *pfVar9 = (float)piVar11[3];
                pfVar9 = pfVar9 + 1;
                piVar11 = piVar11 + 4;
                pfVar8 = pfVar8 + 3;
            }
            *pfVar9 = (float)piVar11[3];
        }
        _DAT_00550c04 = (float)_DAT_00a081e4 * _DAT_00518098;
        _DAT_00550c08 = (float)_DAT_00a08150 * _DAT_00518098;
        _DAT_00550c00 = (float)_DAT_00a081e0 * _DAT_00518098;
        FUN_00420fe0(&_DAT_00550c00, (float *)(piVar1 + 5));
    }
    local_14 = _DAT_005180c4 / param_4;
    local_1c = (float)(int)((unsigned int)param_2 - (unsigned int)(DAT_009d682c >> 1)) * local_14 * _DAT_0099940c;
    local_18 = (float)(int)(((unsigned int)param_3 - (unsigned int)(DAT_009a1fe4 >> 1)) - (unsigned int)DAT_009d7680) *
               local_14 * _DAT_0099940c;
    if (param_1 == DAT_0051c014) {
        local_c = DAT_00550c70;
        local_8 = DAT_00550c74;
        local_4 = DAT_00550c78;
    } else {
        DAT_0051c014 = param_1;
        local_28 = *(float *)((unsigned int)piVar1[0x25] + (unsigned int)*param_1 * 0xc);
        iVar10 = (int)((unsigned int)piVar1[0x25] + (unsigned int)*param_1 * 0xc);
        local_24 = *(float *)((unsigned int)iVar10 + 4);
        local_20 = *(float *)((unsigned int)iVar10 + 8);
        FUN_00420fe0(&local_28, (float *)(piVar1 + 5));
        local_c = local_28;
        local_8 = local_24;
        local_4 = local_20;
    }
    local_10 = 0.0f;
    if (DAT_005233bc != 0) {
        fVar2 = -local_1c;
        fVar3 = -local_18;
        fVar4 = -local_14;
        local_40 = fVar2;
        __builtin_memcpy(&u, &fVar2, 4);
        if (0x80000000u < u) {
            local_40 = -fVar2;
        }
        local_44 = fVar3;
        __builtin_memcpy(&u, &fVar3, 4);
        if (0x80000000u < u) {
            local_44 = -fVar3;
        }
        local_48 = fVar4;
        __builtin_memcpy(&u, &fVar4, 4);
        if (0x80000000u < u) {
            local_48 = -fVar4;
        }
        if ((local_40 < local_44) || (local_40 < local_48)) {
            if ((local_44 < local_40) || (local_44 < local_48)) {
                fVar5 = (local_44 + local_40) * _DAT_005180c8;
                local_40 = local_48;
            } else {
                fVar5 = (local_48 + local_40) * _DAT_005180c8;
                local_40 = local_44;
            }
        } else {
            fVar5 = (local_48 + local_44) * _DAT_005180c8;
        }
        fVar5 = _DAT_005180c4 / (local_40 * _DAT_005180cc + fVar5);
        local_38 = (_DAT_00550c08 + fVar5 * fVar4) * local_4 +
                   (_DAT_00550c04 + fVar3 * fVar5) * local_8 +
                   (_DAT_00550c00 + fVar2 * fVar5) * local_c;
        if (local_38 <= _DAT_005180a0) {
            local_38 = 0.0f;
        } else {
            local_38 = local_38 * _DAT_005180b8;
            local_38 = local_38 * local_38 * local_38;
            local_38 = local_38 * local_38;
            local_38 = local_38 * local_38;
            local_38 = local_38 * local_38;
            local_38 = local_38 * local_38;
        }
        local_38 = local_38 * _DAT_005180d0;
        if (_DAT_005180a0 < local_38) {
            local_10 = local_38;
            __builtin_memcpy(&bits, &local_38, 4);
            if (0x3f7fffff < bits) {
                DAT_00550c70 = local_c;
                DAT_00550c74 = local_8;
                DAT_00550c78 = local_4;
                return (long double)_DAT_005180c4;
            }
        }
    }
    pfVar9 = DAT_00550c10;
    iVar10 = 0;
    if (0 < DAT_0051c010) {
        pfVar8 = DAT_00550be0;
        do {
            fVar2 = *pfVar9 - local_1c;
            fVar3 = pfVar9[1] - local_18;
            fVar4 = pfVar9[2] - local_14;
            fVar5 = local_4 * fVar4 + local_8 * fVar3 + local_c * fVar2;
            if (_DAT_005180a0 < fVar5) {
                local_40 = fVar2;
                __builtin_memcpy(&u, &fVar2, 4);
                if (0x80000000u < u) {
                    local_40 = -fVar2;
                }
                local_44 = fVar3;
                __builtin_memcpy(&u, &fVar3, 4);
                if (0x80000000u < u) {
                    local_44 = -fVar3;
                }
                local_48 = fVar4;
                __builtin_memcpy(&u, &fVar4, 4);
                if (0x80000000u < u) {
                    local_48 = -fVar4;
                }
                if ((local_40 < local_44) || (local_40 < local_48)) {
                    if ((local_44 < local_40) || (local_44 < local_48)) {
                        fVar6 = (local_44 + local_40) * _DAT_005180a8 + local_48;
                    } else {
                        fVar6 = (local_48 + local_40) * _DAT_005180a8 + local_44;
                    }
                } else {
                    fVar6 = (local_48 + local_44) * _DAT_005180a8 + local_40;
                }
                if (DAT_005233bc == 0) {
                    local_38 = 0.0f;
                } else {
                    local_40 = fVar2 - local_1c;
                    local_44 = fVar3 - local_18;
                    local_48 = fVar4 - local_14;
                    fVar2 = local_c * local_40;
                    fVar4 = local_8 * local_44;
                    fVar3 = local_4 * local_48;
                    __builtin_memcpy(&u, &local_40, 4);
                    if (0x80000000u < u) {
                        local_40 = -local_40;
                    }
                    __builtin_memcpy(&u, &local_44, 4);
                    if (0x80000000u < u) {
                        local_44 = -local_44;
                    }
                    __builtin_memcpy(&u, &local_48, 4);
                    if (0x80000000u < u) {
                        local_48 = -local_48;
                    }
                    if ((local_40 < local_44) || (local_40 < local_48)) {
                        if ((local_44 < local_40) || (local_44 < local_48)) {
                            fVar7 = (local_44 + local_40) * _DAT_005180bc;
                            local_40 = local_48;
                        } else {
                            fVar7 = (local_48 + local_40) * _DAT_005180bc;
                            local_40 = local_44;
                        }
                    } else {
                        fVar7 = (local_48 + local_44) * _DAT_005180bc;
                    }
                    local_38 = _DAT_005180c4 / (local_40 * _DAT_005180c0 + fVar7);
                    fVar2 = local_38 * (fVar3 + fVar4 + fVar2) * _DAT_005180b8;
                    if (fVar2 < _DAT_005180a0) {
                        local_38 = 0.0f;
                    } else {
                        fVar2 = fVar2 * fVar2 * fVar2;
                        fVar2 = fVar2 * fVar2;
                        fVar2 = fVar2 * fVar2;
                        fVar2 = fVar2 * fVar2;
                        local_38 = fVar2 * fVar2 * local_38;
                    }
                }
                local_38 = fVar5 / (fVar6 * fVar6) + local_38;
                if (_DAT_005180a0 < local_38) {
                    local_10 = *pfVar8 * local_38 + local_10;
                    if (_DAT_005180c4 <= local_10) {
                        local_10 = 1.0f;
                        break;
                    }
                }
            }
            pfVar8 = pfVar8 + 1;
            iVar10 = iVar10 + 1;
            pfVar9 = pfVar9 + 3;
        } while (iVar10 < DAT_0051c010);
    }
    DAT_00550c70 = local_c;
    DAT_00550c74 = local_8;
    DAT_00550c78 = local_4;
    return (long double)local_10;
}
