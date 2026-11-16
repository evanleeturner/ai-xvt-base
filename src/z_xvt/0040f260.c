/* SPDX-License-Identifier: MIT */
/* z_xvt 0040f260 */
extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern int DAT_0071b704;
extern int DAT_00732904;

unsigned int FUN_004187e0(unsigned short a, unsigned short b);
void FUN_00483410(short a, short b, int c);
unsigned int FUN_00419100(int a, int b);
unsigned short FUN_004184f0(unsigned short a);
void FUN_00483540(short a, int b, int c);
unsigned int FUN_0040a1a0(unsigned int a, unsigned int b, int c);

void __cdecl FUN_0040f260(unsigned short param_1, unsigned short param_2)
{
    unsigned char *puVar1;
    char cVar2;
    unsigned short uVar3;
    int iVar4;
    int iVar5;
    unsigned short uVar6;
    unsigned short uVar7;
    unsigned int uVar8;
    unsigned int uVar9;
    unsigned char *iVar10;
    unsigned char *iVar11;
    short sVar12;
    short sVar13;
    int iVar14;
    short sVar15;
    unsigned int uVar16;
    int iVar17;
    int iVar18;
    unsigned char *iVar19;
    unsigned short local_2a;
    unsigned short local_28;
    unsigned short local_26;
    int local_18;
    int local_10;
    unsigned char *pObjA;
    unsigned char *pObjB;
    unsigned char *pT;
    int iT0;
    int iT1;
    int iVar11i;
    int iVar19i;
    unsigned short usT;
    int iT2;
    short sa;
    short sb;
    unsigned short ua;
    short s16;

    iVar4 = (int)((unsigned int)param_1 * 0x23u);
    cVar2 = *(char *)(DAT_006d4988 + iVar4 + 2);
    if (((cVar2 == '\0') || (cVar2 == '\x01')) || (cVar2 == '\x02')) {
        __builtin_memcpy(&pObjA, DAT_006d4988 + iVar4 + 0x1f, 4);
        __builtin_memcpy(&iVar10, pObjA + 0xa7, 4);
        *(unsigned short *)(iVar10 + 0x70) = param_2;
        uVar16 = (unsigned int)param_2;
        __builtin_memcpy(&pObjA, DAT_006d4988 + iVar4 + 0x1f, 4);
        __builtin_memcpy(&sVar15, pObjA + 0x79, 2);
        iVar5 = (int)(uVar16 * 0x23u);
        __builtin_memcpy(&sa, DAT_006d4988 + iVar5 + 0x10, 2);
        __builtin_memcpy(&sb, DAT_006d4988 + iVar4 + 0x10, 2);
        uVar6 = (unsigned short)((unsigned int)sa - (unsigned int)sb);
        if (0x7fff < uVar6) {
            uVar6 = (unsigned short)(0u - (unsigned int)uVar6);
        }
        if (0x4000 < uVar6) {
            uVar6 = (unsigned short)(0x8000u - (unsigned int)uVar6);
        }
        if ((int)uVar16 < DAT_006d4980) {
            __builtin_memcpy(&pObjB, DAT_006d4988 + iVar5 + 0x1f, 4);
            __builtin_memcpy(&uVar3, pObjB + 0x79, 2);
            if (uVar6 < 0x4000) {
                uVar8 = FUN_004187e0(uVar3, uVar6);
                sVar15 = (short)((unsigned int)sVar15 - (unsigned int)(short)uVar8);
            } else {
                uVar8 = FUN_004187e0(uVar3, uVar6);
                sVar15 = (short)((unsigned int)sVar15 + (unsigned int)(short)uVar8);
            }
        }
        if (sVar15 < 0) {
            sVar15 = (short)(0u - (unsigned int)sVar15);
        }
        iVar19 = DAT_006d4988 + iVar4;
        __builtin_memcpy(&pObjA, iVar19 + 0x1f, 4);
        iVar11 = 0;
        if (pObjA != 0) {
            __builtin_memcpy(&iVar11, DAT_006d4988 + 0x1f + iVar5, 4);
        }
        if ((pObjA == 0) || (iVar11 == 0)) {
            iVar11 = iVar5 + DAT_006d4988;
            __builtin_memcpy(&iT0, iVar19 + 4, 4);
            __builtin_memcpy(&iT1, iVar11 + 4, 4);
            iVar17 = (int)((unsigned int)iT0 - (unsigned int)iT1);
            __builtin_memcpy(&iT0, iVar19 + 8, 4);
            __builtin_memcpy(&iT1, iVar11 + 8, 4);
            local_10 = (int)((unsigned int)iT0 - (unsigned int)iT1);
            __builtin_memcpy(&iVar14, iVar19 + 0xc, 4);
            __builtin_memcpy(&iVar11i, iVar11 + 0xc, 4);
        } else {
            __builtin_memcpy(&iT0, pObjA + 6, 4);
            __builtin_memcpy(&iT1, iVar11 + 6, 4);
            iVar17 = (int)((unsigned int)iT0 - (unsigned int)iT1);
            __builtin_memcpy(&iT0, pObjA + 10, 4);
            __builtin_memcpy(&iT1, iVar11 + 10, 4);
            local_10 = (int)((unsigned int)iT0 - (unsigned int)iT1);
            __builtin_memcpy(&iVar14, pObjA + 0xe, 4);
            __builtin_memcpy(&iVar11i, iVar11 + 0xe, 4);
        }
        iVar14 = (int)((unsigned int)iVar14 - (unsigned int)iVar11i);
        iVar11i = (int)((unsigned int)iVar14 * (unsigned int)iVar14 +
                        (unsigned int)local_10 * (unsigned int)local_10 +
                        (unsigned int)iVar17 * (unsigned int)iVar17);
        if (iVar11i < 0x33) {
            local_26 = 0;
            local_28 = 100;
            iVar18 = iVar17;
            local_18 = local_10;
        } else {
            iVar18 = (int)((unsigned int)iVar17 * (unsigned int)(int)sVar15 * 1000u);
            local_26 = (unsigned short)(iVar18 / iVar11i);
            local_18 = (int)((unsigned int)local_10 * (unsigned int)(int)sVar15 * 1000u);
            local_28 = (unsigned short)(local_18 / iVar11i);
        }
        __builtin_memcpy(&pObjA, iVar19 + 0x1f, 4);
        if (*(char *)(pObjA + 0x89) != '\0') {
            __builtin_memcpy(&sa, iVar19 + 0x12, 2);
            __builtin_memcpy(&sb, iVar19 + 0x10, 2);
            FUN_00483410(sa, sb, (int)iVar19);
        }
        iVar19 = DAT_006d4988 + iVar4;
        __builtin_memcpy(&pObjA, iVar19 + 0x1f, 4);
        uVar6 = *(unsigned short *)(pObjA + 0x8a);
        local_26 = (unsigned short)(local_26 + uVar6);
        if (((local_26 ^ uVar6) & 0x8000) != 0) {
            local_26 = uVar6;
        }
        __builtin_memcpy(&pObjA, iVar19 + 0x1f, 4);
        uVar6 = *(unsigned short *)(pObjA + 0x8c);
        local_28 = (unsigned short)(local_28 + uVar6);
        if (((uVar6 ^ local_28) & 0x8000) != 0) {
            local_28 = uVar6;
        }
        __builtin_memcpy(&local_2a, iVar19 + 0x10, 2);
        uVar9 = FUN_00419100((int)(short)local_26, (int)(short)local_28);
        s16 = (short)uVar9;
        __builtin_memcpy(iVar19 + 0x10, &s16, 2);
        if ((DAT_0071b704 <= (int)uVar16) && ((int)uVar16 < DAT_00732904)) {
            __builtin_memcpy(&pObjB, DAT_006d4988 + 0x1f + iVar5, 4);
            __builtin_memcpy(&sVar13, pObjB + 0x79, 2);
            puVar1 = DAT_006d4988 + 0x10 + iVar4;
            sVar12 = (short)((unsigned int)sVar13 * 8u);
            __builtin_memcpy(&uVar6, puVar1, 2);
            if (uVar6 < local_2a) {
                sVar12 = (short)((unsigned int)sVar13 * (unsigned int)-8);
            }
            uVar6 = (unsigned short)(uVar6 + sVar12);
            __builtin_memcpy(puVar1, &uVar6, 2);
        }
        if ((int)uVar16 < DAT_006d4980) {
            iVar11 = DAT_006d4988 + iVar5;
            __builtin_memcpy(&pObjB, iVar11 + 0x1f, 4);
            __builtin_memcpy(&iVar19, pObjB + 0xa7, 4);
            uVar7 = FUN_004184f0(local_26);
            __builtin_memcpy(iVar11 + 0x12, &uVar7, 2);
            __builtin_memcpy(&usT, DAT_006d4988 + 0x12 + iVar5, 2);
            __builtin_memcpy(iVar19 + 0xd, &usT, 2);
            uVar6 = (unsigned short)((unsigned int)sVar15 * 100u);
            if (0x7fff < uVar6) {
                uVar6 = 0x7fff;
            }
            __builtin_memcpy(&ua, iVar5 + DAT_006d4988 + 0x10, 2);
            if (local_2a < ua) {
                uVar6 = (unsigned short)(0u - (unsigned int)uVar6);
            }
            __builtin_memcpy(&pObjB, iVar5 + DAT_006d4988 + 0x1f, 4);
            __builtin_memcpy(pObjB + 0x77, &uVar6, 2);
            iVar19 = DAT_006d4988 + iVar5;
            __builtin_memcpy(&sa, iVar19 + 0x12, 2);
            __builtin_memcpy(&sb, iVar19 + 0x10, 2);
            FUN_00483410(sa, sb, (int)iVar19);
            __builtin_memcpy(&sa, DAT_006d4988 + iVar5 + 0x14, 2);
            FUN_00483540(sa, 0, (int)(DAT_006d4988 + iVar5));
        }
        if ((int)uVar16 < DAT_006d4980) {
            __builtin_memcpy(&pObjB, DAT_006d4988 + 0x1f + iVar5, 4);
            __builtin_memcpy(&pT, pObjB + 0xa7, 4);
            *(unsigned short *)(pT + 0x70) = param_1;
            iVar19i = (int)((unsigned int)iVar14 * (unsigned int)iVar14 +
                            (unsigned int)local_10 * (unsigned int)local_10 +
                            (unsigned int)iVar17 * (unsigned int)iVar17);
            if (iVar19i < 0x33) {
                sVar12 = 0;
                sVar13 = 100;
            } else {
                sVar12 = (short)(((int)((unsigned int)iVar18 * (unsigned int)(int)sVar15 * 1000u)) / iVar19i);
                sVar13 = (short)(((int)((unsigned int)local_18 * (unsigned int)(int)sVar15 * 1000u)) / iVar19i);
            }
            iVar19 = iVar5 + DAT_006d4988;
            __builtin_memcpy(&pObjB, iVar19 + 0x1f, 4);
            if (*(char *)(pObjB + 0x89) != '\0') {
                __builtin_memcpy(&sa, iVar19 + 0x12, 2);
                __builtin_memcpy(&sb, iVar19 + 0x10, 2);
                FUN_00483410(sa, sb, (int)iVar19);
            }
            iVar19 = DAT_006d4988 + iVar5;
            __builtin_memcpy(&pObjB, iVar19 + 0x1f, 4);
            sVar12 = (short)((unsigned int)sVar12 + (unsigned int)*(short *)(pObjB + 0x8a));
            __builtin_memcpy(&local_2a, iVar19 + 0x10, 2);
            __builtin_memcpy(&pObjB, iVar19 + 0x1f, 4);
            uVar9 = FUN_00419100((int)sVar12,
                                 (int)(short)((unsigned int)sVar13 + (unsigned int)*(short *)(pObjB + 0x8c)));
            s16 = (short)uVar9;
            __builtin_memcpy(iVar19 + 0x10, &s16, 2);
            uVar7 = FUN_004184f0((unsigned short)sVar12);
            __builtin_memcpy(DAT_006d4988 + 0x12 + iVar4, &uVar7, 2);
            __builtin_memcpy(&usT, DAT_006d4988 + 0x12 + iVar4, 2);
            __builtin_memcpy(iVar10 + 0xd, &usT, 2);
        }
        uVar6 = (unsigned short)((unsigned int)sVar15 * 100u);
        if (0x7fff < uVar6) {
            uVar6 = 0x7fff;
        }
        __builtin_memcpy(&ua, iVar4 + DAT_006d4988 + 0x10, 2);
        if (local_2a < ua) {
            uVar6 = (unsigned short)(0u - (unsigned int)uVar6);
        }
        __builtin_memcpy(&pObjA, iVar4 + DAT_006d4988 + 0x1f, 4);
        __builtin_memcpy(pObjA + 0x77, &uVar6, 2);
        iVar19 = DAT_006d4988 + iVar4;
        __builtin_memcpy(&sa, iVar19 + 0x12, 2);
        __builtin_memcpy(&sb, iVar19 + 0x10, 2);
        FUN_00483410(sa, sb, (int)iVar19);
        __builtin_memcpy(&sa, DAT_006d4988 + iVar4 + 0x14, 2);
        FUN_00483540(sa, 0, (int)(DAT_006d4988 + iVar4));
        if ((int)uVar16 < DAT_00732904) {
            iVar19 = DAT_006d4988 + iVar5;
            __builtin_memcpy(&sa, iVar19 + 0x12, 2);
            __builtin_memcpy(&sb, iVar19 + 0x10, 2);
            FUN_00483410(sa, sb, (int)iVar19);
            __builtin_memcpy(&sa, DAT_006d4988 + iVar5 + 0x14, 2);
            FUN_00483540(sa, 0, (int)(DAT_006d4988 + iVar5));
        }
        __builtin_memcpy(&iT2, DAT_006d4988 + 0x1b + iVar4, 4);
        FUN_0040a1a0(0x1f, (unsigned int)param_1, iT2);
        if ((int)uVar16 < DAT_006d4980) {
            __builtin_memcpy(&iT2, DAT_006d4988 + 0x1b + iVar4, 4);
            FUN_0040a1a0(0x20, uVar16, iT2);
        }
    }
    return;
}
