/* SPDX-License-Identifier: MIT */
/* z_xvt 004544b0 */
extern unsigned char *DAT_00732e54;
extern unsigned char DAT_006d9d56[4];
extern unsigned short DAT_006d9d50;
extern unsigned short DAT_006d9d60;
extern unsigned char *DAT_006d4988;
extern int DAT_006d4980;
extern unsigned int DAT_006db580;
extern int DAT_0070ab14;
extern int DAT_0070ab18;
extern int DAT_0070ac20;
extern unsigned short DAT_00732a3c;

extern unsigned char DAT_0070ad8c[];
extern unsigned char DAT_0070ad8e[];
extern unsigned char DAT_0070ad8f[];
extern unsigned char DAT_0070ad90[];
extern char DAT_0070ad40[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_006d4a8a[];
extern unsigned char DAT_006d4a98[];
extern unsigned char DAT_006d4a9a[];
extern unsigned char DAT_006d4aa0[];
extern unsigned char DAT_006d4ab2[];
extern unsigned char DAT_006d4ab9[];
extern unsigned char DAT_006d4aba[];
extern unsigned char DAT_006d4abd[];
extern char DAT_006d4ae6[];
extern char DAT_006d4af0[];
extern char DAT_006d4afa[];
extern unsigned char DAT_006d4b04[];
extern unsigned char DAT_0052b14b[];
extern unsigned char DAT_0052b14d[];
extern unsigned char DAT_0052b14f[];

unsigned long long FUN_00429880(unsigned short a);
void FUN_0042a230(int a, unsigned int b, int c, short d, short e, unsigned short f);
void FUN_0042a3a0(void);
void FUN_00498450(unsigned int a, unsigned int b);
int FUN_0042b150(int a);
void FUN_004a17b0(unsigned int a, unsigned int b, unsigned int c);
void FUN_004a34c0(unsigned int a, void *b, unsigned int c);
void FUN_0049f9c0(unsigned short a, unsigned short b, unsigned short c);
unsigned int FUN_00421580(int a);

/* unaligned accessors (memcpy, per packed-field rule) */
#define LD32(d, p) __builtin_memcpy(&(d), (const void *)(unsigned int)(p), 4)
#define LD16(d, p) __builtin_memcpy(&(d), (const void *)(unsigned int)(p), 2)
#define ST32(p, v) do { unsigned int st_ = (unsigned int)(v); __builtin_memcpy((void *)(unsigned int)(p), &st_, 4); } while (0)
#define LDG() __builtin_memcpy(&g, DAT_006d9d56, 4)

unsigned int __stdcall FUN_004544b0(void)
{
    int iVar1;
    int iVar2;
    int iVar9;
    int iVar10;
    char cVar3;
    char cVar4;
    unsigned char bVar5;
    unsigned char *puVar6;
    unsigned short uVar7;
    unsigned int uVar8;
    unsigned int uVar11;
    unsigned short uVar12;
    unsigned short uVar14;
    unsigned int uVar15;
    unsigned char *pcVar13;
    unsigned int unaff_EBP;
    unsigned int bVar16;
    unsigned long long uVar17;
    unsigned short local_c;
    unsigned short local_8;
    unsigned short extraout_var = 0;
    unsigned short extraout_var_00 = 0;
    unsigned char *g;
    unsigned int t32;
    unsigned short t16;
    unsigned short s1;
    unsigned short s2;

    DAT_00732e54[0xaf] = 1;
    LDG();
    ST32(g + 0xe, 0x1d);
    if (DAT_00732e54[0x1e5] == 0) {
        uVar17 = FUN_00429880((unsigned short)DAT_0070ad8c[(unsigned int)DAT_006d9d60 * 0x566]);
        uVar14 = (unsigned short)uVar17;
        local_c = 0x12;
        if (uVar14 == 0xffff) {
            if (DAT_0070ad8f[(unsigned int)DAT_006d9d60 * 0x566] != 0) {
                uVar17 = FUN_00429880((unsigned short)DAT_0070ad8e[(unsigned int)DAT_006d9d60 * 0x566]);
                uVar14 = (unsigned short)uVar17;
                local_c = 0x13;
            }
            goto LAB_004545a2;
        }
    } else {
        uVar17 = FUN_00429880((unsigned short)DAT_0070ad90[(unsigned int)DAT_006d9d60 * 0x566]);
        uVar14 = (unsigned short)uVar17;
        local_c = 0x14;
LAB_004545a2:
        if (uVar14 != 0xffff) goto LAB_004545ad;
        goto LAB_after;
    }
LAB_004545ad:
    LDG();
    *(unsigned short *)(g + 0x18) = uVar14;
    iVar1 = (int)((unsigned int)uVar14 * 0x23);
    LD16(t16, DAT_006d4988 + iVar1);
    LDG();
    *(unsigned short *)(g + 0x1a) = t16;
    LDG();
    g[0x1e] = 0;
    iVar10 = (int)(DAT_006d4988 + iVar1);
    LD32(t32, iVar10 + 0x1f);
    LD32(t32, t32 + 0xa7);
    iVar9 = (int)((unsigned int)((unsigned char *)t32)[4] * 0xef);
    LD16(t16, &DAT_0052b14f[iVar9]);
    LD16(s1, &DAT_0052b14b[iVar9]);
    LD16(s2, &DAT_0052b14d[iVar9]);
    FUN_0042a230(iVar10, (unsigned int)t16, iVar10, (short)s1, (short)s2, t16);
    LD32(t32, DAT_006d4988 + 4 + iVar1);
    t32 = t32 + (unsigned int)DAT_0070ab18;
    LDG();
    ST32(g + 0x1f, t32);
    unaff_EBP = (unsigned int)DAT_0070ab14;
    LD32(t32, DAT_006d4988 + 8 + iVar1);
    t32 = t32 + (unsigned int)DAT_0070ab14;
    LDG();
    ST32(g + 0x23, t32);
    LD32(t32, DAT_006d4988 + 0xc + iVar1);
    t32 = t32 + (unsigned int)DAT_0070ac20;
    LDG();
    ST32(g + 0x27, t32);

LAB_after:
    FUN_0042a3a0();
    if (uVar14 == 0xffff) {
        FUN_00498450((unsigned int)DAT_006d9d50, 0x23);
        return ((unsigned int)extraout_var << 16) | 1;
    }
    LD32(t32, DAT_006d4988 + (unsigned int)uVar14 * 0x23 + 0x1f);
    LD16(uVar14, t32 + 0x79);
    if (uVar14 < 0x19) {
        uVar8 = 0x28;
    } else {
        uVar8 = (unsigned int)uVar14 + 0x19;
    }
    FUN_00498450((unsigned int)DAT_006d9d50, uVar8);
    uVar8 = DAT_006db580;
    if (0x1ff < (int)DAT_006db580) goto LAB_00454d10;
    uVar8 = (((unaff_EBP >> 0x10) & 0xffff) << 0x10) | (unsigned int)DAT_00732a3c;
    if ((int)(unsigned int)DAT_00732a3c < DAT_006d4980) {
        do {
            iVar1 = (int)((uVar8 & 0xffff) * 0x23);
            iVar9 = (int)(DAT_006d4988 + iVar1);
            if ((((unsigned char *)iVar9)[3] != 0) &&
                (((unsigned char *)iVar9)[0x16] == DAT_006d9d60)) {
                LD32(t32, iVar9 + 0x1f);
                LD32(t32, t32 + 0xa7);
                puVar6 = (unsigned char *)t32;
                iVar9 = FUN_0042b150((int)puVar6);
                if ((char)puVar6[5] != -1) {
                    if ((char)((unsigned char *)iVar9)[0x34] == '\n') {
                        LD32(t32, DAT_006d4988 + 0x1b + iVar1);
                        if ((int)t32 == -1) {
                            if (puVar6[0x1e5] == 0) {
                                if ((((unsigned char *)iVar9)[9] == 0) &&
                                    ((puVar6[0x72] != 0) ||
                                     ((puVar6[0x73] == 0) && (puVar6[0x74] == 0)))) {
                                    uVar11 = (unsigned int)DAT_006d9d60;
                                    iVar10 = (int)(uVar11 * 0x126);
                                    *(unsigned short *)(DAT_006d4a8a + iVar10) =
                                        (unsigned short)(*(unsigned short *)(DAT_006d4a8a + iVar10) + 1);
                                    cVar3 = (char)puVar6[0xb0];
                                    cVar4 = DAT_0070ad40[uVar11 * 0x566];
                                    if (cVar4 == cVar3) {
                                        DAT_006d4ab2[iVar10] = 1;
                                    }
                                    local_8 = (unsigned short)(cVar4 == cVar3);
                                    FUN_004a17b0(0xc,
                                                 ((((unsigned int)iVar10 >> 0x10) & 0xffff) << 0x10) |
                                                     (unsigned int)DAT_006d9d60,
                                                 (unsigned int)local_8);
                                }
                                if ((((puVar6[0x1e5] == 0) && (((unsigned char *)iVar9)[9] == 1)) &&
                                     (puVar6[0x73] == 0)) && (puVar6[0x74] == 0)) {
                                    uVar11 = (unsigned int)DAT_006d9d60;
                                    iVar9 = (int)(uVar11 * 0x126);
                                    *(unsigned short *)(DAT_006d4aa0 + iVar9) =
                                        (unsigned short)(*(unsigned short *)(DAT_006d4aa0 + iVar9) + 1);
                                    if (DAT_0070ad40[uVar11 * 0x566] == (char)puVar6[0xb0]) {
                                        DAT_006d4abd[iVar9] = 1;
                                    }
                                }
                            }
                            FUN_004a34c0(uVar8, puVar6, 0x8c);
                            FUN_0049f9c0((unsigned short)uVar8, DAT_006d9d60, local_c);
                            DAT_006d4988[3 + iVar1] = 0;
                            FUN_00421580((int)puVar6);
                        }
                    }
                }
            }
            uVar14 = (unsigned short)((unsigned short)uVar8 + 1);
            uVar8 = (((uVar8 >> 0x10) & 0xffff) << 0x10) | (unsigned int)uVar14;
        } while ((int)(unsigned int)uVar14 < DAT_006d4980);
    }
    uVar12 = (unsigned short)((unsigned int)DAT_006d4980 >> 0x10);
    LDG();
    if (((DAT_00732e54[0x1e5] == 0) && (g[9] == 0)) &&
        ((DAT_00732e54[0x72] != 0) ||
         ((DAT_00732e54[0x73] == 0) && (DAT_00732e54[0x74] == 0)))) {
        uVar8 = (unsigned int)DAT_006d9d60;
        iVar1 = (int)(uVar8 * 0x126);
        *(unsigned short *)(DAT_006d4a8a + iVar1) =
            (unsigned short)(*(unsigned short *)(DAT_006d4a8a + iVar1) + 1);
        puVar6 = DAT_00732e54;
        cVar3 = DAT_0070ad40[uVar8 * 0x566];
        cVar4 = (char)DAT_00732e54[0xb0];
        if (cVar3 == cVar4) {
            DAT_006d4ab2[iVar1] = 1;
        }
        FUN_004a17b0(0xc,
                     ((((unsigned int)puVar6 >> 0x10) & 0xffff) << 0x10) | (unsigned int)DAT_006d9d60,
                     (unsigned int)(cVar3 == cVar4));
        uVar12 = extraout_var_00;
    }
    pcVar13 = DAT_00732e54 + 0x1e5;
    if (DAT_00732e54[0x1e5] == 0) {
        LDG();
        if (g[9] == 1) {
            if ((DAT_00732e54[0x73] == 0) && (DAT_00732e54[0x74] == 0)) {
                uVar8 = (unsigned int)DAT_006d9d60;
                iVar1 = (int)(uVar8 * 0x126);
                *(unsigned short *)(DAT_006d4aa0 + iVar1) =
                    (unsigned short)(*(unsigned short *)(DAT_006d4aa0 + iVar1) + 1);
                uVar12 = (unsigned short)((unsigned int)DAT_00732e54 >> 0x10);
                if (DAT_0070ad40[uVar8 * 0x566] == (char)DAT_00732e54[0xb0]) {
                    DAT_006d4abd[iVar1] = 1;
                }
            }
        }
        if (*pcVar13 != 0) goto LAB_004549d1;
    } else {
LAB_004549d1:
        uVar8 = (unsigned int)DAT_006d9d60;
        LD32(t32, DAT_006d4988 + (unsigned int)DAT_006d9d50 * 0x23 + 0x1f);
        uVar11 = (unsigned int)((unsigned char *)t32)[0x87];
        iVar1 = (int)(uVar11 + uVar8 * 0x126);
        DAT_006d4ae6[iVar1] = (char)(DAT_006d4ae6[uVar8 * 0x126 + uVar11] + 1);
        cVar3 = (char)DAT_00732e54[0xb0];
        cVar4 = DAT_0070ad40[uVar8 * 0x566];
        if (cVar4 == cVar3) {
            DAT_006d4af0[iVar1] = (char)(DAT_006d4af0[iVar1] + 1);
        }
        FUN_004a17b0(0x2c,
                     ((((unsigned int)iVar1 >> 0x10) & 0xffff) << 0x10) | (unsigned int)DAT_006d9d60,
                     (unsigned int)(cVar4 == cVar3));
        uVar8 = 0;
        uVar12 = 0;
        do {
            puVar6 = DAT_00732e54;
            if (uVar8 != uVar11) {
                uVar15 = (unsigned int)DAT_006d9d60;
                if (DAT_0070ad48[uVar15 * 0x566] != uVar8) {
                    iVar1 = (int)(uVar8 + uVar15 * 0x126);
                    DAT_006d4afa[iVar1] = (char)(DAT_006d4afa[uVar15 * 0x126 + uVar8] + 1);
                    if (DAT_0070ad40[uVar15 * 0x566] == (char)puVar6[0xb0]) {
                        DAT_006d4b04[iVar1] = 1;
                    }
                }
            }
            uVar8 = uVar8 + 1;
        } while (uVar8 < 10);
    }
    FUN_004a34c0((((unsigned int)uVar12) << 0x10) | (unsigned int)DAT_006d9d50, DAT_00732e54, 0x8c);
    FUN_0049f9c0(DAT_006d9d50, DAT_006d9d60, local_c);
    DAT_006d4988[(unsigned int)DAT_006d9d50 * 0x23 + 3] = 0;
    uVar8 = FUN_00421580((int)DAT_00732e54);
    uVar14 = *(unsigned short *)(DAT_00732e54 + 0x66);
    if (uVar14 != 0xffff) {
        iVar1 = (int)((unsigned int)uVar14 * 0x23);
        uVar8 = (unsigned int)(DAT_006d4988 + iVar1);
        LD32(t32, uVar8 + 0x1f);
        if (t32 != 0) {
            uVar7 = (unsigned short)((unsigned char *)uVar8)[0x16];
            FUN_0049f9c0(uVar14, uVar7, local_c);
            LD32(t32, DAT_006d4988 + 0x1f + iVar1);
            iVar9 = (int)t32;
            LD32(t32, iVar9 + 0xa7);
            iVar10 = (int)t32;
            if (((unsigned char *)iVar10)[0x1e5] == 0) {
                iVar9 = (int)((unsigned int)uVar7 * 0x126);
                *(unsigned short *)(DAT_006d4a98 + iVar9) =
                    (unsigned short)(*(unsigned short *)(DAT_006d4a98 + iVar9) + 1);
                bVar16 = DAT_0070ad40[(unsigned int)uVar7 * 0x566] ==
                         (char)((unsigned char *)iVar10)[0xb0];
                if (bVar16) {
                    DAT_006d4ab9[iVar9] = 1;
                }
                FUN_004a17b0(0x2e, (((unsigned int)local_8) << 0x10) | (unsigned int)uVar7, bVar16);
            } else {
                bVar5 = ((unsigned char *)iVar9)[0x87];
                uVar8 = (unsigned int)uVar7;
                local_8 = (unsigned short)bVar5;
                iVar9 = (int)(uVar8 * 0x126);
                DAT_006d4ae6[iVar9 + (unsigned int)local_8] =
                    (char)(DAT_006d4ae6[uVar8 * 0x126 + (unsigned int)local_8] + 1);
                iVar2 = (int)(uVar8 * 0x566);
                bVar16 = DAT_0070ad40[uVar8 * 0x566] == (char)((unsigned char *)iVar10)[0xb0];
                if (bVar16) {
                    DAT_006d4af0[iVar9 + (unsigned int)local_8] =
                        (char)(DAT_006d4af0[iVar9 + (unsigned int)local_8] + 1);
                }
                FUN_004a17b0(0x2c, (((unsigned int)bVar5) << 0x10) | (unsigned int)uVar7, bVar16);
                uVar11 = 0;
                do {
                    if ((uVar11 != local_8) && (DAT_0070ad48[iVar2] != uVar11)) {
                        DAT_006d4afa[iVar9 + uVar11] = (char)(DAT_006d4afa[iVar9 + uVar11] + 1);
                        if (DAT_0070ad40[iVar2] == (char)((unsigned char *)iVar10)[0xb0]) {
                            DAT_006d4b04[iVar9 + uVar11] = 1;
                        }
                    }
                    uVar11 = uVar11 + 1;
                } while (uVar11 < 10);
                cVar3 = DAT_0070ad40[iVar2];
                *(unsigned short *)(DAT_006d4a9a + uVar8 * 0x126) =
                    (unsigned short)(*(unsigned short *)(DAT_006d4a9a + uVar8 * 0x126) + 1);
                if (cVar3 == (char)((unsigned char *)iVar10)[0xb0]) {
                    DAT_006d4aba[uVar8 * 0x126] = 1;
                }
            }
            DAT_006d4988[3 + iVar1] = 0;
            uVar8 = FUN_00421580(iVar10);
        }
    }
LAB_00454d10:
    return uVar8 & 0xffff0000u;
}
