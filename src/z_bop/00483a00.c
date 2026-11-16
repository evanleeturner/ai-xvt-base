/* SPDX-License-Identifier: MIT */
/* z_bop 00483a00 */
/* Prototypes for every callee */
void      __cdecl FUN_00427bd0(short a, short b, void *p);
void      __cdecl FUN_00427d30(short a, short b, void *p);
void      __cdecl FUN_004290b0(int a, int b, int c, short d);
short     __cdecl FUN_0046a460(short a);
unsigned int __cdecl FUN_0046abf0(int a, int b);
long long __cdecl FUN_0046a6d0(short a);
long long __cdecl FUN_0046a3c0(short a);

extern char          *DAT_009a1fe8;
extern short          DAT_009ec474;
extern unsigned int   DAT_00a07c68;
extern int            DAT_00a07c6c;
extern int            DAT_00a07cc4;
extern int            DAT_00a004d0;
extern int            DAT_00a004ac;
extern int            DAT_00a004b4;
extern unsigned int   DAT_00a0049c;
extern unsigned int   DAT_00a0048c;
extern int            DAT_00a00494;

void __cdecl FUN_00483a00(short param_1, short param_2, unsigned int param_3, int param_4)
{
    int iVar2;
    char *iVar10p;
    char *pbase;
    char *pdata;
    short sVar3, sVar4, sVar5;
    short s;
    short sa, sb;
    unsigned int uVar6;
    int iVar7, iVar8, iVar9, iVar10, iVar11, iVar12, iVar13, iVar14, iVar15;
    long long lVar16, lVar17, lVar18;
    short local_5c, local_58, local_54, local_50, local_4c, local_48;
    short local_44, local_40, local_3c, local_38, local_30, local_24, local_1c;
    unsigned char flag;

    iVar2 = (int)((param_3 & 0xffff) * 0x23u);
    iVar10p = DAT_009a1fe8 + iVar2;

    __builtin_memcpy(&pdata, iVar10p + 0x1f, 4);
    if (*(pdata + 0x92) != '\0') {
        __builtin_memcpy(&sa, iVar10p + 0x12, 2);
        __builtin_memcpy(&sb, iVar10p + 0x10, 2);
        FUN_00427bd0(sa, sb, iVar10p);
        __builtin_memcpy(&sa, DAT_009a1fe8 + iVar2 + 0x14, 2);
        FUN_00427d30(sa, 0, DAT_009a1fe8 + iVar2);
    }

    pbase = DAT_009a1fe8 + 0x1f + iVar2;

    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x93, 2);
    DAT_00a07c68 = 0u - (unsigned int)(int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x95, 2);
    DAT_00a07c6c = (int)(0u - (unsigned int)(int)s);
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x97, 2);
    DAT_00a07cc4 = (int)(0u - (unsigned int)(int)s);
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x9f, 2);
    DAT_00a004d0 = (int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0xa1, 2);
    DAT_00a004ac = (int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0xa3, 2);
    DAT_00a004b4 = (int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x99, 2);
    DAT_00a0049c = (unsigned int)(int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x9b, 2);
    DAT_00a0048c = (unsigned int)(int)s;
    __builtin_memcpy(&pdata, pbase, 4);
    __builtin_memcpy(&s, pdata + 0x9d, 2);
    DAT_00a00494 = (int)s;

    FUN_004290b0((int)DAT_00a0049c, (int)DAT_00a0048c, DAT_00a00494, param_1);

    flag = (unsigned char)DAT_009ec474;
    if ((flag & 0xe) != 2) {
        FUN_004290b0(DAT_00a004d0, DAT_00a004ac, DAT_00a004b4, param_2);
    }

    sVar3 = FUN_0046a460((short)(0u - (unsigned int)DAT_00a07cc4));
    __builtin_memcpy((char *)param_4 + 0xd, &sVar3, 2);

    uVar6 = FUN_0046abf0((int)DAT_00a07c68, (int)(0u - (unsigned int)DAT_00a07c6c));
    sVar4 = (short)(0u - (unsigned int)(int)(short)uVar6);

    lVar16 = FUN_0046a6d0(sVar4);
    lVar17 = FUN_0046a3c0(sVar4);
    sVar5 = (short)lVar17;
    lVar17 = FUN_0046a6d0(sVar3);
    lVar18 = FUN_0046a3c0(sVar3);
    sVar3 = (short)lVar18;

    iVar10 = (int)(short)lVar17;
    local_5c = (short)((unsigned int)((int)sVar5 * iVar10) >> 0xf);
    iVar14 = (int)(short)lVar16;
    local_58 = (short)((unsigned int)((int)sVar5 * (int)sVar3) >> 0xf);
    local_54 = (short)((unsigned int)(iVar14 * iVar10) >> 0xf);
    local_50 = (short)((unsigned int)(iVar14 * (int)sVar3) >> 0xf);

    iVar7 = (int)(0u - (unsigned int)(int)sVar5);
    iVar8 = (int)(DAT_00a0049c * (unsigned int)iVar14 + DAT_00a0048c * (unsigned int)iVar7);
    if (0x3fffffff < iVar8) iVar8 = 0x3fffffff;
    if (iVar8 < -0x3fffffff) iVar8 = -0x3fff0000;
    iVar11 = (int)(0u - (unsigned int)(int)sVar3);
    local_4c = (short)(iVar8 >> 0xf);

    iVar8 = (int)local_54;
    iVar15 = (int)local_5c;
    iVar9 = (int)(DAT_00a0049c * (unsigned int)iVar15 + DAT_00a0048c * (unsigned int)iVar8
                  + (unsigned int)DAT_00a00494 * (unsigned int)iVar11);
    if (0x3fffffff < iVar9) iVar9 = 0x3fffffff;
    if (iVar9 < -0x3fffffff) iVar9 = -0x3fff0000;
    local_48 = (short)(iVar9 >> 0xf);

    iVar12 = (int)local_50;
    iVar13 = (int)local_58;
    iVar9 = (int)(DAT_00a0049c * (unsigned int)iVar13 + DAT_00a0048c * (unsigned int)iVar12
                  + (unsigned int)DAT_00a00494 * (unsigned int)iVar10);
    if (0x3fffffff < iVar9) iVar9 = 0x3fffffff;
    if (iVar9 < -0x3fffffff) iVar9 = -0x3fff0000;
    DAT_00a0049c = (unsigned int)(int)local_4c;
    DAT_00a0048c = (unsigned int)(int)local_48;
    local_44 = (short)(iVar9 >> 0xf);
    DAT_00a00494 = (int)local_44;

    iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar14
                  + (unsigned int)DAT_00a004ac * (unsigned int)iVar7);
    if (0x3fffffff < iVar9) iVar9 = 0x3fffffff;
    if (iVar9 < -0x3fffffff) iVar9 = -0x3fff0000;
    local_40 = (short)(iVar9 >> 0xf);

    iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar15
                  + (unsigned int)DAT_00a004ac * (unsigned int)iVar8
                  + (unsigned int)DAT_00a004b4 * (unsigned int)iVar11);
    if (0x3fffffff < iVar9) iVar9 = 0x3fffffff;
    if (iVar9 < -0x3fffffff) iVar9 = -0x3fff0000;
    local_3c = (short)(iVar9 >> 0xf);

    iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar13
                  + (unsigned int)DAT_00a004ac * (unsigned int)iVar12
                  + (unsigned int)DAT_00a004b4 * (unsigned int)iVar10);
    if (0x3fffffff < iVar9) iVar9 = 0x3fffffff;
    if (iVar9 < -0x3fffffff) iVar9 = -0x3fff0000;
    DAT_00a004d0 = (int)local_40;
    DAT_00a004ac = (int)local_3c;
    local_38 = (short)(iVar9 >> 0xf);
    DAT_00a004b4 = (int)local_38;

    iVar7 = (int)(DAT_00a07c68 * (unsigned int)iVar14
                  + (unsigned int)DAT_00a07c6c * (unsigned int)iVar7);
    if (0x3fffffff < iVar7) iVar7 = 0x3fffffff;
    if (iVar7 < -0x3fffffff) iVar7 = -0x3fff0000;
    local_30 = (short)(iVar7 >> 0xf);

    iVar7 = (int)(DAT_00a07c68 * (unsigned int)iVar15
                  + (unsigned int)DAT_00a07c6c * (unsigned int)iVar8
                  + (unsigned int)DAT_00a07cc4 * (unsigned int)iVar11);
    if (0x3fffffff < iVar7) iVar7 = 0x3fffffff;
    if (iVar7 < -0x3fffffff) iVar7 = -0x3fff0000;
    local_24 = (short)(iVar7 >> 0xf);

    iVar10 = (int)(DAT_00a07c68 * (unsigned int)iVar13
                   + (unsigned int)DAT_00a07c6c * (unsigned int)iVar12
                   + (unsigned int)DAT_00a07cc4 * (unsigned int)iVar10);
    if (0x3fffffff < iVar10) iVar10 = 0x3fffffff;
    if (iVar10 < -0x3fffffff) iVar10 = -0x3fff0000;
    DAT_00a07c68 = (unsigned int)(int)local_30;
    DAT_00a07c6c = (int)local_24;
    local_1c = (short)(iVar10 >> 0xf);
    DAT_00a07cc4 = (int)local_1c;

    uVar6 = FUN_0046abf0((int)DAT_00a0048c, (int)DAT_00a0049c);
    __builtin_memcpy(DAT_009a1fe8 + 0x10 + iVar2, &sVar4, 2);
    sa = (short)(0u - (unsigned int)(int)(short)uVar6);
    __builtin_memcpy(DAT_009a1fe8 + 0x14 + iVar2, &sa, 2);
}
