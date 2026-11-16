/* SPDX-License-Identifier: MIT */
/* z_bop 0041ca60 */
unsigned int FUN_00425b70(unsigned int a, unsigned short b);
void FUN_004035d0(void *p, short a, short b, short c);
void FUN_00427bd0(short a, short b, void *p);
int FUN_004ae640(unsigned int a, unsigned int b);
int FUN_004ae6e0(unsigned int a, unsigned int b);
int FUN_004ae5a0(unsigned int a, unsigned int b);
void FUN_004037b0(void *p, int a, int b, int c);
unsigned int FUN_0041c1d0(unsigned short a, unsigned short b);
unsigned int FUN_00446700(unsigned short a, unsigned short b);

extern int DAT_00a08298;
extern int DAT_00a081e8;
extern int DAT_00a07c58;
extern int DAT_00a07c54;
extern int DAT_00a07bdc;
extern int DAT_009ec604;
extern int DAT_009d8c18;
extern int DAT_009d80c0;
extern int DAT_009d77fc;
extern int DAT_009d6828;
extern int DAT_009d1150;
extern int DAT_009cd260;
extern int DAT_009a1fd8;
extern int DAT_009a1fd4;
extern int DAT_009a1fd0;
extern unsigned int DAT_009a1fe8;
extern int DAT_009ecc34;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern int DAT_0051bf58;
extern unsigned char DAT_0051c59e[];
extern unsigned char DAT_009e9697[];
extern unsigned char DAT_0051c5b2[];
extern unsigned char DAT_0051c5b4[];
extern unsigned char DAT_0051c5b6[];
extern unsigned char s_starshipprotectpln_0051a2fc[];
extern unsigned char s_capescortersldr1pln_0051a334[];
extern unsigned char s_waitforboardpln_0051a358[];

unsigned int __cdecl FUN_0041ca60(unsigned short param_1, unsigned short param_2, unsigned int param_3)
{
    unsigned short uVar2;
    unsigned short uVar3;
    long long lVar4;
    int uVar5;
    int uVar6;
    int uVar7;
    int uVar8;
    int uVar9;
    int uVar10;
    int uVar11;
    int uVar12;
    int uVar13;
    int uVar14;
    int uVar15;
    int uVar16;
    int uVar17;
    int uVar18;
    int uVar19;
    unsigned int uVar20;
    int iVar21;
    int iVar22;
    int iVar23;
    int iVar24;
    unsigned char *iVar25;
    unsigned int uVar26;
    unsigned char *pA;
    unsigned char *pB;
    unsigned short us79;
    int tmp;
    int xv;

    uVar19 = DAT_00a08298;
    uVar18 = DAT_00a081e8;
    uVar17 = DAT_00a07c58;
    uVar16 = DAT_00a07c54;
    uVar15 = DAT_00a07bdc;
    uVar14 = DAT_009ec604;
    uVar13 = DAT_009d8c18;
    uVar12 = DAT_009d80c0;
    uVar11 = DAT_009d77fc;
    uVar10 = DAT_009d6828;
    uVar9 = DAT_009d1150;
    uVar8 = DAT_009cd260;
    uVar7 = DAT_009a1fd8;
    uVar6 = DAT_009a1fd4;
    uVar5 = DAT_009a1fd0;

    iVar25 = (unsigned char *)((unsigned int)param_1 * 0x23u + DAT_009a1fe8);
    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    __builtin_memcpy(&pB, pA + 0xa9, 4);
    iVar21 = (int)((unsigned int)pB[4] * 0x101u);
    uVar26 = param_3 & 0xffff;
    uVar20 = (unsigned int)DAT_0051c59e[(unsigned int)iVar21 + (unsigned int)DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bdu]];
    if ((signed char)0x3f < (signed char)pB[0x371 + uVar26 * 4]) {
        uVar20 = (unsigned int)(unsigned short)((unsigned char)DAT_0051c59e[(unsigned int)iVar21 + (unsigned int)DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bdu]] + 1);
    }
    uVar2 = *(unsigned short *)(s_starshipprotectpln_0051a2fc + uVar20 * 2 + 10);
    uVar3 = *(unsigned short *)(s_capescortersldr1pln_0051a334 + uVar20 * 2 + 2);
    uVar20 = FUN_00425b70(0xec, *(unsigned short *)(s_waitforboardpln_0051a358 + uVar20 * 2 + 0xe));
    DAT_009ec604 = *(int *)(iVar25 + 4);
    iVar24 = (int)((uVar20 & 0xffff) + (unsigned int)uVar3 * 0xecu);
    DAT_00a081e8 = *(int *)(iVar25 + 8);
    DAT_00a08298 = *(int *)(iVar25 + 0xc);
    FUN_004035d0(iVar25,
                 *(short *)(DAT_0051c5b2 + uVar26 * 8 + (unsigned int)iVar21),
                 *(short *)(DAT_0051c5b4 + uVar26 * 8 + (unsigned int)iVar21),
                 *(short *)(DAT_0051c5b6 + uVar26 * 8 + (unsigned int)iVar21));
    if (*(char *)(iVar25 + 3) == '5') {
        DAT_009d8a58 = (int)((unsigned int)DAT_009d8a58 * 2u);
        DAT_009d8a54 = (int)((unsigned int)DAT_009d8a54 * 2u);
        DAT_009d8b60 = (int)((unsigned int)DAT_009d8b60 * 2u);
    }
    DAT_009ec604 = (int)((unsigned int)DAT_009ec604 + (unsigned int)DAT_009d8a58);
    DAT_00a081e8 = (int)((unsigned int)DAT_00a081e8 + (unsigned int)DAT_009d8a54);
    DAT_00a08298 = (int)((unsigned int)DAT_00a08298 + (unsigned int)DAT_009d8b60);

    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    __builtin_memcpy(&us79, pA + 0x79, 2);
    tmp = (int)(((unsigned int)us79 + (unsigned int)uVar2) * 0x1234u + 0x80u);
    iVar21 = (int)((unsigned int)(tmp >> 8) * (unsigned int)iVar24);
    iVar21 = iVar21 / 0xec;

    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    if (*(char *)(pA + 0x8b) != '\0') {
        FUN_00427bd0(*(short *)(iVar25 + 0x12), *(short *)(iVar25 + 0x10), iVar25);
    }

    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    lVar4 = (long long)(int)*(short *)(pA + 0x8c) * (long long)iVar21;
    DAT_009a1fd4 = (int)((unsigned int)(lVar4 >> 15) + (unsigned int)DAT_009ec604);
    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    lVar4 = (long long)(int)*(short *)(pA + 0x8e) * (long long)iVar21;
    DAT_009a1fd8 = (int)((unsigned int)DAT_00a081e8 + (unsigned int)(lVar4 >> 15));
    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    lVar4 = (long long)(int)*(short *)(pA + 0x90) * (long long)iVar21;
    DAT_009a1fd0 = (int)((unsigned int)DAT_00a08298 + (unsigned int)(lVar4 >> 15));

    iVar25 = (unsigned char *)((unsigned int)param_2 * 0x23u + DAT_009a1fe8);
    __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
    if (pA == 0) {
        DAT_0051bf58 = 1;
        uVar20 = FUN_00446700(param_1, param_2);
        uVar20 = (unsigned int)(int)(short)uVar20;
    }
    else {
        DAT_009d80c0 = *(int *)(iVar25 + 4);
        DAT_009d8c18 = *(int *)(iVar25 + 8);
        DAT_009d1150 = *(int *)(iVar25 + 0xc);
        __builtin_memcpy(&uVar2, &DAT_009e9697[(unsigned int)DAT_009ecc34 * 0x5bdu + 2], 2);
        if (uVar2 != 0) {
            uVar26 = (unsigned int)uVar2;
            uVar20 = (unsigned int)*(unsigned char *)(iVar25 + 3);
            iVar22 = FUN_004ae640(uVar20, uVar26);
            iVar22 = (int)(0u - (unsigned int)iVar22);
            iVar23 = FUN_004ae6e0(uVar20, uVar26);
            xv = FUN_004ae5a0(uVar20, uVar26);
            FUN_004037b0(iVar25, xv, iVar23, iVar22);
            DAT_009d80c0 = (int)((unsigned int)DAT_009d80c0 + (unsigned int)DAT_009d8a58);
            DAT_009d8c18 = (int)((unsigned int)DAT_009d8c18 + (unsigned int)DAT_009d8a54);
            DAT_009d1150 = (int)((unsigned int)DAT_009d1150 + (unsigned int)DAT_009d8b60);
        }
        __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
        __builtin_memcpy(&us79, pA + 0x79, 2);
        tmp = (int)((unsigned int)us79 * 0x1234u + 0x80u);
        iVar24 = (int)((unsigned int)(tmp >> 8) * (unsigned int)iVar24);
        iVar24 = iVar24 / 0xec;

        __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
        if (*(char *)(pA + 0x8b) != '\0') {
            FUN_00427bd0(*(short *)(iVar25 + 0x12), *(short *)(iVar25 + 0x10), iVar25);
        }

        __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
        lVar4 = (long long)(int)*(short *)(pA + 0x8c) * (long long)iVar24;
        DAT_00a07bdc = (int)((unsigned int)DAT_009d80c0 + (unsigned int)(lVar4 >> 15));
        __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
        lVar4 = (long long)(int)*(short *)(pA + 0x8e) * (long long)iVar24;
        DAT_00a07c54 = (int)((unsigned int)(lVar4 >> 15) + (unsigned int)DAT_009d8c18);
        __builtin_memcpy(&pA, iVar25 + 0x1f, 4);
        lVar4 = (long long)(int)*(short *)(pA + 0x90) * (long long)iVar24;
        DAT_00a07c58 = (int)((unsigned int)(lVar4 >> 15) + (unsigned int)DAT_009d1150);
        DAT_0051bf58 = 1;
        uVar20 = FUN_0041c1d0(param_1, param_2);
        uVar20 = uVar20 & 0xffff;
    }
    DAT_009ec604 = uVar14;
    DAT_00a081e8 = uVar18;
    DAT_00a08298 = uVar19;
    DAT_009a1fd4 = uVar6;
    DAT_009a1fd8 = uVar7;
    DAT_009a1fd0 = uVar5;
    DAT_009d80c0 = uVar12;
    DAT_009d8c18 = uVar13;
    DAT_009d1150 = uVar9;
    DAT_00a07bdc = uVar15;
    DAT_00a07c54 = uVar16;
    DAT_00a07c58 = uVar17;
    DAT_009d77fc = uVar11;
    DAT_009d6828 = uVar10;
    DAT_009cd260 = uVar8;
    DAT_0051bf58 = 0;
    return uVar20;
}
