/* SPDX-License-Identifier: MIT */
/* z_bop 0041bc50 */
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int DAT_009e9640;
extern int DAT_00a00854;

unsigned int __cdecl FUN_0046a700(unsigned short a, unsigned short b);
void __cdecl FUN_00427bd0(short a, short b, int c);
unsigned int __cdecl FUN_0046abf0(int a, int b);
unsigned short __cdecl FUN_0046a460(unsigned short a);
void __cdecl FUN_00427d30(short a, int b, int c);
void __cdecl FUN_0042e4d0(int a, unsigned int b, int c);

#define BASE ((unsigned char *)DAT_009a1fe8)

void __cdecl FUN_0041bc50(unsigned short param_1, unsigned short param_2)
{
    char cVar1;
    unsigned short uVar2;
    int iVar3;
    int iVar4;
    unsigned short uVar5;
    unsigned short uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned char *iVar9;
    int iVar10;
    short sVar11;
    short sVar12;
    unsigned char *iVar13;
    unsigned int uVar14;
    int iVar15;
    int iVar16;
    short sVar17;
    int iVar18;
    unsigned short local_2a;
    unsigned short local_28;
    unsigned short local_26;
    int local_18;
    int local_10;
    unsigned char *pA;
    unsigned char *pB;
    unsigned char *inA;
    unsigned char *inB;
    unsigned char *aux;
    int t0;
    int t1;

    iVar3 = (int)((unsigned int)param_1 * 0x23u);
    cVar1 = *(char *)(BASE + iVar3 + 2);
    if (((cVar1 == '\0') || (cVar1 == '\x01')) || (cVar1 == '\x02')) {
        __builtin_memcpy(&inA, BASE + iVar3 + 0x1f, 4);
        __builtin_memcpy(&iVar9, inA + 0xa9, 4);
        *(unsigned short *)(iVar9 + 0x70) = param_2;
        uVar14 = (unsigned int)param_2;
        __builtin_memcpy(&inA, BASE + iVar3 + 0x1f, 4);
        __builtin_memcpy(&sVar12, inA + 0x79, 2);
        iVar4 = (int)(uVar14 * 0x23u);
        uVar5 = (unsigned short)((unsigned int)(int)*(short *)(BASE + iVar4 + 0x10) -
                                 (unsigned int)(int)*(short *)(BASE + iVar3 + 0x10));
        if (0x7fff < uVar5) {
            uVar5 = (unsigned short)(0u - (unsigned int)uVar5);
        }
        if (0x4000 < uVar5) {
            uVar5 = (unsigned short)(0x8000u - (unsigned int)uVar5);
        }
        if ((int)uVar14 < DAT_009a1fe0) {
            __builtin_memcpy(&inB, BASE + iVar4 + 0x1f, 4);
            __builtin_memcpy(&uVar2, inB + 0x79, 2);
            if (uVar5 < 0x4000) {
                uVar7 = FUN_0046a700(uVar2, uVar5);
                sVar12 = (short)((unsigned int)(int)sVar12 - (unsigned int)(int)(short)uVar7);
            }
            else {
                uVar7 = FUN_0046a700(uVar2, uVar5);
                sVar12 = (short)((unsigned int)(int)sVar12 + (unsigned int)(int)(short)uVar7);
            }
        }
        if (sVar12 < 0) {
            sVar12 = (short)(0u - (unsigned int)(int)sVar12);
        }
        pA = BASE + iVar3;
        pB = BASE + iVar4;
        iVar13 = pA;
        __builtin_memcpy(&inA, pA + 0x1f, 4);
        __builtin_memcpy(&inB, pB + 0x1f, 4);
        if ((inA == 0) || (inB == 0)) {
            iVar15 = (int)((unsigned int)*(int *)(pA + 4) - (unsigned int)*(int *)(pB + 4));
            local_10 = (int)((unsigned int)*(int *)(pA + 8) - (unsigned int)*(int *)(pB + 8));
            iVar18 = *(int *)(pA + 0xc);
            iVar10 = *(int *)(pB + 0xc);
        }
        else {
            __builtin_memcpy(&t0, inA + 6, 4);
            __builtin_memcpy(&t1, inB + 6, 4);
            iVar15 = (int)((unsigned int)t0 - (unsigned int)t1);
            __builtin_memcpy(&t0, inA + 10, 4);
            __builtin_memcpy(&t1, inB + 10, 4);
            local_10 = (int)((unsigned int)t0 - (unsigned int)t1);
            __builtin_memcpy(&iVar18, inA + 0xe, 4);
            __builtin_memcpy(&iVar10, inB + 0xe, 4);
        }
        iVar18 = (int)((unsigned int)iVar18 - (unsigned int)iVar10);
        iVar10 = (int)((unsigned int)iVar18 * (unsigned int)iVar18 +
                       (unsigned int)local_10 * (unsigned int)local_10 +
                       (unsigned int)iVar15 * (unsigned int)iVar15);
        if (iVar10 < 0x33) {
            local_26 = 0;
            local_28 = 100;
            iVar16 = iVar15;
            local_18 = local_10;
        }
        else {
            iVar16 = (int)((unsigned int)iVar15 * (unsigned int)(int)sVar12 * 1000u);
            local_26 = (unsigned short)(iVar16 / iVar10);
            local_18 = (int)((unsigned int)local_10 * (unsigned int)(int)sVar12 * 1000u);
            local_28 = (unsigned short)(local_18 / iVar10);
        }
        __builtin_memcpy(&inA, iVar13 + 0x1f, 4);
        if (*(char *)(inA + 0x8b) != '\0') {
            FUN_00427bd0(*(short *)(iVar13 + 0x12), *(short *)(iVar13 + 0x10), (int)iVar13);
        }
        iVar13 = BASE + iVar3;
        __builtin_memcpy(&inA, iVar13 + 0x1f, 4);
        uVar5 = *(unsigned short *)(inA + 0x8c);
        local_26 = (unsigned short)((unsigned int)local_26 + (unsigned int)uVar5);
        if (((uVar5 ^ local_26) & 0x8000) != 0) {
            local_26 = uVar5;
        }
        uVar5 = *(unsigned short *)(inA + 0x8e);
        local_28 = (unsigned short)((unsigned int)local_28 + (unsigned int)uVar5);
        if (((uVar5 ^ local_28) & 0x8000) != 0) {
            local_28 = uVar5;
        }
        local_2a = *(unsigned short *)(iVar13 + 0x10);
        uVar8 = FUN_0046abf0((int)(short)local_26, (int)(short)local_28);
        *(short *)(iVar13 + 0x10) = (short)uVar8;
        if ((DAT_009e9640 <= (int)uVar14) && ((int)uVar14 < DAT_00a00854)) {
            uVar5 = *(unsigned short *)(BASE + 0x10 + iVar3);
            __builtin_memcpy(&inB, BASE + 0x1f + iVar4, 4);
            __builtin_memcpy(&sVar17, inB + 0x79, 2);
            sVar11 = (short)((unsigned int)(int)sVar17 * 8u);
            if (uVar5 < local_2a) {
                sVar11 = (short)((unsigned int)(int)sVar17 * (unsigned int)-8);
            }
            *(unsigned short *)(BASE + 0x10 + iVar3) =
                (unsigned short)((unsigned int)uVar5 + (unsigned int)(int)sVar11);
        }
        if ((int)uVar14 < DAT_009a1fe0) {
            iVar13 = BASE + iVar4;
            __builtin_memcpy(&inB, iVar13 + 0x1f, 4);
            __builtin_memcpy(&aux, inB + 0xa9, 4);
            uVar6 = FUN_0046a460(local_26);
            *(unsigned short *)(iVar13 + 0x12) = uVar6;
            __builtin_memcpy(aux + 0xd, BASE + 0x12 + iVar4, 2);
            uVar5 = (unsigned short)((unsigned int)(int)sVar12 * 100u);
            if (0x7fff < uVar5) {
                uVar5 = 0x7fff;
            }
            if (local_2a < *(unsigned short *)(iVar4 + BASE + 0x10)) {
                uVar5 = (unsigned short)(0u - (unsigned int)uVar5);
            }
            __builtin_memcpy(&inB, iVar4 + BASE + 0x1f, 4);
            __builtin_memcpy(inB + 0x77, &uVar5, 2);
            iVar13 = BASE + iVar4;
            FUN_00427bd0(*(short *)(iVar13 + 0x12), *(short *)(iVar13 + 0x10), (int)iVar13);
            FUN_00427d30(*(short *)(BASE + iVar4 + 0x14), 0, (int)(BASE + iVar4));
        }
        if ((int)uVar14 < DAT_009a1fe0) {
            __builtin_memcpy(&inB, BASE + 0x1f + iVar4, 4);
            __builtin_memcpy(&aux, inB + 0xa9, 4);
            *(unsigned short *)(aux + 0x70) = param_1;
            iVar10 = (int)((unsigned int)iVar18 * (unsigned int)iVar18 +
                           (unsigned int)local_10 * (unsigned int)local_10 +
                           (unsigned int)iVar15 * (unsigned int)iVar15);
            if (iVar10 < 0x33) {
                sVar17 = 0;
                sVar11 = 100;
            }
            else {
                sVar17 = (short)(int)(((int)((unsigned int)iVar16 * (unsigned int)(int)sVar12 * 1000u)) / iVar10);
                sVar11 = (short)(int)(((int)((unsigned int)local_18 * (unsigned int)(int)sVar12 * 1000u)) / iVar10);
            }
            pB = BASE + iVar4;
            __builtin_memcpy(&inB, pB + 0x1f, 4);
            if (*(char *)(inB + 0x8b) != '\0') {
                FUN_00427bd0(*(short *)(pB + 0x12), *(short *)(pB + 0x10), (int)pB);
            }
            pB = BASE + iVar4;
            __builtin_memcpy(&inB, pB + 0x1f, 4);
            sVar17 = (short)((unsigned int)(int)sVar17 + (unsigned int)(int)*(short *)(inB + 0x8c));
            local_2a = *(unsigned short *)(pB + 0x10);
            uVar8 = FUN_0046abf0((int)sVar17,
                                 (int)(short)((unsigned int)(int)sVar11 +
                                              (unsigned int)(int)*(short *)(inB + 0x8e)));
            *(short *)(pB + 0x10) = (short)uVar8;
            uVar6 = FUN_0046a460((unsigned short)sVar17);
            *(unsigned short *)(BASE + 0x12 + iVar3) = uVar6;
            __builtin_memcpy(iVar9 + 0xd, BASE + 0x12 + iVar3, 2);
        }
        uVar5 = (unsigned short)((unsigned int)(int)sVar12 * 100u);
        if (0x7fff < uVar5) {
            uVar5 = 0x7fff;
        }
        if (local_2a < *(unsigned short *)(iVar3 + BASE + 0x10)) {
            uVar5 = (unsigned short)(0u - (unsigned int)uVar5);
        }
        __builtin_memcpy(&inA, iVar3 + BASE + 0x1f, 4);
        __builtin_memcpy(inA + 0x77, &uVar5, 2);
        iVar13 = BASE + iVar3;
        FUN_00427bd0(*(short *)(iVar13 + 0x12), *(short *)(iVar13 + 0x10), (int)iVar13);
        FUN_00427d30(*(short *)(BASE + iVar3 + 0x14), 0, (int)(BASE + iVar3));
        if ((int)uVar14 < DAT_00a00854) {
            iVar13 = BASE + iVar4;
            FUN_00427bd0(*(short *)(iVar13 + 0x12), *(short *)(iVar13 + 0x10), (int)iVar13);
            FUN_00427d30(*(short *)(BASE + iVar4 + 0x14), 0, (int)(BASE + iVar4));
        }
        __builtin_memcpy(&t0, BASE + 0x1b + iVar3, 4);
        FUN_0042e4d0(0x1f, (unsigned int)param_1, t0);
        if ((int)uVar14 < DAT_009a1fe0) {
            __builtin_memcpy(&t0, BASE + 0x1b + iVar3, 4);
            FUN_0042e4d0(0x20, uVar14, t0);
        }
    }
    return;
}
