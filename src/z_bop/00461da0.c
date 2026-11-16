/* SPDX-License-Identifier: MIT */
/* z_bop 00461da0 */
extern int DAT_009a73bc;
extern unsigned short DAT_009a73ba;
extern int DAT_009a1fe8;
extern int DAT_009a73b6;
extern unsigned short DAT_009a73b0;
extern int DAT_009a1ff8;
extern int DAT_009a1fe0;
extern unsigned short DAT_00a07cec;
extern int DAT_00a08104;
extern int DAT_009d1308;
extern int DAT_00a07ce8;
extern unsigned char DAT_009d8c88[];
extern char DAT_009fd458[];
extern char DAT_009d1320;
extern char DAT_009a73d8;
extern char DAT_005242a0;
extern char s_disableldr1pln_0051a310[];
extern char s_capfreeldr1pln_0051a348[];

unsigned int FUN_00403540(unsigned int a, unsigned int b);
int FUN_00402ec0(unsigned int a, unsigned int b, int c);
unsigned int FUN_00404450(unsigned int a);

unsigned int __stdcall FUN_00461da0(void)
{
    char *pcVar1;
    char cVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    int iVar5;
    int iVar6;
    short sVar7;
    unsigned int uVar8;
    int iVar9;
    unsigned short uVar10;
    unsigned short uVar11;
    unsigned short uVar12;
    int iVar13;
    char *pcVar14;
    char *pcVar15;
    unsigned char bVar16;
    int t1;
    int t2;
    unsigned int u1;
    unsigned int u2;
    unsigned short us;
    short ss;

    iVar13 = DAT_009a73bc;
    uVar3 = (unsigned int)DAT_009a73ba;
    __builtin_memcpy(&iVar6, (unsigned char *)(DAT_009a1fe8 + 0x1b + uVar3 * 0x23), 4);
    uVar8 = (((((unsigned int)DAT_009a1fe8 >> 8) << 8) |
              (unsigned int)*(unsigned char *)(DAT_009a73bc + 0x5e)) & 0xffff00ff);
    DAT_009a73d8 = (uVar8 == 0);
    pcVar1 = (char *)((int)&DAT_009d1320 + (unsigned int)*(unsigned char *)(DAT_009a73b6 + 0xb) * 0x55);
    iVar5 = 0xf;
    pcVar14 = pcVar1;
    pcVar15 = s_disableldr1pln_0051a310;
    do {
        if (iVar5 == 0) break;
        iVar5 = iVar5 + -1;
        DAT_009a73d8 = (*pcVar14 == *pcVar15);
        pcVar14 = pcVar14 + 1;
        pcVar15 = pcVar15 + 1;
    } while ((unsigned char)DAT_009a73d8);
    sVar7 = (short)uVar8;
    if (((sVar7 != 0xc) && (sVar7 != 0x17)) && (iVar6 == -1)) {
        return 0xffff0000;
    }
    __builtin_memcpy(&uVar10, (unsigned char *)(DAT_009a73b6 + 0x2b), 2);
    if ((uVar10 != 0xffff) && (uVar10 != 0xfb)) {
        if (uVar10 == 0xffff) {
            iVar5 = 0;
        }
        else {
            iVar5 = (unsigned int)uVar10 * 0x23;
            iVar9 = DAT_009a1fe8 + iVar5;
            __builtin_memcpy(&t1, (unsigned char *)(iVar9 + 0x1f), 4);
            if (t1 == 0) {
                bVar16 = (*(char *)(iVar9 + 3) == '\0');
LAB_00461f95:
                iVar5 = 1 - (unsigned int)bVar16;
            }
            else if (*(char *)(iVar9 + 3) == '\0') {
                iVar5 = 0;
            }
            else {
                __builtin_memcpy(&iVar9, (unsigned char *)(t1 + 0xa9), 4);
                if (iVar9 == 0) {
                    iVar5 = 1;
                }
                else if ((*(char *)(iVar9 + 0x5e) == '\x15') && (*(char *)(iVar9 + 0x5f) != '\0')) {
                    iVar5 = 0;
                }
                else {
                    cVar2 = *(char *)(iVar9 + 7);
                    if ((cVar2 == '\x01') || ((cVar2 == '\x03' || (cVar2 == '\x04')))) {
                        iVar5 = 0;
                    }
                    else if ((*(char *)(iVar9 + 0x23d) == '\x03') && (*(char *)(iVar9 + 0x241) != '\0')) {
                        FUN_00403540((unsigned int)uVar10, (unsigned int)DAT_009a73b0);
                        cVar2 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                        if ((cVar2 == '\0') || ((cVar2 == '\x01' || (cVar2 == '\x02')))) {
                            if (DAT_009a1ff8 < 0x8001) goto LAB_00461f59;
                            iVar5 = 0;
                        }
                        else {
                            if (DAT_009a1ff8 < 0x10001) goto LAB_00461f59;
                            iVar5 = 0;
                        }
                    }
                    else {
LAB_00461f59:
                        iVar5 = DAT_009a1fe8 + iVar5;
                        __builtin_memcpy(&t2, (unsigned char *)(iVar5 + 0x1b), 4);
                        if (t2 == -1) {
                            __builtin_memcpy(&t1, (unsigned char *)(iVar5 + 0x1f), 4);
                            __builtin_memcpy(&t2, (unsigned char *)(t1 + 0xa9), 4);
                            __builtin_memcpy(&u1, (unsigned char *)(t2 + 0xcb), 4);
                            __builtin_memcpy(&u2, (unsigned char *)(iVar9 + 0xc3), 4);
                            bVar16 = (u1 < u2);
                            goto LAB_00461f95;
                        }
                        iVar5 = 1;
                    }
                }
            }
        }
        if (iVar5 != 0) {
            if ((iVar6 != -1) &&
                (uVar3 = FUN_00403540((unsigned int)DAT_009a73b0, (unsigned int)uVar10), 0x50000 < DAT_009a1ff8)) {
                return uVar3 & 0xffff0000;
            }
            *(unsigned short *)(DAT_009a73b6 + 0x18) = uVar10;
            uVar3 = (unsigned int)DAT_009a1fe8 >> 0x10;
            __builtin_memcpy(&us, (unsigned char *)((unsigned int)uVar10 * 0x23 + DAT_009a1fe8), 2);
            *(unsigned short *)(DAT_009a73b6 + 0x1a) = us;
            *(unsigned char *)(DAT_009a73b6 + 0x1e) = 1;
            return ((uVar3 & 0xffff) << 16) | 1;
        }
        us = 0xffff;
        __builtin_memcpy((unsigned char *)(DAT_009a73b6 + 0x2b), &us, 2);
    }
    uVar10 = 0xffff;
    iVar5 = DAT_009a1fe8 + uVar3 * 0x23;
    __builtin_memcpy(&iVar6, (unsigned char *)(iVar5 + 0x1b), 4);
    uVar12 = DAT_00a07cec;
    if (iVar6 == -1) {
        uVar10 = *(unsigned short *)(iVar13 + 0x42);
        uVar3 = 0xffffffff;
LAB_0046219c:
        if (uVar10 == 0xffff) goto LAB_0046235b;
    }
    else {
        for (; (int)(unsigned int)uVar12 < DAT_009a1fe0; uVar12 = uVar12 + 1) {
            iVar13 = (unsigned int)uVar12 * 0x23 + DAT_009a1fe8;
            if (*(char *)(iVar13 + 3) != '\0') {
                __builtin_memcpy(&t1, (unsigned char *)(iVar13 + 0x1f), 4);
                __builtin_memcpy(&iVar9, (unsigned char *)(t1 + 0xa9), 4);
                __builtin_memcpy(&ss, (unsigned char *)(iVar9 + 0x1e5), 2);
                if ((DAT_009a73d8 == '\0') || (ss != 0)) {
                    if (*(unsigned short *)(DAT_00a08104 + 0x36e) != uVar12) {
                        __builtin_memcpy(&t2, (unsigned char *)((unsigned int)*(unsigned short *)(iVar9 + 0x6a) * 0x23 + 0x1b + DAT_009a1fe8), 4);
                        if (t2 == iVar6) {
                            __builtin_memcpy(&t1, (unsigned char *)(iVar5 + 0x1f), 4);
                            uVar3 = (unsigned int)*(unsigned char *)(t1 + 0x89);
                            if (DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar13 + 0x16) * 0x566] == uVar3) {
                                bVar16 = 0;
                            }
                            else {
                                bVar16 = (DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar13 + 0x16) * 0x566] + uVar3 * 0x1e5] == '\0');
                            }
                            if (bVar16) {
                                uVar10 = uVar12;
                            }
                        }
                    }
                }
            }
        }
        if (uVar10 == 0xffff) {
            do {
                uVar3 = 0;
            } while (0 < (int)(DAT_009d1308 + DAT_00a07ce8));
            goto LAB_0046219c;
        }
    }
    iVar6 = (unsigned int)uVar10 * 0x23 + DAT_009a1fe8;
    __builtin_memcpy(&uVar3, (unsigned char *)(iVar6 + 0x1f), 4);
    if (uVar3 == 0) {
        cVar2 = *(char *)(iVar6 + 0x16);
        uVar12 = uVar10 + 1;
        if ((int)(DAT_009d1308 + DAT_00a07ce8) <= (int)(unsigned int)(unsigned short)(uVar10 + 1)) {
            uVar12 = (unsigned short)DAT_00a07ce8;
        }
        uVar3 = DAT_00a07ce8 & 0xffff;
        uVar10 = (unsigned short)DAT_00a07ce8;
        if ((int)uVar3 < (int)(DAT_009d1308 + DAT_00a07ce8)) {
            do {
                __builtin_memcpy(&ss, (unsigned char *)((unsigned int)uVar12 * 0x23 + 0x17 + DAT_009a1fe8), 2);
                if (((DAT_009a73d8 == '\0') || (ss != 0)) &&
                    (*(unsigned short *)(DAT_00a08104 + 0x36e) != uVar12)) {
                    iVar6 = (unsigned int)uVar12 * 0x23 + DAT_009a1fe8;
                    if (((*(char *)(iVar6 + 3) != '\0') && (*(char *)(iVar6 + 0x16) == cVar2)) &&
                        ((iVar6 = FUN_00402ec0((unsigned int)DAT_009a73b0, (unsigned int)uVar12, 1), iVar6 != 0) &&
                         (uVar4 = FUN_00404450(uVar12), (short)uVar4 != 0))) {
                        *(unsigned short *)(DAT_009a73b6 + 0x18) = uVar12;
                        uVar3 = (unsigned int)DAT_009a1fe8 >> 0x10;
                        __builtin_memcpy(&us, (unsigned char *)((unsigned int)uVar12 * 0x23 + DAT_009a1fe8), 2);
                        *(unsigned short *)(DAT_009a73b6 + 0x1a) = us;
                        *(unsigned char *)(DAT_009a73b6 + 0x1e) = 1;
                        return ((uVar3 & 0xffff) << 16) | 1;
                    }
                    uVar12 = uVar12 + 1;
                    if ((int)(DAT_009d1308 + DAT_00a07ce8) <= (int)(unsigned int)uVar12) {
                        uVar12 = (unsigned short)DAT_00a07ce8;
                    }
                }
                uVar10 = uVar10 + 1;
                if ((int)(DAT_009d1308 + DAT_00a07ce8) <= (int)(unsigned int)uVar10) {
                    return (DAT_009d1308 + DAT_00a07ce8) & 0xffff0000;
                }
            } while (1);
        }
    }
    else {
        __builtin_memcpy(&t1, (unsigned char *)(uVar3 + 0xa9), 4);
        if (t1 != 0) {
            cVar2 = *(char *)(iVar6 + 0x16);
            uVar10 = uVar10 + *(unsigned char *)(DAT_00a08104 + 0xb0);
            if (DAT_009a1fe0 <= (int)(unsigned int)uVar10) {
                uVar10 = DAT_00a07cec;
            }
            uVar3 = (unsigned int)DAT_00a07cec;
            uVar12 = DAT_00a07cec;
            if ((int)uVar3 < DAT_009a1fe0) {
                do {
                    iVar13 = (unsigned int)uVar10 * 0x23 + DAT_009a1fe8;
                    __builtin_memcpy(&t2, (unsigned char *)(iVar13 + 0x1f), 4);
                    __builtin_memcpy(&iVar6, (unsigned char *)(t2 + 0xa9), 4);
                    if (iVar6 != 0) {
                        __builtin_memcpy(&ss, (unsigned char *)(iVar6 + 0x1e5), 2);
                    }
                    if ((iVar6 == 0) ||
                        (((DAT_009a73d8 != '\0') && (ss == 0)) ||
                         (*(unsigned short *)(DAT_00a08104 + 0x36e) == uVar12))) {
                        uVar11 = uVar10 + 1;
                        uVar10 = uVar10 + 1;
                        if (DAT_009a1fe0 <= (int)(unsigned int)uVar11) {
LAB_0046233d:
                            uVar10 = DAT_00a07cec;
                        }
                    }
                    else {
                        if ((*(char *)(iVar13 + 3) != '\0') && (*(char *)(iVar13 + 0x16) == cVar2)) {
                            iVar6 = FUN_00402ec0((unsigned int)DAT_009a73b0, (unsigned int)uVar10, 1);
                            bVar16 = (iVar6 == 0);
                            if (!bVar16) {
                                iVar6 = 0xf;
                                pcVar14 = pcVar1;
                                pcVar15 = s_capfreeldr1pln_0051a348;
                                do {
                                    if (iVar6 == 0) break;
                                    iVar6 = iVar6 + -1;
                                    bVar16 = (*pcVar14 == *pcVar15);
                                    pcVar14 = pcVar14 + 1;
                                    pcVar15 = pcVar15 + 1;
                                } while (bVar16);
                                if (!bVar16) {
                                    iVar6 = 0xf;
                                    pcVar14 = pcVar1;
                                    pcVar15 = s_disableldr1pln_0051a310;
                                    do {
                                        if (iVar6 == 0) break;
                                        iVar6 = iVar6 + -1;
                                        bVar16 = (*pcVar14 == *pcVar15);
                                        pcVar14 = pcVar14 + 1;
                                        pcVar15 = pcVar15 + 1;
                                    } while (bVar16);
                                    if (!bVar16) {
                                        iVar6 = 0xd;
                                        pcVar14 = pcVar1;
                                        pcVar15 = &DAT_005242a0;
                                        do {
                                            if (iVar6 == 0) break;
                                            iVar6 = iVar6 + -1;
                                            bVar16 = (*pcVar14 == *pcVar15);
                                            pcVar14 = pcVar14 + 1;
                                            pcVar15 = pcVar15 + 1;
                                        } while (bVar16);
                                        if (!bVar16) {
                                            *(unsigned short *)(DAT_009a73b6 + 0x18) = uVar10;
                                            uVar3 = (unsigned int)DAT_009a1fe8 >> 0x10;
                                            __builtin_memcpy(&us, (unsigned char *)((unsigned int)uVar10 * 0x23 + DAT_009a1fe8), 2);
                                            *(unsigned short *)(DAT_009a73b6 + 0x1a) = us;
                                            *(unsigned char *)(DAT_009a73b6 + 0x1e) = 1;
                                            return ((uVar3 & 0xffff) << 16) | 1;
                                        }
                                    }
                                }
                                uVar4 = FUN_00404450(uVar10);
                                if ((short)uVar4 != 0) {
                                    *(unsigned short *)(DAT_009a73b6 + 0x18) = uVar10;
                                    uVar3 = (unsigned int)DAT_009a1fe8 >> 0x10;
                                    __builtin_memcpy(&us, (unsigned char *)((unsigned int)uVar10 * 0x23 + DAT_009a1fe8), 2);
                                    *(unsigned short *)(DAT_009a73b6 + 0x1a) = us;
                                    *(unsigned char *)(DAT_009a73b6 + 0x1e) = 1;
                                    return ((uVar3 & 0xffff) << 16) | 1;
                                }
                            }
                        }
                        uVar10 = uVar10 + 1;
                        if (DAT_009a1fe0 <= (int)(unsigned int)uVar10) goto LAB_0046233d;
                    }
                    uVar12 = uVar12 + 1;
                    uVar3 = (unsigned int)uVar12;
                } while ((int)uVar3 < DAT_009a1fe0);
            }
        }
    }
LAB_0046235b:
    return uVar3 & 0xffff0000;
}
