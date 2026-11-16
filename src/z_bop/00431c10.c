/* SPDX-License-Identifier: MIT */
/* z_bop 00431c10 */
unsigned int FUN_00425c20(unsigned short a, unsigned short b);
short FUN_00432fa0(unsigned short a, short b, unsigned short c);

extern unsigned short DAT_00556358;
extern unsigned short DAT_00556354;
extern unsigned char DAT_00521128[];
extern unsigned char DAT_009d738a[];
extern unsigned char DAT_009d738c[];
extern unsigned char DAT_009d7368;
extern unsigned char DAT_009d736b;
extern unsigned char DAT_009d75ba[];
extern unsigned char DAT_009d75b0[];
extern unsigned char DAT_009e9680[];
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009e967a[];
extern short DAT_009a2000;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int _DAT_00a07cec;
extern unsigned char DAT_009d8c94[];
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c87[];
extern unsigned char DAT_009d8c82[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d9192[];
extern unsigned char DAT_0051c573[];
extern unsigned char DAT_0051c5aa[];
extern unsigned char DAT_0051c5ac[];
extern short DAT_009a20ca[];
extern unsigned char DAT_009a20cc[], DAT_009a20ce[], DAT_009a20d0[], DAT_009a20d2[], DAT_009a20d4[];
extern unsigned char DAT_009a20d6[], DAT_009a20d8[], DAT_009a20da[], DAT_009a20dc[], DAT_009a20de[];
extern unsigned char DAT_009a20e0[], DAT_009a20e2[], DAT_009a20e4[], DAT_009a20e6[], DAT_009a20e8[];
extern unsigned char DAT_009a20ea[], DAT_009a20ec[], DAT_009a20ee[], DAT_009a20f0[], DAT_009a20f2[];
extern unsigned char DAT_009a20f4[], DAT_009a20f6[], DAT_009a20f8[], DAT_009a20fa[], DAT_009a20fc[];
extern unsigned char DAT_009a20fe[], DAT_009a2100[];
extern unsigned char DAT_009a2102[], DAT_009a2103[], DAT_009a2104[], DAT_009a2105[], DAT_009a2106[];
extern unsigned char DAT_009a2107[], DAT_009a2108[], DAT_009a2109[], DAT_009a210a[], DAT_009a210b[];
extern unsigned char DAT_009a210c[], DAT_009a210d[], DAT_009a210e[], DAT_009a210f[], DAT_009a2110[];
extern unsigned char DAT_009a2111[], DAT_009a2112[], DAT_009a2113[], DAT_009a2114[], DAT_009a2115[];
extern unsigned char DAT_009a2116[], DAT_009a2117[], DAT_009a2118[], DAT_009a2119[], DAT_009a211a[];
extern unsigned char DAT_009a211b[], DAT_009a211c[], DAT_009a211d[], DAT_009a211e[];
extern unsigned char DAT_009a2128[], DAT_009a2132[], DAT_009a213c[];
extern unsigned char DAT_009a2146[], DAT_009a2150[], DAT_009a215a[], DAT_009a2164[];

#define RS(a, o) (*(short *)((a) + (o)))
#define RB(a, o) ((a)[(o)])
#define SET_LO(x) (local_50 = (local_50 & 0xffff0000U) | (unsigned int)(unsigned short)(x))
#define SET_HI(x) (local_50 = ((unsigned int)(unsigned short)(x) << 16) | (local_50 & 0xffffU))

char FUN_00431c10(unsigned short param_1, short param_2, unsigned short param_3,
                  unsigned short param_4, short param_5, unsigned short param_6)
{
    short *psVar1;
    int bVar2;
    int iVar3;
    int iVar4;
    short sVar5;
    unsigned int uVar6;
    unsigned short uVar7;
    int iVar8;
    unsigned short uVar9;
    unsigned short uVar10;
    unsigned short uVar11;
    unsigned int uVar12;
    unsigned int uVar13;
    char cVar14;
    unsigned short local_5a;
    unsigned short uStack_58;
    unsigned short uStack_56;
    unsigned short local_54;
    unsigned int local_50;
    char *local_4c;
    int local_40[3];
    int local_34[5];
    unsigned int local_20[3];
    unsigned int local_14[5];
    unsigned short t16;
    int t32;
    int p1;
    int s211, s215;
    unsigned int uc3, ucb;
    int iv;

    DAT_00556358 = 0;
    DAT_00556354 = 0;
    uVar12 = (unsigned int)param_1;
    if (DAT_00521128[uVar12] == 0) {
        cVar14 = 4;
        switch (uVar12) {
        case 0:
            return 1;
        case 10:
            return 0;
        case 0xd:
            if (param_2 == 0xc) {
                if (DAT_009d738a[(unsigned int)param_3 * 3] == 1) {
                    return 1;
                }
                if (DAT_009d738a[(unsigned int)param_3 * 3] == 2) {
                    return 2;
                }
            } else {
                if (DAT_009d7368 == 1) {
                    return 1;
                }
                if (DAT_009d7368 == 2) {
                    return 2;
                }
            }
            break;
        case 0xe:
            if (param_2 == 0xc) {
                if (DAT_009d738a[(unsigned int)param_3 * 3] == 2) {
                    return 1;
                }
                if (DAT_009d738a[(unsigned int)param_3 * 3] == 1) {
                    return 2;
                }
            } else {
                if (DAT_009d7368 == 2) {
                    return 1;
                }
                if (DAT_009d7368 == 1) {
                    return 2;
                }
            }
            break;
        case 0x11:
            if (param_2 == 0xc) {
                if (DAT_009d738c[(unsigned int)param_3 * 3] == 2) {
                    return 1;
                }
                if (DAT_009d738c[(unsigned int)param_3 * 3] == 1) {
                    return 2;
                }
            } else {
                if (DAT_009d736b == 1) {
                    return 1;
                }
                if (DAT_009d736b == 2) {
                    return 2;
                }
            }
            break;
        case 0x12:
            if (DAT_009d736b == 2) {
                return 1;
            }
            if (DAT_009d736b == 1) {
                return 2;
            }
            break;
        case 0x14:
            return (char)((DAT_009d75ba[param_3] == 0) + 1);
        case 0x26:
            return 2;
        case 0x29:
        case 0x2a:
            cVar14 = 2;
            uVar10 = 0;
            do {
                uVar9 = uVar10 + 1;
                *(unsigned short *)((char *)local_34 + (unsigned int)uVar10 * 2) = 0;
                *(unsigned short *)((char *)local_14 + (unsigned int)uVar10 * 2) = 0;
                uVar10 = uVar9;
            } while (uVar9 < 10);
            uVar10 = 0;
            do {
                uVar9 = uVar10 + 1;
                *(unsigned short *)((char *)local_40 + (unsigned int)uVar10 * 2) = 0;
                *(unsigned short *)((char *)local_20 + (unsigned int)uVar10 * 2) = 0;
                uVar10 = uVar9;
            } while (uVar9 < 6);
            uVar12 = 0;
            do {
                iVar3 = (int)(uVar12 * 0x5bd);
                if (DAT_009e9680[iVar3] != 0) {
                    __builtin_memcpy(&t16, DAT_009e967c + iVar3, 2);
                    psVar1 = (short *)((char *)local_34 + (unsigned int)t16 * 2);
                    *psVar1 = *psVar1 + 1;
                    __builtin_memcpy(&t16, DAT_009e967a + iVar3, 2);
                    psVar1 = (short *)((char *)local_40 + (unsigned int)t16 * 2);
                    *psVar1 = *psVar1 + 1;
                }
                uVar10 = (unsigned short)((short)uVar12 + 1);
                uVar12 = (unsigned int)uVar10;
            } while (uVar10 < 8);
            uVar12 = 0;
            if (0 < DAT_009a2000) {
                do {
                    if (DAT_009d8c94[uVar12 * 0x566] != 0) {
                        psVar1 = (short *)((char *)local_14 + (unsigned int)DAT_009d8c88[uVar12 * 0x566] * 2);
                        *psVar1 = *psVar1 + 1;
                        psVar1 = (short *)((char *)local_20 + (unsigned int)DAT_009d8c87[uVar12 * 0x566] * 2);
                        *psVar1 = *psVar1 + 1;
                    }
                    uVar10 = (unsigned short)((short)uVar12 + 1);
                    uVar12 = (unsigned int)uVar10;
                } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
            }
            if (param_2 == 0xd) {
                if (param_1 == 0x29) {
                    uVar10 = 0;
                    if (0 < DAT_009a2000) {
                        do {
                            __builtin_memcpy(&t32, DAT_009d9192 + (unsigned int)uVar10 * 0x566, 4);
                            if (((DAT_009d8c94[(unsigned int)uVar10 * 0x566] != 0) &&
                                 ((unsigned int)DAT_009d8c94[(unsigned int)uVar10 * 0x566] - (unsigned int)param_3 == 1)) &&
                                (t32 != -1)) {
                                cVar14 = 1;
                            }
                            uVar10 = uVar10 + 1;
                        } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
                        return cVar14;
                    }
                } else {
                    uVar10 = 0;
                    if (0 < DAT_009a2000) {
                        do {
                            __builtin_memcpy(&t32, DAT_009d9192 + (unsigned int)uVar10 * 0x566, 4);
                            if (((DAT_009d8c94[(unsigned int)uVar10 * 0x566] != 0) &&
                                 ((unsigned int)DAT_009d8c94[(unsigned int)uVar10 * 0x566] - (unsigned int)param_3 == 1)) &&
                                (t32 == -1)) {
                                cVar14 = 1;
                            }
                            uVar10 = uVar10 + 1;
                        } while ((int)(unsigned int)uVar10 < (int)DAT_009a2000);
                        return cVar14;
                    }
                }
            } else {
                uVar12 = 0;
                uVar13 = 0;
                if (param_2 == 0xc) {
                    iVar3 = (int)((unsigned int)param_3 * 2);
                    uVar12 = (unsigned int)*(unsigned short *)((char *)local_14 + iVar3);
                    if (param_1 == 0x29) {
                        uVar13 = (unsigned int)*(unsigned short *)((char *)local_34 + iVar3);
                    } else {
                        uVar13 = (unsigned int)(unsigned short)(*(unsigned short *)((char *)local_14 + iVar3) -
                                                                *(short *)((char *)local_34 + iVar3));
                    }
                } else if (param_2 == 5) {
                    iVar3 = (int)((unsigned int)param_3 * 2);
                    uVar12 = (unsigned int)*(unsigned short *)((char *)local_20 + iVar3);
                    if (param_1 == 0x29) {
                        uVar13 = (unsigned int)*(unsigned short *)((char *)local_40 + iVar3);
                    } else {
                        uVar13 = (unsigned int)(unsigned short)(*(unsigned short *)((char *)local_20 + iVar3) -
                                                                *(short *)((char *)local_40 + iVar3));
                    }
                }
                uVar9 = (unsigned short)uVar13;
                uVar10 = (unsigned short)uVar12;
                switch (param_4) {
                case 0:
                    if (uVar9 == uVar10) {
                        return 1;
                    }
                    break;
                case 1:
                    uVar6 = FUN_00425c20(uVar9, uVar10);
                    if (0xbfff < (unsigned short)uVar6) {
                        return 1;
                    }
                    break;
                case 2:
                    uVar6 = FUN_00425c20(uVar9, uVar10);
                    if (0x7fff < (unsigned short)uVar6) {
                        return 1;
                    }
                    break;
                case 3:
                    uVar6 = FUN_00425c20(uVar9, uVar10);
                    if (0x3fff < (unsigned short)uVar6) {
                        return 1;
                    }
                    break;
                case 4:
                    if (uVar9 != 0) {
                        return 1;
                    }
                    break;
                case 5:
                    if (uVar12 - uVar13 == 1) {
                        return 1;
                    }
                    break;
                case 0x10:
                    uVar6 = FUN_00425c20(uVar9, uVar10);
                    if (0xaaa9 < (unsigned short)uVar6) {
                        return 1;
                    }
                    break;
                case 0x11:
                    uVar6 = FUN_00425c20(uVar9, uVar10);
                    if (0x5554 < (unsigned short)uVar6) {
                        cVar14 = 1;
                    }
                }
            }
        }
        return cVar14;
    }
    if (param_2 == 0) {
        return 2;
    }
    uStack_56 = 0;
    local_54 = 0;
    uVar11 = 0;
    uVar9 = 0;
    uVar10 = 0;
    local_50 = 0;
    uStack_58 = 0;
    if (0 < DAT_009a2000) {
        do {
            uVar13 = (unsigned int)uStack_58;
            iVar3 = (int)(uVar13 * 0x566);
            local_20[0] = uVar13;
            if ((DAT_009d8c82[iVar3] == 0) ||
                (sVar5 = FUN_00432fa0(uStack_58, param_2, param_3), sVar5 == 0))
                goto next_iter;
            iVar8 = (int)(uVar13 * 0x126);
            sVar5 = (short)((unsigned short)local_50 + DAT_009a20ca[uVar13 * 0x93]);
            SET_LO(sVar5);
            local_54 = local_54 + RS(DAT_009a20cc, iVar8);
            uVar7 = (unsigned short)RB(DAT_009a2102, iVar8);
            local_34[0] = (int)(((unsigned int)local_34[0] & 0xffff0000U) | uVar7);
            uStack_56 = uStack_56 + uVar7;
            if (0x2d < uVar12 - 1) goto next_iter;
            switch (uVar12) {
            case 1:
                uVar11 = uVar11 + RS(DAT_009a20cc, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2103, iVar8);
                goto next_iter;
            case 2:
                uVar11 = uVar11 + RS(DAT_009a20ce, iVar8) + RS(DAT_009a20d0, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2104, iVar8) + (unsigned short)RB(DAT_009a2105, iVar8);
                if (param_5 == 0) {
                    uVar10 = uVar10 + RS(DAT_009a20ec, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a2113, iVar8);
                    break;
                }
                uVar11 = uVar11 + RS(DAT_009a20ec, iVar8) + RS(DAT_009a20ee, iVar8) +
                         RS(DAT_009a20f0, iVar8) + RS(DAT_009a20f2, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2113, iVar8) + (unsigned short)RB(DAT_009a2114, iVar8) +
                        (unsigned short)RB(DAT_009a2115, iVar8) + (unsigned short)RB(DAT_009a2116, iVar8);
                goto next_iter;
            case 3:
                uVar11 = uVar11 + RS(DAT_009a20d2, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20d4, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2106, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2107, iVar8);
                break;
            case 4:
                uVar11 = uVar11 + RS(DAT_009a20d6, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20d8, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2108, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2109, iVar8);
                break;
            case 5:
                if (param_6 < 10) {
                    iVar8 = (int)((unsigned int)param_6 + (unsigned int)iVar8);
                    uVar11 = uVar11 + RB(DAT_009a211e, iVar8);
                    uVar9 = uVar9 + RB(DAT_009a2128, iVar8);
                    uVar10 = uVar10 + RB(DAT_009a2132, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a213c, iVar8);
                } else {
                    uVar11 = uVar11 + RS(DAT_009a20da, iVar8);
                    uVar10 = uVar10 + RS(DAT_009a20dc, iVar8);
                    uVar9 = uVar9 + RB(DAT_009a210a, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a210b, iVar8);
                }
                break;
            case 6:
                uVar11 = uVar11 + RS(DAT_009a20de, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20e0, iVar8);
                uVar9 = uVar9 + RB(DAT_009a210c, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a210d, iVar8);
                break;
            case 7:
                uVar11 = uVar11 + RS(DAT_009a20e2, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20e4, iVar8);
                uVar9 = uVar9 + RB(DAT_009a210e, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a210f, iVar8);
                break;
            case 8:
                uVar11 = uVar11 + RS(DAT_009a20e6, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20e8, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2110, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2111, iVar8);
                break;
            case 9:
                uVar11 = (uVar11 - RS(DAT_009a20d0, iVar8)) +
                         (DAT_009a20ca[uVar13 * 0x93] - RS(DAT_009a20ce, iVar8));
                uVar9 = (uVar9 - RB(DAT_009a2104, iVar8)) + (uVar7 - RB(DAT_009a2105, iVar8));
                uVar10 = uVar10 + RS(DAT_009a20ce, iVar8) + RS(DAT_009a20d0, iVar8);
                SET_HI((local_50 >> 16) + RB(DAT_009a2104, iVar8) + (unsigned short)RB(DAT_009a2105, iVar8));
            default:
                goto next_iter;
            case 0xc:
                uVar11 = uVar11 + RS(DAT_009a20ea, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2112, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20f6, iVar8) + RS(DAT_009a20f4, iVar8) +
                         RS(DAT_009a20ce, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2104, iVar8);
                SET_HI((local_50 >> 16) + RB(DAT_009a2118, iVar8) + (unsigned short)RB(DAT_009a2117, iVar8));
                break;
            case 0x15:
            case 0x16:
            case 0x17:
            case 0x18:
            case 0x22:
            case 0x23:
            case 0x24:
            case 0x25:
                if (_DAT_00a07cec < DAT_009a1fe0) {
                    local_4c = (char *)(unsigned int)((unsigned int)_DAT_00a07cec * 0x23U + 3U +
                                                      (unsigned int)DAT_009a1fe8);
                    local_40[0] = (int)((unsigned int)DAT_009a1fe0 - (unsigned int)_DAT_00a07cec);
                    do {
                        if ((*local_4c != 0) && ((unsigned char)local_4c[0x13] == local_20[0])) {
                            __builtin_memcpy(&p1, local_4c + 0x1c, 4);
                            __builtin_memcpy(&iVar8, (char *)(unsigned int)p1 + 0xa9, 4);
                            bVar2 = 0;
                            local_34[0] = (int)((unsigned int)*(unsigned char *)(unsigned int)(iVar8 + 4) * 0x101U);
                            __builtin_memcpy(&iv, DAT_0051c573 + (unsigned int)local_34[0], 4);
                            iVar4 = (int)((unsigned int)iv * 2U);
                            __builtin_memcpy(&s215, (char *)(unsigned int)iVar8 + 0x215, 4);
                            __builtin_memcpy(&s211, (char *)(unsigned int)iVar8 + 0x211, 4);
                            __builtin_memcpy(&uc3, (char *)(unsigned int)iVar8 + 0xc3, 4);
                            __builtin_memcpy(&ucb, (char *)(unsigned int)iVar8 + 0xcb, 4);
                            if (param_1 == 0x15) {
                                if ((int)((unsigned int)s215 + (unsigned int)s211) < 1) goto LAB_004321e0;
                            } else if (param_1 == 0x23) {
                                if ((int)((unsigned int)s215 + (unsigned int)s211) <=
                                    (int)((unsigned int)iVar4 + ((unsigned int)(iVar4 >> 0x1f) & 3U)) >> 2)
                                    goto LAB_004321e0;
                            } else if (param_1 == 0x22) {
                                if ((int)((unsigned int)s215 + (unsigned int)s211) <= iVar4 / 2)
                                    goto LAB_004321e0;
                            } else if (param_1 == 0x16) {
                                if (ucb >> 1 < uc3) goto LAB_004321e0;
                            } else if (param_1 == 0x24) {
                                if (ucb >> 2 < uc3) goto LAB_004321e0;
                            } else if (param_1 == 0x25) {
                                if ((ucb >> 2) * 3U < uc3) goto LAB_004321e0;
                            } else if (param_1 == 0x17) {
                                sVar5 = 0;
                                local_5a = 0;
                                if (*(unsigned char *)(unsigned int)(iVar8 + 0x232) != 0) {
                                    local_14[0] = (unsigned int)*(unsigned char *)(unsigned int)(iVar8 + 0x232);
                                    do {
                                        uVar7 = local_5a + 1;
                                        sVar5 = (short)(sVar5 +
                                                (unsigned short)*(unsigned char *)(unsigned int)(iVar8 + 0x372 +
                                                    (unsigned int)DAT_0051c5aa[(unsigned int)local_34[0] + (unsigned int)local_5a] * 4) +
                                                (unsigned short)*(unsigned char *)(unsigned int)(iVar8 + 0x372 +
                                                    (unsigned int)DAT_0051c5ac[(unsigned int)local_34[0] + (unsigned int)local_5a] * 4));
                                        local_5a = uVar7;
                                    } while (uVar7 < local_14[0]);
                                }
                                if (sVar5 == 0) {
LAB_004321e0:
                                    bVar2 = 1;
                                }
                            } else if ((*(unsigned char *)(unsigned int)(iVar8 + 0x1e5) & 0x10) == 0) {
                                goto LAB_004321e0;
                            }
                            if (bVar2) {
                                uVar11 = uVar11 + 1;
                                if (DAT_009d8c80[iVar3] == *(unsigned char *)(unsigned int)(iVar8 + 0xb0)) {
                                    uVar9 = uVar9 + 1;
                                }
                            } else {
                                uVar10 = uVar10 + 1;
                                if (DAT_009d8c80[iVar3] == *(unsigned char *)(unsigned int)(iVar8 + 0xb0)) {
                                    SET_HI((local_50 >> 16) + 1);
                                }
                            }
                        }
                        local_4c = local_4c + 0x23;
                        local_40[0] = local_40[0] + -1;
                    } while (local_40[0] != 0);
                    local_40[0] = 0;
                }
                goto next_iter;
            case 0x19:
                uVar10 = uVar10 + RS(DAT_009a20cc, iVar8);
                SET_HI((local_50 >> 16) + RB(DAT_009a2103, iVar8));
                goto next_iter;
            case 0x1a:
                uVar11 = uVar11 + RS(DAT_009a20d4, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20d2, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2107, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2106, iVar8);
                break;
            case 0x1b:
                uVar11 = uVar11 + RS(DAT_009a20e8, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20e6, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2111, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2110, iVar8);
                break;
            case 0x1c:
                uVar11 = uVar11 + RS(DAT_009a20d8, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20d6, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2109, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a2108, iVar8);
                break;
            case 0x1d:
                if (param_6 < 10) {
                    iVar8 = (int)((unsigned int)param_6 + (unsigned int)iVar8);
                    uVar11 = uVar11 + RB(DAT_009a2132, iVar8);
                    uVar9 = uVar9 + RB(DAT_009a213c, iVar8);
                    uVar10 = uVar10 + RB(DAT_009a211e, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a2128, iVar8);
                } else {
                    uVar11 = uVar11 + RS(DAT_009a20dc, iVar8);
                    uVar10 = uVar10 + RS(DAT_009a20da, iVar8);
                    uVar9 = uVar9 + RB(DAT_009a210b, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a210a, iVar8);
                }
                break;
            case 0x1e:
                uVar11 = uVar11 + RS(DAT_009a20fc, iVar8);
                uVar9 = uVar9 + RB(DAT_009a211b, iVar8);
                goto next_iter;
            case 0x1f:
                uVar11 = uVar11 + RS(DAT_009a20e0, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20de, iVar8);
                uVar9 = uVar9 + RB(DAT_009a210d, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a210c, iVar8);
                break;
            case 0x20:
                uVar11 = uVar11 + RS(DAT_009a20fe, iVar8);
                uVar9 = uVar9 + RB(DAT_009a211c, iVar8);
                goto next_iter;
            case 0x21:
                uVar11 = uVar11 + RS(DAT_009a20e4, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20e2, iVar8);
                uVar9 = uVar9 + RB(DAT_009a210f, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a210e, iVar8);
                break;
            case 0x2b:
                uVar11 = uVar11 + RS(DAT_009a20ec, iVar8) + RS(DAT_009a20ee, iVar8) +
                         RS(DAT_009a20f0, iVar8) + RS(DAT_009a20f2, iVar8) +
                         RS(DAT_009a20ce, iVar8) + RS(DAT_009a20d0, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2113, iVar8) + (unsigned short)RB(DAT_009a2114, iVar8) +
                        (unsigned short)RB(DAT_009a2115, iVar8) + (unsigned short)RB(DAT_009a2116, iVar8) +
                        (unsigned short)RB(DAT_009a2104, iVar8) + (unsigned short)RB(DAT_009a2105, iVar8);
                goto next_iter;
            case 0x2c:
                if (param_6 < 10) {
                    iVar8 = (int)((unsigned int)param_6 + (unsigned int)iVar8);
                    uVar11 = uVar11 + RB(DAT_009a2146, iVar8);
                    uVar9 = uVar9 + RB(DAT_009a2150, iVar8);
                    uVar10 = uVar10 + RB(DAT_009a215a, iVar8);
                    uVar7 = (unsigned short)RB(DAT_009a2164, iVar8);
                    break;
                }
                uVar13 = 0;
                do {
                    uVar11 = uVar11 + RB(DAT_009a2146, (unsigned int)iVar8 + uVar13);
                    uVar9 = uVar9 + RB(DAT_009a2150, (unsigned int)iVar8 + uVar13);
                    if (DAT_009d75b0[uVar13] != 0) {
                        uVar10 = uVar10 + RB(DAT_009a215a, (unsigned int)iVar8 + uVar13);
                        SET_HI((local_50 >> 16) + RB(DAT_009a2164, (unsigned int)iVar8 + uVar13));
                    }
                    uVar7 = (unsigned short)((short)uVar13 + 1);
                    uVar13 = (unsigned int)uVar7;
                } while (uVar7 < 10);
                goto next_iter;
            case 0x2d:
                uVar11 = uVar11 + RS(DAT_009a20f6, iVar8) + RS(DAT_009a20f4, iVar8) +
                         RS(DAT_009a20ce, iVar8) + RS(DAT_009a20d0, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2118, iVar8) + (unsigned short)RB(DAT_009a2117, iVar8) +
                        (unsigned short)RB(DAT_009a2104, iVar8) + (unsigned short)RB(DAT_009a2105, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20ea, iVar8) + RS(DAT_009a2100, iVar8);
                SET_HI((local_50 >> 16) + RB(DAT_009a2112, iVar8));
                uVar7 = (unsigned short)RB(DAT_009a211d, iVar8);
                break;
            case 0x2e:
                uVar11 = uVar11 + RS(DAT_009a20f8, iVar8);
                uVar10 = uVar10 + RS(DAT_009a20fa, iVar8);
                uVar9 = uVar9 + RB(DAT_009a2119, iVar8);
                uVar7 = (unsigned short)RB(DAT_009a211a, iVar8);
            }
            SET_HI((local_50 >> 16) + uVar7);
next_iter:
            uStack_58 = uStack_58 + 1;
        } while ((int)(unsigned int)uStack_58 < (int)DAT_009a2000);
    }
    cVar14 = 4;
    if ((unsigned short)local_50 == 0) {
        goto done;
    }
    switch (param_4) {
    case 0:
        if ((unsigned short)local_50 <= uVar11) {
            cVar14 = 1;
            goto done;
        }
        goto joined_r0x0043277a;
    case 1:
        uVar6 = FUN_00425c20(uVar11, (unsigned short)local_50);
        if (0xbfff < (unsigned short)uVar6) {
            cVar14 = 1;
            goto done;
        }
        uVar6 = FUN_00425c20(uVar10, (unsigned short)local_50);
        if ((unsigned short)uVar6 < 0x4001) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 2:
        uVar6 = FUN_00425c20(uVar11, (unsigned short)local_50);
        if (0x7fff < (unsigned short)uVar6) {
            cVar14 = 1;
            goto done;
        }
        uVar6 = FUN_00425c20(uVar10, (unsigned short)local_50);
        if ((unsigned short)uVar6 < 0x8001) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 3:
        uVar6 = FUN_00425c20(uVar11, (unsigned short)local_50);
        if (0x3fff < (unsigned short)uVar6) {
            cVar14 = 1;
            goto done;
        }
        uVar6 = FUN_00425c20(uVar10, (unsigned short)local_50);
        if ((unsigned short)uVar6 < 0xc001) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 4:
    case 0xe:
        if (uVar11 != 0) {
            cVar14 = 1;
            goto done;
        }
        if (uVar10 != (unsigned short)local_50) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 5:
        if ((int)((local_50 & 0xffff) - 1) <= (int)(unsigned int)uVar11) {
            cVar14 = 1;
            goto done;
        }
        if (uVar10 < 2) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 6:
        if (uStack_56 == 0) {
            cVar14 = 4;
            goto done;
        }
        if (uVar9 == uStack_56) {
            cVar14 = 1;
            goto done;
        }
        uVar10 = (unsigned short)(local_50 >> 16);
joined_r0x0043277a:
        if (uVar10 == 0) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 7:
        if ((local_50 & 0xffff) - (unsigned int)uVar11 == (unsigned int)uStack_56) {
            cVar14 = 1;
            goto done;
        }
        if ((uVar10 == 0) || (uVar9 != 0)) {
            bVar2 = uVar10 < 2;
            uVar10 = uVar9;
            if (bVar2) {
                cVar14 = 4;
                goto done;
            }
            goto joined_r0x0043277a;
        }
        break;
    default:
        goto done;
    case 10:
        if (uVar11 < local_54) {
            cVar14 = 4;
            goto done;
        }
        cVar14 = 1;
        goto done;
    case 0xb:
        uVar6 = FUN_00425c20(uVar11, local_54);
        if ((unsigned short)uVar6 < 0xc000) {
            cVar14 = 4;
            goto done;
        }
        cVar14 = 1;
        goto done;
    case 0xc:
        uVar6 = FUN_00425c20(uVar11, local_54);
        if ((unsigned short)uVar6 < 0x8000) {
            cVar14 = 4;
            goto done;
        }
        cVar14 = 1;
        goto done;
    case 0xd:
        uVar6 = FUN_00425c20(uVar11, local_54);
        if ((unsigned short)uVar6 < 0x4000) {
            cVar14 = 4;
            goto done;
        }
        cVar14 = 1;
        goto done;
    case 0xf:
        if ((int)(unsigned int)uVar11 < (int)(local_54 - 1)) {
            cVar14 = 4;
            goto done;
        }
        cVar14 = 1;
        goto done;
    case 0x10:
        uVar6 = FUN_00425c20(uVar11, (unsigned short)local_50);
        if (0xaaa9 < (unsigned short)uVar6) {
            cVar14 = 1;
            goto done;
        }
        uVar6 = FUN_00425c20(uVar10, (unsigned short)local_50);
        if ((unsigned short)uVar6 < 0x5556) {
            cVar14 = 4;
            goto done;
        }
        break;
    case 0x11:
        uVar6 = FUN_00425c20(uVar11, (unsigned short)local_50);
        if (0x5554 < (unsigned short)uVar6) {
            cVar14 = 1;
            goto done;
        }
        uVar6 = FUN_00425c20(uVar10, (unsigned short)local_50);
        if ((unsigned short)uVar6 < 0xaaab) {
            cVar14 = 4;
            goto done;
        }
    }
    cVar14 = 2;
done:
    DAT_00556358 = uVar11;
    DAT_00556354 = (unsigned short)local_50;
    return cVar14;
}
