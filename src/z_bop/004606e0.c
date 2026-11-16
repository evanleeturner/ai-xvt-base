/* SPDX-License-Identifier: MIT */
/* z_bop 004606e0 */
extern unsigned int DAT_00a08104;
extern unsigned short DAT_009a73b0;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009a73b6;   /* unaligned: accessed via memcpy only */
extern unsigned short DAT_009a73c0;
extern unsigned short DAT_009a73c2;
extern unsigned int DAT_009a73c4;
extern unsigned int DAT_009a73c8;
extern unsigned int DAT_009a73cc;
extern unsigned char DAT_009a73d8;
extern unsigned char DAT_009a73da;
extern unsigned int DAT_009a73db;   /* unaligned */
extern unsigned int DAT_009a73df;   /* unaligned */
extern unsigned int DAT_009a73e3;   /* unaligned */
extern unsigned int DAT_009d8a54;
extern unsigned int DAT_009d8a58;
extern unsigned int DAT_009d8b60;
extern unsigned int DAT_009ec604;
extern unsigned int DAT_00a08140;
extern unsigned int DAT_00a08144;
extern unsigned int DAT_00a08148;
extern unsigned int DAT_00a081e8;
extern unsigned int DAT_00a08298;
extern char DAT_009d8c84[];
extern char DAT_009d9155[];
extern char DAT_0051c5a6[];
extern char DAT_0051c5b2[];
extern char DAT_0051c5b4[];
extern char DAT_0051c5b6[];
extern char DAT_009d1320[];
extern unsigned char DAT_009d8cd8[];
extern unsigned char DAT_009d8cd9[];
extern unsigned char DAT_009d8cda[];
extern unsigned char DAT_009d8cdb[];
extern unsigned char DAT_009d8cdc[];
extern unsigned char DAT_009d8cde[];
extern unsigned char DAT_009d8cdf[];
extern unsigned char DAT_009d8ce0[];
extern unsigned char DAT_009d8ce1[];
extern unsigned char DAT_009d8ce2[];
extern char s_starshipprotectpln_0051a2fc[];
extern char s_starshipescortpln_005242b0[];
extern char s_starshipdisablepln_0051a2d4[];
extern char s_disableldr1pln_0051a310[];

void FUN_00461460(unsigned short a, unsigned short b, unsigned short c,
                  unsigned short d, unsigned short e, unsigned short f);
void FUN_004035d0(unsigned int a, short b, short c, short d);
unsigned short FUN_00460dd0(unsigned short a, unsigned short b, unsigned short c,
                            unsigned short d, unsigned short e, unsigned short f);
unsigned short FUN_0045d5f0(unsigned int a);

unsigned int __stdcall FUN_004606e0(void)
{
    unsigned short *puVar1;
    int bVar2;
    unsigned int pbVar3;
    unsigned int uVar4;
    unsigned int iVar5;
    unsigned int iVar6;
    unsigned short uVar7;
    const char *pcVar8;
    const char *pcVar9;
    int bVar10;
    int bVar11;
    unsigned short local_a;
    short s1, s2, s3;
    unsigned short w1e5;
    unsigned int t;
    unsigned int local_t6;

    bVar2 = 1;
    if (*(char *)(DAT_00a08104 + 7) == 3) {
        return DAT_00a08104 & 0xffff0000;
    }
    __builtin_memcpy(&w1e5, (const void *)(DAT_00a08104 + 0x1e5), 2);
    if (w1e5 == 0) {
        return DAT_00a08104 & 0xffff0000;
    }
    iVar6 = (unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8;
    uVar4 = (unsigned int)*(unsigned char *)(iVar6 + 0x16);
    if (DAT_009d8c84[uVar4 * 0x566] == 0x0e || DAT_009d9155[uVar4 * 0x566] == 0x0e) {
        return (uVar4 * 0x2b3 >> 0xf) << 0x10;
    }
    uVar4 = 0;
    bVar10 = 0;
    do {
        if (DAT_0051c5a6[(unsigned int)*(unsigned char *)(DAT_00a08104 + 4) * 0x101 + uVar4] == 2) {
            bVar10 = 1;
        }
        uVar7 = (unsigned short)(uVar4 + 1);
        uVar4 = (unsigned int)uVar7;
    } while (uVar7 < 2);
    if (!bVar10) {
        return 0;
    }
    DAT_009a73d8 = 0;
    DAT_009a73da = 0x30;
    __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
    pbVar3 = local_t6 + 0xb;
    iVar5 = 0x13;
    bVar10 = 0;
    pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)pbVar3 * 0x55];
    pcVar9 = s_starshipprotectpln_0051a2fc;
    do {
        if (iVar5 == 0) break;
        iVar5 = iVar5 - 1;
        bVar10 = (*pcVar8 == *pcVar9);
        pcVar8 = pcVar8 + 1;
        pcVar9 = pcVar9 + 1;
    } while (bVar10);
    if (!bVar10) {
        iVar5 = 0x12;
        pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)pbVar3 * 0x55];
        pcVar9 = s_starshipescortpln_005242b0;
        do {
            if (iVar5 == 0) break;
            iVar5 = iVar5 - 1;
            bVar10 = (*pcVar8 == *pcVar9);
            pcVar8 = pcVar8 + 1;
            pcVar9 = pcVar9 + 1;
        } while (bVar10);
        if (!bVar10) {
            iVar5 = 0x13;
            pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)pbVar3 * 0x55];
            pcVar9 = s_starshipdisablepln_0051a2d4;
            do {
                if (iVar5 == 0) break;
                iVar5 = iVar5 - 1;
                bVar10 = (*pcVar8 == *pcVar9);
                pcVar8 = pcVar8 + 1;
                pcVar9 = pcVar9 + 1;
            } while (bVar10);
            if (!bVar10) {
                iVar5 = 0xf;
                pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)pbVar3 * 0x55];
                pcVar9 = s_disableldr1pln_0051a310;
                do {
                    if (iVar5 == 0) break;
                    iVar5 = iVar5 - 1;
                    bVar10 = (*pcVar8 == *pcVar9);
                    pcVar8 = pcVar8 + 1;
                    pcVar9 = pcVar9 + 1;
                } while (bVar10);
                if (!bVar10) {
                    DAT_009a73d8 = 0;
                    goto LAB_0046087c;
                }
            }
            DAT_009a73d8 = 1;
        }
    }
LAB_0046087c:
    bVar10 = (*(char *)(iVar6 + 3) == '6');
    local_a = 0;
    uVar4 = DAT_00a08104;
    if (*(char *)(DAT_00a08104 + 0x21d) != 0) {
        do {
            uVar4 = (unsigned int)local_a;
            puVar1 = (unsigned short *)(DAT_00a08104 + 0x3b2 + uVar4 * 4);
            if (*(char *)(DAT_00a08104 + 0x370 + uVar4 * 4) != 2 || *puVar1 != 0xffff
                || 0 < (short)puVar1[1]) {
                goto LAB_00460d90;
            }
            puVar1[1] = (unsigned short)(puVar1[1] + 0xec);
            if (bVar2) {
                iVar6 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
                FUN_00461460((unsigned short)DAT_009d8cde[iVar6],
                             (unsigned short)DAT_009d8cdf[iVar6],
                             (unsigned short)DAT_009d8ce2[iVar6],
                             (unsigned short)DAT_009d8ce0[iVar6],
                             (unsigned short)DAT_009d8ce1[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52],
                             0);
                iVar6 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
                FUN_00461460((unsigned short)DAT_009d8cd8[iVar6],
                             (unsigned short)DAT_009d8cda[iVar6],
                             (unsigned short)DAT_009d8cdc[iVar6],
                             (unsigned short)DAT_009d8cd9[iVar6],
                             (unsigned short)DAT_009d8cdb[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52],
                             1);
                bVar2 = 0;
            }
            DAT_00a08144 = DAT_009a73c4;
            DAT_00a08140 = DAT_009a73c8;
            DAT_00a08148 = DAT_009a73cc;
            if (!bVar10) {
                iVar5 = (unsigned int)*(unsigned char *)(DAT_00a08104 + 4) * 0x101;
                iVar6 = iVar5 + uVar4 * 8;
                __builtin_memcpy(&s1, &DAT_0051c5b2[iVar6], 2);
                __builtin_memcpy(&s2, &DAT_0051c5b4[iVar6], 2);
                __builtin_memcpy(&s3, &DAT_0051c5b6[uVar4 * 8 + iVar5], 2);
                FUN_004035d0((unsigned int)DAT_009a73b0 * 0x23 + DAT_009a1fe8, s1, s2, s3);
                if (*(char *)((unsigned int)DAT_009a73b0 * 0x23 + 3 + DAT_009a1fe8) == '5') {
                    DAT_009d8a58 = DAT_009d8a58 * 2;
                    DAT_009d8a54 = DAT_009d8a54 * 2;
                    DAT_009d8b60 = DAT_009d8b60 * 2;
                }
                DAT_009ec604 = DAT_00a08144 + DAT_009d8a58;
                DAT_00a08140 = DAT_00a08140 + DAT_009d8a54;
                DAT_00a08148 = DAT_00a08148 + DAT_009d8b60;
                DAT_00a08144 = DAT_009ec604;
                DAT_00a081e8 = DAT_00a08140;
                DAT_00a08298 = DAT_00a08148;
            }
            t = DAT_00a08144;
            __builtin_memcpy(&DAT_009a73db, &t, 4);
            t = DAT_00a08140;
            __builtin_memcpy(&DAT_009a73df, &t, 4);
            t = DAT_00a08148;
            __builtin_memcpy(&DAT_009a73e3, &t, 4);
            *(unsigned char *)(DAT_00a08104 + 0x372 + uVar4 * 4) = 0;
            __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
            pbVar3 = local_t6 + 0xb;
            iVar6 = 0x13;
            bVar11 = 0;
            pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)pbVar3 * 0x55];
            pcVar9 = s_starshipprotectpln_0051a2fc;
            do {
                if (iVar6 == 0) break;
                iVar6 = iVar6 - 1;
                bVar11 = (*pcVar8 == *pcVar9);
                pcVar8 = pcVar8 + 1;
                pcVar9 = pcVar9 + 1;
            } while (bVar11);
            if (bVar11) {
LAB_00460d72:
                uVar7 = FUN_0045d5f0(((pbVar3 >> 0x10) << 0x10) | (unsigned int)DAT_009a73c2);
                *puVar1 = uVar7;
                if (uVar7 != 0xffff) {
                    puVar1[1] = (unsigned short)(puVar1[1] + 0xec);
                }
            } else {
                pbVar3 = (unsigned int)*(unsigned char *)pbVar3;
                iVar6 = 0x12;
                bVar11 = 0;
                pcVar8 = &DAT_009d1320[pbVar3 * 0x55];
                pcVar9 = s_starshipescortpln_005242b0;
                do {
                    if (iVar6 == 0) break;
                    iVar6 = iVar6 - 1;
                    bVar11 = (*pcVar8 == *pcVar9);
                    pcVar8 = pcVar8 + 1;
                    pcVar9 = pcVar9 + 1;
                } while (bVar11);
                if (bVar11) goto LAB_00460d72;
                iVar6 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
                uVar7 = FUN_00460dd0((unsigned short)DAT_009d8cde[iVar6],
                                     (unsigned short)DAT_009d8cdf[iVar6],
                                     (unsigned short)DAT_009d8ce2[iVar6],
                                     (unsigned short)DAT_009d8ce0[iVar6],
                                     (unsigned short)DAT_009d8ce1[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52],
                                     0);
                if (uVar7 == 0xffff) {
                    iVar6 = (unsigned int)DAT_009a73c2 * 0x52 + (unsigned int)DAT_009a73c0 * 0x566;
                    uVar7 = FUN_00460dd0((unsigned short)DAT_009d8cd8[iVar6],
                                         (unsigned short)DAT_009d8cda[iVar6],
                                         (unsigned short)DAT_009d8cdc[iVar6],
                                         (unsigned short)DAT_009d8cd9[iVar6],
                                         (unsigned short)DAT_009d8cdb[(unsigned int)DAT_009a73c0 * 0x566 + (unsigned int)DAT_009a73c2 * 0x52],
                                         1);
                    if (uVar7 != 0xffff) {
                        *puVar1 = uVar7;
                        puVar1 = puVar1 + 1;
                        *puVar1 = (unsigned short)(*puVar1 + 0xec);
                        bVar11 = (*puVar1 == 0);
                        __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
                        iVar6 = 0x13;
                        pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)(local_t6 + 0xb) * 0x55];
                        pcVar9 = s_starshipdisablepln_0051a2d4;
                        do {
                            if (iVar6 == 0) break;
                            iVar6 = iVar6 - 1;
                            bVar11 = (*pcVar8 == *pcVar9);
                            pcVar8 = pcVar8 + 1;
                            pcVar9 = pcVar9 + 1;
                        } while (bVar11);
                        if (!bVar11) {
                            __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
                            iVar6 = 0xf;
                            pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)(local_t6 + 0xb) * 0x55];
                            pcVar9 = s_disableldr1pln_0051a310;
                            do {
                                if (iVar6 == 0) break;
                                iVar6 = iVar6 - 1;
                                bVar11 = (*pcVar8 == *pcVar9);
                                pcVar8 = pcVar8 + 1;
                                pcVar9 = pcVar9 + 1;
                            } while (bVar11);
                            if (!bVar11) goto LAB_00460d90;
                        }
                        *(unsigned char *)(DAT_00a08104 + 0x372 + uVar4 * 4) = 1;
                    }
                } else {
                    *puVar1 = uVar7;
                    puVar1 = puVar1 + 1;
                    *puVar1 = (unsigned short)(*puVar1 + 0xec);
                    bVar11 = (*puVar1 == 0);
                    __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
                    iVar6 = 0x13;
                    pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)(local_t6 + 0xb) * 0x55];
                    pcVar9 = s_starshipdisablepln_0051a2d4;
                    do {
                        if (iVar6 == 0) break;
                        iVar6 = iVar6 - 1;
                        bVar11 = (*pcVar8 == *pcVar9);
                        pcVar8 = pcVar8 + 1;
                        pcVar9 = pcVar9 + 1;
                    } while (bVar11);
                    if (!bVar11) {
                        __builtin_memcpy(&local_t6, &DAT_009a73b6, 4);
                        iVar6 = 0xf;
                        pcVar8 = &DAT_009d1320[(unsigned int)*(unsigned char *)(local_t6 + 0xb) * 0x55];
                        pcVar9 = s_disableldr1pln_0051a310;
                        do {
                            if (iVar6 == 0) break;
                            iVar6 = iVar6 - 1;
                            bVar11 = (*pcVar8 == *pcVar9);
                            pcVar8 = pcVar8 + 1;
                            pcVar9 = pcVar9 + 1;
                        } while (bVar11);
                        if (!bVar11) goto LAB_00460d90;
                    }
                    *(unsigned char *)(DAT_00a08104 + 0x372 + uVar4 * 4) = 1;
                }
            }
LAB_00460d90:
            local_a = (unsigned short)(local_a + 1);
            uVar4 = (unsigned int)local_a;
        } while (uVar4 < (unsigned int)*(unsigned char *)(DAT_00a08104 + 0x21d));
    }
    return uVar4 & 0xffff0000;
}
