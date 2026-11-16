/* SPDX-License-Identifier: MIT */
/* z_bop 00459850 */
extern unsigned char DAT_009e9670[];
extern int DAT_009ecc34;
extern unsigned short DAT_009a8c06;
extern int DAT_009cd274;
extern int DAT_009a8e24;
extern int DAT_009a1fe8;

unsigned int FUN_0041f3d0(unsigned int a, unsigned int b, unsigned int c);
unsigned short FUN_00425d10(void);
void FUN_00427bd0(short a, short b, int c);
void FUN_00427d30(short a, int b, int c);

void __stdcall FUN_00459850(void)
{
    int iVar12;
    int iVar5;
    int iVar8;
    int iVar10;
    int iVar4;
    unsigned int uVar9;
    unsigned int uVar11;
    unsigned int uVar13;
    unsigned char bVar6;
    unsigned short uVar7;
    unsigned char *pObj;
    short sVar2;
    short sVar3;
    short sA;
    short sB;
    short sC;
    int iA;
    int iB;
    short local_20;
    short local_1c;
    short local_18;
    short local_14;
    short local_10;
    short local_c;

    __builtin_memcpy(&iVar12, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bdu], 4);
    if (iVar12 != -1) {
        uVar9 = (unsigned int)DAT_009a8c06;
        DAT_009a8c06 = (unsigned short)(DAT_009a8c06 + 1);
        if ((int)(unsigned int)DAT_009a8c06 == DAT_009cd274) {
            DAT_009a8c06 = (unsigned short)DAT_009a8e24;
        }
        iVar5 = (int)(uVar9 * 0x23u);
        iVar10 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar5);
        iVar12 = (int)((unsigned int)iVar12 * 0x23u);
        iVar8 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar12);

        __builtin_memcpy(&iA, (unsigned char *)iVar10 + 4, 4);
        __builtin_memcpy(&iB, (unsigned char *)iVar8 + 4, 4);
        uVar9 = (unsigned int)iA - (unsigned int)iB;
        __builtin_memcpy(&iA, (unsigned char *)iVar10 + 8, 4);
        __builtin_memcpy(&iB, (unsigned char *)iVar8 + 8, 4);
        uVar13 = (unsigned int)iA - (unsigned int)iB;
        __builtin_memcpy(&iA, (unsigned char *)iVar10 + 0xc, 4);
        __builtin_memcpy(&iB, (unsigned char *)iVar8 + 0xc, 4);
        uVar11 = (unsigned int)iA - (unsigned int)iB;
        if ((int)uVar9 < 0) {
            uVar9 = 0u - uVar9;
        }
        if ((int)uVar13 < 0) {
            uVar13 = 0u - uVar13;
        }
        if ((int)uVar11 < 0) {
            uVar11 = 0u - uVar11;
        }
        uVar9 = FUN_0041f3d0(uVar9, uVar13, uVar11);
        if (0x800 < uVar9) {
            bVar6 = (unsigned char)FUN_00425d10();
            ((unsigned char *)DAT_009a1fe8)[3 + iVar5] = (unsigned char)((bVar6 & 3) + 0x6e);
            ((unsigned char *)DAT_009a1fe8)[2 + iVar5] = 0xb;
            __builtin_memcpy(&pObj, (unsigned char *)DAT_009a1fe8 + 0x1f + iVar5, 4);
            *pObj = 3;
            ((unsigned char *)DAT_009a1fe8)[0x16 + iVar5] = 0xff;

            iVar8 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar12);
            __builtin_memcpy(&pObj, (unsigned char *)DAT_009a1fe8 + 0x1f + iVar12, 4);
            if (pObj[0x92] != 0) {
                __builtin_memcpy(&sA, (unsigned char *)iVar8 + 0x12, 2);
                __builtin_memcpy(&sB, (unsigned char *)iVar8 + 0x10, 2);
                FUN_00427bd0(sA, sB, iVar8);
                __builtin_memcpy(&sC, (unsigned char *)DAT_009a1fe8 + iVar12 + 0x14, 2);
                FUN_00427d30(sC, 0, (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar12));
            }

            uVar7 = FUN_00425d10();
            __builtin_memcpy(&pObj, (unsigned char *)DAT_009a1fe8 + 0x1f + iVar12, 4);
            iVar8 = (int)(short)((uVar7 & 0x3ff) - 0x200);
            __builtin_memcpy(&sA, pObj + 0x99, 2);
            local_20 = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);
            __builtin_memcpy(&sA, pObj + 0x9b, 2);
            local_1c = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);
            __builtin_memcpy(&sA, pObj + 0x9d, 2);
            local_18 = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);

            uVar7 = FUN_00425d10();
            iVar12 = (int)((unsigned int)iVar12 + (unsigned int)DAT_009a1fe8);
            iVar8 = (int)(short)((uVar7 & 0x3ff) - 0x200);
            __builtin_memcpy(&pObj, (unsigned char *)iVar12 + 0x1f, 4);
            __builtin_memcpy(&sA, pObj + 0x9f, 2);
            local_14 = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);
            __builtin_memcpy(&pObj, (unsigned char *)iVar12 + 0x1f, 4);
            __builtin_memcpy(&sA, pObj + 0xa1, 2);
            local_10 = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);
            __builtin_memcpy(&pObj, (unsigned char *)iVar12 + 0x1f, 4);
            __builtin_memcpy(&sA, pObj + 0xa3, 2);
            local_c = (short)((unsigned int)((unsigned int)iVar8 * (unsigned int)(int)sA) >> 0xf);

            __builtin_memcpy(&pObj, (unsigned char *)iVar12 + 0x1f, 4);
            __builtin_memcpy(&sVar2, pObj + 0x95, 2);
            __builtin_memcpy(&iVar10, (unsigned char *)iVar12 + 0xc, 4);
            __builtin_memcpy(&sVar3, pObj + 0x97, 2);
            __builtin_memcpy(&iVar4, (unsigned char *)iVar12 + 8, 4);

            __builtin_memcpy(&sB, pObj + 0x93, 2);
            __builtin_memcpy(&iA, (unsigned char *)iVar12 + 4, 4);
            iB = (int)((unsigned int)iA +
                       (unsigned int)(int)(short)(local_20 + local_14 + (sB >> 4)));
            __builtin_memcpy((unsigned char *)DAT_009a1fe8 + 4 + iVar5, &iB, 4);

            iB = (int)((unsigned int)(int)(short)(local_1c + local_10 + (sVar2 >> 4)) +
                       (unsigned int)iVar4);
            __builtin_memcpy((unsigned char *)DAT_009a1fe8 + 8 + iVar5, &iB, 4);

            iB = (int)((unsigned int)iVar10 +
                       (unsigned int)(int)(short)(local_18 + local_c + (sVar3 >> 4)));
            __builtin_memcpy((unsigned char *)DAT_009a1fe8 + 0xc + iVar5, &iB, 4);

            ((unsigned char *)DAT_009a1fe8)[0x19 + iVar5] = 2;
        }
    }
    return;
}
