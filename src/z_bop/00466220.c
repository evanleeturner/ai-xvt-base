/* SPDX-License-Identifier: MIT */
/* z_bop 00466220 */
extern unsigned short DAT_009a73b0;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009a73b6;
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73d0;
extern unsigned short DAT_009a73d2;
extern unsigned short DAT_009a8070;
extern unsigned int DAT_009a8c00;
extern unsigned short DAT_009e9640;
extern int DAT_00a00854;
extern unsigned short DAT_00a07cec;
extern unsigned short DAT_00a080f0;
extern unsigned int DAT_00a08104;
extern int DAT_009a1fe0;
extern int DAT_005243c8[];
extern int DAT_005243b8[];
extern char DAT_005243d8[];
extern unsigned char DAT_005243e0[];
extern unsigned char DAT_005243e8[];
extern char DAT_009d8c84[];
extern char DAT_009d9155[];
extern unsigned char DAT_009d8c88[];
extern char DAT_009fd458[];
extern unsigned char DAT_0051c583[];

unsigned char __cdecl FUN_00403360(unsigned int a, int b);
void __cdecl FUN_004034e0(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_0049f360(void);
unsigned int __cdecl FUN_00407090(unsigned int a, unsigned int b);
unsigned int __cdecl FUN_00425cb0(void);

unsigned int __cdecl FUN_00466220(void)
{
    signed char cVar1;
    int iVar2;
    char *pcVar3;
    unsigned char bVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned char uVar7;
    unsigned short uVar8;
    unsigned int uVar9;
    unsigned short uVar10;
    unsigned short uVar11;
    unsigned int uVar12;
    unsigned int t;
    unsigned int t2;
    unsigned short s;
    unsigned short s2;
    int iv;

    uVar6 = (unsigned int)DAT_009a73b0;
    cVar1 = *(signed char *)(DAT_009a1fe8 + 2 + uVar6 * 0x23);
    if ((cVar1 != '\0') && (cVar1 != '\x01')) {
        return DAT_009a1fe8 & 0xffff0000;
    }
    uVar12 = (unsigned int)DAT_009a73d2;
    __builtin_memcpy(&t, &DAT_009a73b6, 4);
    if (*(unsigned char *)(t + 0x34) == uVar12) {
        if (*(short *)(DAT_00a08104 + 0x6a) == -1) {
            iVar2 = DAT_005243c8[(unsigned int)DAT_009a73d0];
            uVar11 = (unsigned short)DAT_009e9640;
            if ((int)(unsigned int)(unsigned short)DAT_009e9640 < DAT_00a00854) {
                do {
                    iVar5 = (int)((unsigned int)uVar11 * 0x23 + DAT_009a1fe8);
                    cVar1 = *(signed char *)(iVar5 + 3);
                    if (cVar1 != '\0') {
                        __builtin_memcpy(&t, (void *)(iVar5 + 0x1f), 4);
                        __builtin_memcpy(&iv, (void *)(t + 0xa5), 4);
                        iVar5 = iv;
                        if ((*(char *)(iVar5 + 1) != '\0') &&
                            (*(unsigned short *)(iVar5 + 4) == DAT_009a73b0)) {
                            iVar5 = (int)((unsigned int)iVar2 * 3u);
                            if (cVar1 != -0x70) {
                                iVar5 = iVar2;
                            }
                            bVar4 = FUN_00403360((unsigned int)uVar11, iVar5);
                            if ((unsigned int)bVar4 == 1) {
                                *(unsigned short *)(DAT_00a08104 + 0x6a) = uVar11;
                                FUN_004034e0((unsigned int)DAT_009a73b0, (unsigned int)uVar11);
                                __builtin_memcpy(&s, (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x10 + DAT_009a1fe8), 2);
                                if (DAT_005243d8[(unsigned short)(DAT_009a8070 - (short)s) >> 0xd] == '\0') {
                                    __builtin_memcpy(&t, &DAT_009a73b6, 4);
                                    *(unsigned char *)(t + 0x34) = 0x18;
                                } else {
                                    __builtin_memcpy(&t, &DAT_009a73b6, 4);
                                    *(unsigned char *)(t + 0x34) = 1;
                                }
                                FUN_0049f360();
                                uVar6 = DAT_00a08104;
                                if ((*(char *)(DAT_00a08104 + 0x246) != '\0') &&
                                    (*(char *)(DAT_00a08104 + 0x247) != '\0')) {
                                    if ((*(char *)(DAT_00a08104 + 0x246) == '\x01') &&
                                        ((*(unsigned char *)(DAT_00a08104 + 0x1e5) & 2) != 0)) {
                                        *(unsigned short *)(DAT_00a08104 + 0x248) =
                                            (unsigned short)(*(unsigned short *)(DAT_00a08104 + 0x248) + 10);
                                        uVar12 = DAT_00a08104;
                                        uVar6 = (unsigned int)DAT_009a73c0 * 0x566;
                                        if ((DAT_009d8c84[uVar6] != '\x15') && (DAT_009d9155[uVar6] != '\x15')) {
                                            *(char *)(DAT_00a08104 + 0x247) =
                                                (char)(*(char *)(DAT_00a08104 + 0x247) + -1);
                                            return uVar12 & 0xffff0000;
                                        }
                                    } else if (*(short *)(DAT_00a08104 + 0x24a) == 0) {
                                        uVar6 = FUN_00407090((unsigned int)DAT_009a73b0, 0x9b);
                                    }
                                }
                                return uVar6 & 0xffff0000;
                            }
                        }
                    }
                    uVar11 = (unsigned short)(uVar11 + 1);
                } while ((int)(unsigned int)uVar11 < DAT_00a00854);
            }
            iVar2 = DAT_005243b8[(unsigned int)DAT_009a73d0];
            if ((*(short *)(DAT_00a08104 + 0x6a) == -1) &&
                (uVar11 = DAT_00a07cec, (int)(unsigned int)DAT_00a07cec < DAT_009a1fe0)) {
                do {
                    uVar12 = (unsigned int)uVar11;
                    iVar5 = (int)(DAT_009a1fe8 + uVar12 * 0x23);
                    __builtin_memcpy(&iv, (void *)(iVar5 + 0x1b), 4);
                    if ((iv == -1) && (*(char *)(iVar5 + 3) != '\0')) {
                        __builtin_memcpy(&t, (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                        uVar9 = (unsigned int)*(unsigned char *)(t + 0x89);
                        if (DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar5 + 0x16) * 0x566] == uVar9) {
                            bVar4 = 0;
                        } else {
                            bVar4 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar5 + 0x16) * 0x566] +
                                                 uVar9 * 0x1e5] == '\0';
                        }
                        if ((((bVar4) && (*(char *)(DAT_00a08104 + 7) == '\0')) &&
                             (*(char *)(iVar5 + 2) == '\0')) &&
                            (bVar4 = FUN_00403360(uVar12, iVar2), (unsigned int)bVar4 == 1)) {
                            FUN_004034e0(uVar12, uVar6);
                            iVar5 = (int)(DAT_009a1fe8 + uVar12 * 0x23);
                            __builtin_memcpy(&s, (void *)(iVar5 + 0x10), 2);
                            uVar8 = (unsigned short)(DAT_009a8070 - (short)s);
                            if (0x7fff < uVar8) {
                                uVar8 = (unsigned short)(-(int)uVar8);
                            }
                            __builtin_memcpy(&s2, (void *)(iVar5 + 0x12), 2);
                            uVar10 = (unsigned short)(DAT_00a080f0 - (short)s2);
                            if (0x7fff < uVar10) {
                                uVar10 = (unsigned short)(-(int)uVar10);
                            }
                            if ((uVar8 < 0x2000) && (uVar10 < 0x2000)) {
                                *(unsigned short *)(DAT_00a08104 + 0x6a) = uVar11;
                                break;
                            }
                        }
                    }
                    uVar11 = (unsigned short)(uVar11 + 1);
                } while ((int)(unsigned int)uVar11 < DAT_009a1fe0);
            }
        }
        uVar11 = *(unsigned short *)(DAT_00a08104 + 0x6a);
        uVar12 = (DAT_00a08104 & 0xffff0000) | (unsigned int)uVar11;
        if (uVar11 != 0xffff) {
            FUN_004034e0(uVar6, (unsigned int)uVar11);
            __builtin_memcpy(&s, (void *)(DAT_009a1fe8 + 0x10 + uVar6 * 0x23), 2);
            cVar1 = DAT_005243d8[(unsigned short)(DAT_009a8070 - (short)s) >> 0xd];
            __builtin_memcpy(&uVar11, &DAT_0051c583[(unsigned int)*(unsigned char *)(DAT_00a08104 + 4) * 0x101], 2);
            __builtin_memcpy(&t, (void *)((unsigned int)*(unsigned short *)(DAT_00a08104 + 0x6a) * 0x23 + 0x1f + DAT_009a1fe8), 4);
            pcVar3 = (char *)t;
            if (*pcVar3 == '\0') {
                __builtin_memcpy(&t2, pcVar3 + 0xa9, 4);
                __builtin_memcpy(&uVar8, &DAT_0051c583[(unsigned int)*(unsigned char *)(t2 + 4) * 0x101], 2);
            } else {
                uVar8 = 900;
            }
            uVar6 = FUN_00425cb0();
            if (cVar1 == '\x01') {
                if ((DAT_009a8c00 < 0x2000) && ((unsigned short)uVar6 < 0x4000)) {
                    uVar7 = 1;
                } else {
                    uVar7 = DAT_005243e0[uVar6 & 3];
                }
            } else if (cVar1 == '\0') {
                if ((unsigned short)uVar6 < 0x8001) {
                    uVar7 = DAT_005243e0[uVar6 & 3];
                } else {
                    uVar7 = 9;
                }
            } else if (uVar11 < uVar8) {
                uVar7 = DAT_005243e8[uVar6 & 7];
            } else {
                uVar7 = 0x10;
                if (DAT_009a8c00 < 0x8001) {
                    uVar7 = DAT_005243e8[uVar6 & 7];
                }
            }
            __builtin_memcpy(&t, &DAT_009a73b6, 4);
            *(unsigned char *)(t + 0x34) = uVar7;
            uVar12 = FUN_0049f360();
        }
    }
    return uVar12 & 0xffff0000;
}
