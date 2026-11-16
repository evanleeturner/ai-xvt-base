/* SPDX-License-Identifier: MIT */
/* z_bop 004a5490 */
extern int DAT_009a1fe8;
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009a1fd0;
extern int DAT_009ec604;
extern int DAT_00a081e8;
extern int DAT_00a08298;
extern int DAT_00a08104;
extern int DAT_009a8d8c, DAT_009a8d80, DAT_009a8d60;
extern int DAT_009d1154, DAT_009d12e8, DAT_009d1264;
extern int DAT_009a8e28, DAT_009a8e20, DAT_009a8e1c;
extern float _DAT_00622c50, _DAT_00622c54, _DAT_00622c58;
extern float _DAT_00622c60, _DAT_00622c64, _DAT_00622c68;
extern int DAT_00622c6c;
extern float _DAT_00622c98;
extern float DAT_00622c70, DAT_00622c74, DAT_00622c78;
extern float DAT_00622c80, DAT_00622c84, DAT_00622c88;
extern int DAT_00622c8c;
extern int DAT_00527e88;
extern float DAT_00622c90;
extern int DAT_00622c94;
extern unsigned short DAT_009a7ed0[];
extern int DAT_00a00870[];
extern int DAT_00a00874[];
extern int DAT_00a0093c[];
extern unsigned int DAT_009d6940;
extern float _DAT_00518200;
extern float _DAT_00518204;
extern float _DAT_005181fc;
extern int DAT_009d77fc;
extern int DAT_009d6828;
extern int DAT_009cd260;

void __cdecl FUN_00427bd0(short a, short b, int c);
void __cdecl FUN_00427d30(short a, int b, int c);
int * __cdecl FUN_004ac720(unsigned int a);
void __cdecl FUN_00471ff0(int *a);
int __cdecl FUN_004ae2a0(unsigned int a, int b);
int __cdecl FUN_004ae200(unsigned int a, int b);
/* UNRESOLVED: __ftol takes its operand in ST0; the float expression before each call is not in the evidence */
long long __cdecl __ftol(void);
void __cdecl FUN_004a6080(int a, unsigned int *b);
void __cdecl FUN_004ac740(void);

#define MULSHR(a, b) \
    ((unsigned int)((long long)(a) * (long long)(b)) >> 0xf | \
     (unsigned int)((unsigned long long)((long long)(a) * (long long)(b)) >> 0x20) << 0x11)

int __cdecl FUN_004a5490(unsigned short param_1, unsigned short param_2)
{
    unsigned char bVar1;
    unsigned int *puVar2;
    int *piVar8;
    unsigned int uVar9;
    int iVar5, iVar6, iVar7, iVar10, iVar11, iVar12, iVar13, iVar14;
    long long lVar15;
    unsigned short uStack_180;
    int local_16c, local_168, local_164, local_160, local_15c;
    unsigned int local_158;
    int local_154;
    unsigned char *pb;
    int tmp_p, tmp_a, tmp_b, tmp_c;
    short s0, s1, s2, s3, s4, s5, s6, s7, s8;
    unsigned int tmp_cnt;

    iVar5 = (int)((unsigned int)param_2 * 0x23u);
    iVar13 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar5);
    pb = (unsigned char *)iVar13;
    __builtin_memcpy(&tmp_p, pb + 0x1f, 4);
    if (tmp_p != 0) {
        __builtin_memcpy(&DAT_00a08104, (unsigned char *)tmp_p + 0xa9, 4);
    }
    __builtin_memcpy(&tmp_a, pb + 4, 4);
    __builtin_memcpy(&tmp_b, pb + 8, 4);
    __builtin_memcpy(&tmp_c, pb + 0xc, 4);
    iVar6 = (int)((unsigned int)DAT_009a1fd4 - (unsigned int)tmp_a);
    iVar7 = (int)((unsigned int)DAT_009ec604 - (unsigned int)tmp_a);
    uStack_180 = (unsigned short)iVar7;
    iVar14 = (int)((unsigned int)DAT_009a1fd8 - (unsigned int)tmp_b);
    iVar10 = (int)((unsigned int)DAT_00a081e8 - (unsigned int)tmp_b);
    iVar12 = (int)((unsigned int)DAT_009a1fd0 - (unsigned int)tmp_c);
    iVar11 = (int)((unsigned int)DAT_00a08298 - (unsigned int)tmp_c);

    __builtin_memcpy(&tmp_p, pb + 0x1f, 4);
    if (tmp_p == 0) {
        __builtin_memcpy(&s0, pb + 0x12, 2);
        __builtin_memcpy(&s1, pb + 0x10, 2);
        __builtin_memcpy(&s2, pb + 0x14, 2);
        FUN_00427bd0(s0, s1, 0);
        FUN_00427d30(s2, 0, 0);
        local_15c = (int)(MULSHR(DAT_009a8d8c, iVar12) + MULSHR(DAT_009a8d80, iVar6) + MULSHR(DAT_009a8d60, iVar14));
        local_164 = (int)(MULSHR(DAT_009d1154, iVar12) + MULSHR(DAT_009d12e8, iVar6) + MULSHR(DAT_009d1264, iVar14));
        local_16c = (int)(MULSHR(DAT_009a8e28, iVar12) + MULSHR(DAT_009a8e20, iVar6) + MULSHR(DAT_009a8e1c, iVar14));
        local_160 = (int)(MULSHR(DAT_009a8d8c, iVar11) + MULSHR(DAT_009a8d80, iVar7) + MULSHR(DAT_009a8d60, iVar10));
        local_168 = (int)(MULSHR(DAT_009d1154, iVar11) + MULSHR(DAT_009d12e8, iVar7) + MULSHR(DAT_009d1264, iVar10));
        iVar6 = (int)(MULSHR(DAT_009a8e28, iVar11) + MULSHR(DAT_009a8e20, iVar7) + MULSHR(DAT_009a8e1c, iVar10));
    } else {
        if (*((unsigned char *)tmp_p + 0x92) != 0) {
            __builtin_memcpy(&s0, pb + 0x12, 2);
            __builtin_memcpy(&s1, pb + 0x10, 2);
            __builtin_memcpy(&s2, pb + 0x14, 2);
            FUN_00427bd0(s0, s1, iVar13);
            FUN_00427d30(s2, 0, iVar13);
        }
        __builtin_memcpy(&s0, (unsigned char *)tmp_p + 0x99, 2);
        __builtin_memcpy(&s1, (unsigned char *)tmp_p + 0x9b, 2);
        __builtin_memcpy(&s2, (unsigned char *)tmp_p + 0x9d, 2);
        __builtin_memcpy(&s3, (unsigned char *)tmp_p + 0x93, 2);
        __builtin_memcpy(&s4, (unsigned char *)tmp_p + 0x95, 2);
        __builtin_memcpy(&s5, (unsigned char *)tmp_p + 0x97, 2);
        __builtin_memcpy(&s6, (unsigned char *)tmp_p + 0x9f, 2);
        __builtin_memcpy(&s7, (unsigned char *)tmp_p + 0xa1, 2);
        __builtin_memcpy(&s8, (unsigned char *)tmp_p + 0xa3, 2);
        local_15c = (int)(MULSHR((int)s2, iVar12) + MULSHR((int)s0, iVar6) + MULSHR((int)s1, iVar14));
        local_164 = (int)(MULSHR((int)s5, iVar12) + MULSHR((int)s3, iVar6) + MULSHR((int)s4, iVar14));
        local_16c = (int)(MULSHR((int)s8, iVar12) + MULSHR((int)s6, iVar6) + MULSHR((int)s7, iVar14));
        local_160 = (int)(MULSHR((int)s2, iVar11) + MULSHR((int)s0, iVar7) + MULSHR((int)s1, iVar10));
        local_168 = (int)(MULSHR((int)s5, iVar11) + MULSHR((int)s3, iVar7) + MULSHR((int)s4, iVar10));
        iVar6 = (int)(MULSHR((int)s8, iVar11) + MULSHR((int)s6, iVar7) + MULSHR((int)s7, iVar10));
    }
    local_164 = (int)(0u - (unsigned int)local_164);
    local_168 = (int)(0u - (unsigned int)local_168);
    _DAT_00622c50 = (float)local_160;
    _DAT_00622c54 = (float)local_168;
    _DAT_00622c58 = (float)iVar6;
    _DAT_00622c60 = (float)local_15c;
    _DAT_00622c64 = (float)local_164;
    _DAT_00622c68 = (float)local_16c;
    DAT_00622c6c = 0;
    _DAT_00622c98 = 2.0f;
    DAT_00622c70 = _DAT_00622c50;
    DAT_00622c74 = _DAT_00622c54;
    DAT_00622c78 = _DAT_00622c58;
    DAT_00622c80 = _DAT_00622c60;
    DAT_00622c84 = _DAT_00622c64;
    DAT_00622c88 = _DAT_00622c68;

    piVar8 = FUN_004ac720(((unsigned int)uStack_180 << 16) | (unsigned int)DAT_009a7ed0[pb[3]]);
    if (piVar8 == (int *)0) {
        return 0;
    }
    if ((int *)*piVar8 != piVar8) {
        FUN_00471ff0(piVar8);
    }
    DAT_00622c8c = 0;
    DAT_00527e88 = 0;
    local_158 = 0;
    __builtin_memcpy(&tmp_cnt, (unsigned char *)piVar8 + 6, 4);
    if (tmp_cnt != 0) {
        local_154 = 0;
        do {
            iVar7 = DAT_00622c8c;
            DAT_00622c90 = 0.0f;
            __builtin_memcpy(&tmp_a, (unsigned char *)piVar8 + 10, 4);
            puVar2 = *(unsigned int **)((unsigned int)tmp_a + (unsigned int)local_154);
            if (puVar2[1] != 0x14) {
                iVar10 = (int)((unsigned int)DAT_00622c8c + 1u);
                if (param_1 == param_2) {
                    bVar1 = *(unsigned char *)((unsigned int)DAT_009a1fe8 + 3u + (unsigned int)iVar5);
                    uVar9 = (unsigned int)bVar1;
                    if (bVar1 < 0x49) {
                        if (DAT_00622c8c < 0) {
                            iVar7 = 0;
                            DAT_00622c8c = iVar10;
                        } else {
                            if (DAT_00a00870[uVar9 * 0x65] <= DAT_00622c8c) {
                                DAT_00622c8c = (int)((unsigned int)DAT_00a00870[uVar9 * 0x65] - 1u);
                            }
                            iVar7 = DAT_00a00874[uVar9 * 0x65 + (unsigned int)DAT_00622c8c];
                            DAT_00622c8c = iVar10;
                        }
                    } else {
                        DAT_00622c8c = iVar10;
                        iVar7 = FUN_004ae2a0(uVar9, iVar7);
                    }
                    if (((iVar7 == 4) || (iVar7 == 5)) || (iVar7 == 0x15)) goto LAB_004a5f8f;
                    iVar10 = DAT_00622c8c;
                    if (*(char *)((unsigned int)DAT_009a1fe8 + 3u + (unsigned int)iVar5) == '6' &&
                        (int)((unsigned int)DAT_00622c8c - (unsigned int)DAT_00622c94) == 1) goto LAB_004a5f8f;
                }
                DAT_00622c8c = iVar10;
                __builtin_memcpy(&tmp_p, pb + 0x1f, 4);
                if (tmp_p != 0) {
                    __builtin_memcpy(&iVar7, (unsigned char *)tmp_p + 0xa9, 4);
                    if (iVar7 != 0) {
                        iVar7 = (int)((unsigned int)DAT_00622c8c + (unsigned int)iVar7);
                        if (*((unsigned char *)iVar7 + 0x33b) == 0) goto LAB_004a5f8f;
                        bVar1 = *((unsigned char *)iVar7 + 0x309);
                        if (bVar1 != 0 && ((unsigned char *)&DAT_009d6940)[1] == 0) {
                            __builtin_memcpy(&tmp_c, (unsigned char *)((unsigned int)param_1 * 0x23u + 0x1bu + (unsigned int)DAT_009a1fe8), 4);
                            if (tmp_c != -1) goto LAB_004a5f8f;
                        }
                        DAT_00622c90 = (float)bVar1 * _DAT_00518200;
                    }
                }
                iVar7 = (int)((unsigned int)DAT_00622c8c - 1u);
                uVar9 = (unsigned int)pb[3];
                if (pb[3] < 0x49) {
                    if (iVar7 < 0) {
                        iVar7 = 0;
                    } else {
                        if (DAT_00a00870[uVar9 * 0x65] <= iVar7) {
                            iVar7 = (int)((unsigned int)DAT_00a00870[uVar9 * 0x65] - 1u);
                        }
                        iVar7 = DAT_00a0093c[(unsigned int)iVar7 + uVar9 * 0x65];
                    }
                } else {
                    iVar7 = FUN_004ae200(uVar9, iVar7);
                }
                if (iVar7 != 0) {
                    /* UNRESOLVED: ST0 operand of each __ftol below */
                    lVar15 = __ftol();
                    if (((int)lVar15 <= local_15c) || ((int)lVar15 <= local_160)) {
                        lVar15 = __ftol();
                        if (((int)lVar15 <= local_164) || ((int)lVar15 <= local_168)) {
                            lVar15 = __ftol();
                            if (((int)lVar15 <= local_16c) || ((int)lVar15 <= iVar6)) {
                                lVar15 = __ftol();
                                if ((local_15c <= (int)lVar15) || (local_160 <= (int)lVar15)) {
                                    lVar15 = __ftol();
                                    if ((local_164 <= (int)lVar15) || (local_168 <= (int)lVar15)) {
                                        lVar15 = __ftol();
                                        if ((local_16c <= (int)lVar15) || (iVar6 <= (int)lVar15)) {
                                            FUN_004a6080((int)piVar8, puVar2);
                                            _DAT_00622c50 = DAT_00622c70;
                                            _DAT_00622c54 = DAT_00622c74;
                                            _DAT_00622c58 = DAT_00622c78;
                                            _DAT_00622c60 = DAT_00622c80;
                                            _DAT_00622c64 = DAT_00622c84;
                                            _DAT_00622c68 = DAT_00622c88;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
LAB_004a5f8f:
            local_154 = (int)((unsigned int)local_154 + 4u);
            local_158 = local_158 + 1;
            __builtin_memcpy(&tmp_cnt, (unsigned char *)piVar8 + 6, 4);
        } while (local_158 < tmp_cnt);
    }
    FUN_004ac740();
    if (DAT_00622c6c != 0) {
        _DAT_00622c98 = _DAT_00622c98 - _DAT_00518204;
        if (_DAT_00622c98 < _DAT_005181fc) {
            _DAT_00622c98 = 0.0f;
        }
        /* UNRESOLVED: ST0 operands for the three __ftol stores */
        lVar15 = __ftol();
        DAT_009d77fc = (int)lVar15;
        lVar15 = __ftol();
        DAT_009d6828 = (int)lVar15;
        lVar15 = __ftol();
        DAT_009cd260 = (int)lVar15;
    }
    return DAT_00622c6c;
}
