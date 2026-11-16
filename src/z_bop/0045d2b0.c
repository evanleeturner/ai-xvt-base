/* SPDX-License-Identifier: MIT */
/* z_bop 0045d2b0 */
extern short DAT_009a2000;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern unsigned int DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned char DAT_009a73b6[4];
extern unsigned char DAT_009a73da;
extern unsigned short DAT_00a07cec;
extern int DAT_00a08104;
extern unsigned char DAT_009d1320[];
extern char s_escortldr1pln_0051a1e4[];

unsigned short __cdecl FUN_00432fa0(unsigned short, unsigned short, unsigned short);
void __cdecl FUN_00403540(unsigned int, unsigned int);
unsigned short __cdecl FUN_00403070(unsigned int);
unsigned short __cdecl FUN_0045dba0(unsigned short, unsigned int);

unsigned short __cdecl FUN_0045d2b0(unsigned short param_1, unsigned short param_2, short param_3,
                                    unsigned short param_4, unsigned short param_5)
{
    char cVar1;
    unsigned short uVar2;
    unsigned short uVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned int unaff_EBP;
    char *pcVar7;
    char *pcVar8;
    unsigned char bVar9;
    unsigned short local_8;
    unsigned short uStack_6;
    unsigned short local_4;
    unsigned short uStack_2;
    int iVar10;
    unsigned int uVar11;
    unsigned int uVar12;
    unsigned short uVar13;
    unsigned short uVar14;
    int iVar15;

    uStack_6 = 0xffff;
    local_4 = 0xffff;
    uStack_2 = 0xffff;
    local_8 = 0;
    if (0 < DAT_009a2000) {
        do {
            uVar2 = FUN_00432fa0(local_8, param_1, param_2);
            uVar3 = FUN_00432fa0(local_8, param_4, param_5);
            if (param_3 == 1) {
                uVar2 = (unsigned short)(uVar2 | uVar3);
            } else {
                uVar2 = (unsigned short)(uVar2 & uVar3);
            }
            if ((uVar2 != 0) &&
                (unaff_EBP = (((unsigned int)(unaff_EBP >> 0x10)) << 0x10) | (unsigned int)DAT_00a07cec,
                 (int)(unsigned int)DAT_00a07cec < DAT_009a1fe0)) {
                do {
                    uVar6 = unaff_EBP & 0xffff;
                    iVar4 = (int)(uVar6 * 0x23 + (unsigned int)DAT_009a1fe8);
                    bVar9 = (*(char *)(iVar4 + 3) == '\0');
                    uVar2 = (unsigned short)unaff_EBP;
                    if (!bVar9) {
                        __builtin_memcpy(&iVar4, (char *)(iVar4 + 0x1f), 4);
                        __builtin_memcpy(&iVar4, (char *)(iVar4 + 0xa9), 4);
                        iVar5 = 0xe;
                        pcVar7 = (char *)((int)DAT_009d1320 + (unsigned int)*(unsigned char *)(iVar4 + 0x35) * 0x55);
                        pcVar8 = s_escortldr1pln_0051a1e4;
                        do {
                            if (iVar5 == 0) break;
                            iVar5 = iVar5 + -1;
                            bVar9 = (*pcVar7 == *pcVar8);
                            pcVar7 = pcVar7 + 1;
                            pcVar8 = pcVar8 + 1;
                        } while (bVar9);
                        if (((bVar9) && (*(unsigned char *)(iVar4 + 0x57) == local_8)) &&
                            (*(unsigned short *)(DAT_00a08104 + 0x36e) != uVar2)) {
                            if (uVar2 == 0xffff) {
                                iVar4 = 0;
                            } else {
                                iVar4 = (int)((unsigned int)DAT_009a1fe8 + uVar6 * 0x23);
                                __builtin_memcpy(&iVar5, (char *)(iVar4 + 0x1f), 4);
                                if (iVar5 == 0) {
                                    bVar9 = (*(char *)(iVar4 + 3) == '\0');
LAB_0045d4f8:
                                    iVar4 = (int)(1 - (unsigned int)bVar9);
                                } else if (*(char *)(iVar4 + 3) == '\0') {
                                    iVar4 = 0;
                                } else {
                                    __builtin_memcpy(&iVar4, (char *)(iVar5 + 0xa9), 4);
                                    if (iVar4 == 0) {
                                        iVar4 = 1;
                                    } else if ((*(char *)(iVar4 + 0x5e) == '\x15') && (*(char *)(iVar4 + 0x5f) != '\0')) {
                                        iVar4 = 0;
                                    } else {
                                        cVar1 = *(char *)(iVar4 + 7);
                                        if ((cVar1 == '\x01') || ((cVar1 == '\x03' || (cVar1 == '\x04')))) {
                                            iVar4 = 0;
                                        } else if ((*(char *)(iVar4 + 0x23d) == '\x03') &&
                                                   (*(char *)(iVar4 + 0x241) != '\0')) {
                                            FUN_00403540(uVar6, (unsigned int)DAT_009a73b0);
                                            cVar1 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + (unsigned int)DAT_009a1fe8);
                                            if ((cVar1 == '\0') || ((cVar1 == '\x01' || (cVar1 == '\x02')))) {
                                                if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045d4bc;
                                                iVar4 = 0;
                                            } else {
                                                if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045d4bc;
                                                iVar4 = 0;
                                            }
                                        } else {
LAB_0045d4bc:
                                            iVar5 = (int)((unsigned int)DAT_009a1fe8 + uVar6 * 0x23);
                                            __builtin_memcpy(&iVar10, (char *)(iVar5 + 0x1b), 4);
                                            if (iVar10 == -1) {
                                                __builtin_memcpy(&iVar10, (char *)(iVar5 + 0x1f), 4);
                                                __builtin_memcpy(&iVar10, (char *)(iVar10 + 0xa9), 4);
                                                __builtin_memcpy(&uVar11, (char *)(iVar10 + 0xcb), 4);
                                                __builtin_memcpy(&uVar12, (char *)(iVar4 + 0xc3), 4);
                                                bVar9 = (uVar11 < uVar12);
                                                goto LAB_0045d4f8;
                                            }
                                            iVar4 = 1;
                                        }
                                    }
                                }
                            }
                            if (iVar4 != 0) {
                                if ((DAT_009a73da & 4) != 0) {
                                    uVar14 = FUN_00403070(unaff_EBP);
                                    if (uVar14 == 0) goto LAB_0045d55a;
                                }
                                if ((DAT_009a73da & 1) != 0) {
                                    uVar14 = FUN_0045dba0(uVar2, 0xff);
                                    if (uVar14 == 0) goto LAB_0045d55a;
                                }
                                FUN_00403540((unsigned int)DAT_009a73b0, uVar6);
                                if (DAT_009a1ff8 < ((((unsigned int)uStack_2) << 16) | (unsigned int)local_4)) {
                                    local_4 = (unsigned short)DAT_009a1ff8;
                                    uStack_2 = (unsigned short)(DAT_009a1ff8 >> 0x10);
                                    uStack_6 = uVar2;
                                }
                            }
                        }
                    }
LAB_0045d55a:
                    unaff_EBP = (((unsigned int)(unaff_EBP >> 0x10)) << 0x10) | (unsigned int)(unsigned short)(uVar2 + 1);
                } while ((int)(unsigned int)(unsigned short)(uVar2 + 1) < DAT_009a1fe0);
            }
            local_8 = (unsigned short)(local_8 + 1);
        } while ((int)(unsigned int)local_8 < (int)DAT_009a2000);
    }
    if (uStack_6 != 0xffff) {
        __builtin_memcpy(&iVar15, DAT_009a73b6, 4);
        *(unsigned short *)(iVar15 + 0x18) = uStack_6;
        __builtin_memcpy(&uVar13, (char *)((unsigned int)uStack_6 * 0x23 + (unsigned int)DAT_009a1fe8), 2);
        *(unsigned short *)(iVar15 + 0x1a) = uVar13;
        *(unsigned char *)(iVar15 + 0x1e) = 1;
    }
    return uStack_6;
}
