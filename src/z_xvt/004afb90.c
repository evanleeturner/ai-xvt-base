/* SPDX-License-Identifier: MIT */
/* z_xvt 004afb90 */
extern unsigned char DAT_0071bb37[];
extern unsigned char DAT_0071b730[];
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern unsigned short DAT_00655184;
extern unsigned short DAT_00655a84;
extern unsigned char DAT_00656c90[];
extern unsigned char DAT_00655ab0[];
extern unsigned char DAT_00655ab2[];
extern unsigned char DAT_00655ab4[];
extern unsigned char DAT_00655ab6[];
extern unsigned int DAT_0071f2f0[];
extern void (*DAT_0070a184)(unsigned int, unsigned short, unsigned short, unsigned short, int);
extern unsigned char DAT_00656d42[];
extern unsigned char DAT_00656d44[];
extern unsigned char DAT_00656d3e[];
extern unsigned char DAT_00656d0c[];
extern unsigned char DAT_00656d0a[];
extern unsigned char DAT_00656d04[];
extern unsigned short DAT_00656d38[];
extern unsigned short DAT_00656cea[];
extern unsigned short DAT_00655cd0[];
extern unsigned short DAT_00655ccc[];
extern unsigned short DAT_00655cce[];
extern unsigned short DAT_00655cd2[];

void __stdcall FUN_004afb90(void)
{
    char cVar1;
    int bVar2;
    int iVar3;
    int iVar4;
    unsigned int uVar5;
    unsigned short uVar6;
    unsigned short uVar7;
    unsigned int idx;
    int p;
    int q;
    unsigned short f;
    unsigned short f2;

    idx = (unsigned int)DAT_0071ecf4 * 0x5bd;

    if (DAT_0071bb37[idx] != 0) {
        if (DAT_0071bb37[idx] == 0x13) {
            uVar7 = 1;
            uVar6 = 0;
            do {
                if (((uVar7 < 5) && (uVar7 != 4)) && (uVar7 == 1)) {
                    __builtin_memcpy(&iVar4, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
                    if ((iVar4 == -1) ||
                        (((cVar1 = *(char *)(DAT_006d4988 + 3 + (int)((unsigned int)iVar4 * 0x23)), cVar1 != 1) &&
                          (cVar1 != 2)) && ((cVar1 != 3 && ((cVar1 != 0xe && (cVar1 != 4))))))) {
                        bVar2 = 0;
                    } else {
                        bVar2 = 1;
                    }
                    iVar3 = 0;
                    if (!bVar2) {
                        f = 0; /* not evaluated */
                    }
                    if (!bVar2) {
                        goto check_d9;
                    }
                    __builtin_memcpy(&p, (unsigned char *)(DAT_006d4988 + 0x1f + (int)((unsigned int)iVar4 * 0x23)), 4);
                    __builtin_memcpy(&q, (unsigned char *)(p + 0xa7), 4);
                    __builtin_memcpy(&f, (unsigned char *)(q + 0x1db), 2);
                    if ((f & 1) != 0) {
                        goto skip_block;
                    }
check_d9:
                    iVar3 = (int)((unsigned int)iVar4 * 0x23) + DAT_006d4988;
                    __builtin_memcpy(&p, (unsigned char *)(iVar3 + 0x1f), 4);
                    __builtin_memcpy(&q, (unsigned char *)(p + 0xa7), 4);
                    __builtin_memcpy(&f2, (unsigned char *)(q + 0x1d9), 2);
                    if ((f2 & 1) != 0) {
                        if ((iVar4 == -1) ||
                            (((cVar1 = *(char *)(iVar3 + 3), cVar1 != 1) && (cVar1 != 2)) &&
                             ((cVar1 != 3 && ((cVar1 != 0xe && (cVar1 != 4))))))) {
                            bVar2 = 0;
                        } else {
                            bVar2 = 1;
                        }
                        if ((bVar2) || (DAT_00655184 != 0)) {
                            iVar4 = DAT_00655184 + 0x54;
                            if (*(unsigned short *)(DAT_00656c90 + iVar4 * 2) != 0) {
                                *(unsigned short *)(DAT_00656c90 + iVar4 * 2) = 0;
                                iVar4 = iVar4 * 0xc;
                                (*DAT_0070a184)(DAT_0071f2f0[*(unsigned short *)(DAT_00655ab4 + iVar4)],
                                                *(unsigned short *)(DAT_00655ab0 + iVar4),
                                                *(unsigned short *)(DAT_00655ab2 + iVar4),
                                                *(unsigned short *)(DAT_00655ab6 + iVar4), 0);
                            }
                            uVar5 = (unsigned int)DAT_00655184;
                            iVar4 = (int)(uVar5 * 2);
                            *(unsigned short *)(DAT_00656d42 + iVar4) = 0xffff;
                            *(unsigned short *)(DAT_00656d44 + iVar4) = 0xffff;
                            *(unsigned short *)(DAT_00656d3e + iVar4) = 0xffff;
                            *(unsigned short *)(DAT_00656d0c + iVar4) = 0xffff;
                            *(unsigned short *)(DAT_00656d0a + iVar4) = 0xffff;
                            *(unsigned short *)(DAT_00656d04 + iVar4) = 0xffff;
                            DAT_00656d38[uVar5] = 0xffff;
                        } else if (DAT_00656cea[0] != 0) {
                            DAT_00656cea[0] = 0;
                            (*DAT_0070a184)(DAT_0071f2f0[DAT_00655cd0[0]], DAT_00655ccc[0],
                                            DAT_00655cce[0], DAT_00655cd2[0], 0);
                        }
                    }
                }
skip_block:
                uVar7 = (unsigned short)(uVar7 * 2);
                uVar6 = (unsigned short)(uVar6 + 1);
            } while (uVar6 < 0xd);
            DAT_00655a84 = 0xffff;
        }
        return;
    }

    uVar7 = 1;
    uVar5 = 0;
    do {
        __builtin_memcpy(&iVar4, DAT_0071b730 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        if ((iVar4 == -1) ||
            ((((cVar1 = *(char *)(DAT_006d4988 + 3 + (int)((unsigned int)iVar4 * 0x23)), cVar1 != 1) &&
               (cVar1 != 2)) && (cVar1 != 3)) && ((cVar1 != 0xe && (cVar1 != 4))))) {
            bVar2 = 0;
        } else {
            bVar2 = 1;
        }
        __builtin_memcpy(&p, (unsigned char *)(DAT_006d4988 + 0x1f + (int)((unsigned int)iVar4 * 0x23)), 4);
        __builtin_memcpy(&q, (unsigned char *)(p + 0xa7), 4);
        __builtin_memcpy(&f, (unsigned char *)(q + 0x1db), 2);
        if (bVar2) {
            if ((f & uVar7) == 0) {
                uVar6 = 0;
                goto LAB_004afc6c;
            }
        } else {
            uVar6 = (unsigned short)(-(unsigned short)((f & uVar7) == 0) & 0xd);
LAB_004afc6c:
            __builtin_memcpy(&p, (unsigned char *)(DAT_006d4988 + 0x1f + (int)((unsigned int)iVar4 * 0x23)), 4);
            __builtin_memcpy(&q, (unsigned char *)(p + 0xa7), 4);
            __builtin_memcpy(&f2, (unsigned char *)(q + 0x1d9), 2);
            if (((f2 & uVar7) != 0) && (DAT_00656cea[uVar5] != uVar6)) {
                DAT_00656cea[uVar5] = uVar6;
                (*DAT_0070a184)(DAT_0071f2f0[(unsigned int)DAT_00655cd0[uVar5 * 6] + (unsigned int)uVar6],
                                DAT_00655ccc[uVar5 * 6], DAT_00655cce[uVar5 * 6],
                                DAT_00655cd2[uVar5 * 6], 0);
            }
        }
        uVar7 = (unsigned short)(uVar7 * 2);
        uVar6 = (unsigned short)(uVar5 + 1);
        uVar5 = (unsigned int)uVar6;
        if (0xc < uVar6) {
            DAT_00655a84 = 0xffff;
            return;
        }
    } while (1);
}
