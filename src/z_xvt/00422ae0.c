/* SPDX-License-Identifier: MIT */
/* z_xvt 00422ae0 */
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070ad4d[];
extern unsigned char DAT_0070ad4e[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052687b[];
extern unsigned char DAT_0052687a[];
extern unsigned char DAT_0052686b[];
extern unsigned char DAT_006d4a66[];
extern unsigned char DAT_0070b156[];
extern unsigned char DAT_0070b182[];
extern unsigned char DAT_0070b1ae[];
extern unsigned char DAT_0052b157[];
extern unsigned char DAT_0052b159[];
extern unsigned char DAT_0052b15b[];
extern unsigned char DAT_0052b15d[];
extern unsigned char DAT_0052a7b0[];
extern unsigned char DAT_0052a948[];
extern unsigned char DAT_0052aae0[];
extern int DAT_006d4988;
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;

long long __cdecl FUN_00449580(unsigned int, unsigned int, unsigned int);
void __cdecl FUN_0042a230(unsigned int, int, int, unsigned short, short, short);

void __cdecl FUN_00422ae0(unsigned int param_1, unsigned short param_2, unsigned short param_3)
{
    short sVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned char bVar4;
    unsigned short uVar5;
    short sVar6;
    int iVar7;
    int iVar8;
    short sVar9;
    unsigned int uVar10;
    short sVar11;
    long long lVar12;
    short local_16;
    short local_12;
    short local_10;
    short s157, s159, s15b;

    uVar10 = param_1 & 0xffff;
    iVar7 = (int)(uVar10 * 0x566u);
    uVar3 = (unsigned int)DAT_005247e8[DAT_0070ad42[iVar7]];
    bVar4 = DAT_0052687b[uVar3 * 0x16];
    lVar12 = FUN_00449580(uVar3 * 5, uVar3 * 0xb, uVar3);
    iVar8 = (int)((uVar10 * 0x2b3u + (unsigned int)((int)*(unsigned short *)(DAT_006d4a66 + uVar10 * 0x126) - 0x8000)) * 2u);
    DAT_00732a2c = (int)((unsigned int)(int)*(short *)(DAT_0070b156 + iVar8) << 8);
    DAT_00732a28 = (int)((unsigned int)(int)*(short *)(DAT_0070b182 + iVar8) * 0xffffff00u);
    DAT_00732a24 = (int)((unsigned int)(int)*(short *)(DAT_0070b1ae + iVar8) << 8);

    if ((DAT_0052687a[uVar3 * 0x16] & 0x80) == 0) {
        iVar8 = (int)((unsigned int)bVar4 * 0xefu);
        __builtin_memcpy(&sVar1, DAT_0052b15d + iVar8, 2);
        __builtin_memcpy(&s159, DAT_0052b159 + iVar8, 2);
        __builtin_memcpy(&s15b, DAT_0052b15b + iVar8, 2);
        sVar6 = (short)((int)DAT_0070ad4e[iVar7] + 1);
        iVar7 = (int)((unsigned int)param_2 + (unsigned int)DAT_0070ad4d[iVar7] * 6u);
        uVar2 = (unsigned short)((unsigned int)(int)s159 * (unsigned int)(int)*(short *)(DAT_0052a7b0 + iVar7 * 2) * (unsigned int)(int)sVar6);
        sVar11 = (short)((unsigned int)(int)sVar1 * (unsigned int)(int)*(short *)(DAT_0052a948 + iVar7 * 2) * (unsigned int)(int)sVar6);
        sVar9 = (short)((unsigned int)(int)s15b * (unsigned int)(int)*(short *)(DAT_0052aae0 + iVar7 * 2) * (unsigned int)(int)sVar6);
        iVar7 = iVar7 * 2;
        if (sVar6 == 1) {
            uVar2 = (unsigned short)((unsigned int)uVar2 + (unsigned int)(int)(s159 / 2) * (unsigned int)(int)*(short *)(DAT_0052a7b0 + iVar7));
            sVar9 = (short)((unsigned int)(int)sVar9 + (unsigned int)(int)(s15b / 2) * (unsigned int)(int)*(short *)(DAT_0052aae0 + iVar7));
            sVar11 = (short)((unsigned int)(int)sVar11 +
                             (unsigned int)(int)(short)(((int)sVar1 + (int)((unsigned int)((int)sVar1 >> 0x1f) & 3u)) >> 2) *
                             (unsigned int)(int)*(short *)(DAT_0052a948 + iVar7));
        }
        if (param_3 == 0xffff) {
            DAT_0070ab18 = 0;
            DAT_0070ab14 = 0;
            DAT_0070ac20 = 0;
        } else {
            iVar7 = (int)((unsigned int)param_3 * 0x23u + (unsigned int)DAT_006d4988);
            FUN_0042a230((unsigned int)uVar2, iVar7, iVar7, uVar2, sVar9, sVar11);
        }
        __builtin_memcpy(&s157, DAT_0052b157 + iVar8, 2);
        if (s157 != 0) {
            bVar4 = (unsigned char)s157;
            DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 << (bVar4 & 0x1f));
            DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 << (bVar4 & 0x1f));
            DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 << (bVar4 & 0x1f));
        }
        DAT_00732a2c = (int)((unsigned int)DAT_00732a2c + (unsigned int)DAT_0070ab18);
        DAT_00732a28 = (int)((unsigned int)DAT_00732a28 + (unsigned int)DAT_0070ab14);
        DAT_00732a24 = (int)((unsigned int)DAT_00732a24 + (unsigned int)DAT_0070ac20);
    }
    else if (DAT_0052686b[uVar3 * 0x16] == 9) {
        DAT_00732a2c = (int)((unsigned int)(int)*(short *)(DAT_0070b156 + iVar7) << 8);
        DAT_00732a28 = (int)((unsigned int)(int)*(short *)(DAT_0070b182 + iVar7) * 0xffffff00u);
        DAT_00732a24 = (int)((unsigned int)(int)*(short *)(DAT_0070b1ae + iVar7) << 8);
    }
    else if (DAT_0052686b[uVar3 * 0x16] == 8) {
        sVar1 = 0;
        sVar6 = 0;
        local_10 = 0;
        local_16 = 0;
        bVar4 = DAT_0070ad44[iVar7] & 3;
        if ((DAT_0070ad44[iVar7] & 3) == 0) {
            sVar6 = 0x40;
            sVar1 = 0x40;
        }
        else if (bVar4 == 1) {
            sVar1 = 0x40;
            local_16 = 0x40;
        }
        else if (bVar4 == 2) {
            sVar6 = 0x40;
            local_10 = 0x40;
        }
        uVar5 = (unsigned short)DAT_0070ad43[iVar7];
        iVar8 = (int)(short)(uVar5 - 1);
        uVar2 = 0;
        local_12 = 0;
        if (DAT_0070ad43[iVar7] != 0) {
            do {
                sVar9 = 0;
                if (DAT_0070ad43[iVar7] != 0) {
                    do {
                        if (param_2 == uVar2) {
                            DAT_00732a2c = (int)(((unsigned int)((int)sVar6 * (int)sVar9) +
                                                  (unsigned int)(int)(short)((unsigned short)*(short *)(DAT_0070b156 + iVar7) -
                                                                             (unsigned short)(short)((sVar6 * iVar8) / 2))) * 0x100u);
                            DAT_00732a28 = (int)(((unsigned int)((int)sVar1 * (int)local_12) +
                                                  (unsigned int)(int)(short)-(short)((unsigned short)*(short *)(DAT_0070b182 + iVar7) +
                                                                                      (unsigned short)(short)((sVar1 * iVar8) / 2))) * 0x100u);
                            DAT_00732a24 = (int)(((unsigned int)((int)local_16 * (int)sVar9) +
                                                  (unsigned int)((int)local_10 * (int)local_12) +
                                                  (unsigned int)(int)(short)(((unsigned short)*(short *)(DAT_0070b1ae + iVar7) -
                                                                              (unsigned short)(short)((local_10 * iVar8) / 2)) -
                                                                             (unsigned short)(short)((local_16 * iVar8) / 2))) * 0x100u +
                                                 (unsigned int)(int)(short)lVar12);
                            return;
                        }
                        uVar2 = (unsigned short)(uVar2 + 1);
                        sVar9 = (short)(sVar9 + 1);
                    } while ((int)sVar9 < (int)(unsigned int)uVar5);
                }
                local_12 = (short)(local_12 + 1);
            } while ((int)local_12 < (int)(unsigned int)uVar5);
        }
    }
    DAT_00732a24 = (int)((unsigned int)DAT_00732a24 + (unsigned int)(int)(short)lVar12);
    return;
}
