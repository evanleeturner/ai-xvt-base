/* SPDX-License-Identifier: MIT */
/* z_bop 0043d2b0 */
extern unsigned short DAT_00a08374;
extern int DAT_009a806c;
extern int DAT_009a7bac;
extern int DAT_009a8c14;
extern int DAT_005233d8;
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009e9697[];
extern unsigned char DAT_009e9698[];
extern unsigned char *DAT_009a1fe8;
extern unsigned short DAT_00a0a216[];
extern unsigned char DAT_00a08de4[];
extern unsigned char DAT_00a08de6[];
extern unsigned char DAT_00a08de8[];
extern unsigned char DAT_005233c8;
extern unsigned char DAT_009ed232;
extern unsigned char DAT_009a20ae;
extern void (*DAT_009d1300)(void);
extern unsigned char DAT_009ec464;

void __cdecl FUN_0040e0c0(int a, int b, int c, int d);
void __cdecl FUN_004a9640(char a);
void __cdecl FUN_004a9520(short a, short b, short c, short d);
void __cdecl FUN_004a9500(short a, short b);
void __cdecl FUN_004a95c0(unsigned int a);
void __cdecl FUN_004a9590(unsigned int a);
unsigned int __cdecl FUN_004269e0(unsigned int a);
void __cdecl FUN_004277f0(unsigned int a, unsigned int b, unsigned int c);
void __cdecl FUN_00440760(unsigned int a, unsigned int b);

void __cdecl FUN_0043d2b0(unsigned int param_1, unsigned short param_2, unsigned int param_3)
{
    unsigned char *piVar1;
    unsigned char bVar2;
    unsigned int iVar3;
    unsigned int iVar4;
    int bVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    char cVar8;
    short sVar9;
    unsigned short uVar10;
    unsigned int uVar11;
    unsigned int slot;
    unsigned int tmp;
    unsigned short us;
    unsigned char *pb;

    if (DAT_00a08374 != 0) {
        FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                     (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
        iVar4 = (unsigned int)DAT_009ecc34 * 0x5bd;
        __builtin_memcpy(&slot, &DAT_009e9670[iVar4], 4);
        iVar3 = slot * 0x23;
        piVar1 = DAT_009a1fe8 + (iVar3 + 0x1f);
        __builtin_memcpy(&tmp, piVar1, 4);
        __builtin_memcpy(&iVar3, (unsigned char *)tmp + 0xa9, 4);
        pb = (unsigned char *)iVar3;
        if (pb[0x232] == 0) {
            uVar10 = 0;
        } else {
            uVar10 = (unsigned short)pb[0x372 + (param_1 & 0xffff) * 4];
        }
        DAT_00a0a216[(unsigned int)DAT_00a08374 + (unsigned int)param_2] = uVar10;
        if (uVar10 == 0 ||
            (__builtin_memcpy(&tmp, piVar1, 4),
             __builtin_memcpy(&iVar3, (unsigned char *)tmp + 0xa9, 4),
             pb = (unsigned char *)iVar3,
             (pb[0x1e5] & 8) == 0)) {
            cVar8 = 0;
        } else if (DAT_009e9698[iVar4] == 0) {
            cVar8 = 1;
        } else if ((param_3 & 0xffff) == (unsigned int)DAT_009e9697[iVar4]) {
            bVar2 = pb[DAT_009e9697[iVar4] + 0x235];
            if ((bVar2 & 0x7f) == 3) {
                cVar8 = 2;
            } else {
                cVar8 = (char)(((param_2 & 1) == (unsigned short)(bVar2 >> 7)) + 1);
            }
        } else {
            cVar8 = 1;
        }
        if (uVar10 == 0) {
            FUN_004a9640(2);
            sVar9 = (short)((DAT_009ed232 + 1) * param_2);
            FUN_004a9520((short)(sVar9 + 2), 2,
                         (short)(sVar9 + (unsigned short)DAT_009ed232 + 3),
                         (short)(DAT_009a20ae + 3));
            FUN_004a95c0((unsigned int)DAT_005233c8);
            (*DAT_009d1300)();
        } else {
            FUN_004a9640(2);
            FUN_004a9520(0, 0, (short)DAT_009a7bac, (short)DAT_009a8c14);
            FUN_004a9500((short)((DAT_009ed232 + 1) * param_2 + 2), 2);
            FUN_004a95c0((unsigned int)DAT_005233c8);
            if (cVar8 == 2) {
                uVar11 = 0x52;
            } else {
                uVar11 = 0x4a;
            }
            FUN_004a9590(uVar11);
            DAT_009ec464 = 0;
            __builtin_memcpy(&us, &DAT_00a08de8[((unsigned int)DAT_00a08374 + (unsigned int)param_2) * 0xc], 2);
            if (us != 0) {
                FUN_004a9640(0);
                __builtin_memcpy(&slot, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
                uVar11 = FUN_004269e0(((((unsigned int)DAT_009a1fe8 >> 8) << 8) |
                                       (unsigned int)DAT_009a1fe8[slot * 0x23 + 3]) & 0xffff00ff);
                uVar6 = FUN_004269e0(0xc);
                if ((short)uVar11 == (short)uVar6) {
                    FUN_004277f0(uVar10, 2, 1);
                } else {
                    FUN_004277f0(uVar10, 1, 1);
                }
            }
        }
        FUN_0040e0c0(0, 0x140, 200, 0);
        return;
    }

    __builtin_memcpy(&slot, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
    __builtin_memcpy(&tmp, DAT_009a1fe8 + (slot * 0x23 + 0x1f), 4);
    __builtin_memcpy(&iVar3, (unsigned char *)tmp + 0xa9, 4);
    pb = (unsigned char *)iVar3;
    if (pb[0x232] == 0) {
        uVar10 = 0;
    } else {
        uVar10 = (unsigned short)pb[0x372 + (param_1 & 0xffff) * 4];
    }
    uVar11 = (unsigned int)param_2;
    if (DAT_00a0a216[uVar11] != uVar10) {
        short s1, s2;
        DAT_00a0a216[uVar11] = uVar10;
        FUN_004a9640(2);
        FUN_004a9520(0, 0, (short)DAT_009a7bac, (short)DAT_009a8c14);
        __builtin_memcpy(&s1, &DAT_00a08de4[((unsigned int)DAT_00a08374 + uVar11) * 0xc], 2);
        __builtin_memcpy(&s2, &DAT_00a08de6[((unsigned int)DAT_00a08374 + uVar11) * 0xc], 2);
        FUN_004a9500(s1, s2);
        FUN_004a95c0((unsigned int)DAT_005233c8);
        FUN_004a9590(0x4a);
        DAT_009ec464 = 0;
        __builtin_memcpy(&slot, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
        uVar6 = FUN_004269e0(((((unsigned int)DAT_009a1fe8 >> 8) << 8) |
                              (unsigned int)DAT_009a1fe8[slot * 0x23 + 3]) & 0xffff00ff);
        uVar7 = FUN_004269e0(0xc);
        if ((short)uVar6 == (short)uVar7) {
            uVar6 = 2;
        } else {
            uVar6 = 1;
        }
        FUN_004277f0(uVar10, uVar6, 1);
    }
    if (uVar10 != 0) {
        __builtin_memcpy(&slot, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
        __builtin_memcpy(&tmp, DAT_009a1fe8 + (slot * 0x23 + 0x1f), 4);
        __builtin_memcpy(&iVar3, (unsigned char *)tmp + 0xa9, 4);
        pb = (unsigned char *)iVar3;
        if ((pb[0x1e5] & 8) != 0) {
            if (DAT_009e9698[(unsigned int)DAT_009ecc34 * 0x5bd] == 0) {
                uVar10 = 1;
            } else if ((param_3 & 0xffff) == (unsigned int)DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bd]) {
                bVar2 = pb[DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bd] + 0x235];
                if ((bVar2 & 0x7f) == 3) {
                    uVar10 = 2;
                } else {
                    uVar10 = (unsigned short)(((param_2 & 1) == (unsigned short)(bVar2 >> 7)) + 1);
                }
            } else {
                uVar10 = 1;
            }
            goto LAB_0043d778;
        }
    }
    uVar10 = 0;
LAB_0043d778:
    __builtin_memcpy(&iVar3, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
    if (iVar3 == 0xffffffffu ||
        (cVar8 = (char)DAT_009a1fe8[iVar3 * 0x23 + 3], cVar8 != 1 && cVar8 != 2 &&
         cVar8 != 3 && cVar8 != 0x0e && cVar8 != 4)) {
        bVar5 = 0;
    } else {
        bVar5 = 1;
    }
    if (bVar5 && uVar10 == 2) {
        uVar10 = 4;
    }
    FUN_00440760(uVar11 + 0x13, (unsigned int)uVar10);
    return;
}
