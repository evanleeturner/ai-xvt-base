/* SPDX-License-Identifier: MIT */
/* z_xvt 00436120 */
extern unsigned char DAT_0071b743[];
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb23[];
extern unsigned char DAT_0071bb27[];
extern unsigned char DAT_0071bb29[];
extern unsigned char DAT_0071bb2b[];
extern unsigned char DAT_0071bb2d[];
extern unsigned char DAT_0071bb2f[];
extern unsigned char DAT_0071bb33[];
extern unsigned char DAT_0071bb35[];
extern unsigned char DAT_0052686c[];
extern int DAT_006d4988;
extern short DAT_006da9f0;
extern short DAT_00732e40;
extern int DAT_006db594;
extern int DAT_006da538;
extern int DAT_006db7ac;
extern int DAT_006db7b0;
extern int DAT_006db7b4;
extern int DAT_0070acc4;
extern int DAT_0071e53c;
extern int DAT_00732a38;
extern int DAT_00703c88;

void __cdecl FUN_00483180(short, short, short, short, short, short, int);
void __cdecl FUN_00418cd0(int, int, int);
unsigned char __cdecl FUN_004340e0(unsigned int, unsigned int, int);
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

unsigned int __cdecl FUN_00436120(int param_1)
{
    unsigned char bVar1;
    int iVar2;
    unsigned char bVar3;
    short sVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int iVar9;
    short sVar10;
    short sVar11;
    unsigned int local_8;
    unsigned int local_4;
    short s0, s1, s2, s3, s4;
    unsigned short us0, us1, us2;
    int i0, i1, i2, j0, j1, j2;
    int dividend;

    iVar2 = (int)((unsigned int)param_1 * 0x5bdU);
    if (DAT_0071b743[iVar2] < 2) {
        __builtin_memcpy(&s0, DAT_0071bb2f + iVar2, 2);
        __builtin_memcpy(&s1, DAT_0071bb2b + iVar2, 2);
        __builtin_memcpy(&s2, DAT_0071bb2d + iVar2, 2);
        __builtin_memcpy(&s3, DAT_0071bb33 + iVar2, 2);
        __builtin_memcpy(&s4, DAT_0071bb35 + iVar2, 2);
        FUN_00483180(s0, s1, s2, 0, s3, s4, 0);
        __builtin_memcpy(&us0, DAT_0071bb29 + iVar2, 2);
        if (us0 == 0xffff) goto LAB_0043620b;
        iVar9 = (int)((unsigned int)us0 * 0x23U + (unsigned int)DAT_006d4988);
        __builtin_memcpy(&i0, (unsigned char *)(iVar9 + 4), 4);
        __builtin_memcpy(&i1, (unsigned char *)(iVar9 + 8), 4);
        __builtin_memcpy(&i2, (unsigned char *)(iVar9 + 0xc), 4);
        __builtin_memcpy(&j0, DAT_0071bb1b + iVar2, 4);
        __builtin_memcpy(&j1, DAT_0071bb1f + iVar2, 4);
        __builtin_memcpy(&j2, DAT_0071bb23 + iVar2, 4);
        FUN_00418cd0((int)((unsigned int)i0 - (unsigned int)j0),
                     (int)((unsigned int)i1 - (unsigned int)j1),
                     (int)((unsigned int)i2 - (unsigned int)j2));
        sVar4 = 0;
        sVar11 = DAT_006da9f0;
        sVar10 = DAT_00732e40;
    } else {
        sVar4 = (short)((int)((unsigned int)(DAT_0071b743[iVar2] & 0x7f) << 0xe) / -0x7f);
        sVar11 = 0;
        sVar10 = 0x4000;
    }
    FUN_00483180(0, sVar10, sVar11, 0, sVar4, 0, 0);
LAB_0043620b:
    local_4 = 0xffff;
    iVar9 = 0;
    uVar8 = 0;
    local_8 = ((unsigned int)DAT_006db594 * (unsigned int)DAT_006db594 +
               (unsigned int)DAT_006da538 * (unsigned int)DAT_006da538) >> 3;
    if (0 < DAT_0071e53c) {
        do {
            uVar7 = (unsigned int)*(unsigned char *)(DAT_006d4988 + iVar9 + 3);
            if (uVar7 != 0) {
                if (*(unsigned char *)(DAT_006d4988 + iVar9 + 2) <= 5) {
                    __builtin_memcpy(&us0, DAT_0052686c + uVar7 * 0x16, 2);
                    bVar3 = FUN_004340e0(uVar8, (unsigned int)us0, param_1);
                    if (bVar3 != 0) {
                        dividend = (int)((unsigned int)DAT_006db7ac << (DAT_0070acc4 & 0x1f));
                        if (DAT_006db7b4 == 0 || (dividend == (int)0x80000000 && DAT_006db7b4 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar5 = dividend / DAT_006db7b4;
                        dividend = (int)((unsigned int)DAT_006db7b0 << (DAT_0070acc4 & 0x1f));
                        if (DAT_006db7b4 == 0 || (dividend == (int)0x80000000 && DAT_006db7b4 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar6 = dividend / DAT_006db7b4;
                        uVar7 = (unsigned int)iVar6 * (unsigned int)iVar6 +
                                (unsigned int)iVar5 * (unsigned int)iVar5;
                        __builtin_memcpy(&us1, DAT_0071bb29 + iVar2, 2);
                        __builtin_memcpy(&us2, DAT_0071bb27 + iVar2, 2);
                        if ((unsigned int)us1 == uVar8 || (unsigned int)us2 == uVar8) {
                            uVar7 = uVar7 + (unsigned int)(DAT_006db594 >> 4) * (unsigned int)(DAT_006db594 >> 4) +
                                            (unsigned int)(DAT_006da538 >> 4) * (unsigned int)(DAT_006da538 >> 4);
                        }
                        if ((int)uVar7 < (int)local_8) {
                            local_8 = uVar7;
                            local_4 = uVar8;
                        }
                    }
                }
            }
            iVar9 = (int)((unsigned int)iVar9 + 0x23U);
            uVar8 = uVar8 + 1;
        } while ((int)uVar8 < DAT_0071e53c);
    }
    if (DAT_00732a38 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
        iVar9 = (int)((unsigned int)DAT_00732a38 * 0x23U);
        uVar8 = (unsigned int)DAT_00732a38;
        do {
            uVar7 = (unsigned int)*(unsigned char *)(DAT_006d4988 + iVar9 + 3);
            if (uVar7 != 0) {
                bVar1 = *(unsigned char *)(DAT_006d4988 + iVar9 + 2);
                if (7 < bVar1 && bVar1 < 10) {
                    __builtin_memcpy(&us0, DAT_0052686c + uVar7 * 0x16, 2);
                    bVar3 = FUN_004340e0(uVar8, (unsigned int)us0, param_1);
                    if (bVar3 != 0) {
                        dividend = (int)((unsigned int)DAT_006db7ac << (DAT_0070acc4 & 0x1f));
                        if (DAT_006db7b4 == 0 || (dividend == (int)0x80000000 && DAT_006db7b4 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar5 = dividend / DAT_006db7b4;
                        dividend = (int)((unsigned int)DAT_006db7b0 << (DAT_0070acc4 & 0x1f));
                        if (DAT_006db7b4 == 0 || (dividend == (int)0x80000000 && DAT_006db7b4 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar6 = dividend / DAT_006db7b4;
                        uVar7 = (unsigned int)iVar6 * (unsigned int)iVar6 +
                                (unsigned int)iVar5 * (unsigned int)iVar5;
                        __builtin_memcpy(&us1, DAT_0071bb29 + iVar2, 2);
                        __builtin_memcpy(&us2, DAT_0071bb27 + iVar2, 2);
                        if ((unsigned int)us1 == uVar8 || (unsigned int)us2 == uVar8) {
                            uVar7 = uVar7 + (unsigned int)(DAT_006db594 >> 4) * (unsigned int)(DAT_006db594 >> 4) +
                                            (unsigned int)(DAT_006da538 >> 4) * (unsigned int)(DAT_006da538 >> 4);
                        }
                        if ((int)uVar7 < (int)local_8) {
                            local_8 = uVar7;
                            local_4 = uVar8;
                        }
                    }
                }
            }
            iVar9 = (int)((unsigned int)iVar9 + 0x23U);
            uVar8 = uVar8 + 1;
        } while ((int)uVar8 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
    }
    return local_4;
}
