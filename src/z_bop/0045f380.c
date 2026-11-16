/* SPDX-License-Identifier: MIT */
/* z_bop 0045f380 */
/* Unresolved: calling conventions and return types of the callees are not in the
   evidence. FUN_004035d0 and FUN_00403540 are assumed void. FUN_00425cb0 is assumed
   to return a byte. The globals' types come from Ghidra's usage.
   DAT_009a73b6 sits at an unaligned address, so it is declared as bytes and read by memcpy. */
extern unsigned int DAT_00a08104;
extern unsigned short DAT_009a73b0;
extern unsigned int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1ff8;
extern int DAT_009a73c4;
extern int DAT_009a73c8;
extern int DAT_009a73cc;
extern int DAT_00a08144;
extern int DAT_00a08140;
extern int DAT_00a08148;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern unsigned short DAT_009e9640;
extern int DAT_00a00854;
extern unsigned short DAT_00a07cec;
extern unsigned int DAT_009a8e18;
extern unsigned char DAT_009a73b6[4];
extern unsigned char DAT_0051c5aa[];
extern unsigned char DAT_0051c5ac[];
extern unsigned char DAT_0051c5b2[];
extern unsigned char DAT_0051c5b4[];
extern unsigned char DAT_0051c5b6[];
extern unsigned char DAT_0051c5b9[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009fd458[];
extern unsigned char DAT_009e968a[];

void FUN_004035d0(unsigned int, short, short, short);
unsigned int FUN_0041f410(int, int, int);
void FUN_00403540(unsigned int, unsigned int);
unsigned int FUN_0045f1e0(unsigned int);
int FUN_00406290(unsigned int, unsigned int, unsigned int, int);
unsigned char FUN_00425cb0(void);

#define S8(a) (*(char *)(unsigned int)(a))
#define U8(a) (*(unsigned char *)(unsigned int)(a))
#define U16(a) (*(unsigned short *)(unsigned int)(a))

unsigned int FUN_0045f380(void)
{
    char cVar1;
    unsigned short uVar2;
    unsigned char bVar3;
    int iVar4;
    unsigned short *puVar5;
    int iVar6;
    unsigned int uVar7;
    unsigned int uVar8;
    unsigned short uVar9;
    unsigned short uVar10;
    unsigned int uVar11;
    unsigned short uVar12;
    short sVar13;
    int bVar14;
    unsigned short local_1a;
    unsigned short uStack_18;
    unsigned short local_16;
    unsigned int local_10;
    unsigned short t16;
    short s1, s2, s3;
    int ta, tb, tc;
    unsigned int tp1;
    unsigned int t_b6;

    if (S8(DAT_00a08104 + 7) == '\x03') {
        return DAT_00a08104 & 0xffff0000;
    }
    __builtin_memcpy(&t16, (void *)(DAT_00a08104 + 0x1e5), 2);
    if (t16 == 0) {
        return DAT_00a08104 & 0xffff0000;
    }
    __builtin_memcpy(&t16, (void *)(DAT_00a08104 + 0x1e7), 2);
    if (t16 != 0) {
        return DAT_00a08104 & 0xffff0000;
    }
    if (S8((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8) == '\0') {
        return DAT_009a1fe8 & 0xffff0000;
    }
    local_1a = 0;
    uVar8 = DAT_00a08104;
    if (S8(DAT_00a08104 + 0x232) != '\0') {
        do {
            iVar4 = (int)((unsigned int)local_1a + (unsigned int)U8(DAT_00a08104 + 4) * 0x101);
            uStack_18 = (unsigned short)DAT_0051c5aa[iVar4];
            if (uStack_18 < (unsigned short)(DAT_0051c5ac[iVar4] + 1)) {
                do {
                    uVar8 = (unsigned int)uStack_18;
                    if (S8(DAT_0051c5b9[uVar8 * 8 + (unsigned int)U8(DAT_00a08104 + 4) * 0x101]
                           + 0x2d8 + DAT_00a08104) == '\0') {
                        iVar4 = (int)(DAT_00a08104 + uVar8 * 4);
                        if (((S8(iVar4 + 0x370) == -0x70) || (S8(iVar4 + 0x370) == -0x6b)) &&
                            (S8(iVar4 + 0x372) != '\0')) {
                            if (S8(iVar4 + 0x373) == '\0') {
                                U16(iVar4 + 0x3b2) = 0xffff;
                                puVar5 = (unsigned short *)(iVar4 + 0x3b2);
                                DAT_00a08144 = DAT_009a73c4;
                                DAT_00a08140 = DAT_009a73c8;
                                DAT_00a08148 = DAT_009a73cc;
                                iVar6 = (int)((unsigned int)U8(DAT_00a08104 + 4) * 0x101);
                                iVar4 = (int)((unsigned int)iVar6 + uVar8 * 8);
                                __builtin_memcpy(&s1, &DAT_0051c5b2[iVar4], 2);
                                __builtin_memcpy(&s2, &DAT_0051c5b4[iVar4], 2);
                                __builtin_memcpy(&s3, &DAT_0051c5b6[uVar8 * 8 + (unsigned int)iVar6], 2);
                                FUN_004035d0((unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8,
                                             s1, s2, s3);
                                if (S8((unsigned int)DAT_009a73b0 * 0x23 + 3 + DAT_009a1fe8) == '5') {
                                    DAT_009d8a58 = (int)((unsigned int)DAT_009d8a58 * 2);
                                    DAT_009d8a54 = (int)((unsigned int)DAT_009d8a54 * 2);
                                    DAT_009d8b60 = (int)((unsigned int)DAT_009d8b60 * 2);
                                }
                                DAT_00a08144 = (int)((unsigned int)DAT_00a08144 + (unsigned int)DAT_009d8a58);
                                DAT_00a08140 = (int)((unsigned int)DAT_00a08140 + (unsigned int)DAT_009d8a54);
                                DAT_00a08148 = (int)((unsigned int)DAT_00a08148 + (unsigned int)DAT_009d8b60);
                                local_10 = 0x40000;
                                local_16 = 0xffff;
                                uVar12 = (unsigned short)DAT_009e9640;
                                if ((int)(unsigned int)(unsigned short)DAT_009e9640 < DAT_00a00854) {
                                    do {
                                        iVar4 = (int)((unsigned int)uVar12 * 0x23 + DAT_009a1fe8);
                                        if ((S8(iVar4 + 3) != '\0') &&
                                            (iVar6 = (int)(DAT_009a8e18 +
                                                           (unsigned int)(unsigned short)(uVar12 - (unsigned short)DAT_009e9640) * 10),
                                             S8(iVar6 + 1) != '\0') &&
                                            (U16(iVar6 + 4) == DAT_009a73b0)) {
                                            sVar13 = 0;
                                            for (uVar9 = (unsigned short)DAT_009e9640;
                                                 (int)(unsigned int)uVar9 < DAT_00a00854;
                                                 uVar9 = uVar9 + 1) {
                                                if ((S8((unsigned int)uVar9 * 0x23 + 3 + DAT_009a1fe8) != '\0') &&
                                                    (uVar9 != uVar12) &&
                                                    (iVar6 = (int)(DAT_009a8e18 +
                                                                   (unsigned int)(unsigned short)(uVar9 - (unsigned short)DAT_009e9640) * 10),
                                                     4 < U8(iVar6 + 1) && (U16(iVar6 + 4) == uVar12))) {
                                                    sVar13 = sVar13 + 1;
                                                }
                                            }
                                            if (sVar13 == 0) {
                                                __builtin_memcpy(&ta, (void *)(iVar4 + 4), 4);
                                                __builtin_memcpy(&tb, (void *)(iVar4 + 8), 4);
                                                __builtin_memcpy(&tc, (void *)(iVar4 + 0xc), 4);
                                                DAT_009a1ff8 = FUN_0041f410(
                                                    (int)((unsigned int)ta - (unsigned int)DAT_00a08144),
                                                    (int)((unsigned int)tb - (unsigned int)DAT_00a08140),
                                                    (int)((unsigned int)tc - (unsigned int)DAT_00a08148));
                                                if ((0x4000 < (int)DAT_009a1ff8) && (DAT_009a1ff8 < local_10)) {
                                                    local_16 = uVar12;
                                                    local_10 = DAT_009a1ff8;
                                                }
                                            }
                                        }
                                        uVar12 = uVar12 + 1;
                                    } while ((int)(unsigned int)uVar12 < DAT_00a00854);
                                }
                                if (local_16 == 0xffff) {
                                    local_10 = 0x40000;
                                    local_16 = 0xffff;
                                    uVar12 = DAT_00a07cec;
                                    if ((int)(unsigned int)DAT_00a07cec < DAT_009a1fe0) {
                                        do {
                                            uVar7 = (unsigned int)uVar12;
                                            iVar4 = (int)(DAT_009a1fe8 + uVar7 * 0x23);
                                            if (S8(iVar4 + 3) != '\0') {
                                                __builtin_memcpy(&tp1, (void *)(iVar4 + 0x1f), 4);
                                                __builtin_memcpy(&iVar6, (void *)(tp1 + 0xa9), 4);
                                                if ((((U16(iVar6 + 0x42) == DAT_009a73b0) &&
                                                      ((S8(iVar6 + 0x5e) == '\f') ||
                                                       (S8(iVar6 + 0x5e) == '\x17'))) ||
                                                     (U16(iVar6 + 0x6a) == uVar12)) ||
                                                    (U16(iVar6 + 0x6e) == uVar12)) {
LAB_0045f8b0:
                                                    if (uVar12 == 0xffff) {
                                                        iVar4 = 0;
                                                    } else {
                                                        iVar4 = (int)(DAT_009a1fe8 + uVar7 * 0x23);
                                                        __builtin_memcpy(&tp1, (void *)(iVar4 + 0x1f), 4);
                                                        if (tp1 == 0) {
                                                            bVar14 = S8(iVar4 + 3) == '\0';
LAB_0045f9e2:
                                                            iVar4 = (int)(1u - (unsigned int)bVar14);
                                                        } else if (S8(iVar4 + 3) == '\0') {
                                                            iVar4 = 0;
                                                        } else {
                                                            __builtin_memcpy(&tp1, (void *)(iVar4 + 0x1f), 4);
                                                            __builtin_memcpy(&iVar4, (void *)(tp1 + 0xa9), 4);
                                                            if (iVar4 == 0) {
                                                                iVar4 = 1;
                                                            } else if ((S8(iVar4 + 0x5e) == '\x15') &&
                                                                       (S8(iVar4 + 0x5f) != '\0')) {
                                                                iVar4 = 0;
                                                            } else {
                                                                cVar1 = S8(iVar4 + 7);
                                                                if ((cVar1 == '\x01') || ((cVar1 == '\x03') || (cVar1 == '\x04'))) {
                                                                    iVar4 = 0;
                                                                } else if ((S8(iVar4 + 0x23d) == '\x03') &&
                                                                           (S8(iVar4 + 0x241) != '\0')) {
                                                                    FUN_00403540(uVar7, (unsigned int)DAT_009a73b0);
                                                                    cVar1 = S8((unsigned int)DAT_009a73b0 * 0x23 + 2 + DAT_009a1fe8);
                                                                    if ((cVar1 == '\0') || ((cVar1 == '\x01') || (cVar1 == '\x02'))) {
                                                                        if ((int)DAT_009a1ff8 < 0x8001) goto LAB_0045f9a6;
                                                                        iVar4 = 0;
                                                                    } else {
                                                                        if ((int)DAT_009a1ff8 < 0x10001) goto LAB_0045f9a6;
                                                                        iVar4 = 0;
                                                                    }
                                                                } else {
LAB_0045f9a6:
                                                                    iVar6 = (int)(DAT_009a1fe8 + uVar7 * 0x23);
                                                                    __builtin_memcpy(&ta, (void *)(iVar6 + 0x1b), 4);
                                                                    if (ta == -1) {
                                                                        __builtin_memcpy(&tp1, (void *)(iVar6 + 0x1f), 4);
                                                                        __builtin_memcpy(&tb, (void *)(tp1 + 0xa9), 4);
                                                                        __builtin_memcpy(&tp1, (void *)(tb + 0xcb), 4);
                                                                        __builtin_memcpy(&uVar11, (void *)(iVar4 + 0xc3), 4);
                                                                        bVar14 = tp1 < uVar11;
                                                                        goto LAB_0045f9e2;
                                                                    }
                                                                    iVar4 = 1;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if (iVar4 != 0) {
                                                        uVar9 = 0;
                                                        for (uVar10 = (unsigned short)DAT_009e9640;
                                                             (int)(unsigned int)uVar10 < DAT_00a00854;
                                                             uVar10 = uVar10 + 1) {
                                                            if ((((S8((unsigned int)uVar10 * 0x23 + 3 + DAT_009a1fe8) != '\0') &&
                                                                  (uVar10 != uVar12)) &&
                                                                 (iVar4 = (int)(DAT_009a8e18 +
                                                                                (unsigned int)(unsigned short)(uVar10 - (unsigned short)DAT_009e9640) * 10),
                                                                  S8(iVar4 + 1) != '\0')) &&
                                                                (U16(iVar4 + 4) == uVar12)) {
                                                                uVar9 = uVar9 + 1;
                                                            }
                                                        }
                                                        if (uVar9 < 2) {
                                                            iVar4 = (int)(DAT_009a1fe8 + uVar7 * 0x23);
                                                            __builtin_memcpy(&ta, (void *)(iVar4 + 4), 4);
                                                            __builtin_memcpy(&tb, (void *)(iVar4 + 8), 4);
                                                            __builtin_memcpy(&tc, (void *)(iVar4 + 0xc), 4);
                                                            DAT_009a1ff8 = FUN_0041f410(
                                                                (int)((unsigned int)ta - (unsigned int)DAT_00a08144),
                                                                (int)((unsigned int)tb - (unsigned int)DAT_00a08140),
                                                                (int)((unsigned int)tc - (unsigned int)DAT_00a08148));
                                                            if (DAT_009a1ff8 < local_10) {
                                                                local_16 = uVar12;
                                                                local_10 = DAT_009a1ff8;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    __builtin_memcpy(&ta, (void *)(iVar4 + 0x1b), 4);
                                                    if (ta != -1) {
                                                        __builtin_memcpy(&tp1, (void *)((unsigned int)DAT_009a73b0 * 0x23 + 0x1f + DAT_009a1fe8), 4);
                                                        uVar11 = (unsigned int)U8(tp1 + 0x89);
                                                        if (uVar11 == DAT_009d8c88[(unsigned int)U8(iVar4 + 0x16) * 0x566]) {
                                                            bVar14 = 0;
                                                        } else {
                                                            bVar14 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)U8(iVar4 + 0x16) * 0x566] +
                                                                                  uVar11 * 0x1e5] == '\0';
                                                        }
                                                        if (bVar14) {
                                                            __builtin_memcpy(&ta, (void *)(iVar4 + 0x1b), 4);
                                                            __builtin_memcpy(&t16, &DAT_009e968a[(unsigned int)ta * 0x5bd], 2);
                                                            if (t16 == DAT_009a73b0) goto LAB_0045f8b0;
                                                        }
                                                    }
                                                }
                                            }
                                            uVar12 = uVar12 + 1;
                                        } while ((int)(unsigned int)uVar12 < DAT_009a1fe0);
                                    }
                                } else {
                                    *puVar5 = local_16;
                                }
                                if (local_16 != 0xffff) {
                                    *puVar5 = local_16;
                                }
                                if (*puVar5 != 0xffff) {
                                    __builtin_memcpy(&t_b6, DAT_009a73b6, 4);
                                    uVar2 = U16(t_b6 + 0x18);
                                    __builtin_memcpy(&t_b6, DAT_009a73b6, 4);
                                    U16(t_b6 + 0x18) = *puVar5;
                                    __builtin_memcpy(&t_b6, DAT_009a73b6, 4);
                                    uVar7 = FUN_0045f1e0(((t_b6 >> 0x10) << 0x10) |
                                                         (unsigned int)U16(t_b6 + 0x18));
                                    __builtin_memcpy(&t_b6, DAT_009a73b6, 4);
                                    U16(t_b6 + 0x1c) = (unsigned short)uVar7;
                                    iVar4 = FUN_00406290((unsigned int)DAT_009a73b0, uVar8,
                                                         (unsigned int)U8(DAT_00a08104 + 0x370 + uVar8 * 4), 0xffff);
                                    if (iVar4 != -1) {
                                        bVar3 = FUN_00425cb0();
                                        U8(DAT_009a8e18 + 1 + (unsigned int)iVar4 * 10) = (unsigned char)((bVar3 & 3) + 3);
                                        U8(DAT_00a08104 + 0x373 + uVar8 * 4) = 0x14;
                                    }
                                    __builtin_memcpy(&t_b6, DAT_009a73b6, 4);
                                    U16(t_b6 + 0x18) = uVar2;
                                }
                            } else {
                                S8(iVar4 + 0x373) = (char)(S8(iVar4 + 0x373) + -1);
                            }
                        }
                    }
                    uStack_18 = uStack_18 + 1;
                } while (uStack_18 <
                         (unsigned short)(DAT_0051c5ac[(unsigned int)local_1a + (unsigned int)U8(DAT_00a08104 + 4) * 0x101] + 1));
            }
            local_1a = local_1a + 1;
            uVar8 = (unsigned int)local_1a;
        } while (uVar8 < U8(DAT_00a08104 + 0x232));
    }
    return uVar8 & 0xffff0000;
}
