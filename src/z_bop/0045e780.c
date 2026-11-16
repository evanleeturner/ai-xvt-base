/* SPDX-License-Identifier: MIT */
/* z_bop 0045e780 */
extern unsigned short *DAT_00a08104;
extern unsigned short *DAT_009a73b6;
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern unsigned int DAT_009a1ff8;
extern unsigned short DAT_009a73b0;
extern unsigned short DAT_009a73d0;
extern unsigned short DAT_009a8070;
extern unsigned short DAT_00a080f0;
extern unsigned int DAT_009a8c00;
extern unsigned short DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_00a07ce8;
extern unsigned short DAT_00a08294;
extern unsigned char DAT_009d1320[];
extern unsigned int DAT_00524290[];
extern unsigned char DAT_0052429c[];
extern unsigned char DAT_0051a44f[];
extern unsigned int DAT_0051a194[];
extern unsigned char DAT_0051c5aa[];
extern char s_disableldr1pln_0051a310[];

void FUN_00403540(unsigned int, unsigned int);
void FUN_004034e0(unsigned int, unsigned int);
unsigned int FUN_00484f80(unsigned int);
unsigned int FUN_00425b70(unsigned int, unsigned int);
unsigned int FUN_0045f1e0(unsigned int);
void FUN_00406030(unsigned int, unsigned int);

#define RD8(a)  (*(unsigned char *)(unsigned int)(a))
#define WR8(a, v) (*(unsigned char *)(unsigned int)(a) = (unsigned char)(v))
#define RD16(a) ({ unsigned short r_; __builtin_memcpy(&r_, (const void *)(unsigned int)(a), 2); r_; })
#define RD32(a) ({ unsigned int r_; __builtin_memcpy(&r_, (const void *)(unsigned int)(a), 4); r_; })
#define WR16(a, v) ({ unsigned short w_ = (unsigned short)(v); __builtin_memcpy((void *)(unsigned int)(a), &w_, 2); })
#define G ((unsigned int)DAT_00a08104)
#define S ((unsigned int)DAT_009a73b6)

unsigned int __stdcall FUN_0045e780(void)
{
    unsigned char uVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    char cVar4;
    char cVar5;
    unsigned int psVar6;
    unsigned short uVar7;
    unsigned short uVar8;
    int iVar9;
    unsigned int psVar10;
    unsigned char bVar11;
    unsigned short uVar12;
    unsigned int iVar13;
    unsigned int uVar14;
    unsigned int psVar15;
    unsigned char bVar16;
    unsigned int iVar17;
    unsigned short uVar18;
    unsigned int uVar19;
    unsigned char *pbVar20;
    unsigned char *pbVar22;
    int bVar25;
    int bVar26;
    unsigned short local_22;
    unsigned short local_20;
    unsigned short uStack_1c;
    unsigned short local_1a;
    unsigned short uStack_14;
    unsigned int local_10;
    unsigned int local_c;
    unsigned int iVar24;
    int iVar9b;

    if (RD16(G + 0x1e5) == 0) {
        return G & 0xffff0000;
    }
    uVar18 = DAT_009a73b6[0xc];
    iVar9 = 0;
    uVar1 = 0;
    if (uVar18 == 0xffff) goto LAB_0045e8f4;
    iVar9 = (int)((unsigned int)uVar18 * 0x23);
    iVar17 = (unsigned int)DAT_009a1fe8 + (unsigned int)iVar9;
    if (RD32(iVar17 + 0x1f) == 0) {
        bVar26 = RD8(iVar17 + 3) == 0;
    } else {
        if (RD8(iVar17 + 3) == 0) {
            iVar9 = 0;
            goto LAB_0045e8f4;
        }
        iVar17 = RD32(RD32(iVar17 + 0x1f) + 0xa9);
        if (iVar17 == 0) {
            iVar9 = 1;
            goto LAB_0045e8f4;
        }
        if ((RD8(iVar17 + 0x5e) == 0x15) && (RD8(iVar17 + 0x5f) != 0)) {
            iVar9 = 0;
            goto LAB_0045e8f4;
        }
        cVar4 = (char)RD8(iVar17 + 7);
        if ((cVar4 == 1) || (cVar4 == 3) || (cVar4 == 4)) {
            iVar9 = 0;
            goto LAB_0045e8f4;
        }
        if ((RD8(iVar17 + 0x23d) == 3) && (RD8(iVar17 + 0x241) != 0)) {
            FUN_00403540((unsigned int)uVar18, (unsigned int)DAT_009a73b0);
            cVar4 = (char)RD8((unsigned int)DAT_009a73b0 * 0x23 + 2 + (unsigned int)DAT_009a1fe8);
            if ((cVar4 == 0) || ((cVar4 == 1 || (cVar4 == 2)))) {
                if (0x8000 < DAT_009a1ff8) {
                    iVar9 = 0;
                    goto LAB_0045e8f4;
                }
            } else if (0x10000 < DAT_009a1ff8) {
                iVar9 = 0;
                goto LAB_0045e8f4;
            }
        }
        iVar9 = (int)((unsigned int)DAT_009a1fe8 + (unsigned int)iVar9);
        if (RD32((unsigned int)iVar9 + 0x1b) != 0xffffffffu) {
            iVar9 = 1;
            goto LAB_0045e8f4;
        }
        bVar26 = RD32(RD32(RD32((unsigned int)iVar9 + 0x1f) + 0xa9) + 0xcb) < RD32(iVar17 + 0xc3);
    }
    iVar9 = (int)(1u - (unsigned int)bVar26);
LAB_0045e8f4:
    if (iVar9 == 0) {
        local_c = G;
        uVar18 = 0;
        if (RD8(G + 0x21b) != 0) {
            do {
                WR8((unsigned int)uVar18 + 0x220 + G, 0);
                local_c = (unsigned int)(unsigned short)(uVar18 + 1);
                uVar18 = (unsigned short)(uVar18 + 1);
            } while (local_c < (unsigned int)RD8(G + 0x21b));
        }
    } else {
        uVar14 = (unsigned int)uVar18;
        uVar19 = DAT_00524290[DAT_009a73d0];
        if ((int)uVar14 < DAT_00a07ce8) {
            iVar9b = (int)((unsigned int)DAT_009a1fe8 + uVar14 * 0x23);
            if (RD32((unsigned int)iVar9b + 0x1f) != 0) {
                if (0x19 < RD16(RD32((unsigned int)iVar9b + 0x1f) + 0x79)) {
                    uVar7 = (unsigned short)(RD16((unsigned int)DAT_009a73b0 * 0x23 + 0x10 + (unsigned int)DAT_009a1fe8) -
                                             RD16((unsigned int)iVar9b + 0x10));
                    if (0x7fff < uVar7) {
                        uVar7 = (unsigned short)(-(int)uVar7);
                    }
                    if (uVar7 < 0x2000) {
                        uVar19 = uVar19 - 0x4000;
                    } else if (uVar7 < 0x5000) {
                        uVar19 = uVar19 - 0x2000;
                    }
                }
                if ((RD8((unsigned int)iVar9b + 2) == 5) || (RD8((unsigned int)iVar9b + 2) == 4)) {
                    uVar19 = uVar19 + 0x6000;
                }
            }
        }
        FUN_004034e0((unsigned int)DAT_009a73b0, uVar14);
        uStack_1c = (unsigned short)(DAT_009a8070 - RD16((unsigned int)DAT_009a73b0 * 0x23 + 0x10 + (unsigned int)DAT_009a1fe8));
        if (0x7fff < uStack_1c) {
            uStack_1c = (unsigned short)(-(int)uStack_1c);
        }
        local_1a = (unsigned short)(DAT_00a080f0 - RD16(G + 0xd));
        if (0x7fff < local_1a) {
            local_1a = (unsigned short)(-(int)local_1a);
        }
        if (((uStack_1c < 0x800) && (local_1a < 0x800)) && (DAT_009a8c00 < uVar19)) {
            bVar16 = 3;
            if (0x1fff < (int)DAT_009a8c00) {
                bVar16 = (unsigned char)(((int)DAT_009a8c00 < 0x4000) + 1);
            }
            uVar1 = DAT_0052429c[DAT_009a73d0];
        } else {
            bVar16 = 0;
        }
        bVar2 = RD8(G + 0x21b);
        bVar3 = RD8(S + 0xb);
        uVar12 = (unsigned short)bVar3;
        uVar7 = 0;
        if (bVar2 != 0) {
            do {
                if (bVar16 == 0) {
LAB_0045eb37:
                    bVar11 = 0;
                } else {
                    bVar11 = bVar16;
                    if ((char)RD8((unsigned int)uVar7 + 0x21e + G) == -0x73) {
                        const unsigned char *pcVar21 = &DAT_009d1320[(unsigned int)uVar12 * 0x55];
                        const char *pcVar23 = s_disableldr1pln_0051a310;
                        bVar26 = uVar12 == 0;
                        iVar9 = 0xf;
                        do {
                            if (iVar9 == 0) break;
                            iVar9 = iVar9 + -1;
                            bVar26 = (char)*pcVar21 == *pcVar23;
                            pcVar21 = pcVar21 + 1;
                            pcVar23 = pcVar23 + 1;
                        } while (bVar26);
                        if ((!bVar26) || ((char)DAT_009a73b6[0xf] != 1)) goto LAB_0045eb37;
                    } else if ((char)DAT_009a73b6[0xf] != 0) {
                        iVar9 = 0xf;
                        bVar26 = 0;
                        iVar17 = 0;
                        bVar25 = 1;
                        pbVar20 = &DAT_009d1320[(unsigned int)uVar12 * 0x55];
                        pbVar22 = (unsigned char *)s_disableldr1pln_0051a310;
                        do {
                            if (iVar9 == 0) break;
                            iVar9 = iVar9 + -1;
                            bVar26 = *pbVar20 < *pbVar22;
                            bVar25 = *pbVar20 == *pbVar22;
                            pbVar20 = pbVar20 + 1;
                            pbVar22 = pbVar22 + 1;
                        } while (bVar25);
                        if (!bVar25) {
                            iVar17 = (1u - (unsigned int)bVar26) - (unsigned int)(bVar26 != 0);
                        }
                        bVar11 = (unsigned char)((((unsigned int)(iVar17 == 0)) - 1u) & (unsigned int)bVar16);
                    }
                }
                uVar8 = (unsigned short)(uVar7 + 1);
                WR8((unsigned int)uVar7 + 0x220 + G, bVar11);
                WR8((unsigned int)uVar7 + 0x222 + G, uVar1);
                uVar7 = uVar8;
            } while (uVar8 < bVar2);
        }
        local_c = (unsigned int)DAT_009a1fe0;
        if (((int)uVar14 < DAT_009a1fe0) &&
            (local_c = FUN_00484f80((unsigned int)uVar18), psVar6 = G, (char)local_c == 0)) {
            iVar9 = (int)(uVar14 * 0x23);
            iVar17 = (unsigned int)DAT_009a1fe8 + (unsigned int)iVar9;
            cVar4 = (char)RD8(iVar17 + 2);
            if (((cVar4 == 4) || ((cVar4 == 5 || (cVar4 == 3)))) ||
                ((cVar4 == 1) && (DAT_00a08294 == 0xe))) {
                cVar4 = 2;
                local_20 = 6;
                local_22 = 0x10;
                local_10 = ((0u - (unsigned int)(DAT_009a73d0 == 2)) & 0x9f12) + 0x31b5a;
            } else {
                local_10 = 0x18dad;
                local_22 = 2;
                cVar4 = 1;
                local_20 = 1;
            }
            local_c = 0;
            iVar24 = RD32(RD32(iVar17 + 0x1f) + 0xa9);
            {
                const unsigned char *pcVar21 = &DAT_009d1320[(unsigned int)uVar12 * 0x55];
                const char *pcVar23 = s_disableldr1pln_0051a310;
                int iVar13i = 0xf;
                bVar26 = uVar12 == 0;
                psVar15 = RD32(iVar24 + 0x211);
                do {
                    if (iVar13i == 0) break;
                    iVar13i = iVar13i + -1;
                    bVar26 = (char)*pcVar21 == *pcVar23;
                    pcVar21 = pcVar21 + 1;
                    pcVar23 = pcVar23 + 1;
                } while (bVar26);
            }
            if (bVar26) {
                uVar19 = RD32(iVar24 + 0xcb) / 10;
                if (uVar19 < RD32(iVar24 + 0xc3)) {
                    psVar15 = psVar15 + uVar19;
                }
                uVar19 = 0;
                if (RD8(G + 0x232) != 0) {
                    do {
                        bVar16 = RD8(uVar19 + 0x233 + G);
                        if (DAT_0051a44f[bVar16] == cVar4) {
LAB_0045ecef:
                            uVar14 = DAT_0051a194[bVar16];
                            cVar5 = (char)RD8(iVar17 + 2);
                            if ((cVar5 == 4) || (cVar5 == 5)) {
                                uVar14 = uVar14 >> 4;
                            }
                            if (cVar5 == 3) {
                                uVar14 = uVar14 >> 2;
                            }
                            local_c = local_c + uVar14;
                        } else {
                            if ((DAT_00a08294 != 0xe) ||
                                (((bVar16 != 0x98) && (bVar16 != 0x99)) || (bVar26 = 1, cVar4 != 2))) {
                                bVar26 = 0;
                            }
                            if (bVar26) goto LAB_0045ecef;
                        }
                        uVar19 = (unsigned int)(unsigned short)((short)uVar19 + 1);
                    } while (uVar19 < RD8(G + 0x232));
                }
            } else {
                psVar15 = psVar15 + (RD32(iVar24 + 0xcb) - RD32(iVar24 + 0xc3));
            }
            uVar7 = DAT_009e9640;
            psVar10 = (unsigned int)DAT_009e9640;
            while ((int)psVar10 < DAT_00a00854) {
                iVar24 = (unsigned int)uVar7 * 0x23 + (unsigned int)DAT_009a1fe8;
                if (((0x88 < RD8(iVar24 + 3)) &&
                     (iVar13 = RD32(RD32(iVar24 + 0x1f) + 0xa5), RD8(iVar13 + 1) != 0)) &&
                    (RD16(iVar13 + 4) == uVar18)) {
                    uVar19 = DAT_0051a194[RD8(iVar24 + 3)];
                    cVar5 = (char)RD8(iVar17 + 2);
                    if ((cVar5 == 4) || (cVar5 == 5)) {
                        uVar19 = uVar19 >> 4;
                    }
                    if (cVar5 == 3) {
                        uVar19 = uVar19 >> 2;
                    }
                    local_c = local_c + uVar19;
                }
                uVar7 = (unsigned short)(uVar7 + 1);
                psVar10 = (unsigned int)uVar7;
            }
            if (local_c < psVar15) {
                uVar7 = 0;
                uVar12 = DAT_009e9640;
                local_c = (unsigned int)DAT_00a00854;
                psVar10 = (unsigned int)DAT_009e9640;
                while ((int)psVar10 < DAT_00a00854) {
                    iVar17 = (unsigned int)uVar12 * 0x23 + (unsigned int)DAT_009a1fe8;
                    bVar16 = RD8(iVar17 + 3);
                    uVar8 = uVar7;
                    if (bVar16 != 0) {
                        if (DAT_0051a44f[bVar16] != cVar4) {
                            if ((DAT_00a08294 != 0xe) ||
                                (((bVar16 != 0x98) && (bVar16 != 0x99)) || (bVar26 = 1, cVar4 != 2))) {
                                bVar26 = 0;
                            }
                            if (!bVar26) goto LAB_0045ee9e;
                        }
                        iVar17 = RD32(RD32(iVar17 + 0x1f) + 0xa5);
                        if ((RD8(iVar17 + 1) != 0) && (RD16(iVar17 + 4) == uVar18)) {
                            uVar8 = (unsigned short)(uVar7 + 1);
                            if (((DAT_00a08294 != 0xe) || ((bVar16 != 0x98) && (bVar16 != 0x99))) ||
                                (bVar26 = 1, cVar4 != 2)) {
                                bVar26 = 0;
                            }
                            if (bVar26) {
                                uVar8 = (unsigned short)(uVar7 + 8);
                            }
                        }
                    }
LAB_0045ee9e:
                    uVar12 = (unsigned short)(uVar12 + 1);
                    local_c = (unsigned int)uVar12;
                    uVar7 = uVar8;
                    psVar10 = local_c;
                }
                if ((((uVar7 < local_22) && (local_c = S, (char)DAT_009a73b6[0x1a] == 0x17)) &&
                     (local_c = G, RD16(G + 0x1e7) == 0)) &&
                    ((RD32(G + 0x1d) == 0 &&
                      (local_c = (unsigned int)local_20, (unsigned int)RD8(G + 0x78) < local_c)))) {
                    if ((uStack_1c < 0x300) && ((local_1a < 0x300) && (DAT_009a8c00 < local_10))) {
                        uVar7 = (unsigned short)((DAT_009a73d0 + 1) * 0x1d8);
                        if (DAT_009a73d0 == 2) {
                            WR16(G + 0x23b, (unsigned short)(RD16(G + 0x23b) + (unsigned short)RD32(S + 0xe)));
                            if (RD32(G + 199) <= RD32(G + 0xc3)) {
                                uVar7 = 0x2c4;
                            }
                        } else {
                            uVar19 = FUN_00425b70(RD32(S + 0xe), 0xc000);
                            WR16(psVar6 + 0x23b, (unsigned short)(RD16(psVar6 + 0x23b) + (unsigned short)uVar19));
                        }
                        local_c = G;
                        if (((int)(unsigned int)uVar7 <= (int)(short)RD16(G + 0x23b)) &&
                            (uVar19 = 0, RD8(G + 0x232) != 0)) {
                            do {
                                psVar6 = G;
                                uStack_14 = (unsigned short)psVar15;
                                if (RD16(G + uVar19 * 2 + 0x237) == 0) {
                                    bVar16 = RD8(G + uVar19 + 0x233);
                                    if (DAT_0051a44f[bVar16] != cVar4) {
                                        if ((DAT_00a08294 == 0xe) &&
                                            (((bVar16 == 0x98) || (bVar16 == 0x99)) && (cVar4 == 2))) {
                                            if ((uVar18 == 0xffff) ||
                                                (uVar7 = RD16(RD32(RD32((unsigned int)DAT_009a1fe8 + 0x1f + (unsigned int)iVar9) + 0xa9) + 0x1e7),
                                                 uVar7 == 0)) {
                                                bVar26 = 1;
                                            } else {
                                                bVar26 = uVar7 < 0x49c;
                                            }
                                        } else {
                                            bVar26 = 0;
                                        }
                                        if (!bVar26) goto LAB_0045f132;
                                    }
                                    if ((DAT_009a73d0 == 2) && (RD32(G + 199) <= RD32(G + 0xc3))) {
                                        WR8(G + uVar19 + 0x235, 3);
                                    } else {
                                        unsigned int idx = (unsigned int)DAT_0051c5aa[uVar19 + (unsigned int)RD8(G + 4) * 0x101];
                                        bVar16 = RD8(G + idx * 4 + 0x376);
                                        bVar2 = RD8(G + idx * 4 + 0x372);
                                        WR8(G + uVar19 + 0x235, 1);
                                        if (bVar2 < bVar16) {
                                            WR8(psVar6 + uVar19 + 0x235, 0x81);
                                        }
                                    }
                                    uVar14 = FUN_0045f1e0(((unsigned int)uStack_14 << 16) | (unsigned int)uVar18);
                                    DAT_009a73b6[0xe] = (unsigned short)uVar14;
                                    FUN_00406030((unsigned int)DAT_009a73b0, uVar19);
                                    WR8(G + 0x78, (unsigned char)(RD8(G + 0x78) + 1));
                                    cVar5 = (char)RD8((unsigned int)DAT_009a1fe8 + 2 + (unsigned int)iVar9);
                                    if ((cVar5 == 0) || (cVar5 == 1)) {
                                        WR16(G + 0x23b, 0);
                                    }
                                }
LAB_0045f132:
                                uVar19 = (unsigned int)(unsigned short)((short)uVar19 + 1);
                                if (RD8(G + 0x232) <= uVar19) {
                                    return 0;
                                }
                            } while (1);
                        }
                    } else {
                        WR16(G + 0x23b, (unsigned short)(RD16(G + 0x23b) - (unsigned short)RD32(S + 0xe)));
                        local_c = G + 0x23b;
                        if ((short)RD16(local_c) < 0) {
                            WR16(local_c, 0);
                            return local_c & 0xffff0000;
                        }
                    }
                }
            }
        }
    }
    return local_c & 0xffff0000;
}
