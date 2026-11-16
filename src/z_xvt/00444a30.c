/* SPDX-License-Identifier: MIT */
/* z_xvt 00444a30 */
/* Unaligned field access via memcpy (offsets are not multiples of their widths). */
#define RS16(p) ({ short t_s16; __builtin_memcpy(&t_s16, (const void *)(p), 2); t_s16; })
#define RU16(p) ({ unsigned short t_u16; __builtin_memcpy(&t_u16, (const void *)(p), 2); t_u16; })
#define RI32(p) ({ int t_i32; __builtin_memcpy(&t_i32, (const void *)(p), 4); t_i32; })
#define RPB(p)  ({ unsigned char *t_pb; __builtin_memcpy(&t_pb, (const void *)(p), sizeof(t_pb)); t_pb; })
#define W16(p, v) do { unsigned short t_w16 = (unsigned short)(v); __builtin_memcpy((void *)(p), &t_w16, 2); } while (0)

/* Callee calling conventions are not established by the evidence; Ghidra defaults kept. */
unsigned char *FUN_0042b150(unsigned char *param_1);
unsigned int FUN_00478320(unsigned int param_1, unsigned short param_2);
void FUN_00445d10(unsigned int param_1);
void FUN_00445920(unsigned int param_1, unsigned int param_2, int param_3, unsigned short param_4);

extern int DAT_006d4980;
extern unsigned char *DAT_006d4988;
extern unsigned short DAT_006d9d34;
extern short DAT_0070a1d2;
extern int DAT_0071b71c;
extern int DAT_0071e690;
extern unsigned short DAT_0071e6be;
extern unsigned short DAT_00732a3c;
extern unsigned char *DAT_00732e54;
extern unsigned char DAT_0052b095[];

void __stdcall FUN_00444a30(void)
{
    unsigned char *psVar1;
    short sVar2;
    char cVar3;
    short sVar4;
    short sVar5;
    short sVar6;
    unsigned int uVar7;
    int iVar8;
    unsigned int uVar9;
    int bVar10;
    short sVar11;
    unsigned short uVar12;
    unsigned short uVar13;
    int iVar14;
    unsigned char *puVar14;
    unsigned int uVar15;
    unsigned char *puVar16;
    unsigned short uVar17;
    short sVar18;
    unsigned char *iVar19;
    unsigned int uVar20;
    unsigned short uVar21;
    unsigned short uVar22;
    unsigned short local_16;
    unsigned short uStack_14;
    unsigned int local_10;

    uVar12 = DAT_0071e6be;
    sVar11 = DAT_0070a1d2;
    local_16 = DAT_00732a3c;
    local_10 = 0; /* upper half is uninitialised stack in the original */
    bVar10 = 0;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
        do {
            if (DAT_0071e690 != -1) {
                if (bVar10) {
                    DAT_0070a1d2 = sVar11;
                    DAT_0071e6be = uVar12;
                    return;
                }
                bVar10 = 1;
                local_16 = (unsigned short)DAT_0071e690;
            }
            uVar9 = (unsigned int)local_16;
            iVar8 = (int)(uVar9 * 0x23);
            iVar19 = DAT_006d4988 + iVar8;
            DAT_0070a1d2 = sVar11;
            DAT_0071e6be = uVar12;
            if ((RI32(iVar19 + 0x1f) == 0) ||
                (iVar14 = RI32(RPB(iVar19 + 0x1f) + 2), iVar14 == 0)) {
LAB_00444b21:
                if ((iVar19[3] != 0) && (*RPB(iVar19 + 0x1f) == 0)) {
                    DAT_00732e54 = RPB(RPB(iVar19 + 0x1f) + 0xa7);
                    puVar14 = FUN_0042b150(DAT_00732e54);
                    sVar4 = RS16(DAT_006d4988 + 0x12 + iVar8);
                    iVar19 = DAT_006d4988 + iVar8;
                    sVar5 = RS16(iVar19 + 0x10);
                    sVar6 = RS16(iVar19 + 0x14);
                    uVar22 = 0;
                    DAT_006d9d34 = (unsigned short)DAT_00732e54[4];
                    if (RI32(iVar19 + 0x1b) == -1) {
                        if (RS16(DAT_00732e54 + 0x1df) != 0) goto LAB_00444bb0;
                    } else if ((DAT_00732e54[0x1df] & 0x40) != 0) {
LAB_00444bb0:
                        uVar22 = RU16(DAT_00732e54 + 0xbd);
                    }
                    if (((RI32(iVar19 + 0x1b) == -1) && (RS16(DAT_00732e54 + 0x1df) != 0)) &&
                        (RI32(DAT_00732e54 + 0x19) == 0)) {
                        if ((DAT_00732e54[0xa4] != 0) && (DAT_00732e54[0xa4] < 4)) {
                            uVar13 = (unsigned short)(RS16(puVar14 + 0x30) - RS16(iVar19 + 0x14));
                            uVar15 = FUN_00478320((unsigned int)((int)((unsigned int)RU16(DAT_00732e54 + 0xa0) *
                                                                       (unsigned int)DAT_0071e6be) / 0xec),
                                                  RU16(DAT_00732e54 + 0xa2));
                            uVar15 = FUN_00478320(uVar15, RU16(DAT_00732e54 + 0xa5));
                            uVar17 = (unsigned short)(uVar15 * 2);
                            if (DAT_00732e54[0xa4] == 3) {
                                if (RU16(puVar14 + 0x30) < 0x8000) {
                                    psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                    W16(psVar1, RS16(psVar1) + uVar17);
                                } else {
LAB_00444ce6:
                                    psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                    W16(psVar1, RS16(psVar1) + (short)uVar15 * -2);
                                }
                            } else if (uVar13 < 0x8000) {
                                if (uVar17 < uVar13) {
                                    psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                    W16(psVar1, RS16(psVar1) + uVar17);
                                } else {
                                    W16(DAT_006d4988 + 0x14 + iVar8, RS16(puVar14 + 0x30));
                                    DAT_00732e54[0xa4] = 4;
                                }
                            } else {
                                if (uVar17 < (unsigned short)(0U - uVar13)) goto LAB_00444ce6;
                                W16(DAT_006d4988 + 0x14 + iVar8, RS16(puVar14 + 0x30));
                                DAT_00732e54[0xa4] = 4;
                            }
                        }
                        if (DAT_00732e54[0x9c] != 0) {
                            uStack_14 = (unsigned short)(RS16(puVar14 + 0x2e) - RS16(DAT_00732e54 + 0xd));
                            if (0x7fff < uStack_14) {
                                uStack_14 = (unsigned short)(0U - uStack_14);
                            }
                            uVar15 = FUN_00478320((unsigned int)((int)((unsigned int)RU16(DAT_00732e54 + 0x98) *
                                                                       (unsigned int)DAT_0071e6be) / 0xec),
                                                  RU16(DAT_00732e54 + 0x9a));
                            uVar15 = FUN_00478320(uVar15, RU16(DAT_00732e54 + 0x9e));
                            uVar13 = (unsigned short)uVar15;
                            local_10 = (local_10 & 0xffff0000U) | uVar13;
                            FUN_00478320(local_10, 0x8000);
                            if (DAT_00732e54[0x9c] == 1) {
                                if ((uVar13 < uStack_14) || (DAT_00732e54[0x9d] != 0)) {
                                    W16(DAT_00732e54 + 0xd, RS16(DAT_00732e54 + 0xd) - uVar13);
                                    if (0xdfff < RU16(DAT_00732e54 + 0xd)) {
                                        W16(DAT_00732e54 + 0xd, 0U - RU16(DAT_00732e54 + 0xd));
                                        psVar1 = DAT_006d4988 + 0x10 + iVar8;
                                        W16(psVar1, RS16(psVar1) + -0x8000);
                                        psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                        W16(psVar1, RS16(psVar1) + -0x8000);
                                        DAT_00732e54[0x9d] = 0;
                                        DAT_00732e54[0x9c] = 2;
                                    }
                                } else {
                                    W16(DAT_00732e54 + 0xd, RS16(puVar14 + 0x2e));
                                    DAT_00732e54[0x9c] = 3;
                                }
                            } else if (DAT_00732e54[0x9c] == 2) {
                                if ((uVar13 < uStack_14) || (DAT_00732e54[0x9d] != 0)) {
                                    W16(DAT_00732e54 + 0xd, RS16(DAT_00732e54 + 0xd) + uVar13);
                                    if (0x7fff < RU16(DAT_00732e54 + 0xd)) {
                                        W16(DAT_00732e54 + 0xd, 0U - RU16(DAT_00732e54 + 0xd));
                                        psVar1 = DAT_006d4988 + 0x10 + iVar8;
                                        W16(psVar1, RS16(psVar1) + -0x8000);
                                        psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                        W16(psVar1, RS16(psVar1) + -0x8000);
                                        DAT_00732e54[0x9d] = 0;
                                        DAT_00732e54[0x9c] = 1;
                                    }
                                } else {
                                    W16(DAT_00732e54 + 0xd, RS16(puVar14 + 0x2e));
                                    DAT_00732e54[0x9c] = 3;
                                }
                            }
                        }
                        if ((DAT_00732e54[7] != 2) && (DAT_00732e54[0xab] != 0)) {
                            uVar13 = (unsigned short)(RS16(puVar14 + 0x32) -
                                                      RS16(DAT_006d4988 + 0x10 + iVar8));
                            local_10 = (local_10 & 0xffff0000U) | uVar13;
                            if (uVar13 != 0) {
                                uVar15 = FUN_00478320((unsigned int)((int)((unsigned int)RU16(DAT_00732e54 + 0xa7) *
                                                                           (unsigned int)DAT_0071e6be) / 0xec),
                                                      RU16(DAT_00732e54 + 0xa9));
                                uVar15 = FUN_00478320(uVar15, RU16(DAT_00732e54 + 0xac));
                                uVar17 = (unsigned short)uVar15;
                                if (uVar13 < 0x8000) {
                                    if (uVar17 < uVar13) {
                                        psVar1 = DAT_006d4988 + 0x10 + iVar8;
                                        W16(psVar1, RS16(psVar1) + uVar17);
                                    } else {
LAB_00444f8f:
                                        W16(DAT_006d4988 + 0x10 + iVar8, RS16(puVar14 + 0x32));
                                        uVar15 = uVar15 & 0xffff0000U;
                                        DAT_00732e54[0xab] = 3;
                                    }
                                } else {
                                    if ((unsigned short)(0U - uVar13) <= uVar17) goto LAB_00444f8f;
                                    psVar1 = DAT_006d4988 + 0x10 + iVar8;
                                    W16(psVar1, RS16(psVar1) - uVar17);
                                }
                                if ((DAT_00732e54[0xa4] == 0) || (DAT_00732e54[0xa4] == 4)) {
                                    uVar15 = FUN_00478320(uVar15,
                                                          RU16(&DAT_0052b095[(unsigned int)DAT_006d9d34 * 0xef]));
                                    if (uVar13 < 0x8000) {
                                        psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                        W16(psVar1, RS16(psVar1) - (short)uVar15);
                                    } else {
                                        psVar1 = DAT_006d4988 + 0x14 + iVar8;
                                        W16(psVar1, RS16(psVar1) + (short)uVar15);
                                    }
                                }
                            }
                        }
                    }
                    if ((((RI32(DAT_006d4988 + iVar8 + 0x1b) == -1) &&
                          (DAT_00732e54[0x96] == 1)) &&
                         (RS16(DAT_00732e54 + 0x1df) != 0)) &&
                        ((RI32(DAT_00732e54 + 0x19) == 0 &&
                          (RI32(puVar14 + 0x27) <= RI32(DAT_006d4988 + iVar8 + 0xc))))) {
                        DAT_00732e54[0x96] = 0;
                        W16(DAT_00732e54 + 0xd, 0x4000);
                    }
                    if (((RI32(DAT_006d4988 + 0x1b + iVar8) == -1) &&
                         (DAT_00732e54[0x97] == 1)) &&
                        ((RS16(DAT_00732e54 + 0x1df) != 0 && (RI32(DAT_00732e54 + 0x19) == 0)))) {
                        FUN_00445d10(uVar9);
                    }
                    switch (DAT_00732e54[7]) {
                    case 0:
                        W16(DAT_006d4988 + 0x12 + iVar8, RS16(DAT_00732e54 + 0xd));
                        uVar15 = (unsigned int)RU16(DAT_00732e54 + 0xc1);
                        if ((RU16(DAT_00732e54 + 0xc1) == 0) || (uVar22 != 0xffff)) {
                            sVar2 = RS16(DAT_00732e54 + 0x92);
                            uVar21 = (unsigned short)((unsigned int)(unsigned long)puVar14 >> 0x10);
                            sVar18 = (short)(((6 - (unsigned short)DAT_00732e54[0x234]) -
                                              (unsigned short)DAT_00732e54[0x212]) -
                                             (unsigned short)DAT_00732e54[0x20f]);
                            cVar3 = (char)DAT_006d4988[3 + iVar8];
                            if (cVar3 == '\a') {
                                uVar13 = 0x1000;
                            } else if ((cVar3 == '\x05') && (0 < sVar18)) {
                                uVar13 = 0x3000;
                            } else {
                                uVar13 = 0x2000;
                            }
                            uVar15 = FUN_00478320(((unsigned int)uVar21 << 16) | (unsigned short)sVar2, uVar13);
                            uVar15 = FUN_00478320(((unsigned int)uVar21 << 16) |
                                                      (unsigned short)(sVar2 + (short)uVar15 * sVar18),
                                                  uVar22);
                            if (RS16(DAT_00732e54 + 0xbf) == 0) {
                                uVar15 = (unsigned int)(unsigned short)((short)uVar15 * 2);
                            }
                        }
                        uVar13 = RU16(RPB(DAT_006d4988 + 0x1f + iVar8) + 0x79);
                        if ((unsigned short)uVar15 < uVar13) {
                            uVar20 = (unsigned int)uVar13 - (uVar15 & 0xffff);
                            if (uVar20 < 200) {
                                FUN_00445920(uVar9, uVar15 & 0xffff, 1, uVar22);
                            } else {
                                uVar15 = (uVar20 / 3) * (unsigned int)DAT_0071e6be;
                                uVar7 = uVar15 / 0xec;
                                iVar14 = (int)(uVar9 * 0x23);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                uVar22 = RU16(iVar19 + 0x7b);
                                local_10 = (local_10 & 0xffff0000U) | uVar22;
                                W16(iVar19 + 0x7b,
                                    uVar22 - (short)(((unsigned int)(uVar7 * 0xFFFFFF14U + uVar15) * 0x10000U) / 0xec));
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                if (uVar22 < RU16(iVar19 + 0x7b)) {
                                    psVar1 = iVar19 + 0x79;
                                    W16(psVar1, RS16(psVar1) + -1);
                                }
                                psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                W16(psVar1, RS16(psVar1) - (short)uVar7);
                                puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                if (0x8000 < RU16(puVar16)) {
                                    W16(puVar16, 0);
                                }
                            }
                        } else {
                            FUN_00445920(uVar9, uVar15 & 0xffff, 1, uVar22);
                        }
                        break;
                    case 1:
                    case 3:
                    case 4:
                        uVar22 = RU16(RPB(DAT_006d4988 + 0x1f + iVar8) + 0x79);
                        if (RU16(DAT_00732e54 + 0x92) < uVar22) {
                            uVar15 = (unsigned int)DAT_0071e6be *
                                     (unsigned int)((int)((unsigned int)uVar22 -
                                                          (unsigned int)RU16(DAT_00732e54 + 0x92)) / 3);
                            uVar7 = uVar15 / 0xec;
                            iVar14 = (int)(uVar9 * 0x23);
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            uVar22 = RU16(iVar19 + 0x7b);
                            local_10 = (local_10 & 0xffff0000U) | uVar22;
                            W16(iVar19 + 0x7b,
                                uVar22 - (short)(((unsigned int)(uVar7 * 0xFFFFFF14U + uVar15) * 0x10000U) / 0xec));
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            if (uVar22 < RU16(iVar19 + 0x7b)) {
                                psVar1 = iVar19 + 0x79;
                                W16(psVar1, RS16(psVar1) + -1);
                            }
                            psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            W16(psVar1, RS16(psVar1) - (short)uVar7);
                            puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            if (0x8000 < RU16(puVar16)) {
                                W16(puVar16, 0);
                            }
                        }
                        /* fall through */
                    case 6:
                        DAT_00732e54[0x96] = 0;
                        DAT_00732e54[0x97] = 0;
                        DAT_00732e54[0xa4] = 0;
                        DAT_00732e54[0x9c] = 0;
                        break;
                    case 2:
                        if (RS16(RPB(DAT_006d4988 + 0x1f + iVar8) + 0x79) != 0) {
                            uVar7 = ((unsigned int)DAT_0071e6be * 0x14U) / 0xec;
                            iVar14 = (int)(uVar9 * 0x23);
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            uVar22 = RU16(iVar19 + 0x7b);
                            sVar2 = (short)(uVar22 - (short)(((unsigned int)DAT_0071e6be * 0x140000U +
                                                              uVar7 * 0xFF140000U) / 0xec));
                            local_10 = (local_10 & 0xffff0000U) | (unsigned short)sVar2;
                            W16(iVar19 + 0x7b, sVar2);
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            if (uVar22 < RU16(iVar19 + 0x7b)) {
                                psVar1 = iVar19 + 0x79;
                                W16(psVar1, RS16(psVar1) + -1);
                            }
                            psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            W16(psVar1, RS16(psVar1) - (short)uVar7);
                            puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            if (0x8000 < RU16(puVar16)) {
                                W16(puVar16, 0);
                            }
                        }
                        break;
                    case 5:
                        if (RS16(DAT_00732e54 + 0x1df) == 0) {
                            if (RS16(RPB(DAT_006d4988 + 0x1f + iVar8) + 0x79) != 0) {
                                uVar7 = ((unsigned int)DAT_0071e6be * 0x14U) / 0xec;
                                iVar14 = (int)(uVar9 * 0x23);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                uVar22 = RU16(iVar19 + 0x7b);
                                sVar2 = (short)(uVar22 - (short)(((unsigned int)DAT_0071e6be * 0x140000U +
                                                                  uVar7 * 0xFF140000U) / 0xec));
                                local_10 = (local_10 & 0xffff0000U) | (unsigned short)sVar2;
                                W16(iVar19 + 0x7b, sVar2);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                if (uVar22 < RU16(iVar19 + 0x7b)) {
                                    psVar1 = iVar19 + 0x79;
                                    W16(psVar1, RS16(psVar1) + -1);
                                }
                                psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                W16(psVar1, RS16(psVar1) - (short)uVar7);
                                puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                if (0x8000 < RU16(puVar16)) {
                                    W16(puVar16, 0);
                                }
                            }
                            if (RS16(RPB(DAT_006d4988 + 0x1f + iVar8) + 0x79) == 0) {
                                DAT_00732e54[7] = 0;
                            }
                        } else if (RS16(puVar14 + 0x3a) == 0) {
                            if (RI32(puVar14 + 0x36) == 0) {
                                uVar7 = ((unsigned int)DAT_0071e6be * 500U) / 0xec;
                                iVar14 = (int)(uVar9 * 0x23);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                uVar22 = RU16(iVar19 + 0x7b);
                                local_10 = (unsigned int)(unsigned long)(iVar19 + 0x7b);
                                W16(iVar19 + 0x7b,
                                    (short)(((unsigned int)DAT_0071e6be * 0x1f40000U + uVar7 * 0xFF140000U) / 0xec) +
                                    uVar22);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                if (RU16(iVar19 + 0x7b) < uVar22) {
                                    psVar1 = iVar19 + 0x79;
                                    W16(psVar1, RS16(psVar1) + 1);
                                }
                                psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                W16(psVar1, RS16(psVar1) + (short)uVar7);
                                puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                if (0xe10 < RU16(puVar16)) {
                                    W16(puVar16, 0xe10);
                                }
                            } else {
                                uVar7 = ((unsigned int)DAT_0071e6be * 200U) / 0xec;
                                iVar14 = (int)(uVar9 * 0x23);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                uVar22 = RU16(iVar19 + 0x7b);
                                W16(iVar19 + 0x7b,
                                    (short)(((unsigned int)DAT_0071e6be * 0xc80000U + uVar7 * 0xFF140000U) / 0xec) +
                                    uVar22);
                                iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                                if (RU16(iVar19 + 0x7b) < uVar22) {
                                    psVar1 = iVar19 + 0x79;
                                    W16(psVar1, RS16(psVar1) + 1);
                                }
                                psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                W16(psVar1, RS16(psVar1) + (short)uVar7);
                                puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                                if (0xe10 < RU16(puVar16)) {
                                    W16(puVar16, 0xe10);
                                }
                            }
                        } else {
                            uVar7 = ((unsigned int)DAT_0071e6be * 0x32U) / 0xec;
                            iVar14 = (int)(uVar9 * 0x23);
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            uVar22 = RU16(iVar19 + 0x7b);
                            W16(iVar19 + 0x7b,
                                (short)(((unsigned int)DAT_0071e6be * 0x320000U + uVar7 * 0xFF140000U) / 0xec) +
                                uVar22);
                            iVar19 = RPB(DAT_006d4988 + 0x1f + iVar14);
                            if (RU16(iVar19 + 0x7b) < uVar22) {
                                psVar1 = iVar19 + 0x79;
                                W16(psVar1, RS16(psVar1) + 1);
                            }
                            psVar1 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            W16(psVar1, RS16(psVar1) + (short)uVar7);
                            puVar16 = RPB(DAT_006d4988 + 0x1f + iVar14) + 0x79;
                            if (0xe10 < RU16(puVar16)) {
                                W16(puVar16, 0xe10);
                            }
                        }
                    }
                    W16(DAT_00732e54 + 0xf, RS16(DAT_006d4988 + 0x10 + iVar8));
                    iVar19 = DAT_006d4988 + iVar8;
                    if (((RS16(iVar19 + 0x12) != sVar4) || (RS16(iVar19 + 0x10) != sVar5)) ||
                        (RS16(iVar19 + 0x14) != sVar6)) {
                        RPB(iVar19 + 0x1f)[0x89] = 1;
                        RPB(DAT_006d4988 + 0x1f + iVar8)[0x90] = 1;
                    }
                    if (RU16(DAT_00732e54 + 0x66) != 0xffff) {
                        iVar14 = (int)((unsigned int)RU16(DAT_00732e54 + 0x66) * 0x23);
                        if (RI32(DAT_006d4988 + iVar14 + 0x1f) != 0) {
                            W16(DAT_006d4988 + iVar14 + 0x12, RS16(DAT_006d4988 + 0x12 + iVar8));
                            W16(DAT_006d4988 + 0x10 + iVar14, RS16(DAT_006d4988 + 0x10 + iVar8));
                            W16(DAT_006d4988 + 0x14 + iVar14, RS16(DAT_006d4988 + 0x14 + iVar8));
                            W16(RPB(RPB(DAT_006d4988 + 0x1f + iVar14) + 0xa7) + 0xd,
                                RS16(DAT_00732e54 + 0xd));
                            RPB(DAT_006d4988 + 0x1f + iVar14)[0x89] = 1;
                            RPB(DAT_006d4988 + 0x1f + iVar14)[0x90] = 1;
                        }
                    }
                }
            } else {
                DAT_0071e6be = (unsigned short)(uVar12 + ((short)DAT_0071b71c - (short)iVar14));
                if (DAT_0071e6be != 0) {
                    DAT_0070a1d2 = (short)(0xecU / (unsigned int)DAT_0071e6be);
                    if (DAT_0070a1d2 == 0) {
                        DAT_0070a1d2 = 1;
                    }
                    goto LAB_00444b21;
                }
            }
            local_16 = (unsigned short)(local_16 + 1);
        } while ((int)(unsigned int)local_16 < DAT_006d4980);
    }
    DAT_0070a1d2 = sVar11;
    DAT_0071e6be = uVar12;
    return;
}
