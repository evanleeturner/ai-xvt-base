/* SPDX-License-Identifier: MIT */
/* z_bop 00461460 */
extern unsigned short DAT_00a07cec;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009a1ff8;
extern char DAT_009a73d8;
extern unsigned short DAT_009a73b0;
extern unsigned char DAT_0099f930[];

unsigned char __cdecl FUN_00433390(unsigned int a, unsigned short b, unsigned short c);
void __cdecl FUN_00403540(unsigned int a, unsigned int b);

void __cdecl FUN_00461460(unsigned short param_1, unsigned short param_2, short param_3,
                          unsigned short param_4, unsigned short param_5, unsigned int param_6)
{
    char cVar1;
    unsigned char bVar2;
    unsigned short uVar3;
    int iVar4;
    int iVar5;
    unsigned short uVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    int tmpA;
    int tmpB;
    unsigned int utmpA;
    unsigned int utmpB;
    short stmp;

    /* High 16 bits come from the caller's EBP (unaff_EBP) in the original;
       unresolved, zero is used here. */
    uVar7 = (unsigned int)DAT_00a07cec;
    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
        do {
            iVar4 = DAT_009a1fe8;
            uVar8 = uVar7 & 0xffff;
            DAT_0099f930[uVar8 * 2 + (param_6 & 0xffff)] = 0;
            if (*(char *)(iVar4 + 3 + uVar8 * 0x23) != '\0') {
                bVar2 = FUN_00433390(uVar7, param_1, param_2);
                uVar6 = (unsigned short)bVar2;
                bVar2 = FUN_00433390(uVar7, param_4, param_5);
                uVar3 = (unsigned short)bVar2;
                if (param_3 == 1) {
                    uVar6 = uVar6 | uVar3;
                } else {
                    uVar6 = uVar6 & uVar3;
                }
                if (uVar6 != 0) {
                    int pass = 0;
                    if (DAT_009a73d8 == '\0') {
                        pass = 1;
                    } else {
                        __builtin_memcpy(&tmpA, (unsigned char *)(DAT_009a1fe8 + 0x1f + uVar8 * 0x23), 4);
                        __builtin_memcpy(&tmpB, (unsigned char *)(tmpA + 0xa9), 4);
                        __builtin_memcpy(&stmp, (unsigned char *)(tmpB + 0x1e5), 2);
                        if (stmp != 0) {
                            pass = 1;
                        }
                    }
                    if (pass) {
                        if ((short)uVar7 == -1) {
                            iVar4 = 0;
                        } else {
                            iVar4 = DAT_009a1fe8 + uVar8 * 0x23;
                            __builtin_memcpy(&tmpA, (unsigned char *)(iVar4 + 0x1f), 4);
                            if (tmpA == 0) {
                                bVar2 = (*(char *)(iVar4 + 3) == '\0');
                                goto LAB_0046165c;
                            } else if (*(char *)(iVar4 + 3) == '\0') {
                                iVar4 = 0;
                            } else {
                                __builtin_memcpy(&tmpB, (unsigned char *)(tmpA + 0xa9), 4);
                                iVar4 = tmpB;
                                if (iVar4 == 0) {
                                    iVar4 = 1;
                                } else if (*(char *)(iVar4 + 0x5e) == '\x15' && *(char *)(iVar4 + 0x5f) != '\0') {
                                    iVar4 = 0;
                                } else {
                                    cVar1 = *(char *)(iVar4 + 7);
                                    if (cVar1 == '\x01' || cVar1 == '\x03' || cVar1 == '\x04') {
                                        iVar4 = 0;
                                    } else if (*(char *)(iVar4 + 0x23d) == '\x03' && *(char *)(iVar4 + 0x241) != '\0') {
                                        FUN_00403540(uVar8, (unsigned int)DAT_009a73b0);
                                        cVar1 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                        if (cVar1 == '\0' || cVar1 == '\x01' || cVar1 == '\x02') {
                                            if (DAT_009a1ff8 < 0x8001) goto LAB_00461620;
                                            iVar4 = 0;
                                        } else {
                                            if (DAT_009a1ff8 < 0x10001) goto LAB_00461620;
                                            iVar4 = 0;
                                        }
                                    } else {
LAB_00461620:
                                        iVar5 = DAT_009a1fe8 + uVar8 * 0x23;
                                        __builtin_memcpy(&tmpA, (unsigned char *)(iVar5 + 0x1b), 4);
                                        if (tmpA == -1) {
                                            __builtin_memcpy(&tmpA, (unsigned char *)(iVar5 + 0x1f), 4);
                                            __builtin_memcpy(&tmpB, (unsigned char *)(tmpA + 0xa9), 4);
                                            __builtin_memcpy(&utmpA, (unsigned char *)(tmpB + 0xcb), 4);
                                            __builtin_memcpy(&utmpB, (unsigned char *)(iVar4 + 0xc3), 4);
                                            bVar2 = (utmpA < utmpB);
                                            goto LAB_0046165c;
                                        }
                                        iVar4 = 1;
                                    }
                                }
                            }
                            goto LAB_0046165f;
LAB_0046165c:
                            iVar4 = 1 - (unsigned int)bVar2;
                        }
LAB_0046165f:
                        if (iVar4 != 0) {
                            DAT_0099f930[uVar8 * 2 + (param_6 & 0xffff)] = 1;
                        }
                    }
                }
            }
            uVar6 = (unsigned short)((short)uVar7 + 1);
            uVar7 = (uVar7 & 0xffff0000u) | (unsigned int)uVar6;
        } while ((int)(unsigned int)uVar6 < DAT_009a1fe0);
    }
    return;
}
