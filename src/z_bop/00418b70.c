/* SPDX-License-Identifier: MIT */
/* z_bop 00418b70 */
extern int DAT_00523430;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009ec458[];
extern unsigned char *DAT_009a1fe8;
extern int DAT_009ecc34;
extern unsigned char DAT_00550a08[];
extern unsigned char DAT_00550a0e[];
extern unsigned char DAT_00550a12[];
extern unsigned char DAT_00550a16[];
extern unsigned char DAT_00550a26[];
extern unsigned char DAT_00550a28[];
extern unsigned char DAT_00550a2a[];
extern unsigned char DAT_00550888[];
extern unsigned char DAT_0055088c[];
extern unsigned char DAT_0055088e[];
extern unsigned char DAT_00550892[];
extern unsigned char DAT_00550896[];
extern unsigned char DAT_0055089a[];
extern unsigned char DAT_0055089e[];
extern unsigned char DAT_005508a2[];
extern unsigned char DAT_005508a6[];
extern unsigned char DAT_005508a8[];
extern unsigned char DAT_005508aa[];
extern unsigned char DAT_005508ac[];
extern unsigned char DAT_005508ae[];
extern unsigned char DAT_005508b0[];
extern unsigned char DAT_005508b2[];
extern unsigned char DAT_005508b4[];

/* Calling convention of FUN_0041f410 is unresolved; left as default (cdecl). */
int FUN_0041f410(int a, int b, int c);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

#define RD32(v, p) __builtin_memcpy(&(v), (const void *)(p), 4)
#define WR32(p, v) __builtin_memcpy((void *)(p), &(v), 4)
#define RD16(v, p) __builtin_memcpy(&(v), (const void *)(p), 2)
#define WR16(p, v) __builtin_memcpy((void *)(p), &(v), 2)

void __stdcall FUN_00418b70(void)
{
    long long lVar1;
    long long lVar2;
    long long lVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    int iVar11;
    int local_1c;
    int local_18;
    int local_14;
    int local_10;
    int local_8;
    unsigned char *local_c;
    unsigned char *iVar10;
    unsigned char *p2;
    unsigned int divisor;
    unsigned int dividend;
    int t32;
    short t16a;
    short t16b;
    unsigned short tu16;

    iVar11 = 0;
    if (DAT_00523430 != 0) {
        local_c = DAT_009e9670;
        local_1c = 0;
        do {
            t32 = 0;
            WR32(DAT_00550a08 + iVar11, t32);

            if ((char)local_c[0x10] == '\0') goto LAB_next;
            RD32(t32, local_c);
            if (t32 == -1) goto LAB_next;
            iVar10 = DAT_009a1fe8 + (unsigned int)t32 * 0x23u;
            if (iVar10[3] == 0) goto LAB_next;
            RD32(p2, iVar10 + 0x1f);
            if (p2 == 0) goto LAB_next;
            RD32(t32, DAT_00550888 + iVar11);
            if (t32 == 0) goto LAB_next;
            if (DAT_009ecc34 == local_1c) goto LAB_next;
            RD16(tu16, DAT_0055088c + iVar11);
            RD32(t32, local_c + 4);
            if ((unsigned int)tu16 != (unsigned int)t32) goto LAB_next;

            RD16(t16a, iVar10 + 0x14);
            WR16(DAT_00550a26 + iVar11, t16a);
            RD16(t16a, iVar10 + 0x12);
            WR16(DAT_00550a28 + iVar11, t16a);
            RD16(t16a, iVar10 + 0x10);
            WR16(DAT_00550a2a + iVar11, t16a);
            RD32(t32, iVar10 + 4);
            WR32(DAT_00550a0e + iVar11, t32);
            RD32(local_18, DAT_0055088e + iVar11);
            RD32(t32, iVar10 + 8);
            WR32(DAT_00550a12 + iVar11, t32);
            RD32(local_14, DAT_00550892 + iVar11);
            RD32(t32, iVar10 + 0xc);
            WR32(DAT_00550a16 + iVar11, t32);
            RD32(local_10, DAT_00550896 + iVar11);
            t32 = 1;
            WR32(DAT_00550a08 + iVar11, t32);

            RD32(p2, iVar10 + 0x1f);
            RD32(iVar5, p2 + 2);
            RD32(t32, DAT_005508b4 + iVar11);
            iVar4 = (int)((unsigned int)iVar5 - (unsigned int)t32);
            if (-1 < iVar4) {
                iVar9 = 0;
                RD32(t32, DAT_005508b4 + iVar11);
                RD16(tu16, DAT_005508b2 + iVar11);
                if ((iVar4 != 0 && t32 <= iVar5) && (tu16 != 0)) {
                    iVar9 = (int)((unsigned int)((int)((unsigned int)tu16 * 0x1234u + 0x80u) >> 8) *
                                  (unsigned int)iVar4) / 0xec;
                    RD16(t16a, DAT_005508ac + iVar11);
                    lVar1 = (long long)(int)t16a * (long long)iVar9;
                    local_18 = (int)((unsigned int)local_18 + (unsigned int)(lVar1 >> 15));
                    RD16(t16a, DAT_005508ae + iVar11);
                    lVar1 = (long long)(int)t16a * (long long)iVar9;
                    local_14 = (int)((unsigned int)local_14 + (unsigned int)(lVar1 >> 15));
                    RD16(t16a, DAT_005508b0 + iVar11);
                    lVar1 = (long long)(int)t16a * (long long)iVar9;
                    local_10 = (int)((unsigned int)local_10 + (unsigned int)(lVar1 >> 15));
                }
                RD32(t32, iVar10 + 4);
                iVar5 = (int)((unsigned int)t32 - (unsigned int)local_18);
                RD32(t32, iVar10 + 8);
                iVar6 = (int)((unsigned int)t32 - (unsigned int)local_14);
                RD32(t32, iVar10 + 0xc);
                iVar7 = (int)((unsigned int)t32 - (unsigned int)local_10);
                iVar8 = FUN_0041f410(iVar5, iVar6, iVar7);
                if (((int)((unsigned int)iVar9 * 0x20u) < iVar8) || (iVar8 == 0)) {
                    iVar9 = 0x4000;
                } else {
                    divisor = (unsigned int)iVar9 * 0x20u;
                    dividend = (unsigned int)iVar8 << 0xe;
                    if (divisor == 0 ||
                        ((int)dividend == (int)0x80000000 && (int)divisor == -1)) {
                        RaiseException(0xC0000094UL, 0, 0, NULL);
                    }
                    iVar9 = (int)dividend / (int)divisor;
                }
                lVar1 = (long long)iVar9 * (long long)iVar5;
                lVar2 = (long long)iVar9 * (long long)iVar6;
                lVar3 = (long long)iVar9 * (long long)iVar7;
                t32 = (int)((unsigned int)local_18 + (unsigned int)(lVar1 >> 15));
                WR32(iVar10 + 4, t32);
                t32 = (int)((unsigned int)local_14 + (unsigned int)(lVar2 >> 15));
                WR32(iVar10 + 8, t32);
                t32 = (int)((unsigned int)local_10 + (unsigned int)(lVar3 >> 15));
                WR32(iVar10 + 0xc, t32);

                /* axis 0x14 */
                RD16(t16a, iVar10 + 0x14);
                RD16(t16b, DAT_005508a6 + iVar11);
                iVar5 = (int)(short)(t16a - t16b);
                RD32(t32, DAT_0055089a + iVar11);
                if (t32 < 1) {
                    if ((t32 < 0) && (0 < iVar5)) goto LAB_00418e34;
LAB_00418e3c:
                    iVar9 = iVar5;
                    if (iVar5 < 0) {
                        iVar9 = (int)(0u - (unsigned int)iVar5);
                    }
                } else {
                    iVar9 = iVar5;
                    if (iVar5 < 0) {
LAB_00418e34:
                        RD16(t16b, DAT_005508a6 + iVar11);
                        WR16(iVar10 + 0x14, t16b);
                        iVar5 = 0;
                        goto LAB_00418e3c;
                    }
                }
                iVar4 = (int)((unsigned int)iVar4 * 0x1800u) / 0xec;
                if (iVar4 < iVar9) {
                    RD16(t16b, DAT_005508a6 + iVar11);
                    iVar5 = (int)((unsigned int)(int)t16b + (unsigned int)iVar5);
                    RD16(t16a, iVar10 + 0x14);
                    iVar9 = (int)((unsigned int)(int)t16a - (unsigned int)iVar5);
                    if (iVar9 < 0) {
                        iVar9 = (int)(0u - (unsigned int)iVar9);
                    }
                    if ((unsigned int)iVar4 * 8u - (unsigned int)iVar9 != 0 &&
                        iVar9 <= (int)((unsigned int)iVar4 * 8u)) {
                        t16a = (short)iVar5;
                        WR16(iVar10 + 0x14, t16a);
                    }
                }

                /* axis 0x12 */
                RD16(t16a, iVar10 + 0x12);
                RD16(t16b, DAT_005508a8 + iVar11);
                local_8 = (int)(short)(t16a - t16b);
                RD32(t32, DAT_0055089e + iVar11);
                if (t32 < 1) {
                    if ((t32 < 0) && (0 < local_8)) goto LAB_00418eb1;
LAB_00418eba:
                    iVar5 = local_8;
                    if (local_8 < 0) {
                        iVar5 = (int)(0u - (unsigned int)local_8);
                    }
                } else {
                    iVar5 = local_8;
                    if (local_8 < 0) {
LAB_00418eb1:
                        RD16(t16b, DAT_005508a8 + iVar11);
                        WR16(iVar10 + 0x12, t16b);
                        local_8 = 0;
                        goto LAB_00418eba;
                    }
                }
                if (iVar4 < iVar5) {
                    RD16(t16b, DAT_005508a8 + iVar11);
                    local_8 = (int)((unsigned int)local_8 + (unsigned int)(int)t16b);
                    RD16(t16a, iVar10 + 0x12);
                    iVar5 = (int)((unsigned int)(int)t16a - (unsigned int)local_8);
                    if (iVar5 < 0) {
                        iVar5 = (int)(0u - (unsigned int)iVar5);
                    }
                    if ((unsigned int)iVar4 * 8u - (unsigned int)iVar5 != 0 &&
                        iVar5 <= (int)((unsigned int)iVar4 * 8u)) {
                        t16a = (short)local_8;
                        WR16(iVar10 + 0x12, t16a);
                    }
                }

                /* axis 0x10 */
                RD16(t16a, iVar10 + 0x10);
                RD16(t16b, DAT_005508aa + iVar11);
                local_8 = (int)(short)(t16a - t16b);
                RD32(t32, DAT_005508a2 + iVar11);
                if (t32 < 1) {
                    if ((t32 < 0) && (0 < local_8)) goto LAB_00418f1e;
LAB_00418f27:
                    iVar5 = local_8;
                    if (local_8 < 0) {
                        iVar5 = (int)(0u - (unsigned int)local_8);
                    }
                } else {
                    iVar5 = local_8;
                    if (local_8 < 0) {
LAB_00418f1e:
                        RD16(t16b, DAT_005508aa + iVar11);
                        WR16(iVar10 + 0x10, t16b);
                        local_8 = 0;
                        goto LAB_00418f27;
                    }
                }
                if (iVar4 < iVar5) {
                    RD16(t16b, DAT_005508aa + iVar11);
                    local_8 = (int)((unsigned int)(int)t16b + (unsigned int)local_8);
                    RD16(t16a, iVar10 + 0x10);
                    iVar5 = (int)((unsigned int)(int)t16a - (unsigned int)local_8);
                    if (iVar5 < 0) {
                        iVar5 = (int)(0u - (unsigned int)iVar5);
                    }
                    if ((unsigned int)iVar4 * 8u - (unsigned int)iVar5 != 0 &&
                        iVar5 <= (int)((unsigned int)iVar4 * 8u)) {
                        t16a = (short)local_8;
                        WR16(iVar10 + 0x10, t16a);
                    }
                }
            }
LAB_next:
            iVar11 = iVar11 + 0x30;
            local_c = local_c + 0x5bd;
            local_1c = local_1c + 1;
        } while (local_c < DAT_009ec458);
    }
    return;
}
