/* SPDX-License-Identifier: MIT */
/* z_bop 004a8740 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern int DAT_009a1fe8;
extern unsigned char DAT_00521f1c[];
extern int DAT_00a00870[];
extern int DAT_00a0093c[];
extern unsigned char DAT_009e969d[];
extern unsigned char DAT_009e969f[];

int FUN_0041f410(int, int, int);
void FUN_00427bd0(short, short, int);
void FUN_00427d30(short, int, int);
int FUN_004adc40(unsigned int);
int *FUN_004ae200(unsigned int, int);
long long __cdecl __ftol(void); /* FINDING: x87 operand (st0) not recovered by decompiler */
unsigned int FUN_0041dc30(unsigned int, unsigned short, unsigned int, unsigned int,
                          unsigned int, unsigned short, unsigned int, unsigned short,
                          unsigned int);

#define FIX15(l) ((unsigned int)((unsigned long long)(l) >> 15))

void FUN_004a8740(unsigned int param_1, int param_2)
{
    unsigned char bVar1;
    char cVar2;
    int iVar3;
    long long lVar4, lVar5, lVar6, lVar7, lVar8, lVar19;
    unsigned int uVar9;
    int iVar10;
    unsigned int uVar11;
    int iVar12;
    int iVar13, iVar14, iVar15, iVar16, iVar17;
    int *piVar18;
    int local_6c, local_68, local_5c;
    unsigned char *p;
    unsigned char *q;
    int iObj;
    short s0;
    int tmp;

    uVar9 = (unsigned int)param_2 * 0x23 + (unsigned int)DAT_009a1fe8;
    p = (unsigned char *)uVar9;
    __builtin_memcpy(&iVar3, &DAT_00521f1c[(unsigned int)p[3] * 0x1a], 4);
    iVar12 = (int)((unsigned int)DAT_009a1fe8 + param_1 * 0x23);
    {
        int a, b;
        __builtin_memcpy(&a, (unsigned char *)iVar12 + 4, 4);
        __builtin_memcpy(&b, p + 4, 4);
        iVar15 = (int)((unsigned int)a - (unsigned int)b);
        __builtin_memcpy(&a, (unsigned char *)iVar12 + 8, 4);
        __builtin_memcpy(&b, p + 8, 4);
        iVar17 = (int)((unsigned int)a - (unsigned int)b);
        __builtin_memcpy(&a, (unsigned char *)iVar12 + 0xc, 4);
        __builtin_memcpy(&b, p + 0xc, 4);
        iVar13 = (int)((unsigned int)a - (unsigned int)b);
    }
    iVar12 = FUN_0041f410(iVar15, iVar17, iVar13);
    __builtin_memcpy(&iObj, p + 0x1f, 4);
    if ((iVar12 <= (int)((unsigned int)iVar3 * 3)) && (iObj != 0)) {
        q = (unsigned char *)iObj;
        if (q[0x92] != 0) {
            short a, b, c;
            __builtin_memcpy(&a, p + 0x12, 2);
            __builtin_memcpy(&b, p + 0x10, 2);
            FUN_00427bd0(a, b, (int)uVar9);
            __builtin_memcpy(&c, p + 0x14, 2);
            FUN_00427d30(c, 0, (int)uVar9);
        }
        __builtin_memcpy(&iObj, p + 0x1f, 4);
        q = (unsigned char *)iObj;
        __builtin_memcpy(&s0, q + 0x99, 2);
        lVar4 = (long long)(int)s0 * (long long)iVar15;
        __builtin_memcpy(&s0, q + 0x9b, 2);
        lVar5 = (long long)(int)s0 * (long long)iVar17;
        __builtin_memcpy(&s0, q + 0x9d, 2);
        lVar6 = (long long)(int)s0 * (long long)iVar13;
        __builtin_memcpy(&s0, q + 0x93, 2);
        lVar19 = (long long)(int)s0 * (long long)iVar15;
        __builtin_memcpy(&s0, q + 0x95, 2);
        lVar7 = (long long)(int)s0 * (long long)iVar17;
        __builtin_memcpy(&s0, q + 0x97, 2);
        lVar8 = (long long)(int)s0 * (long long)iVar13;
        iVar12 = (int)(FIX15(lVar8) + FIX15(lVar19) + FIX15(lVar7));
        iVar10 = (int)(0u - (unsigned int)iVar12);
        if (iVar12 == iVar3 || (int)(0u - (unsigned int)iVar3) < iVar10) {
            __builtin_memcpy(&s0, q + 0x9f, 2);
            lVar19 = (long long)(int)s0 * (long long)iVar15;
            __builtin_memcpy(&s0, q + 0xa1, 2);
            lVar7 = (long long)(int)s0 * (long long)iVar17;
            __builtin_memcpy(&s0, q + 0xa3, 2);
            lVar8 = (long long)(int)s0 * (long long)iVar13;
            iVar12 = (int)(FIX15(lVar8) + FIX15(lVar19) + FIX15(lVar7));
            bVar1 = p[3];
            if (bVar1 < 0x49) {
                local_5c = DAT_00a00870[(unsigned int)bVar1 * 0x65];
            } else {
                local_5c = FUN_004adc40((unsigned int)bVar1);
            }
            local_6c = 0;
            if (0 < local_5c) {
                do {
                    unsigned int uIdx = (unsigned int)p[3];
                    if (p[3] < 0x49) {
                        if (local_6c < 0) {
                            piVar18 = (int *)0;
                        } else {
                            iVar13 = local_6c;
                            if (DAT_00a00870[uIdx * 0x65] <= local_6c) {
                                iVar13 = DAT_00a00870[uIdx * 0x65] - 1;
                            }
                            piVar18 = (int *)DAT_00a0093c[iVar13 + uIdx * 0x65];
                        }
                    } else {
                        piVar18 = FUN_004ae200(uIdx, local_6c);
                    }
                    if ((piVar18 != (int *)0) && (*piVar18 == 6)) {
                        lVar19 = __ftol();
                        iVar13 = (int)lVar19;
                        cVar2 = (char)p[3];
                        if (cVar2 == '6') {
                            iVar13 = iVar13 >> 6;
                        }
                        lVar19 = __ftol();
                        iVar15 = (int)lVar19;
                        lVar19 = __ftol();
                        iVar17 = (int)lVar19;
                        if (((cVar2 != '6') || ((float)iVar12 <= (float)piVar18[0xd])) &&
                            ((cVar2 != '1') ||
                             (((unsigned int)piVar18[7] < 0x80000001u) ||
                              ((float)iVar12 <= (float)piVar18[0xd])))) {
                            if (iVar13 < iVar15) {
                                iVar13 = iVar15;
                            }
                            if (iVar13 < iVar17) {
                                iVar13 = iVar17;
                            }
                            iVar16 = (int)((unsigned int)iVar13 * 8u);
                            if ((int)((unsigned int)iVar3 * 2u) < iVar16) {
                                iVar16 = (int)((unsigned int)iVar3 * 2u);
                            }
                            if (cVar2 == '6') {
                                lVar19 = __ftol();
                                local_68 = (int)((unsigned int)(iVar16 >> 8) - (unsigned int)(int)lVar19);
                            } else {
                                lVar19 = __ftol();
                                local_68 = (int)(0u - (unsigned int)(int)lVar19);
                            }
                            local_68 = (int)((unsigned int)iVar10 + (unsigned int)local_68);
                            if ((0 < local_68) && (local_68 <= iVar16)) {
                                lVar19 = __ftol();
                                iVar13 = (int)((FIX15(lVar6) + FIX15(lVar4) + FIX15(lVar5)) -
                                               (unsigned int)(int)lVar19);
                                lVar19 = __ftol();
                                iVar14 = (int)((unsigned int)iVar12 - (unsigned int)(int)lVar19);
                                iVar15 = (int)((unsigned int)iVar15 + (unsigned int)(iVar15 >> 1));
                                if (iVar16 == 0) {
                                    RaiseException(0xC0000094UL, 0, 0, NULL);
                                }
                                iVar15 = (int)((unsigned int)iVar15 +
                                               (unsigned int)(((int)((unsigned int)iVar15 * (unsigned int)local_68)) / iVar16));
                                iVar17 = (int)((unsigned int)iVar17 + (unsigned int)(iVar17 >> 1));
                                iVar17 = (int)((unsigned int)iVar17 +
                                               (unsigned int)(((int)((unsigned int)iVar17 * (unsigned int)local_68)) / iVar16));
                                if (iVar13 < 0) {
                                    iVar13 = (int)(0u - (unsigned int)iVar13);
                                }
                                if (iVar14 < 0) {
                                    iVar14 = (int)(0u - (unsigned int)iVar14);
                                }
                                if ((iVar13 <= iVar15) && (iVar14 <= iVar17)) {
                                    int d1, d2, n1, n2;
                                    d1 = (int)((unsigned int)iVar17 + (unsigned int)iVar15);
                                    if (d1 == 0) {
                                        RaiseException(0xC0000094UL, 0, 0, NULL);
                                    }
                                    n1 = (int)((((unsigned int)iVar17 - (unsigned int)iVar14) -
                                                (unsigned int)iVar13 + (unsigned int)iVar15) * 100u);
                                    n1 = n1 / d1;
                                    n2 = (int)(((unsigned int)iVar16 - (unsigned int)local_68) * 100u);
                                    n2 = n2 / iVar16;
                                    d2 = (int)((unsigned int)n1 * (unsigned int)n2);
                                    d2 = d2 / 100;
                                    uVar11 = (unsigned int)(((int)((unsigned int)d2 * (unsigned int)(iVar16 >> 8))) / 100);
                                    if (cVar2 == '6') {
                                        uVar11 = (unsigned int)(((int)uVar11 >> 4) + ((int)uVar11 >> 5));
                                    }
                                    if (0x40 < (int)uVar11) {
                                        uVar11 = 0x40;
                                    }
                                    iVar15 = (int)((unsigned int)DAT_009a1fe8 + param_1 * 0x23);
                                    __builtin_memcpy(&iVar13, (unsigned char *)iVar15 + 0x1b, 4);
                                    if (iVar13 != -1) {
                                        unsigned short us;
                                        iVar13 = (int)((unsigned int)iVar13 * 0x5bdu);
                                        __builtin_memcpy(&us, DAT_009e969f + iVar13, 2);
                                        if ((int)(unsigned int)us < (int)uVar11) {
                                            unsigned short w0 = (unsigned short)param_2;
                                            unsigned short w1 = (unsigned short)uVar11;
                                            __builtin_memcpy(DAT_009e969d + iVar13, &w0, 2);
                                            __builtin_memcpy(DAT_009e969f + iVar13, &w1, 2);
                                        }
                                    }
                                    __builtin_memcpy(&iVar13, (unsigned char *)iVar15 + 0x1f, 4);
                                    if (iVar13 != 0) {
                                        __builtin_memcpy(&iVar13, (unsigned char *)iVar13 + 0xa9, 4);
                                        if (iVar13 != 0) {
                                            int e1, e2;
                                            __builtin_memcpy(&e1, (unsigned char *)iVar13 + 0x215, 4);
                                            __builtin_memcpy(&e2, (unsigned char *)iVar13 + 0x211, 4);
                                            if ((int)((unsigned int)e1 + (unsigned int)e2) == 0) {
                                                uVar11 = (unsigned int)((int)uVar11 >> 3);
                                            }
                                        }
                                    }
                                    if ((int)uVar11 < 1) {
                                        uVar11 = 1;
                                    }
                                    /* FINDING: params 5-8 were unaff_* registers in the decompile; 0 is a placeholder */
                                    FUN_0041dc30(param_1, 0xffff, 0xfffe, uVar11,
                                                 0, 0, 0, 0, uVar9);
                                }
                            }
                        }
                    }
                    local_6c = local_6c + 1;
                } while (local_6c < local_5c);
            }
        }
    }
    (void)tmp;
    return;
}
