/* SPDX-License-Identifier: MIT */
/* z_bop 0042c410 */
extern int DAT_009a1fe8;
extern unsigned short DAT_009a8078;
extern int DAT_009ecc34;
extern unsigned char DAT_009e9670[];

long long __cdecl FUN_004ae080(int param_1);

unsigned int __cdecl FUN_0042c410(unsigned short param_1)
{
    long long lVar1;
    long long lVar2;
    long long lVar12;
    short sVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    int iVar10;
    int iVar11;
    short local_14;
    short local_10;
    short local_c;
    short local_8;
    unsigned char *iVar9;
    unsigned char *pObj;
    unsigned char *pTgt;
    unsigned char *pTgtObj;
    int tmpI;
    int tmpJ;
    short s93;
    short s95;
    short s97;
    int t4;
    int t8;
    int tc;
    int idx;

    if (*((unsigned char *)DAT_009a1fe8 + 3 + (unsigned int)param_1 * 0x23) == 'b') {
        lVar12 = FUN_004ae080(0x62);
        sVar3 = (short)(0u - (unsigned int)(unsigned short)(short)lVar12);
    } else {
        sVar3 = 0;
    }
    if (param_1 == DAT_009a8078) {
        sVar3 = (short)((unsigned int)(int)sVar3 - 0x400u);
    } else {
        sVar3 = (short)((unsigned int)(int)sVar3 + 0x20u);
    }
    iVar9 = (unsigned char *)DAT_009a1fe8 + (unsigned int)param_1 * 0x23;
    iVar5 = (int)sVar3;
    __builtin_memcpy(&pObj, iVar9 + 0x1f, 4);
    __builtin_memcpy(&s93, pObj + 0x93, 2);
    __builtin_memcpy(&s95, pObj + 0x95, 2);
    __builtin_memcpy(&s97, pObj + 0x97, 2);
    lVar12 = (long long)iVar5 * (long long)(int)s93;
    lVar1 = (long long)iVar5 * (long long)(int)s95;
    lVar2 = (long long)iVar5 * (long long)(int)s97;
    __builtin_memcpy(&t4, iVar9 + 4, 4);
    __builtin_memcpy(&t8, iVar9 + 8, 4);
    __builtin_memcpy(&tc, iVar9 + 0xc, 4);
    iVar5 = (int)((unsigned int)t4 + (unsigned int)(int)(lVar12 >> 15) * 2u);
    iVar10 = (int)((unsigned int)t8 + (unsigned int)(int)(lVar1 >> 15) * 2u);
    iVar11 = (int)((unsigned int)tc + (unsigned int)(int)(lVar2 >> 15) * 2u);
    __builtin_memcpy(&idx, DAT_009e9670 + (unsigned int)DAT_009ecc34 * 0x5bdu, 4);
    pTgt = (unsigned char *)((unsigned int)idx * 0x23u + (unsigned int)DAT_009a1fe8);
    __builtin_memcpy(&t4, pTgt + 4, 4);
    __builtin_memcpy(&t8, pTgt + 8, 4);
    __builtin_memcpy(&tc, pTgt + 0xc, 4);
    iVar6 = (int)((unsigned int)t4 - (unsigned int)iVar5);
    iVar7 = (int)((unsigned int)t8 - (unsigned int)iVar10);
    iVar8 = (int)((unsigned int)tc - (unsigned int)iVar11);
    if (0x4000 < iVar6) {
        return 0;
    }
    if (iVar6 < -0x4000) {
        return 0;
    }
    if (0x4000 < iVar7) {
        return 0;
    }
    if (iVar7 < -0x4000) {
        return 0;
    }
    if (0x4000 < iVar8) {
        return 0;
    }
    if (iVar8 < -0x4000) {
        return 0;
    }
    __builtin_memcpy(&pTgtObj, pTgt + 0x1f, 4);
    __builtin_memcpy(&tmpI, pTgtObj + 6, 4);
    iVar5 = (int)((unsigned int)tmpI - (unsigned int)iVar5);
    __builtin_memcpy(&tmpI, pTgtObj + 10, 4);
    iVar10 = (int)((unsigned int)tmpI - (unsigned int)iVar10);
    __builtin_memcpy(&tmpJ, pTgtObj + 0xe, 4);
    iVar11 = (int)((unsigned int)tmpJ - (unsigned int)iVar11);
    if (iVar5 < 0x4001) {
        if (iVar5 < -0x4000) {
            return 0;
        }
        if (0x4000 < iVar10) {
            return 0;
        }
        if (iVar10 < -0x4000) {
            return 0;
        }
        if (iVar11 < 0x4001) {
            if (iVar11 < -0x4000) {
                return 0;
            }
            iVar4 = (int)((unsigned int)((int)(short)iVar6 * (int)s93) +
                          (unsigned int)((int)(short)iVar7 * (int)s95) +
                          (unsigned int)((int)(short)iVar8 * (int)s97));
            if (0x3fffffff < iVar4) {
                iVar4 = 0x3fffffff;
            }
            if (iVar4 < -0x3fffffff) {
                iVar4 = -0x3fff0000;
            }
            local_14 = (short)(iVar4 >> 0xf);
            local_10 = (short)iVar10;
            local_c = (short)iVar5;
            iVar5 = (int)((unsigned int)((int)local_c * (int)s93) +
                          (unsigned int)((int)local_10 * (int)s95) +
                          (unsigned int)((int)(short)iVar11 * (int)s97));
            if (0x3fffffff < iVar5) {
                iVar5 = 0x3fffffff;
            }
            if (iVar5 < -0x3fffffff) {
                iVar5 = -0x3fff0000;
            }
            local_8 = (short)(iVar5 >> 0xf);
            if (((local_14 < 0) || (0 < local_8)) && ((0 < local_14) || (local_8 < 0))) {
                return 0;
            }
            return 1;
        }
        return 0;
    }
    return 0;
}
