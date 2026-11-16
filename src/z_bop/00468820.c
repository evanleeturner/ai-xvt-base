/* SPDX-License-Identifier: MIT */
/* z_bop 00468820 */
/* Globals: types inferred from the decompile only. */
extern unsigned short DAT_009a73b0;
extern unsigned int   DAT_009a1fe8;
/* 0x9a73b6 is not 4-aligned: declared as raw bytes, read via memcpy at every use. */
extern unsigned char  DAT_009a73b6[4];
extern unsigned short DAT_009a73d2;
extern unsigned short DAT_009a73d0;
extern unsigned int   DAT_00a08104;
extern int            DAT_005243c8[];
extern int            DAT_005243b8[];
extern unsigned char  DAT_005243d8[];
extern unsigned short DAT_009e9640;
extern int            DAT_00a00854;
extern unsigned short DAT_009a8070;
extern unsigned short DAT_009a73c0;
extern unsigned char  DAT_009d8c84[];
extern unsigned char  DAT_009d9155[];
extern unsigned char  DAT_009d8c88[];
extern unsigned char  DAT_009fd458[];
extern unsigned short DAT_00a07cec;
extern int            DAT_009a1fe0;
extern unsigned short DAT_00a080f0;
extern unsigned short DAT_009a8c00;

/* Prototypes: calling conventions not confirmed by evidence. */
unsigned char FUN_00403360(unsigned int, int);
void          FUN_004034e0(unsigned int, unsigned int);
void          FUN_0049f360(void);
unsigned int  FUN_00407090(unsigned int, unsigned int);
unsigned int  FUN_004a4d70(unsigned int, unsigned int);

unsigned int __stdcall FUN_00468820(void)
{
    char cVar1;
    short sVar2;
    int iVar3;
    unsigned char bVar4;
    unsigned short uVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned short uVar8;
    unsigned short uVar9;
    unsigned int uVar10;
    /* memcpy targets (not Ghidra placeholders) */
    unsigned int uTmp;
    unsigned int uTmp2;
    int iTmp;
    int iCond;
    unsigned int uBase;
    unsigned short sTmp;

    cVar1 = *(char *)((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
    if ((cVar1 == '\x03') || (cVar1 == '\x04')) {
        return DAT_009a1fe8 & 0xffff0000;
    }
    uVar10 = (unsigned int)DAT_009a73d2;
    __builtin_memcpy(&uBase, DAT_009a73b6, 4);
    if ((unsigned int)*(unsigned char *)(uBase + 0x34) == (unsigned int)DAT_009a73d2) {
        if (*(short *)(DAT_00a08104 + 0x6a) == -1) {
            iVar3 = DAT_005243c8[(unsigned int)DAT_009a73d0];
            uVar9 = DAT_009e9640;
            if ((int)(unsigned int)DAT_009e9640 < DAT_00a00854) {
                do {
                    iVar6 = (int)((unsigned int)uVar9 * 0x23 + DAT_009a1fe8);
                    cVar1 = *(char *)(iVar6 + 3);
                    if (cVar1 != '\0') {
                        __builtin_memcpy(&uTmp, (void *)(iVar6 + 0x1f), 4);
                        __builtin_memcpy(&uTmp2, (void *)(uTmp + 0xa5), 4);
                        iVar6 = (int)uTmp2;
                        if ((*(char *)(iVar6 + 1) != '\0') &&
                            (*(unsigned short *)(iVar6 + 4) == DAT_009a73b0)) {
                            iVar6 = iVar3;
                            if ((cVar1 == -0x70) || (cVar1 == -0x6b)) {
                                iVar6 = (int)((unsigned int)iVar3 * 3u);
                            }
                            bVar4 = FUN_00403360((unsigned int)uVar9, iVar6);
                            if (bVar4 == 1) {
                                *(unsigned short *)(DAT_00a08104 + 0x6a) = uVar9;
                                FUN_004034e0((unsigned int)DAT_009a73b0, (unsigned int)uVar9);
                                __builtin_memcpy(&sTmp,
                                    (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x10 + DAT_009a1fe8), 2);
                                if (DAT_005243d8[(unsigned short)(DAT_009a8070 - sTmp) >> 0xd] == '\0') {
                                    __builtin_memcpy(&uBase, DAT_009a73b6, 4);
                                    *(unsigned char *)(uBase + 0x34) = 1;
                                }
                                else {
                                    __builtin_memcpy(&uBase, DAT_009a73b6, 4);
                                    *(unsigned char *)(uBase + 0x34) = 0x20;
                                }
                                FUN_0049f360();
                                uVar10 = DAT_00a08104;
                                if (*(char *)(DAT_00a08104 + 0x247) != '\0') {
                                    if ((*(char *)(DAT_00a08104 + 0x246) == '\x01') &&
                                        ((*(unsigned char *)(DAT_00a08104 + 0x1e5) & 2) != 0)) {
                                        sVar2 = *(short *)(DAT_00a08104 + 0x248);
                                        uVar10 = (DAT_00a08104 & 0xffff0000) | (unsigned int)(unsigned short)sVar2;
                                        if (sVar2 == 0) {
                                            *(short *)(DAT_00a08104 + 0x248) = 10;
                                            uVar7 = DAT_00a08104;
                                            uVar10 = (unsigned int)DAT_009a73c0 * 0x566;
                                            if ((DAT_009d8c84[uVar10] != 0x15) &&
                                                (DAT_009d9155[uVar10] != 0x15)) {
                                                *(char *)(DAT_00a08104 + 0x247) =
                                                    (char)(*(char *)(DAT_00a08104 + 0x247) + -1);
                                                return uVar7 & 0xffff0000;
                                            }
                                        }
                                    }
                                    else if ((*(char *)(DAT_00a08104 + 0x246) == '\x02') &&
                                             (*(short *)(DAT_00a08104 + 0x24a) == 0)) {
                                        uVar10 = FUN_00407090((unsigned int)DAT_009a73b0, 0x9b);
                                    }
                                }
                                return uVar10 & 0xffff0000;
                            }
                        }
                    }
                    uVar9 = (unsigned short)(uVar9 + 1);
                } while ((int)(unsigned int)uVar9 < DAT_00a00854);
            }
            iVar3 = DAT_005243b8[(unsigned int)DAT_009a73d0];
            if ((*(short *)(DAT_00a08104 + 0x6a) == -1) &&
                (uVar9 = DAT_00a07cec, (int)(unsigned int)DAT_00a07cec < DAT_009a1fe0)) {
                do {
                    uVar10 = (unsigned int)uVar9;
                    iVar6 = (int)(DAT_009a1fe8 + uVar10 * 0x23);
                    if (*(char *)(iVar6 + 3) != '\0') {
                        __builtin_memcpy(&uTmp,
                            (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                        uVar7 = (unsigned int)*(unsigned char *)(uTmp + 0x89);
                        if ((unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar6 + 0x16) * 0x566] == uVar7) {
                            bVar4 = 0;
                        }
                        else {
                            bVar4 = (DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(iVar6 + 0x16) * 0x566] +
                                                  uVar7 * 0x1e5] == '\0');
                        }
                        if ((((bVar4) && (*(char *)(DAT_00a08104 + 7) == '\0')) &&
                             (*(char *)(iVar6 + 2) == '\0')) &&
                            (bVar4 = FUN_00403360(uVar10, iVar3), bVar4 == 1)) {
                            FUN_004034e0(uVar10, (unsigned int)DAT_009a73b0);
                            iVar6 = (int)(DAT_009a1fe8 + uVar10 * 0x23);
                            __builtin_memcpy(&sTmp, (void *)(iVar6 + 0x10), 2);
                            uVar5 = (unsigned short)(DAT_009a8070 - sTmp);
                            if (0x7fff < uVar5) {
                                uVar5 = (unsigned short)(0u - uVar5);
                            }
                            __builtin_memcpy(&sTmp, (void *)(iVar6 + 0x12), 2);
                            uVar8 = (unsigned short)(DAT_00a080f0 - sTmp);
                            if (0x7fff < uVar8) {
                                uVar8 = (unsigned short)(0u - uVar8);
                            }
                            if ((uVar5 < 0x2000) && (uVar8 < 0x2000)) {
                                *(unsigned short *)(DAT_00a08104 + 0x6a) = uVar9;
                                break;
                            }
                        }
                    }
                    uVar9 = (unsigned short)(uVar9 + 1);
                } while ((int)(unsigned int)uVar9 < DAT_009a1fe0);
            }
        }
        uVar10 = DAT_00a08104;
        if (*(unsigned short *)(DAT_00a08104 + 0x6a) != 0xffff) {
            uVar7 = (unsigned int)DAT_009a73b0;
            cVar1 = *(char *)(uVar7 * 0x23 + 2 + DAT_009a1fe8);
            uVar10 = (((uVar7 * 7 >> 8) & 0xffffff) << 8) | (unsigned int)(unsigned char)cVar1;
            if (cVar1 != '\x02') {
                iCond = 0;
                if ((*(unsigned char *)(DAT_00a08104 + 0x1e5) & 1) != 0) {
                    __builtin_memcpy(&iTmp, (void *)(DAT_00a08104 + 0x211), 4);
                    if (iTmp < 500) {
                        iCond = 1;
                    }
                }
                if (!iCond) {
                    uVar10 = DAT_00a08104;
                    __builtin_memcpy(&uTmp, (void *)(DAT_00a08104 + 199), 4);
                    __builtin_memcpy(&uTmp2, (void *)(DAT_00a08104 + 0xc3), 4);
                    if (uTmp <= uTmp2) {
                        iCond = 1;
                    }
                }
                if (iCond) {
                    FUN_004034e0((unsigned int)*(unsigned short *)(DAT_00a08104 + 0x6a), uVar7);
                    if ((((DAT_009a8c00 < 0x8000) && (*(char *)(DAT_00a08104 + 0x246) == '\x02')) &&
                         (*(char *)(DAT_00a08104 + 0x247) != '\0')) &&
                        (*(short *)(DAT_00a08104 + 0x24a) == 0)) {
                        FUN_00407090((unsigned int)DAT_009a73b0, 0x9b);
                    }
                    uVar10 = DAT_009a1fe8;
                    __builtin_memcpy(&iTmp,
                        (void *)((unsigned int)*(unsigned short *)(DAT_00a08104 + 0x6a) * 0x23 + 0x1b + DAT_009a1fe8), 4);
                    if (iTmp != -1) {
                        __builtin_memcpy(&uBase, DAT_009a73b6, 4);
                        *(unsigned char *)(uBase + 0x34) = 0x20;
                        FUN_0049f360();
                        uVar10 = FUN_004a4d70((unsigned int)DAT_009a73b0, 0xffff);
                    }
                }
            }
        }
    }
    return uVar10 & 0xffff0000;
}
