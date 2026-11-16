/* SPDX-License-Identifier: MIT */
/* z_bop 00438010 */
extern unsigned char DAT_009e9683[];
extern unsigned char DAT_009e9a69[];
extern unsigned char DAT_009e9a67[];
extern unsigned char DAT_009e9a6b[];
extern unsigned char DAT_009e9a6d[];
extern unsigned char DAT_009e9a6f[];
extern unsigned char DAT_009e9a73[];
extern unsigned char DAT_009e9a75[];
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern int DAT_009a1fe8;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern short DAT_009a8070;
extern short DAT_00a080f0;
extern int DAT_009ec47c;
extern int DAT_009a8e2c;
extern int DAT_009a8e30;
extern int DAT_009a8e34;
extern int DAT_009d8c04;
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009d1308;
extern unsigned int DAT_00521f1c[];

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
void __cdecl FUN_00427940(short a, short b, short c, short d, short e, short f, int g);
void __cdecl FUN_0046a8d0(int a, int b, int c);
unsigned char __cdecl FUN_00436470(unsigned int a, unsigned int b, int c);

unsigned int __cdecl FUN_00438010(int param_1)
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
    unsigned short us0, us1;
    short s0, s1, s2, s3, s4;
    int i0, i1, i2;
    int e1, e2, e3;
    int tmp;

    iVar2 = (int)((unsigned int)param_1 * 0x5bdu);
    if (DAT_009e9683[iVar2] < 2) {
        __builtin_memcpy(&s0, &DAT_009e9a6f[iVar2], 2);
        __builtin_memcpy(&s1, &DAT_009e9a6b[iVar2], 2);
        __builtin_memcpy(&s2, &DAT_009e9a6d[iVar2], 2);
        __builtin_memcpy(&s3, &DAT_009e9a73[iVar2], 2);
        __builtin_memcpy(&s4, &DAT_009e9a75[iVar2], 2);
        FUN_00427940(s0, s1, s2, 0, s3, s4, 0);
        __builtin_memcpy(&us0, &DAT_009e9a69[iVar2], 2);
        if (us0 == 0xffff) goto LAB_004380fb;
        iVar9 = (int)((unsigned int)us0 * 0x23u + (unsigned int)DAT_009a1fe8);
        __builtin_memcpy(&e1, (unsigned char *)iVar9 + 4, 4);
        __builtin_memcpy(&e2, (unsigned char *)iVar9 + 8, 4);
        __builtin_memcpy(&e3, (unsigned char *)iVar9 + 0xc, 4);
        __builtin_memcpy(&i0, &DAT_009e9a5b[iVar2], 4);
        __builtin_memcpy(&i1, &DAT_009e9a5f[iVar2], 4);
        __builtin_memcpy(&i2, &DAT_009e9a63[iVar2], 4);
        FUN_0046a8d0((int)((unsigned int)e1 - (unsigned int)i0),
                     (int)((unsigned int)e2 - (unsigned int)i1),
                     (int)((unsigned int)e3 - (unsigned int)i2));
        sVar4 = 0;
        sVar11 = DAT_009a8070;
        sVar10 = DAT_00a080f0;
    } else {
        sVar4 = (short)((int)((unsigned int)(DAT_009e9683[iVar2] & 0x7f) << 0xe) / -0x7f);
        sVar11 = 0;
        sVar10 = 0x4000;
    }
    FUN_00427940(0, sVar10, sVar11, 0, sVar4, 0, 0);
LAB_004380fb:
    local_4 = 0xffff;
    iVar9 = 0;
    uVar8 = 0;
    local_8 = (unsigned int)((int)((unsigned int)DAT_009a7bac * (unsigned int)DAT_009a7bac +
                                   (unsigned int)DAT_009a8c14 * (unsigned int)DAT_009a8c14) >> 3);
    if (0 < DAT_009ec47c) {
        do {
            uVar7 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar9 + 3);
            if (uVar7 != 0) {
                switch (*(unsigned char *)(DAT_009a1fe8 + iVar9 + 2)) {
                case 0:
                case 1:
                case 2:
                case 3:
                case 4:
                case 5:
                    bVar3 = FUN_00436470(uVar8, DAT_00521f1c[uVar7 * 0x1a], param_1);
                    if (bVar3 != 0) {
                        tmp = (int)((unsigned int)DAT_009a8e2c << (DAT_009d8c04 & 0x1f));
                        if (DAT_009a8e34 == 0 || (tmp == (int)0x80000000 && DAT_009a8e34 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar5 = tmp / DAT_009a8e34;
                        tmp = (int)((unsigned int)DAT_009a8e30 << (DAT_009d8c04 & 0x1f));
                        if (DAT_009a8e34 == 0 || (tmp == (int)0x80000000 && DAT_009a8e34 == -1))
                            RaiseException(0xC0000094UL, 0, 0, NULL);
                        iVar6 = tmp / DAT_009a8e34;
                        uVar7 = (unsigned int)iVar6 * (unsigned int)iVar6 +
                                (unsigned int)iVar5 * (unsigned int)iVar5;
                        __builtin_memcpy(&us0, &DAT_009e9a69[iVar2], 2);
                        __builtin_memcpy(&us1, &DAT_009e9a67[iVar2], 2);
                        if ((us0 == uVar8) || (us1 == uVar8)) {
                            uVar7 = uVar7 + (unsigned int)(DAT_009a8c14 >> 4) * (unsigned int)(DAT_009a8c14 >> 4) +
                                    (unsigned int)(DAT_009a7bac >> 4) * (unsigned int)(DAT_009a7bac >> 4);
                        }
                        if ((int)uVar7 < (int)local_8) {
                            local_8 = uVar7;
                            local_4 = uVar8;
                        }
                    }
                }
            }
            iVar9 = iVar9 + 0x23;
            uVar8 = uVar8 + 1;
        } while ((int)uVar8 < DAT_009ec47c);
    }
    if ((int)DAT_00a07ce8 < (int)(DAT_009d1308 + DAT_00a07ce8)) {
        iVar9 = (int)(DAT_00a07ce8 * 0x23u);
        uVar8 = DAT_00a07ce8;
        do {
            uVar7 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + iVar9 + 3);
            if ((((uVar7 != 0) &&
                  (bVar1 = *(unsigned char *)(DAT_009a1fe8 + iVar9 + 2), 7 < bVar1)) &&
                 (bVar1 < 10)) &&
                ((bVar3 = FUN_00436470(uVar8, DAT_00521f1c[uVar7 * 0x1a], param_1)) != 0)) {
                tmp = (int)((unsigned int)DAT_009a8e2c << (DAT_009d8c04 & 0x1f));
                if (DAT_009a8e34 == 0 || (tmp == (int)0x80000000 && DAT_009a8e34 == -1))
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                iVar5 = tmp / DAT_009a8e34;
                tmp = (int)((unsigned int)DAT_009a8e30 << (DAT_009d8c04 & 0x1f));
                if (DAT_009a8e34 == 0 || (tmp == (int)0x80000000 && DAT_009a8e34 == -1))
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                iVar6 = tmp / DAT_009a8e34;
                uVar7 = (unsigned int)iVar6 * (unsigned int)iVar6 +
                        (unsigned int)iVar5 * (unsigned int)iVar5;
                __builtin_memcpy(&us0, &DAT_009e9a69[iVar2], 2);
                __builtin_memcpy(&us1, &DAT_009e9a67[iVar2], 2);
                if ((us0 == uVar8) || (us1 == uVar8)) {
                    uVar7 = uVar7 + (unsigned int)(DAT_009a8c14 >> 4) * (unsigned int)(DAT_009a8c14 >> 4) +
                            (unsigned int)(DAT_009a7bac >> 4) * (unsigned int)(DAT_009a7bac >> 4);
                }
                if ((int)uVar7 < (int)local_8) {
                    local_8 = uVar7;
                    local_4 = uVar8;
                }
            }
            iVar9 = iVar9 + 0x23;
            uVar8 = uVar8 + 1;
        } while ((int)uVar8 < (int)(DAT_009d1308 + DAT_00a07ce8));
    }
    return local_4;
}
