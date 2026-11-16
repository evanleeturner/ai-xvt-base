/* SPDX-License-Identifier: MIT */
/* z_xvt 00453c90 */
extern unsigned short DAT_006d9d50;
extern int DAT_006d4988;
extern unsigned short DAT_006d9d72;
extern int DAT_006d9d56;
extern int DAT_00732e54;
extern unsigned short DAT_0071b704;
extern unsigned short DAT_006d9d70;
extern int DAT_00525e50[];
extern int DAT_00525e40[];
extern int DAT_00732904;
extern int DAT_006d4980;
extern unsigned short DAT_006da9f0;
extern char DAT_00525e60[];
extern unsigned short DAT_006d9d60;
extern char DAT_0070ad44[];
extern char DAT_0070b215[];
extern unsigned short DAT_00732a3c;
extern unsigned short DAT_00732e40;
extern unsigned char DAT_0052b08d[];
extern unsigned int DAT_006db580;
extern unsigned char DAT_00525e68[];
extern unsigned char DAT_00525e70[];

extern unsigned char __cdecl FUN_00429fc0(unsigned int, int);
extern unsigned char __cdecl FUN_00444040(unsigned int, unsigned int);
extern void __cdecl FUN_0042a140(unsigned int, unsigned int);
extern unsigned int __cdecl FUN_00491ad0(void);
extern unsigned int __cdecl FUN_00448840(unsigned int, int);
extern unsigned int __cdecl FUN_00478470(void);

unsigned int __stdcall FUN_00453c90(void)
{
    char cVar1;
    int iVar2;
    char *pcVar3;
    unsigned char bVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned char uVar7;
    unsigned short uVar8;
    int iVar9;
    unsigned int uVar10;
    unsigned short uVar11;
    unsigned int unaff_ESI = 0;
    int iTmp;
    int iTmp2;
    unsigned int uTmp;

    uVar5 = (unsigned int)DAT_006d9d50;
    cVar1 = *(char *)((unsigned int)DAT_006d4988 + 2u + uVar5 * 0x23u);
    if ((cVar1 != '\0') && (cVar1 != '\x01')) {
        return (uVar5 * 7u) & 0xffff0000u;
    }
    uVar6 = (unsigned int)DAT_006d9d72;
    if ((unsigned int)*(unsigned char *)(DAT_006d9d56 + 0x34) == uVar6) {
        if (*(short *)(DAT_00732e54 + 0x6a) == -1) {
            uVar6 = (unaff_ESI & 0xffff0000u) | (unsigned int)DAT_0071b704;
            iVar2 = DAT_00525e50[(unsigned int)DAT_006d9d70];
            if ((int)(unsigned int)DAT_0071b704 < DAT_00732904) {
                do {
                    iVar9 = (int)((uVar6 & 0xffffu) * 0x23u + (unsigned int)DAT_006d4988);
                    cVar1 = *(char *)(iVar9 + 3);
                    if (cVar1 != '\0') {
                        __builtin_memcpy(&iTmp, (void *)(iVar9 + 0x1f), 4);
                        __builtin_memcpy(&iVar9, (void *)(iTmp + 0xa3), 4);
                        if ((*(char *)(iVar9 + 1) != '\0') &&
                            (*(unsigned short *)(iVar9 + 4) == DAT_006d9d50)) {
                            iVar9 = (int)((unsigned int)iVar2 * 3u);
                            if (cVar1 != -0x70) {
                                iVar9 = iVar2;
                            }
                            bVar4 = FUN_00429fc0(uVar6 & 0xffffu, iVar9);
                            if ((unsigned int)bVar4 == 1) {
                                *(short *)(DAT_00732e54 + 0x6a) = (short)uVar6;
                                FUN_0042a140((unsigned int)DAT_006d9d50, uVar6 & 0xffffu);
                                if (DAT_00525e60[(unsigned short)(DAT_006da9f0 -
                                        *(short *)((unsigned int)DAT_006d9d50 * 0x23u + 0x10u +
                                                   (unsigned int)DAT_006d4988)) >> 0xd] == '\0') {
                                    *(unsigned char *)(DAT_006d9d56 + 0x34) = 0x18;
                                } else {
                                    *(unsigned char *)(DAT_006d9d56 + 0x34) = 1;
                                }
                                FUN_00491ad0();
                                uVar5 = (unsigned int)DAT_00732e54;
                                if ((*(char *)(DAT_00732e54 + 0x23c) != '\0') &&
                                    (*(char *)(DAT_00732e54 + 0x23d) != '\0')) {
                                    if ((*(char *)(DAT_00732e54 + 0x23c) == '\x01') &&
                                        ((*(unsigned char *)(DAT_00732e54 + 0x1df) & 2) != 0)) {
                                        *(short *)(DAT_00732e54 + 0x23e) =
                                            (short)((unsigned int)*(short *)(DAT_00732e54 + 0x23e) + 10u);
                                        uVar6 = (unsigned int)DAT_00732e54;
                                        uVar5 = (unsigned int)DAT_006d9d60 * 0x566u;
                                        if ((DAT_0070ad44[uVar5] != '\x15') && (DAT_0070b215[uVar5] != '\x15')) {
                                            *(char *)(DAT_00732e54 + 0x23d) =
                                                (char)((unsigned int)(int)*(char *)(DAT_00732e54 + 0x23d) - 1u);
                                            return uVar6 & 0xffff0000u;
                                        }
                                    } else if (*(short *)(DAT_00732e54 + 0x240) == 0) {
                                        uVar5 = FUN_00448840((unsigned int)DAT_006d9d50, 0x9b);
                                    }
                                }
                                return uVar5 & 0xffff0000u;
                            }
                        }
                    }
                    uVar11 = (unsigned short)((short)uVar6 + 1);
                    uVar6 = (uVar6 & 0xffff0000u) | (unsigned int)uVar11;
                } while ((int)(unsigned int)uVar11 < DAT_00732904);
            }
            iVar2 = DAT_00525e40[(unsigned int)DAT_006d9d70];
            if ((*(short *)(DAT_00732e54 + 0x6a) == -1) &&
                (uVar6 = (uVar6 & 0xffff0000u) | (unsigned int)DAT_00732a3c,
                 (int)(unsigned int)DAT_00732a3c < DAT_006d4980)) {
                do {
                    uVar10 = uVar6 & 0xffffu;
                    iVar9 = (int)(uVar10 * 0x23u);
                    __builtin_memcpy(&uTmp, (void *)(DAT_006d4988 + iVar9 + 0x1b), 4);
                    if (uTmp == 0xffffffffu) {
                        __builtin_memcpy(&iTmp2,
                                         (void *)((int)((unsigned int)DAT_006d9d50 * 0x23u + 0x1fu +
                                                        (unsigned int)DAT_006d4988)),
                                         4);
                        if ((*(char *)(DAT_006d4988 + iVar9 + 3) != '\0') &&
                            ((bVar4 = FUN_00444040(uVar6, (unsigned int)*(unsigned char *)(iTmp2 + 0x87)),
                              (unsigned int)bVar4 != 0) &&
                             (*(char *)(DAT_00732e54 + 7) == '\0') &&
                             (*(char *)(DAT_006d4988 + 2 + iVar9) == '\0') &&
                             (bVar4 = FUN_00429fc0(uVar10, iVar2), (unsigned int)bVar4 == 1))) {
                            FUN_0042a140(uVar10, uVar5);
                            uVar11 = (unsigned short)(DAT_006da9f0 - *(short *)(DAT_006d4988 + iVar9 + 0x10));
                            if (0x7fff < uVar11) {
                                uVar11 = (unsigned short)(0u - (unsigned int)uVar11);
                            }
                            uVar8 = (unsigned short)(DAT_00732e40 - *(short *)(DAT_006d4988 + iVar9 + 0x12));
                            if (0x7fff < uVar8) {
                                uVar8 = (unsigned short)(0u - (unsigned int)uVar8);
                            }
                            if ((uVar11 < 0x2000) && (uVar8 < 0x2000)) {
                                *(short *)(DAT_00732e54 + 0x6a) = (short)uVar6;
                                break;
                            }
                        }
                    }
                    uVar11 = (unsigned short)((short)uVar6 + 1);
                    uVar6 = (uVar6 & 0xffff0000u) | (unsigned int)uVar11;
                } while ((int)(unsigned int)uVar11 < DAT_006d4980);
            }
        }
        uVar11 = *(unsigned short *)(DAT_00732e54 + 0x6a);
        uVar6 = (((unsigned int)DAT_00732e54 >> 0x10) << 0x10) | (unsigned int)uVar11;
        if (uVar11 != 0xffff) {
            FUN_0042a140(uVar5, (unsigned int)uVar11);
            cVar1 = DAT_00525e60[(unsigned short)(DAT_006da9f0 -
                        *(short *)((unsigned int)DAT_006d4988 + 0x10u + uVar5 * 0x23u)) >> 0xd];
            __builtin_memcpy(&uVar11,
                             &DAT_0052b08d[(unsigned int)*(unsigned char *)(DAT_00732e54 + 4) * 0xefu], 2);
            __builtin_memcpy(&pcVar3,
                             (void *)((unsigned int)*(unsigned short *)(DAT_00732e54 + 0x6a) * 0x23u + 0x1fu +
                                      (unsigned int)DAT_006d4988),
                             4);
            if (*pcVar3 == '\0') {
                __builtin_memcpy(&iTmp, pcVar3 + 0xa7, 4);
                __builtin_memcpy(&uVar8,
                                 &DAT_0052b08d[(unsigned int)*(unsigned char *)(iTmp + 4) * 0xefu], 2);
            } else {
                uVar8 = 900;
            }
            uVar5 = FUN_00478470();
            if (cVar1 == '\x01') {
                if ((DAT_006db580 < 0x2000) && ((unsigned short)uVar5 < 0x4000)) {
                    uVar7 = 1;
                } else {
                    uVar7 = DAT_00525e68[uVar5 & 3];
                }
            } else if (cVar1 == '\0') {
                if ((unsigned short)uVar5 < 0x8001) {
                    uVar7 = DAT_00525e68[uVar5 & 3];
                } else {
                    uVar7 = 9;
                }
            } else if ((uVar11 < uVar8) || (uVar7 = 0x10, DAT_006db580 < 0x8001)) {
                uVar7 = DAT_00525e70[uVar5 & 7];
            }
            *(unsigned char *)(DAT_006d9d56 + 0x34) = uVar7;
            uVar6 = FUN_00491ad0();
        }
    }
    return uVar6 & 0xffff0000u;
}
