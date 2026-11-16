/* SPDX-License-Identifier: MIT */
/* z_xvt 0049f010 */
/* Globals: only their addresses/widths are known from the evidence. */
extern unsigned int DAT_006d4988;
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_005247e9[];
extern unsigned char DAT_0052686b[];
extern unsigned char DAT_00524860[];
extern unsigned char DAT_0052686a[];
extern unsigned char DAT_0052486c[];
extern unsigned char DAT_0070ad47[];
extern unsigned char DAT_0070ad92[];
extern char DAT_0070ad40[];
extern unsigned char DAT_0052b0b4[];
extern unsigned char DAT_0052b0b6[];
extern unsigned char DAT_0070ad4f[];
extern unsigned char DAT_0070ad49[];
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0071b73e[];
extern unsigned char DAT_0070ad54[];
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern unsigned char DAT_0070b216[];

unsigned int __cdecl FUN_00478320(unsigned int param_1, unsigned int param_2);

unsigned char __cdecl FUN_0049f010(unsigned int param_1, unsigned short param_2, unsigned short param_3)
{
    unsigned char *iVar8;
    unsigned int iVar6;
    unsigned char *iVar10;
    unsigned int iVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned char bVar9;
    unsigned short local_a;
    unsigned short uVar4;
    unsigned short uVar7;
    unsigned int uVar5;
    short sVar11;

    iVar8 = (unsigned char *)(((param_1 & 0xffff) * 0x23) + DAT_006d4988);
    __builtin_memcpy(&iVar6, iVar8 + 0x1f, 4);
    if (iVar6 == 0) {
        /* original uses uninitialized stack halves here; never read when iVar6 == 0 */
        iVar10 = (unsigned char *)0;
        bVar2 = DAT_0070ad48[(unsigned int)iVar8[0x16] * 0x566];
    } else {
        __builtin_memcpy(&iVar10, (unsigned char *)iVar6 + 0xa7, 4);
        bVar2 = *((unsigned char *)iVar6 + 0x87);
    }
    local_a = (unsigned short)iVar8[0x16];
    bVar9 = 0;
    iVar1 = (unsigned int)local_a * 0x566;
    bVar3 = DAT_005247e8[DAT_0070ad42[(unsigned int)local_a * 0x566]];
    switch (param_2) {
    case 0:
        return 0;
    case 1:
        if (local_a == param_3) {
            return 1;
        }
        break;
    case 2:
        if (DAT_005247e9[param_3] == bVar3) {
            return 1;
        }
        break;
    case 3:
        if (DAT_0052686b[(unsigned int)bVar3 * 0x16] == DAT_00524860[param_3]) {
            return 1;
        }
        break;
    case 4:
        if (DAT_0052686a[(unsigned int)bVar3 * 0x16] == DAT_0052486c[param_3]) {
            return 1;
        }
        break;
    case 5:
        if (iVar6 == 0) {
            if (DAT_0070ad47[iVar1] == param_3) {
                return 1;
            }
        } else if (*((unsigned char *)iVar6 + 0x86) == param_3) {
            return 1;
        }
        break;
    case 6:
        if (DAT_0070ad92[iVar1] == param_3) {
            return 1;
        }
        break;
    case 7:
        if (iVar6 != 0) {
            switch (param_3) {
            case 0:
                if (iVar10[0x1e5] != 0) {
                    return 1;
                }
                break;
            case 1:
                uVar4 = 0;
                do {
                    if (((unsigned int)*((unsigned char *)iVar6 + 0x87) != (unsigned int)uVar4) &&
                        (iVar10[(unsigned int)uVar4 + 0x1f0] != 0)) {
                        bVar9 = 1;
                    }
                    uVar4 = (unsigned short)(uVar4 + 1);
                } while (uVar4 < 10);
                return bVar9;
            case 2:
                if (iVar10[0x7b] != 0) {
                    return 1;
                }
                break;
            case 3:
                if (iVar10[0x7d] != 0) {
                    return 1;
                }
                break;
            case 4: {
                unsigned short t;
                __builtin_memcpy(&t, iVar10 + 0x1df, 2);
                if (t == 0) {
                    return 1;
                }
                break;
            }
            case 5:
                uVar4 = 0;
                do {
                    if (((unsigned int)*((unsigned char *)iVar6 + 0x87) != (unsigned int)uVar4) &&
                        (iVar10[(unsigned int)uVar4 + 0x1e6] != 0)) {
                        bVar9 = 1;
                    }
                    uVar4 = (unsigned short)(uVar4 + 1);
                } while (uVar4 < 10);
                return bVar9;
            case 6: {
                unsigned short t;
                __builtin_memcpy(&t, iVar10 + 0xc3, 2);
                if (t != 0) {
                    return 1;
                }
                break;
            }
            case 7:
                if (DAT_0070ad40[iVar1] == (char)iVar10[0xb0]) {
                    return 1;
                }
                break;
            case 8:
                if (DAT_0070ad40[iVar1] != (char)iVar10[0xb0]) {
                    return 1;
                }
                break;
            case 9: {
                int t;
                __builtin_memcpy(&t, iVar8 + 0x1b, 4);
                if (t != -1) {
                    return 1;
                }
                break;
            }
            case 10: {
                int t;
                __builtin_memcpy(&t, iVar8 + 0x1b, 4);
                if (t == -1) {
                    return 1;
                }
                break;
            }
            case 0xb:
                uVar5 = 0;
                do {
                    if (iVar10[uVar5 + 0x1e6] == 0) {
                        bVar9 = 1;
                    }
                    uVar4 = (unsigned short)((short)uVar5 + 1);
                    uVar5 = (unsigned int)uVar4;
                } while (uVar4 < 10);
                return bVar9;
            case 0xc: {
                unsigned short t;
                __builtin_memcpy(&t, iVar10 + 0x1df, 2);
                if (t != 0) {
                    return 1;
                }
                break;
            }
            case 0xd:
                if (iVar10[0x1e5] == 0) {
                    return 1;
                }
                break;
            case 0xe:
                uVar5 = 0;
                do {
                    if (iVar10[uVar5 + 0x1f0] == 0) {
                        bVar9 = 1;
                    }
                    uVar4 = (unsigned short)((short)uVar5 + 1);
                    uVar5 = (unsigned int)uVar4;
                } while (uVar4 < 10);
                return bVar9;
            case 0xf:
                return 0;
            case 0x10:
                if (iVar10[0x7b] == 0) {
                    return 1;
                }
                break;
            case 0x11:
                return 0;
            case 0x12:
                if (iVar10[0x7d] == 0) {
                    return 1;
                }
                break;
            case 0x16: {
                unsigned short a, b;
                __builtin_memcpy(&a, iVar10 + 199, 2);
                __builtin_memcpy(&b, iVar10 + 0xc3, 2);
                if ((unsigned int)(a >> 2) <= (unsigned int)b) {
                    return 1;
                }
                break;
            }
            case 0x17: {
                unsigned short a, b;
                __builtin_memcpy(&a, iVar10 + 199, 2);
                __builtin_memcpy(&b, iVar10 + 0xc3, 2);
                if ((unsigned int)(a >> 1) <= (unsigned int)b) {
                    return 1;
                }
                break;
            }
            case 0x18: {
                unsigned short a, b;
                __builtin_memcpy(&a, iVar10 + 199, 2);
                __builtin_memcpy(&b, iVar10 + 0xc3, 2);
                uVar5 = FUN_00478320((unsigned int)a, 0xc000);
                if ((unsigned short)uVar5 <= b) {
                    return 1;
                }
                break;
            }
            case 0x19:
                sVar11 = 0;
                if (iVar10[0x228] != 0) {
                    uVar4 = 0;
                    do {
                        unsigned int idx;
                        uVar7 = (unsigned short)(uVar4 + 1);
                        idx = (unsigned int)iVar10[4] * 0xef + (unsigned int)uVar4;
                        sVar11 = (short)((unsigned int)(unsigned short)sVar11 +
                                         (unsigned int)(unsigned short)iVar10[0x346 + (unsigned int)DAT_0052b0b4[idx] * 4] +
                                         (unsigned int)(unsigned short)iVar10[0x346 + (unsigned int)DAT_0052b0b6[idx] * 4]);
                        uVar4 = uVar7;
                    } while (uVar7 < iVar10[0x228]);
                }
                if (sVar11 == 0) {
                    return 1;
                }
            }
        }
        break;
    case 8:
        if (DAT_0070ad4f[iVar1] == param_3) {
            return 1;
        }
        break;
    case 9:
        if (DAT_0070ad49[iVar1] == param_3) {
            return 1;
        }
        break;
    case 10:
        if (DAT_0070ad44[iVar1] == param_3) {
            return 1;
        }
        break;
    case 0xb:
        return 1;
    case 0xc:
        if (bVar2 == param_3) {
            return 1;
        }
        break;
    case 0xd: {
        unsigned int t;
        unsigned short w;
        __builtin_memcpy(&t, iVar8 + 0x1b, 4);
        __builtin_memcpy(&w, DAT_0071b73e + t * 0x5bd, 2);
        if ((unsigned int)DAT_0070ad54[(unsigned int)w * 0x566] - (unsigned int)param_3 == 1) {
            return 1;
        }
        break;
    }
    case 0xe:
        uVar5 = (unsigned int)(unsigned short)(param_3 * 5) / 0x3c;
        if (uVar5 < DAT_0071f2dc) {
            return 0;
        }
        if (DAT_0071f2dc < uVar5) {
            return 1;
        }
        return (unsigned char)(1 - ((unsigned int)(unsigned short)(param_3 * 5) % 0x3c < (unsigned int)DAT_0071f2dd));
    case 0xf:
        if (local_a != param_3) {
            return 1;
        }
        break;
    case 0x10:
        if (DAT_005247e9[param_3] != bVar3) {
            return 1;
        }
        break;
    case 0x11:
        if (DAT_0052686b[(unsigned int)bVar3 * 0x16] != DAT_00524860[param_3]) {
            return 1;
        }
        break;
    case 0x12:
        if (DAT_0052686a[(unsigned int)bVar3 * 0x16] != DAT_0052486c[param_3]) {
            return 1;
        }
        break;
    case 0x13:
        if (iVar6 == 0) {
            if (DAT_0070ad47[iVar1] != param_3) {
                return 1;
            }
        } else if (*((unsigned char *)iVar6 + 0x86) != param_3) {
            return 1;
        }
        break;
    case 0x14:
        if (DAT_0070ad4f[iVar1] != param_3) {
            return 1;
        }
        break;
    case 0x15:
        if (bVar2 != param_3) {
            return 1;
        }
        break;
    case 0x16: {
        unsigned int t;
        __builtin_memcpy(&t, iVar8 + 0x1b, 4);
        if (t != (unsigned int)param_3) {
            return 1;
        }
        break;
    }
    case 0x17:
        if (DAT_0070b216[iVar1] == param_3) {
            return 1;
        }
        break;
    case 0x18:
        if (DAT_0070b216[iVar1] != param_3) {
            bVar9 = 1;
        }
    }
    return bVar9;
}
