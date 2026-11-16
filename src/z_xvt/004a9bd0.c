/* SPDX-License-Identifier: MIT */
/* z_xvt 004a9bd0 */
extern int DAT_006d4988;
extern unsigned char DAT_0071b730[];
extern int DAT_0071ecf4;
extern short DAT_00655178;
extern short _DAT_0071f2de;
extern int DAT_006d499c;
extern short DAT_00655abc[];
extern short DAT_00655abe[];
extern short DAT_00655ab0[];
extern short DAT_00655ab2[];
extern unsigned short DAT_00655184;
extern short DAT_00655a8c;
extern short DAT_00655a88;
extern int DAT_00655150;
extern int DAT_00655814;
extern unsigned short DAT_00655a8e;
extern unsigned short DAT_00655812;
extern short DAT_00656c82;
extern short DAT_00656c84;

/* FINDING: the original pushes 8 argument bytes and the caller cleans them, so
   only the last two of Ghidra's four arguments are stack arguments (__cdecl).
   The first two Ghidra arguments are register leftovers (unresolved). */
long long __cdecl FUN_00413ca0(int c, int d);
void __cdecl FUN_00483410(short a, short b, int p);
void __cdecl FUN_00483540(short a, int b, int p);
void __cdecl FUN_0042a1a0(unsigned int a, unsigned int b);
void __cdecl FUN_004785e0(int a, int b, int c);

void __cdecl FUN_004a9bd0(unsigned short param_1)
{
    char *pcVar1;
    unsigned short uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int iVar9;
    _Bool bVar10;
    long long uVar11;
    long long uVar12;
    long long uVar13;
    long long uVar14;
    long long uVar15;
    long long uVar16;

    iVar3 = (int)((unsigned int)param_1 * 0x23u);
    iVar5 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar3);
    __builtin_memcpy(&iVar7, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu, 4);
    iVar7 = (int)((unsigned int)iVar7 * 0x23u);
    iVar4 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar7);

    __builtin_memcpy(&iVar9, (void *)(iVar5 + 4), 4);
    __builtin_memcpy(&iVar6, (void *)(iVar4 + 4), 4);
    iVar9 = (int)((unsigned int)iVar9 - (unsigned int)iVar6);
    __builtin_memcpy(&iVar8, (void *)(iVar5 + 8), 4);
    __builtin_memcpy(&iVar6, (void *)(iVar4 + 8), 4);
    iVar8 = (int)((unsigned int)iVar8 - (unsigned int)iVar6);
    __builtin_memcpy(&iVar6, (void *)(iVar5 + 0xc), 4);
    __builtin_memcpy(&iVar5, (void *)(iVar4 + 0xc), 4);
    iVar5 = (int)((unsigned int)iVar6 - (unsigned int)iVar5);

    __builtin_memcpy(&pcVar1, (void *)(iVar4 + 0x1f), sizeof(pcVar1));
    if (pcVar1[0x90] != '\0') {
        __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x10), 2);
        iVar6 = (short)uVar2;
        __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x12), 2);
        FUN_00483410((short)uVar2, (short)iVar6, iVar4);
        __builtin_memcpy(&uVar2, (void *)(DAT_006d4988 + iVar7 + 0x14), 2);
        FUN_00483540((short)uVar2, 0, DAT_006d4988 + iVar7);
    }

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x91), 2);
    iVar6 = (short)uVar2;
    uVar11 = FUN_00413ca0(iVar9, iVar6);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x93), 2);
    iVar4 = (short)uVar2;
    uVar12 = FUN_00413ca0(iVar8, iVar4);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x95), 2);
    iVar4 = (short)uVar2;
    uVar13 = FUN_00413ca0(iVar5, iVar4);

    iVar6 = (int)((unsigned int)(int)uVar11 + (unsigned int)(int)uVar12 + (unsigned int)(int)uVar13);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x97), 2);
    iVar4 = (short)uVar2;
    uVar11 = FUN_00413ca0(iVar9, iVar4);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x99), 2);
    uVar12 = FUN_00413ca0(iVar8, (short)uVar2);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x9b), 2);
    iVar4 = (short)uVar2;
    uVar13 = FUN_00413ca0(iVar5, iVar4);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x9d), 2);
    iVar4 = (short)uVar2;
    uVar14 = FUN_00413ca0(iVar9, iVar4);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0x9f), 2);
    iVar4 = (short)uVar2;
    uVar15 = FUN_00413ca0(iVar8, iVar4);

    __builtin_memcpy(&iVar4, (void *)(DAT_006d4988 + 0x1f + iVar7), 4);
    __builtin_memcpy(&uVar2, (void *)(iVar4 + 0xa1), 2);
    iVar7 = (short)uVar2;
    uVar16 = FUN_00413ca0(iVar5, iVar7);

    bVar10 = iVar6 < 0;
    if (bVar10) {
        iVar6 = (int)(0u - (unsigned int)iVar6);
    }

    iVar3 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar3);
    __builtin_memcpy(&pcVar1, (void *)(iVar3 + 0x1f), sizeof(pcVar1));
    if (pcVar1 == (char *)0) {
        DAT_00655178 = 0x2f;
    }
    else if (*(char *)(iVar3 + 2) == '\t') {
        DAT_00655178 = 0x2f;
    }
    else if (*pcVar1 == '\x01') {
        if (((int)((unsigned int)(int)_DAT_0071f2de + ((unsigned int)((int)_DAT_0071f2de >> 0x1f) & 3U)) >> 2 & 1U) == 0) {
            DAT_00655178 = 0x37;
        }
        else {
            DAT_00655178 = 0x3b;
        }
    }
    else {
        switch (pcVar1[0x86]) {
        case '\0':
            DAT_00655178 = 0x3f;
            break;
        case '\x01':
        case '\x04':
            DAT_00655178 = 0x37;
            break;
        case '\x02':
            DAT_00655178 = 0x33;
            break;
        case '\x03':
            DAT_00655178 = 0x3b;
            break;
        case '\x05':
            DAT_00655178 = 0xd3;
        }
    }

    __builtin_memcpy(&iVar4, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bdu, 4);
    FUN_0042a1a0((unsigned int)iVar4, (unsigned int)param_1);

    if (DAT_006d499c < 0x1dd37) {
        if (0xee9b < DAT_006d499c) {
            if (DAT_00655178 == 0x2f) {
                DAT_00655178 = 0x2e;
            }
            else if (DAT_00655178 == 0xd3) {
                DAT_00655178 = 0xd4;
            }
            else {
                DAT_00655178 = DAT_00655178 + -1;
            }
        }
    }
    else if (DAT_00655178 == 0x2f) {
        DAT_00655178 = 0x2d;
    }
    else if (DAT_00655178 == 0xd3) {
        DAT_00655178 = 0xd5;
    }
    else {
        DAT_00655178 = DAT_00655178 + -2;
    }

    FUN_004785e0((int)((unsigned int)(int)uVar11 + (unsigned int)(int)uVar12 + (unsigned int)(int)uVar13),
                 (int)(0u - ((unsigned int)(int)uVar14 + (unsigned int)(int)uVar15 + (unsigned int)(int)uVar16)),
                 iVar6);

    if (bVar10) {
        DAT_00655a8c = DAT_00655a8c + DAT_00655abc[(unsigned int)DAT_00655184 * 6];
        DAT_00655a88 = DAT_00655a88 + DAT_00655abe[(unsigned int)DAT_00655184 * 6];
        if (DAT_00655a88 < 0) {
            DAT_00655a88 = 0;
        }
        __builtin_memcpy((void *)(DAT_00655150 + (unsigned int)DAT_00655a8e * 6), &DAT_00655a8c, 2);
        __builtin_memcpy((void *)(DAT_00655150 + 2 + (unsigned int)DAT_00655a8e * 6), &DAT_00655a88, 2);
        __builtin_memcpy((void *)(DAT_00655150 + 4 + (unsigned int)DAT_00655a8e * 6), &DAT_00655178, 2);
        uVar2 = DAT_00655a8e;
        DAT_00655a8e = DAT_00655a8e + 1;
        if (DAT_00655a8e == 0x30) {
            DAT_00655a8e = uVar2;
        }
    }
    else {
        DAT_00655a8c = DAT_00655a8c + DAT_00655ab0[(unsigned int)DAT_00655184 * 6];
        DAT_00655a88 = DAT_00655a88 + DAT_00655ab2[(unsigned int)DAT_00655184 * 6];
        if (DAT_00655a88 < 0) {
            DAT_00655a88 = 0;
        }
        __builtin_memcpy((void *)(DAT_00655814 + (unsigned int)DAT_00655812 * 6), &DAT_00655a8c, 2);
        __builtin_memcpy((void *)(DAT_00655814 + 2 + (unsigned int)DAT_00655812 * 6), &DAT_00655a88, 2);
        __builtin_memcpy((void *)(DAT_00655814 + 4 + (unsigned int)DAT_00655812 * 6), &DAT_00655178, 2);
        uVar2 = DAT_00655812;
        DAT_00655812 = DAT_00655812 + 1;
        if (DAT_00655812 == 0x30) {
            DAT_00655812 = uVar2;
        }
    }

    __builtin_memcpy(&uVar2, DAT_0071b730 + 0x1a + (unsigned int)DAT_0071ecf4 * 0x5bdu, 2);
    if (uVar2 == param_1) {
        DAT_00656c82 = DAT_00655a8c;
        DAT_00656c84 = DAT_00655a88;
    }
    return;
}
