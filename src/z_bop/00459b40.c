/* SPDX-License-Identifier: MIT */
/* z_bop 00459b40 */
extern unsigned char DAT_009d8c82[];
extern unsigned char DAT_009d8c83[];
extern unsigned char DAT_009d8c84[];
extern unsigned char DAT_009d8c8d[];
extern unsigned char DAT_009d8c8e[];
extern unsigned char DAT_00524170[];
extern unsigned char DAT_00521f2e[];
extern unsigned char DAT_00521f2f[];
extern unsigned char DAT_00521f1b[];
extern unsigned char DAT_0051c659[];
extern unsigned char DAT_0051c65b[];
extern unsigned char DAT_0051c65d[];
extern unsigned char DAT_0051c65f[];
extern unsigned char DAT_00527b08[];
extern unsigned char DAT_00527ca0[];
extern unsigned char DAT_00527970[];
extern unsigned char DAT_009d9096[];
extern unsigned char DAT_009d90c2[];
extern unsigned char DAT_009d90ee[];
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern unsigned int DAT_009a1fe8;

long long __cdecl FUN_004ae0b0(unsigned int param_1);
void __cdecl FUN_00458e20(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_004035d0(unsigned int param_1, short param_2, short param_3, short param_4);

void __cdecl FUN_00459b40(unsigned int param_1, unsigned short param_2, unsigned short param_3)
{
    int iVar1;
    unsigned char uVar2;
    unsigned short uVar3;
    unsigned int uVar4;
    unsigned char bVar5;
    unsigned short uVar6;
    short sVar7;
    short sVar8;
    unsigned int uVar9;
    short sVar10;
    short sVar11;
    int iVar12;
    short sVar13;
    long long lVar14;
    short local_12;
    short local_10;
    short local_8;

    uVar9 = param_1 & 0xffff;
    iVar1 = (int)(uVar9 * 0x566);
    uVar4 = (unsigned int)DAT_00524170[DAT_009d8c82[iVar1]];
    uVar2 = DAT_00521f2f[uVar4 * 0x1a];
    lVar14 = FUN_004ae0b0(uVar4);
    FUN_00458e20(0x8000, uVar9);
    if ((DAT_00521f2e[uVar4 * 0x1a] & 0x80) == 0) {
        short s659, s65b, s65d, s65f;
        short sA, sB, sC;
        uVar3 = (unsigned short)(((unsigned int)uVar2 << 8) | uVar2);
        __builtin_memcpy(&s65f, DAT_0051c65f + uVar3, 2);
        __builtin_memcpy(&s65d, DAT_0051c65d + uVar3, 2);
        __builtin_memcpy(&s65b, DAT_0051c65b + uVar3, 2);
        sVar8 = s65f;
        sVar10 = (short)(DAT_009d8c8e[iVar1] + 1);
        iVar1 = (int)((unsigned int)param_2 + (unsigned int)DAT_009d8c8d[iVar1] * 6);
        sA = *(short *)(DAT_00527b08 + iVar1 * 2);
        iVar1 = iVar1 * 2;
        sB = *(short *)(DAT_00527ca0 + iVar1);
        sC = *(short *)(DAT_00527970 + iVar1);
        sVar13 = (short)((unsigned int)(int)sVar10 * (unsigned int)(int)sA * (unsigned int)(int)sVar8);
        sVar11 = (short)((unsigned int)(int)sVar10 * (unsigned int)(int)sB * (unsigned int)(int)s65d);
        sVar7 = (short)((unsigned int)(int)sVar10 * (unsigned int)(int)sC * (unsigned int)(int)s65b);
        if (sVar10 == 1) {
            sVar7 = (short)((unsigned int)(int)sVar7 + (unsigned int)(int)(s65b / 2) * (unsigned int)(int)sC);
            sVar11 = (short)((unsigned int)(int)sVar11 + (unsigned int)(int)(s65d / 2) * (unsigned int)(int)sB);
            sVar13 = (short)((unsigned int)(int)sVar13 +
                             (unsigned int)(int)(short)((int)((int)sVar8 + ((int)sVar8 >> 0x1f & 3U)) >> 2) *
                             (unsigned int)(int)sA);
        }
        if (param_3 == 0xffff) {
            FUN_00458e20(0x8000, uVar9);
            DAT_009d8a58 = 0;
            DAT_009d8a54 = 0;
            DAT_009d8b60 = 0;
        }
        else {
            FUN_004035d0((unsigned int)param_3 * 0x23 + DAT_009a1fe8, sVar7, sVar11, sVar13);
        }
        __builtin_memcpy(&s659, DAT_0051c659 + uVar3, 2);
        if (s659 != 0) {
            bVar5 = (unsigned char)s659;
            DAT_009d8a58 = (int)((unsigned int)DAT_009d8a58 << (bVar5 & 0x1f));
            DAT_009d8a54 = (int)((unsigned int)DAT_009d8a54 << (bVar5 & 0x1f));
            DAT_009d8b60 = (int)((unsigned int)DAT_009d8b60 << (bVar5 & 0x1f));
        }
        DAT_00a07cd4 = (int)((unsigned int)DAT_00a07cd4 + (unsigned int)DAT_009d8b60);
        DAT_00a07cdc = (int)((unsigned int)DAT_00a07cdc + (unsigned int)DAT_009d8a58);
        DAT_00a07cd8 = (int)((unsigned int)DAT_00a07cd8 + (unsigned int)DAT_009d8a54);
    }
    else if (DAT_00521f1b[uVar4 * 0x1a] == 9) {
        DAT_00a07cdc = (int)((unsigned int)(int)*(short *)(DAT_009d9096 + iVar1) << 8);
        DAT_00a07cd8 = (int)((unsigned int)(int)*(short *)(DAT_009d90c2 + iVar1) * 0xffffff00U);
        DAT_00a07cd4 = (int)((unsigned int)(int)*(short *)(DAT_009d90ee + iVar1) << 8);
    }
    else if (DAT_00521f1b[uVar4 * 0x1a] == 8) {
        sVar8 = 0;
        sVar7 = 0;
        local_8 = 0;
        local_10 = 0;
        bVar5 = DAT_009d8c84[iVar1] & 3;
        if ((DAT_009d8c84[iVar1] & 3) == 0) {
            sVar7 = 0x40;
            sVar8 = 0x40;
        }
        else if (bVar5 == 1) {
            sVar8 = 0x40;
            local_10 = 0x40;
        }
        else if (bVar5 == 2) {
            sVar7 = 0x40;
            local_8 = 0x40;
        }
        uVar6 = (unsigned short)DAT_009d8c83[iVar1];
        iVar12 = (int)(short)(uVar6 - 1);
        uVar3 = 0;
        local_12 = 0;
        if (DAT_009d8c83[iVar1] != 0) {
            do {
                sVar10 = 0;
                if (DAT_009d8c83[iVar1] != 0) {
                    do {
                        if (uVar3 == param_2) {
                            DAT_00a07cdc = (int)(((unsigned int)((int)sVar10 * (int)sVar7) +
                                                  (unsigned int)(int)(short)((unsigned short)*(short *)(DAT_009d9096 + iVar1) -
                                                                             (unsigned short)(short)((iVar12 * sVar7) / 2))) * 0x100U);
                            DAT_00a07cd8 = (int)(((unsigned int)((int)local_12 * (int)sVar8) +
                                                  (unsigned int)(int)(short)-(short)((unsigned short)*(short *)(DAT_009d90c2 + iVar1) +
                                                                                     (unsigned short)(short)((iVar12 * sVar8) / 2))) * 0x100U);
                            DAT_00a07cd4 = (int)((unsigned int)(int)(short)lVar14 +
                                                 ((unsigned int)((int)local_12 * (int)local_8) +
                                                  (unsigned int)((int)sVar10 * (int)local_10) +
                                                  (unsigned int)(int)(short)(((unsigned short)*(short *)(DAT_009d90ee + iVar1) -
                                                                              (unsigned short)(short)((iVar12 * local_8) / 2)) -
                                                                             (unsigned short)(short)((iVar12 * local_10) / 2))) * 0x100U);
                            return;
                        }
                        uVar3 = uVar3 + 1;
                        sVar10 = sVar10 + 1;
                    } while ((int)sVar10 < (int)(unsigned int)uVar6);
                }
                local_12 = local_12 + 1;
            } while ((int)local_12 < (int)(unsigned int)uVar6);
        }
    }
    DAT_00a07cd4 = (int)((unsigned int)DAT_00a07cd4 + (unsigned int)(int)(short)lVar14);
    return;
}
