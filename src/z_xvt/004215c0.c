/* SPDX-License-Identifier: MIT */
/* z_xvt 004215c0 */
extern unsigned short DAT_00703abc;
extern unsigned short DAT_0073b374;
extern unsigned short DAT_006d4a6a[];
extern unsigned char DAT_006d4a6c[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0070ad57[];
extern unsigned char DAT_0070ad58[];
extern unsigned char DAT_0070ad59[];
extern unsigned char DAT_0070b156[];
extern unsigned char DAT_0070b182[];
extern unsigned char DAT_0070b1ae[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0052687a[];
extern unsigned char DAT_0052686b[];
extern int DAT_007394b0;
extern int DAT_007394b4;
extern int DAT_007394b8;
extern unsigned short DAT_007394bc;
extern unsigned short DAT_007394be;
extern unsigned short DAT_007394c0;
extern unsigned short DAT_006d9d90;
extern int DAT_00732a38;
extern int DAT_00703c88;
extern int DAT_006d4988;

void __cdecl FUN_00421a80(unsigned short, int, int);
unsigned int __cdecl FUN_00478470(void);

void __cdecl FUN_004215c0(unsigned short param_1)
{
    unsigned char bVar1;
    char cVar2;
    short sVar3;
    short sVar4;
    short sVar5;
    unsigned short uVar6;
    unsigned int uVar7;
    char *pcVar8;
    unsigned char bVar9;
    int iVar10;
    int iVar11;
    int iVar12;
    int iVar13;
    unsigned int local_24;
    int local_20;
    int local_1c;
    int local_18;
    int local_14;

    uVar6 = DAT_00703abc;
    if (DAT_006d4a6a[(unsigned int)DAT_0073b374 * 0x93] != 0) {
        iVar12 = (unsigned int)DAT_0073b374 * 0x566;
        bVar1 = DAT_005247e8[DAT_0070ad42[iVar12]];
        if ((DAT_0052687a[(unsigned int)bVar1 * 0x16] & 0x80) != 0) {
            cVar2 = (char)DAT_0052686b[(unsigned int)bVar1 * 0x16];
            if (cVar2 == '\b') {
                bVar9 = DAT_0070ad44[iVar12] & 3;
                local_18 = 0;
                local_1c = 0;
                local_20 = 0;
                local_14 = 0;
                if ((DAT_0070ad44[iVar12] & 3) == 0) {
                    local_18 = 0x40;
                    local_1c = 0x40;
                }
                else if (bVar9 == 1) {
                    local_1c = 0x40;
                    local_14 = 0x40;
                }
                else if (bVar9 == 2) {
                    local_18 = 0x40;
                    local_20 = 0x40;
                }
                iVar11 = (int)((unsigned int)DAT_0070ad43[iVar12] - 1u);
                sVar3 = *(short *)(DAT_0070b156 + iVar12);
                sVar4 = *(short *)(DAT_0070b182 + iVar12);
                sVar5 = *(short *)(DAT_0070b1ae + iVar12);
                DAT_007394bc = (unsigned short)DAT_0070ad57[iVar12];
                uVar7 = 0;
                DAT_007394be = (unsigned short)DAT_0070ad58[iVar12];
                local_24 = 0;
                DAT_007394c0 = (unsigned short)DAT_0070ad59[iVar12];
                if (DAT_0070ad43[iVar12] != 0) {
                    do {
                        iVar12 = 0;
                        if (DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566] != '\0') {
                            iVar10 = (int)((((unsigned int)(int)sVar5 -
                                             (((unsigned int)iVar11 * (unsigned int)local_20) >> 1)) -
                                            (((unsigned int)iVar11 * (unsigned int)local_14) >> 1)) +
                                           (unsigned int)local_20 * local_24);
                            iVar13 = (int)((unsigned int)(int)sVar3 -
                                           (((unsigned int)local_18 * (unsigned int)iVar11) >> 1));
                            do {
                                if (((param_1 == 0xffff) || (param_1 == uVar7)) &&
                                    (*(unsigned short *)(DAT_006d4a6c + (unsigned int)DAT_0073b374 * 0x126) <
                                     DAT_006d4a6a[(unsigned int)DAT_0073b374 * 0x93])) {
                                    DAT_007394b4 = (int)(local_24 * (unsigned int)local_1c -
                                                         ((unsigned int)(int)sVar4 +
                                                          (((unsigned int)iVar11 * (unsigned int)local_1c) >> 1)));
                                    DAT_007394b0 = iVar13;
                                    DAT_007394b8 = iVar10;
                                    FUN_00421a80(DAT_0073b374, 8, bVar1);
                                }
                                uVar7 = uVar7 + 1;
                                iVar10 = (int)((unsigned int)iVar10 + (unsigned int)local_14);
                                iVar13 = (int)((unsigned int)iVar13 + (unsigned int)local_18);
                                iVar12 = iVar12 + 1;
                            } while (iVar12 < (int)(unsigned int)DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566]);
                        }
                        local_24 = local_24 + 1;
                    } while (local_24 < DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566]);
                    return;
                }
            }
            else {
                if (cVar2 == '\t') {
                    DAT_007394b0 = (int)*(short *)(DAT_0070b156 + iVar12);
                    DAT_007394b8 = (int)*(short *)(DAT_0070b1ae + iVar12);
                    DAT_007394b4 = (int)(0u - (unsigned int)(int)*(short *)(DAT_0070b182 + iVar12));
                    DAT_007394bc = (unsigned short)DAT_0070ad57[iVar12];
                    DAT_007394be = (unsigned short)DAT_0070ad58[iVar12];
                    DAT_007394c0 = (unsigned short)DAT_0070ad59[iVar12];
                    FUN_00421a80(DAT_0073b374, 9, bVar1);
                    return;
                }
                if (cVar2 != '\n') {
                    return;
                }
                sVar3 = *(short *)(DAT_0070b156 + iVar12);
                local_24 = 0;
                sVar4 = *(short *)(DAT_0070b182 + iVar12);
                sVar5 = *(short *)(DAT_0070b1ae + iVar12);
                DAT_00703abc = DAT_006d9d90;
                for (; local_24 < DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566]; local_24 = local_24 + 1) {
                    do {
                        uVar7 = FUN_00478470();
                        iVar12 = (int)(((uVar7 & 0x1ff) - 0x100) + (unsigned int)(int)sVar3);
                        uVar7 = FUN_00478470();
                        iVar11 = (int)(((uVar7 & 0x1ff) - 0x100) - (unsigned int)(int)sVar4);
                        uVar7 = FUN_00478470();
                        iVar13 = (int)(((uVar7 & 0x1ff) - 0x100) + (unsigned int)(int)sVar5);
                        iVar10 = DAT_00732a38;
                        if (DAT_00732a38 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38)) {
                            pcVar8 = (char *)((unsigned int)DAT_00732a38 * 0x23u + 3u +
                                              (unsigned int)DAT_006d4988);
                            do {
                                if (*pcVar8 != '\0') {
                                    int t1, t2, t3;
                                    __builtin_memcpy(&t1, pcVar8 + 1, 4);
                                    if (t1 == iVar12) {
                                        __builtin_memcpy(&t2, pcVar8 + 5, 4);
                                        if (t2 == iVar11) {
                                            __builtin_memcpy(&t3, pcVar8 + 9, 4);
                                            if (t3 == iVar13) break;
                                        }
                                    }
                                }
                                pcVar8 = pcVar8 + 0x23;
                                iVar10 = iVar10 + 1;
                            } while (iVar10 < (int)((unsigned int)DAT_00703c88 + (unsigned int)DAT_00732a38));
                        }
                    } while (iVar10 < DAT_00703c88);
                    DAT_007394bc = 0;
                    DAT_007394be = 0;
                    DAT_007394c0 = 0;
                    DAT_007394b0 = iVar12;
                    DAT_007394b4 = iVar11;
                    DAT_007394b8 = iVar13;
                    uVar7 = FUN_00478470();
                    FUN_00421a80(DAT_0073b374, 10, (int)((char)((uVar7 & 0xffff) % 6) + 'd'));
                }
                DAT_006d9d90 = DAT_00703abc;
            }
        }
    }
    DAT_00703abc = uVar6;
    return;
}
