/* SPDX-License-Identifier: MIT */
/* z_xvt 00442e60 */
extern unsigned char *DAT_006d4988;
extern unsigned short DAT_0071e534;
extern unsigned int DAT_00732550;
extern unsigned int DAT_00732540;
extern int DAT_00732544;
extern int DAT_00732580;
extern int DAT_00732560;
extern int DAT_00732564;
extern int DAT_007329c4;
extern int DAT_007329c8;
extern int DAT_00732a14;

void __cdecl FUN_00483410(short, short, int);
void __cdecl FUN_00483540(short, short, int);
void __cdecl FUN_00483cf0(int, int, int, short);
short __cdecl FUN_004184f0(short);
int __cdecl FUN_00419100(int, int);
int __cdecl FUN_004187b0(short);
int __cdecl FUN_00418450(short);
int __cdecl FUN_00413ca0(int, int);
int __cdecl FUN_00413d00(int, int, int, int, int, int);

void __cdecl FUN_00442e60(short param_1, short param_2, unsigned int param_3, int param_4)
{
    unsigned int iVar2;
    unsigned char *iVar11;
    unsigned char *pInner;
    short s0, s1, s2;
    short sVar3, sVar4, sVar5, sOut;
    int uVar6;
    int iVar7, iVar8, iVar9, iVar10;
    int iVar12, iVar13, iVar14, iVar15, iVar16, iVar17;
    int lVar18, lVar19, lVar20;
    int uVar21, uVar22, uVar23, uVar24;

    iVar2 = (param_3 & 0xffff) * 0x23;
    iVar11 = DAT_006d4988 + iVar2;
    __builtin_memcpy(&pInner, iVar11 + 0x1f, 4);
    if (*(char *)(pInner + 0x90) != '\0') {
        __builtin_memcpy(&s1, iVar11 + 0x12, 2);
        __builtin_memcpy(&s0, iVar11 + 0x10, 2);
        FUN_00483410(s1, s0, (int)iVar11);
        __builtin_memcpy(&s2, DAT_006d4988 + iVar2 + 0x14, 2);
        FUN_00483540(s2, 0, (int)(DAT_006d4988 + iVar2));
    }

    __builtin_memcpy(&pInner, DAT_006d4988 + 0x1f + iVar2, 4);

    __builtin_memcpy(&s0, pInner + 0x91, 2);
    DAT_007329c4 = -(int)s0;
    __builtin_memcpy(&s0, pInner + 0x93, 2);
    DAT_007329c8 = -(int)s0;
    __builtin_memcpy(&s0, pInner + 0x95, 2);
    DAT_00732a14 = -(int)s0;
    __builtin_memcpy(&s0, pInner + 0x9d, 2);
    DAT_00732580 = (int)s0;
    __builtin_memcpy(&s0, pInner + 0x9f, 2);
    DAT_00732560 = (int)s0;
    __builtin_memcpy(&s0, pInner + 0xa1, 2);
    DAT_00732564 = (int)s0;
    __builtin_memcpy(&s0, pInner + 0x97, 2);
    DAT_00732550 = (unsigned int)(int)s0;
    __builtin_memcpy(&s0, pInner + 0x99, 2);
    DAT_00732540 = (unsigned int)(int)s0;
    __builtin_memcpy(&s0, pInner + 0x9b, 2);
    DAT_00732544 = (int)s0;

    FUN_00483cf0((int)DAT_00732550, (int)DAT_00732540, DAT_00732544, param_1);
    if (((unsigned char)DAT_0071e534 & 0xe) != 2) {
        FUN_00483cf0(DAT_00732580, DAT_00732560, DAT_00732564, param_2);
    }

    sVar3 = FUN_004184f0((short)(-(int)(short)DAT_00732a14));
    __builtin_memcpy((unsigned char *)param_4 + 0xd, &sVar3, 2);

    uVar6 = FUN_00419100(DAT_007329c4, (int)(0u - (unsigned int)DAT_007329c8));
    sVar4 = (short)(-(int)(short)uVar6);

    lVar18 = FUN_004187b0(sVar4);
    lVar19 = FUN_00418450(sVar4);
    sVar5 = (short)lVar19;
    lVar19 = FUN_004187b0(sVar3);
    lVar20 = FUN_00418450(sVar3);
    sVar3 = (short)lVar20;
    iVar11 = 0;
    {
        int iVarA = (int)(short)lVar19;
        uVar21 = FUN_00413ca0((int)sVar5, iVarA);
        uVar22 = FUN_00413ca0((int)sVar5, (int)sVar3);
        iVar14 = (int)(short)lVar18;
        uVar23 = FUN_00413ca0(iVar14, iVarA);
        uVar24 = FUN_00413ca0(iVar14, (int)sVar3);

        iVar7 = (int)-sVar5;
        iVar8 = FUN_00413d00((int)DAT_00732550, (int)DAT_00732540, DAT_00732544, iVar14, iVar7, 0);
        iVar12 = (int)-sVar3;
        iVar13 = (int)(short)uVar23;
        iVar16 = (int)(short)uVar21;
        iVar9 = FUN_00413d00((int)DAT_00732550, (int)DAT_00732540, DAT_00732544, iVar16, iVar13, iVar12);
        iVar15 = (int)(short)uVar24;
        iVar17 = (int)(short)uVar22;
        iVar10 = FUN_00413d00((int)DAT_00732550, (int)DAT_00732540, DAT_00732544, iVar17, iVar15, iVarA);
        DAT_00732550 = (unsigned int)(int)(short)iVar8;
        DAT_00732540 = (unsigned int)(int)(short)iVar9;
        DAT_00732544 = (int)(short)iVar10;

        iVar8 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, iVar14, iVar7, 0);
        iVar9 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, iVar16, iVar13, iVar12);
        iVar10 = FUN_00413d00(DAT_00732580, DAT_00732560, DAT_00732564, iVar17, iVar15, iVarA);
        DAT_00732580 = (int)(short)iVar8;
        DAT_00732560 = (int)(short)iVar9;
        DAT_00732564 = (int)(short)iVar10;

        iVar7 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, iVar14, iVar7, 0);
        iVar8 = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, iVar16, iVar13, iVar12);
        iVarA = FUN_00413d00(DAT_007329c4, DAT_007329c8, DAT_00732a14, iVar17, iVar15, iVarA);
        DAT_007329c4 = (int)(short)iVar7;
        DAT_007329c8 = (int)(short)iVar8;
        DAT_00732a14 = (int)(short)iVarA;
    }

    uVar6 = FUN_00419100((int)DAT_00732540, (int)DAT_00732550);
    __builtin_memcpy(DAT_006d4988 + 0x10 + iVar2, &sVar4, 2);
    sOut = (short)(-(int)(short)uVar6);
    __builtin_memcpy(DAT_006d4988 + 0x14 + iVar2, &sOut, 2);
}
