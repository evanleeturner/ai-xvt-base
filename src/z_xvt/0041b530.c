/* SPDX-License-Identifier: MIT */
/* z_xvt 0041b530 */
extern int DAT_006d4988;
extern unsigned char DAT_0052686c[];
extern unsigned char DAT_0071b75d;
extern unsigned char DAT_0071b75f;

int  __cdecl FUN_00412810(int, int, int);
void __cdecl FUN_00483410(short, short, unsigned int);
void __cdecl FUN_00483540(short, int, unsigned int);
int  __cdecl FUN_00413cc0(int, int, int, int, int, int);
int  __cdecl FUN_004490f0(unsigned int);
int *__cdecl FUN_004496d0(unsigned int, int);
long long __cdecl __ftol(void);
void __cdecl FUN_00449770(void);
void __cdecl FUN_004110e0(unsigned int, int, int, unsigned int, unsigned int,
                          unsigned short, unsigned int, unsigned short, unsigned int);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

/* Unaligned access via memcpy. */
#define LD(dst, addr, n) __builtin_memcpy(&(dst), (const void *)(addr), (n))
#define ST(addr, src, n) __builtin_memcpy((void *)(addr), &(src), (n))

/* Unresolved: the x87 operands feeding each __ftol are not in the evidence
 * (the original passes floats via the FPU stack); calls kept in decompile order.
 * Unresolved: unaff_E* arguments to FUN_004110e0 are leaked callee-saved
 * registers in the decompile; kept as placeholders, zero-initialised only so
 * the build passes. */
void __cdecl FUN_0041b530(unsigned int param_1, int param_2)
{
    unsigned int uVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    int *piVar6;
    unsigned int uVar7;
    int iVar8;
    unsigned int unaff_EBX = 0;
    int iVar9;
    unsigned int unaff_EBP = 0;
    int iVar10;
    unsigned int unaff_ESI = 0;
    int iVar11;
    int iVar12;
    unsigned int unaff_EDI = 0;
    int iVar13;
    int iVar14;
    long long lVar15;
    long long lVar16;
    int local_20;
    int local_24;
    int local_28;

    unsigned int t0;
    unsigned int t1;
    unsigned short s0;
    unsigned short s1;
    unsigned short s2;
    unsigned short w16;

    uVar1 = (unsigned int)param_2 * 0x23u + (unsigned int)DAT_006d4988;
    LD(s0, &DAT_0052686c[(unsigned int)*(unsigned char *)(uVar1 + 3) * 0x16u], 2);
    uVar2 = (unsigned int)s0;
    iVar3 = (int)((unsigned int)DAT_006d4988 + param_1 * 0x23u);
    LD(t0, iVar3 + 4, 4);
    LD(t1, uVar1 + 4, 4);
    iVar11 = (int)(t0 - t1);
    LD(t0, iVar3 + 8, 4);
    LD(t1, uVar1 + 8, 4);
    iVar13 = (int)(t0 - t1);
    LD(t0, iVar3 + 0xc, 4);
    LD(t1, uVar1 + 0xc, 4);
    iVar9 = (int)(t0 - t1);
    iVar3 = FUN_00412810(iVar11, iVar13, iVar9);
    LD(t0, uVar1 + 0x1f, 4);
    if ((iVar3 <= (int)(uVar2 * 3u)) && (t0 != 0)) {
        if (*(char *)(t0 + 0x90) != '\0') {
            LD(s0, uVar1 + 0x12, 2);
            LD(s1, uVar1 + 0x10, 2);
            FUN_00483410((short)s0, (short)s1, uVar1);
            LD(s0, uVar1 + 0x14, 2);
            FUN_00483540((short)s0, 0, uVar1);
        }
        LD(t0, uVar1 + 0x1f, 4);
        LD(s0, t0 + 0x97, 2);
        LD(s1, t0 + 0x99, 2);
        LD(s2, t0 + 0x9b, 2);
        iVar4 = FUN_00413cc0((int)(short)s0, (int)(short)s1, (int)(short)s2, iVar11, iVar13, iVar9);
        LD(t0, uVar1 + 0x1f, 4);
        LD(s0, t0 + 0x91, 2);
        LD(s1, t0 + 0x93, 2);
        LD(s2, t0 + 0x95, 2);
        uVar5 = (unsigned int)FUN_00413cc0((int)(short)s0, (int)(short)s1, (int)(short)s2,
                                           iVar11, iVar13, iVar9);
        if (uVar5 == uVar2 || (int)(0u - uVar2) < (int)(0u - uVar5)) {
            LD(t0, uVar1 + 0x1f, 4);
            LD(s0, t0 + 0x9d, 2);
            LD(s1, t0 + 0x9f, 2);
            LD(s2, t0 + 0xa1, 2);
            iVar3 = FUN_00413cc0((int)(short)s0, (int)(short)s1, (int)(short)s2,
                                 iVar11, iVar13, iVar9);
            iVar9 = FUN_004490f0((unsigned int)*(unsigned char *)(uVar1 + 3));
            local_20 = 0;
            if (0 < iVar9) {
                do {
                    piVar6 = FUN_004496d0((unsigned int)*(unsigned char *)(uVar1 + 3), local_20);
                    if (piVar6 != (int *)0) {
                        if (*piVar6 == 6) {
                            lVar15 = __ftol();
                            lVar16 = __ftol();
                            iVar13 = (int)lVar16;
                            lVar16 = __ftol();
                            iVar8 = (int)lVar16;
                            iVar11 = (int)lVar15;
                            if ((int)lVar15 < iVar13) {
                                iVar11 = iVar13;
                            }
                            if (iVar11 < iVar8) {
                                iVar11 = iVar8;
                            }
                            iVar14 = (int)((unsigned int)iVar11 * 8u);
                            if ((int)(uVar2 * 2u) < (int)((unsigned int)iVar11 * 8u)) {
                                iVar14 = (int)(uVar2 * 2u);
                            }
                            lVar15 = __ftol();
                            iVar11 = (int)(0u - uVar5 - (unsigned int)(int)lVar15);
                            if ((0 < iVar11) && (iVar11 <= iVar14)) {
                                lVar15 = __ftol();
                                iVar12 = (int)((unsigned int)iVar4 - (unsigned int)(int)lVar15);
                                lVar15 = __ftol();
                                iVar10 = (int)((unsigned int)iVar3 - (unsigned int)(int)lVar15);
                                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)(iVar13 >> 1));
                                iVar13 = (int)((unsigned int)iVar13 +
                                               (unsigned int)((int)((unsigned int)iVar13 * (unsigned int)iVar11) / iVar14));
                                iVar8 = (int)((unsigned int)iVar8 + (unsigned int)(iVar8 >> 1));
                                iVar8 = (int)((unsigned int)iVar8 +
                                              (unsigned int)((int)((unsigned int)iVar8 * (unsigned int)iVar11) / iVar14));
                                if (iVar12 < 0) {
                                    iVar12 = (int)(0u - (unsigned int)iVar12);
                                }
                                if (iVar10 < 0) {
                                    iVar10 = (int)(0u - (unsigned int)iVar10);
                                }
                                if ((iVar12 <= iVar13) && (iVar10 <= iVar8)) {
                                    FUN_00449770();
                                    local_24 = (int)((((unsigned int)iVar8 - (unsigned int)iVar10 -
                                                       (unsigned int)iVar12) + (unsigned int)iVar13) * 100u);
                                    local_28 = (int)((unsigned int)iVar8 + (unsigned int)iVar13);
                                    if (local_28 == 0 || (local_28 == -1 && local_24 == (int)0x80000000)) {
                                        RaiseException(0xC0000094UL, 0, 0, NULL);
                                    }
                                    uVar7 = (unsigned int)
                                        (((int)((unsigned int)
                                                ((int)((unsigned int)(local_24 / local_28) *
                                                       (unsigned int)(((int)(((unsigned int)iVar14 - (unsigned int)iVar11) * 100u)) / iVar14)) / 100) *
                                                (unsigned int)(iVar14 >> 8)) / 100));
                                    if (0x40 < (int)uVar7) {
                                        uVar7 = 0x40;
                                    }
                                    iVar13 = (int)((unsigned int)DAT_006d4988 + param_1 * 0x23u);
                                    LD(iVar11, iVar13 + 0x1b, 4);
                                    if (iVar11 != -1) {
                                        iVar11 = (int)((unsigned int)iVar11 * 0x5bdu);
                                        LD(w16, (unsigned char *)&DAT_0071b75f + iVar11, 2);
                                        if ((int)(unsigned int)w16 < (int)uVar7) {
                                            w16 = (unsigned short)param_2;
                                            ST((unsigned char *)&DAT_0071b75d + iVar11, w16, 2);
                                            w16 = (unsigned short)uVar7;
                                            ST((unsigned char *)&DAT_0071b75f + iVar11, w16, 2);
                                        }
                                    }
                                    LD(iVar11, iVar13 + 0x1f, 4);
                                    if (iVar11 != 0) {
                                        LD(iVar11, iVar11 + 0xa7, 4);
                                        if (iVar11 != 0) {
                                            LD(s0, iVar11 + 0x20b, 2);
                                            LD(s1, iVar11 + 0x20d, 2);
                                            if ((int)(short)s0 + (int)(short)s1 == 0) {
                                                uVar7 = (unsigned int)((int)uVar7 >> 3);
                                            }
                                        }
                                    }
                                    if ((int)uVar7 < 1) {
                                        uVar7 = 1;
                                    }
                                    FUN_004110e0(param_1, 0xffff, 0xfffe, uVar7, unaff_EBP,
                                                 (unsigned short)unaff_EDI, unaff_ESI,
                                                 (unsigned short)unaff_EBX, uVar1);
                                    goto LAB_0041b8fd;
                                }
                            }
                        }
                        FUN_00449770();
                    }
LAB_0041b8fd:
                    local_20 = local_20 + 1;
                } while (local_20 < iVar9);
            }
        }
    }
    return;
}
