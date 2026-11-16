/* SPDX-License-Identifier: MIT */
/* z_bop 00446c60 */
/* Packed-field access: the 0x23-byte entries put these fields at unaligned addresses. */
#define LD(T, p) ({ T ld_t_; __builtin_memcpy(&ld_t_, (p), sizeof(T)); ld_t_; })
#define ST(T, p, v) do { T st_t_ = (T)(v); __builtin_memcpy((p), &st_t_, sizeof(T)); } while (0)
#define OBJ(o) LD(unsigned char *, DAT_009a1fe8 + 0x1f + (o))

extern unsigned char *DAT_009a1fe8;
extern unsigned short DAT_009ec5fe;
extern unsigned char DAT_009a73d8;
extern int DAT_00a08140;
extern int DAT_00a08144;
extern int DAT_00a08148;
extern unsigned char DAT_009d8cde[];
extern unsigned char DAT_009d8cdf[];
extern unsigned char DAT_009d8ce0[];
extern unsigned char DAT_009d8ce1[];
extern unsigned char DAT_009d8ce2[];
extern unsigned char DAT_009d8cd8[];
extern unsigned char DAT_009d8cd9[];
extern unsigned char DAT_009d8cda[];
extern unsigned char DAT_009d8cdb[];
extern unsigned char DAT_009d8cdc[];
extern unsigned char DAT_009d8c87[];
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern unsigned int DAT_009a8c00;
extern unsigned short DAT_009d8112;
extern unsigned short DAT_00a080f0;
extern unsigned short DAT_009a8070;
extern unsigned char s_starshipprotectpln_0051a2fc[];
extern unsigned char s_formevadeldr1pln_0051a394[];
extern unsigned char DAT_0051a194[];
extern int DAT_009a8d84;
extern int DAT_009a8d88;
extern int DAT_009a8da4;
extern int DAT_009ecc34;
extern unsigned short DAT_009e9640;
extern unsigned char *DAT_009a8e18;

unsigned short __cdecl FUN_00461690(unsigned short, unsigned short, unsigned short, unsigned short, unsigned short, int);
void __cdecl FUN_00458e20(unsigned int, int);
unsigned int __cdecl FUN_0041f410(int, int, int);
void __cdecl FUN_0046a8d0(int, int, int);
unsigned short __cdecl FUN_00425cb0(void);
unsigned int __cdecl FUN_00425b70(unsigned int, unsigned short);
unsigned short __cdecl FUN_00459750(int);
short __cdecl FUN_00405860(int);
void __cdecl FUN_00427bd0(unsigned short, unsigned short, unsigned char *);
void __cdecl FUN_0042e720(unsigned int, int);

void __cdecl FUN_00446c60(unsigned short param_1)
{
    unsigned char bVar1;
    long long lVar2;
    long long lVar3;
    long long lVar5;
    int iVar4;
    unsigned short uVar5;
    unsigned short uVar6;
    short sVar7;
    int iVar8;
    int iVar9;
    unsigned int uVar10;
    int iVar11;
    unsigned short uVar12;
    int iVar13;
    int iVar14;
    unsigned short uVar15;
    int local_3c;
    int local_38;
    int local_14;
    int local_10;
    unsigned short local_c;
    unsigned short uStack_8;
    unsigned char *pEnt;
    unsigned char *pTgt;
    unsigned char *pObj;

    iVar13 = (int)((unsigned int)param_1 * 0x23);
    pEnt = DAT_009a1fe8 + iVar13;
    if (LD(unsigned short, pEnt + 0x17) == 0) {
        return;
    }
    if ((unsigned short)(DAT_009ec5fe >> 1) < (unsigned short)pEnt[0x1a]) {
        pEnt[0x1a] = (unsigned char)(pEnt[0x1a] - (unsigned char)(DAT_009ec5fe >> 1));
        return;
    }
    pEnt[0x1a] = (unsigned char)-0x14;
    pEnt = DAT_009a1fe8 + iVar13;
    iVar14 = LD(int, pEnt + 4);
    local_14 = LD(int, pEnt + 8);
    local_10 = LD(int, pEnt + 0xc);
    DAT_009a73d8 = (pEnt[3] == 'L');
    iVar8 = (int)((unsigned int)pEnt[0x16] * 0x566);
    DAT_00a08140 = local_14;
    DAT_00a08144 = iVar14;
    DAT_00a08148 = local_10;
    uStack_8 = FUN_00461690((unsigned short)DAT_009d8cde[iVar8], (unsigned short)DAT_009d8cdf[iVar8],
                            (unsigned short)DAT_009d8ce2[iVar8], (unsigned short)DAT_009d8ce0[iVar8],
                            (unsigned short)DAT_009d8ce1[(unsigned int)pEnt[0x16] * 0x566], 0);
    if (uStack_8 == 0xffff) {
        uStack_8 = FUN_00461690((unsigned short)DAT_009d8cd8[iVar8],
                                (unsigned short)DAT_009d8cda[iVar8],
                                (unsigned short)DAT_009d8cdc[iVar8],
                                (unsigned short)DAT_009d8cd9[iVar8],
                                (unsigned short)DAT_009d8cdb[iVar8], 0);
    }
    if (uStack_8 == 0xffff) {
        return;
    }
    FUN_00458e20((unsigned int)uStack_8, 0);
    local_38 = DAT_00a07cdc;
    local_3c = DAT_00a07cd8;
    iVar9 = DAT_00a07cd4;
    uVar10 = FUN_0041f410((int)((unsigned int)DAT_00a07cdc - (unsigned int)iVar14),
                          (int)((unsigned int)DAT_00a07cd8 - (unsigned int)local_14),
                          (int)((unsigned int)DAT_00a07cd4 - (unsigned int)local_10));
    if (0xffff < uVar10) {
        return;
    }
    iVar4 = (int)((unsigned int)uStack_8 * 0x23);
    pTgt = DAT_009a1fe8 + iVar4;
    if (LD(int, pTgt + 0x1f) != 0) {
        FUN_0046a8d0((int)((unsigned int)LD(int, pTgt + 4) - (unsigned int)iVar14),
                     (int)((unsigned int)LD(int, pTgt + 8) - (unsigned int)local_14),
                     (int)((unsigned int)LD(int, pTgt + 0xc) - (unsigned int)local_10));
        if (DAT_009a1fe8[3 + iVar13] == 'L') {
            DAT_009a8c00 = (unsigned int)((int)((unsigned int)DAT_009d8112 * DAT_009a8c00) >> 0xf);
        } else {
            DAT_009a8c00 = (unsigned int)((int)((unsigned int)DAT_009d8112 * DAT_009a8c00) >> 0xe);
        }
        sVar7 = (short)(unsigned short)DAT_009a8c00;
        uVar5 = FUN_00425cb0();
        pTgt = DAT_009a1fe8 + iVar4;
        uVar10 = (unsigned int)(unsigned short)((sVar7 + (uVar5 & 3)) - 1);
        local_38 = (int)((unsigned int)LD(int, pTgt + 4) +
                         ((unsigned int)LD(int, pTgt + 4) -
                          (unsigned int)LD(int, LD(unsigned char *, pTgt + 0x1f) + 6)) * uVar10);
        local_3c = (int)((unsigned int)LD(int, pTgt + 8) +
                         ((unsigned int)LD(int, pTgt + 8) -
                          (unsigned int)LD(int, LD(unsigned char *, pTgt + 0x1f) + 10)) * uVar10);
        iVar9 = (int)((unsigned int)LD(int, pTgt + 0xc) +
                      ((unsigned int)LD(int, pTgt + 0xc) -
                       (unsigned int)LD(int, LD(unsigned char *, pTgt + 0x1f) + 0xe)) * uVar10);
    }
    FUN_0046a8d0((int)((unsigned int)local_38 - (unsigned int)iVar14),
                 (int)((unsigned int)local_3c - (unsigned int)local_14),
                 (int)((unsigned int)iVar9 - (unsigned int)local_10));
    uVar5 = DAT_00a080f0;
    local_c = DAT_009a8070;
    bVar1 = DAT_009a1fe8[3 + iVar13];
    iVar9 = (int)(((unsigned int)iVar13 & 0xffffff00u) | (unsigned int)bVar1);
    uVar10 = (unsigned int)(unsigned short)((-(int)(unsigned short)(0x4c < bVar1) & 0x14) + 0x96);
    if (DAT_00a080f0 < 0x2000) {
        iVar9 = (int)((unsigned int)local_10 + uVar10);
        local_10 = iVar9;
    } else if (DAT_00a080f0 < 0x6001) {
        if ((0x1fff < DAT_009a8070) && (DAT_009a8070 < 0xe001)) {
            if (DAT_009a8070 < 0x6000) {
                iVar14 = (int)((unsigned int)iVar14 + uVar10);
                goto LAB_00446ff4;
            }
            if (0x9fff < DAT_009a8070) {
                iVar14 = (int)((unsigned int)iVar14 - uVar10);
                goto LAB_00446ff4;
            }
            uVar10 = 0u - uVar10;
        }
        iVar9 = (int)((unsigned int)local_14 + uVar10);
        local_14 = iVar9;
    } else {
        if (0x4c < bVar1) {
            return;
        }
        iVar9 = (int)((unsigned int)local_10 - uVar10);
        local_10 = iVar9;
    }
LAB_00446ff4:
    uVar6 = 0xffff;
    if (DAT_009a8c00 < 0x10000) {
        uVar6 = (unsigned short)DAT_009a8c00;
    }
    iVar11 = LD(int, DAT_009a1fe8 + 0x1f + iVar4);
    if (iVar11 == 0) {
        uVar12 = 0xffff;
    } else {
        uVar15 = LD(unsigned short, (unsigned char *)iVar11 + 0x79);
        uVar12 = 0xffff;
        if (0xbb < uVar15) {
            uVar12 = (unsigned short)((int)uVar15 * -0x80 + 0x5dff);
        }
    }
    uVar10 = FUN_00425b70((((unsigned int)iVar9 >> 0x10) << 0x10) | (unsigned int)(unsigned short)~uVar6,
                          uVar12);
    uVar6 = FUN_00425cb0();
    if ((unsigned short)uVar10 < uVar6) {
        sVar7 = (short)FUN_00425cb0();
        uVar6 = FUN_00425cb0();
        uVar15 = (unsigned short)((unsigned int)(int)sVar7 - 0x100U & 0x3ff);
        if (0x7fff < uVar6) {
            uVar15 = (unsigned short)-(int)uVar15;
        }
        local_c = (unsigned short)(local_c + uVar15);
        sVar7 = (short)FUN_00425cb0();
        uVar6 = FUN_00425cb0();
        uVar15 = (unsigned short)((unsigned int)(int)sVar7 - 0x100U & 0x3ff);
        if (uVar6 < 0x8000) {
            uVar5 = (unsigned short)(uVar5 + uVar15);
            if ((uVar5 & 0x8000) != 0) {
                uVar5 = 0x7fff;
            }
        } else {
            uVar5 = (unsigned short)(uVar5 - uVar15);
            if ((uVar5 & 0x8000) != 0) {
                uVar5 = 0;
            }
        }
    }
    uVar6 = FUN_00459750(7);
    if (uVar6 != 0xffff) {
        iVar9 = (int)((unsigned int)uVar6 * 0x23);
        *OBJ(iVar9) = 1;
        DAT_009a1fe8[2 + iVar9] = 7;
        if (DAT_009a1fe8[3 + iVar13] == 'L') {
            iVar11 = 0x8e;
        } else {
            iVar11 = 0x8a;
            if (DAT_009d8c87[iVar8] == 1 || DAT_009d8c87[iVar8] == 4) {
                iVar11 = 0x8c;
            }
        }
        DAT_009a1fe8[3 + iVar9] = (unsigned char)iVar11;
        ST(unsigned short, OBJ(iVar9) + 0x83, 1);
        ST(unsigned short, OBJ(iVar9) + 0x85, param_1);
        OBJ(iVar9)[0x87] = 0;
        OBJ(iVar9)[0x88] = DAT_009d8c87[(unsigned int)DAT_009a1fe8[0x16 + iVar13] * 0x566];
        ST(unsigned short, DAT_009a1fe8 + 0x12 + iVar9, uVar5);
        ST(unsigned short, DAT_009a1fe8 + 0x14 + iVar9, 0);
        ST(unsigned short, DAT_009a1fe8 + 0x10 + iVar9, local_c);
        OBJ(iVar9)[0x92] = 1;
        pObj = OBJ(iVar9);
        pObj[0x8b] = pObj[0x92];
        ST(unsigned short, OBJ(iVar9) + 0x79,
           LD(unsigned short, s_starshipprotectpln_0051a2fc + iVar11 * 2 + 10) >> 1);
        sVar7 = FUN_00405860(iVar11);
        ST(short, OBJ(iVar9) + 0x81, sVar7 * 2);
        ST(unsigned int, OBJ(iVar9) + 0x7d, LD(unsigned int, DAT_0051a194 + iVar11 * 4));
        FUN_00427bd0(uVar5, local_c, DAT_009a1fe8 + iVar9);
        ST(int, OBJ(iVar9) + 6, iVar14);
        ST(int, OBJ(iVar9) + 10, local_14);
        ST(int, OBJ(iVar9) + 0xe, local_10);
        iVar13 = (int)LD(short, s_formevadeldr1pln_0051a394 + iVar11 * 2 + 2);
        lVar2 = (long long)DAT_009a8d84;
        lVar3 = (long long)DAT_009a8d88;
        lVar5 = (long long)DAT_009a8da4 * (long long)iVar13;
        ST(unsigned int, DAT_009a1fe8 + 4 + iVar9,
           (unsigned int)iVar14 +
           ((unsigned int)lVar5 >> 0xf |
            (unsigned int)(int)((unsigned long long)lVar5 >> 0x20) << 0x11));
        lVar5 = lVar2 * iVar13;
        ST(unsigned int, DAT_009a1fe8 + 8 + iVar9,
           (unsigned int)local_14 +
           ((unsigned int)lVar5 >> 0xf |
            (unsigned int)(int)((unsigned long long)lVar5 >> 0x20) << 0x11));
        lVar5 = lVar3 * iVar13;
        ST(unsigned int, DAT_009a1fe8 + 0xc + iVar9,
           (unsigned int)local_10 +
           ((unsigned int)lVar5 >> 0xf |
            (unsigned int)(int)((unsigned long long)lVar5 >> 0x20) << 0x11));
        FUN_0042e720((unsigned int)uVar6, DAT_009ecc34);
        iVar13 = (int)((unsigned int)(unsigned short)(uVar6 - (short)DAT_009e9640) * 10);
        DAT_009a8e18[1 + iVar13] = 0;
        ST(unsigned short, DAT_009a8e18 + 4 + iVar13, uStack_8);
        if ((uStack_8 == 0xffff) || (0x7fff < uStack_8)) {
            ST(unsigned short, DAT_009a8e18 + 6 + iVar13, 0);
        } else {
            ST(unsigned short, DAT_009a8e18 + 6 + iVar13,
               LD(unsigned short, DAT_009a1fe8 + iVar4));
        }
        DAT_009a8e18[iVar13] = 0xff;
        ST(unsigned char *, OBJ(iVar9) + 0xa5, DAT_009a8e18 + iVar13);
    }
    return;
}
