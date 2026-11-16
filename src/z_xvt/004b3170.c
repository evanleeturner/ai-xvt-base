/* SPDX-License-Identifier: MIT */
/* z_xvt 004b3170 */
/* Prototypes */
extern unsigned int (__cdecl *DAT_006db5ac)(unsigned short, unsigned short);
extern void __cdecl FUN_00426840(unsigned short, unsigned short, unsigned int, unsigned int);
extern void __cdecl FUN_0045f5c0(void);
extern void __cdecl FUN_004b3b20(unsigned int, int, int);
extern void __cdecl FUN_00465970(int);
extern int  __cdecl FUN_0044a2a0(unsigned int, unsigned int);
extern int  __cdecl FUN_0044a440(unsigned int, unsigned int);
extern int  __cdecl FUN_0044a370(unsigned int, unsigned int);
extern void __cdecl FUN_0042a230(int, int, int, short, short, short);
extern int  __cdecl FUN_0040ca00(int, int, int);
extern int  __cdecl FUN_0040ca30(int, int, int);
extern int  __cdecl FUN_0040ca60(int, int, int);
extern void __cdecl FUN_004832a0(short, short, short, int, int);
extern void __cdecl FUN_0045b760(unsigned int);
extern void __cdecl FUN_0049c2b0(unsigned int);
extern void __cdecl FUN_00486310(int);
extern void __cdecl FUN_0049c590(unsigned int);
extern void __cdecl FUN_00412e60(void *, unsigned short);
extern int  __cdecl FUN_0045b880(unsigned int, unsigned int);
extern void __cdecl FUN_0047f0f0(unsigned int);
extern void __cdecl FUN_00466020(void);
extern void __cdecl FUN_0047f3d0(int);
extern int  __cdecl FUN_0040ca90(int, int);
extern int  __cdecl FUN_0040cad0(int, int);
extern void __cdecl FUN_00477c90(int, int, int, int, int, int);
extern void __cdecl FUN_00465c40(void);
extern void __cdecl FUN_00426970(void);

/* Globals (types inferred from the decompile) */
extern unsigned int   DAT_00709ffc;
extern unsigned short DAT_006fedd0;
extern int            DAT_0071ecf4;
extern unsigned char  DAT_0071bb1b[];
extern unsigned char  DAT_0071bb1f[];
extern unsigned char  DAT_0071bb23[];
extern unsigned char  DAT_0071bb27[];
extern unsigned char  DAT_0071bb44[];
extern unsigned char  DAT_0071b743[];
extern unsigned char  DAT_0071b749[];
extern unsigned char  DAT_0071b74a[];
extern unsigned char  DAT_0071b759[];
extern unsigned char  DAT_0071b730[];
extern unsigned short DAT_00527e74;
extern unsigned int   DAT_006da9f4;
extern short          DAT_00655184;
extern unsigned char  DAT_00646a58[];
extern unsigned char  DAT_00646878[];
extern unsigned char  DAT_00646c38[];
extern int            DAT_0052552c;
extern int            DAT_0052ae18;
extern unsigned char  DAT_00525548;
extern unsigned char  DAT_00525508;
extern short          DAT_006da9e2;
extern int            DAT_00732a2c;
extern int            DAT_00732a28;
extern int            DAT_00732a24;
extern int            DAT_0072f44c;
extern int            DAT_0072f450;
extern int            DAT_0072f448;
extern int            DAT_006d4980;
extern int            DAT_006d4988;
extern int            DAT_0070ab18;
extern int            DAT_0070ab14;
extern int            DAT_0070ac20;
extern int            DAT_006db7ac;
extern int            DAT_006db7b0;
extern int            DAT_006db7b4;
extern int            DAT_00732e54;
extern short          DAT_0071e536;
extern unsigned int   DAT_00732a38;
extern unsigned int   DAT_006db7a4;
extern char           DAT_006d9d94;
extern char           DAT_007092c1;
extern short          DAT_00703ac0;
extern int            DAT_00732910;
extern unsigned short DAT_006db718;
extern unsigned char  DAT_0052686c[];
extern int            DAT_00525734;

/* Unaligned read helper (GNU statement expression, memcpy based) */
#define RD(T, addr) ({ T _t; __builtin_memcpy(&_t, (const void *)(addr), sizeof(T)); _t; })
#define PIDX ((unsigned int)DAT_0071ecf4 * 0x5bdU)
#define ENT(n) ((unsigned char *)((unsigned int)DAT_006d4988 + (unsigned int)(n) * 0x23U))

void __cdecl FUN_004b3170(unsigned short param_1, unsigned short param_2,
                          unsigned short param_3, unsigned short param_4, short param_5)
{
    unsigned char *puVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int uVar9;
    unsigned short uVar8;
    unsigned int uVar12;
    unsigned char *puVar15;
    unsigned char *puVar16;
    short sVar11;
    unsigned short uVar17;
    unsigned short uVar3;
    unsigned char bVar2;
    unsigned char *iVar13p;
    unsigned char *iVar14p;
    unsigned int iVar7;
    int iVar13;
    int iVar14;
    int iStack_1c;
    int iStack_18;
    int iStack_14;
    void *this_ = NULL;
    short s1, s2, s3;
    unsigned int pidx;

    uVar9 = DAT_00709ffc;
    uVar8 = DAT_006fedd0;
    puVar4 = RD(unsigned char *, &DAT_0071bb1b[PIDX]);
    uVar5 = RD(unsigned int, &DAT_0071bb1f[PIDX]);
    uVar6 = RD(unsigned int, &DAT_0071bb23[PIDX]);
    DAT_00709ffc = 0;
    uVar12 = (*DAT_006db5ac)(param_1, param_2);
    /* high half of 3rd arg is leftover ESI in the original */
    FUN_00426840(param_3, param_4, (unsigned int)(unsigned short)param_5, uVar12);
    if (param_5 != 0) {
        puVar16 = (unsigned char *)((unsigned int)DAT_00527e74 + DAT_006da9f4);
        if (DAT_00655184 == 0) {
            puVar15 = DAT_00646a58;
        } else if (DAT_00655184 == 0x78) {
            puVar15 = DAT_00646878;
        } else {
            puVar15 = puVar4;
            if (DAT_00655184 == 0xf0) {
                puVar15 = DAT_00646c38;
            }
        }
        if (DAT_0052552c == 0x13) {
            sVar11 = 200;
            do {
                *puVar16 = *puVar15;
                puVar15 = puVar15 + 1;
                puVar16 = puVar16 + 1;
                sVar11 = sVar11 - 1;
            } while (sVar11 != 0);
        } else {
            sVar11 = 0x1e0;
            do {
                *puVar16 = *puVar15;
                puVar15 = puVar15 + 1;
                puVar16 = puVar16 + 1;
                sVar11 = sVar11 - 1;
            } while (sVar11 != 0);
        }
    }
    if (DAT_0052ae18 != 0) {
        FUN_0045f5c0();
    }
    FUN_004b3b20((((unsigned int)DAT_0071ecf4 * 0x16fU) >> 16 << 16) |
                     (unsigned int)RD(unsigned short, &DAT_0071b74a[PIDX]),
                 1, DAT_0071ecf4);
    if (DAT_0071b749[PIDX] == 0) {
        DAT_006fedd0 = DAT_006fedd0 | 0x400;
    } else {
        DAT_006fedd0 = DAT_006fedd0 | 0x200;
    }
    DAT_00525548 = 0x30;
    FUN_00465970(DAT_0071ecf4);
    DAT_006da9e2 = 0;
    pidx = PIDX;
    DAT_0072f44c = (int)((unsigned int)DAT_00732a2c - RD(unsigned int, &DAT_0071bb1b[PIDX]));
    DAT_0072f450 = (int)((unsigned int)DAT_00732a28 - RD(unsigned int, &DAT_0071bb1f[PIDX]));
    DAT_0072f448 = (int)((unsigned int)DAT_00732a24 - RD(unsigned int, &DAT_0071bb23[pidx]));
    if ((DAT_0071b749[pidx] != 0) &&
        ((int)(unsigned int)RD(unsigned short, &DAT_0071b74a[pidx]) < DAT_006d4980)) {
        DAT_0070ab18 = FUN_0044a2a0((unsigned int)ENT(RD(unsigned short, &DAT_0071b74a[pidx]))[3],
                                    (unsigned int)RD(unsigned short, &DAT_0071b759[pidx]));
        DAT_0070ab14 = FUN_0044a440((unsigned int)ENT(RD(unsigned short, &DAT_0071b74a[PIDX]))[3],
                                    (unsigned int)RD(unsigned short, &DAT_0071b759[PIDX]));
        iVar13 = FUN_0044a370((unsigned int)ENT(RD(unsigned short, &DAT_0071b74a[PIDX]))[3],
                              (unsigned int)RD(unsigned short, &DAT_0071b759[PIDX]));
        DAT_0070ac20 = (int)(0U - (unsigned int)iVar13);
        iVar13p = ENT(RD(unsigned short, &DAT_0071b74a[PIDX]));
        iVar13 = (int)iVar13p;
        if (iVar13p[3] == '5') {
            DAT_0070ab18 = DAT_0070ab18 >> 1;
            DAT_0070ab14 = DAT_0070ab14 >> 1;
            DAT_0070ac20 = DAT_0070ac20 >> 1;
        }
        /* first three args are decompiler leftovers (register garbage); signature unresolved */
        FUN_0042a230(DAT_0070ab14, iVar13, iVar13, (short)DAT_0070ab18, (short)DAT_0070ab14,
                     (short)DAT_0070ac20);
        pidx = PIDX;
        if (ENT(RD(unsigned short, &DAT_0071b74a[pidx]))[3] == '5') {
            DAT_0070ab18 = (int)((unsigned int)DAT_0070ab18 * 2U);
            DAT_0070ab14 = (int)((unsigned int)DAT_0070ab14 * 2U);
            DAT_0070ac20 = (int)((unsigned int)DAT_0070ac20 * 2U);
        }
        iStack_14 = (int)(((unsigned int)DAT_0070ab18 - RD(unsigned int, &DAT_0071bb1b[pidx])) +
                          (unsigned int)DAT_00732a2c);
        iStack_18 = (int)(((unsigned int)DAT_0070ab14 - RD(unsigned int, &DAT_0071bb1f[pidx])) +
                          (unsigned int)DAT_00732a28);
        iStack_1c = (int)(((unsigned int)DAT_0070ac20 - RD(unsigned int, &DAT_0071bb23[pidx])) +
                          (unsigned int)DAT_00732a24);
    }
    DAT_006db7ac = FUN_0040ca00(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    DAT_006db7b0 = FUN_0040ca30(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    DAT_006db7b4 = FUN_0040ca60(DAT_0072f44c, DAT_0072f450, DAT_0072f448);
    uVar17 = RD(unsigned short, &DAT_0071b74a[PIDX]);
    iVar13p = ENT(uVar17);
    if (RD(int, iVar13p + 0x1f) == 0) {
        if ((7 < iVar13p[2]) && (iVar13p[2] < 0xc)) {
            s1 = RD(short, iVar13p + 0x14);
            s2 = RD(short, iVar13p + 0x12);
            s3 = RD(short, iVar13p + 0x10);
            FUN_004832a0(s1, s2, s3, 0, 0);
            DAT_0071e536 = 1;
            FUN_00412e60(this_, uVar17);
        }
    } else {
        switch (iVar13p[2]) {
        case 0:
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
            DAT_00732e54 = RD(int, RD(int, iVar13p + 0x1f) + 0xa7);
            s1 = RD(short, iVar13p + 0x14);
            s2 = RD(short, iVar13p + 0x12);
            s3 = RD(short, iVar13p + 0x10);
            FUN_004832a0(s1, s2, s3, 0, (int)iVar13p);
            FUN_0045b760((unsigned int)RD(unsigned short, &DAT_0071b74a[PIDX]));
            FUN_0049c2b0((((unsigned int)DAT_0071ecf4 >> 16) << 16) |
                         (unsigned int)RD(unsigned short, &DAT_0071b74a[PIDX]));
            FUN_00486310((int)ENT(RD(unsigned short, &DAT_0071b74a[PIDX])));
            break;
        case 6:
        case 7:
            s1 = RD(short, iVar13p + 0x14);
            s2 = RD(short, iVar13p + 0x12);
            s3 = RD(short, iVar13p + 0x10);
            FUN_004832a0(s1, s2, s3, 0, (int)iVar13p);
            FUN_0049c590((unsigned int)RD(unsigned short, &DAT_0071b74a[PIDX]));
            break;
        }
    }
    uVar12 = 0;
    if (0 < (int)DAT_00732a38) {
        do {
            if ((uVar12 == DAT_006db7a4) &&
                (((DAT_006d9d94 == '\0') || (DAT_007092c1 != '\0')) &&
                 (uVar12 = (unsigned int)(unsigned short)DAT_00703ac0,
                  (unsigned short)DAT_00703ac0 == DAT_00732a38))) {
                break;
            }
            uVar17 = (unsigned short)uVar12;
            pidx = PIDX;
            if ((RD(unsigned short, &DAT_0071bb27[PIDX]) != uVar17) ||
                (RD(short, &DAT_0071bb44[pidx]) != 0) || (DAT_00732910 != 0)) {
                iVar7 = uVar12 * 0x23U;
                bVar2 = *(unsigned char *)((unsigned int)DAT_006d4988 + 3U + iVar7);
                iVar14p = (unsigned char *)((unsigned int)DAT_006d4988 + iVar7);
                if (bVar2 != 0) {
                    DAT_006db718 = RD(unsigned short, &DAT_0052686c[(unsigned int)bVar2 * 0x16U]);
                    bVar2 = iVar14p[2];
                    if (5 < bVar2) {
                        if (bVar2 < 8) {
                            if (DAT_0071b743[pidx] == 0) {
                                uVar3 = RD(unsigned short, RD(int, iVar14p + 0x1f) + 0x83);
                                if (((RD(unsigned short, &DAT_0071b74a[pidx]) == uVar3) ||
                                     (RD(unsigned int, &DAT_0071b730[pidx]) == (unsigned int)uVar3)) &&
                                    (FUN_0045b880(uVar12, (unsigned int)DAT_006db718) != 0)) {
                                    iVar13p = (unsigned char *)((unsigned int)DAT_006d4988 + iVar7);
                                    s1 = RD(short, iVar13p + 0x14);
                                    s2 = RD(short, iVar13p + 0x12);
                                    s3 = RD(short, (unsigned char *)((unsigned int)DAT_006d4988 + 0x10U + iVar7));
                                    FUN_004832a0(s1, s2, s3, 0, (int)iVar13p);
                                    FUN_0049c590((unsigned int)uVar17);
                                }
                            } else if ((RD(short, RD(int, iVar14p + 0x1f) + 0x83) ==
                                        RD(short, &DAT_0071b74a[pidx])) &&
                                       (FUN_0045b880(uVar12, (unsigned int)DAT_006db718) != 0)) {
                                iVar13p = (unsigned char *)((unsigned int)DAT_006d4988 + iVar7);
                                s1 = RD(short, iVar13p + 0x14);
                                s2 = RD(short, iVar13p + 0x12);
                                s3 = RD(short, (unsigned char *)((unsigned int)DAT_006d4988 + 0x10U + iVar7));
                                FUN_004832a0(s1, s2, s3, 0, (int)iVar13p);
                                FUN_0049c590((unsigned int)uVar17);
                            }
                        } else if ((bVar2 == 0xd) &&
                                   (FUN_0045b880(uVar12, (unsigned int)DAT_006db718) != 0)) {
                            iVar13p = (unsigned char *)((unsigned int)DAT_006d4988 + iVar7);
                            s1 = RD(short, iVar13p + 0x14);
                            s2 = RD(short, iVar13p + 0x12);
                            s3 = RD(short, (unsigned char *)((unsigned int)DAT_006d4988 + 0x10U + iVar7));
                            FUN_004832a0(s1, s2, s3, 0, (int)iVar13p);
                            FUN_0047f0f0(uVar12);
                        }
                    }
                }
            }
            uVar12 = (unsigned int)(unsigned short)(uVar17 + 1);
        } while ((int)(unsigned int)(unsigned short)(uVar17 + 1) < (int)DAT_00732a38);
    }
    FUN_00466020();
    if (DAT_006da9e2 != 0) {
        DAT_00525734 = 0;
        FUN_0047f3d0(0);
        DAT_00525734 = 0;
    }
    if (((DAT_0071b749[PIDX] != 0) &&
         (uVar12 = (unsigned int)RD(unsigned short, &DAT_0071b74a[PIDX]),
          (int)uVar12 < DAT_006d4980)) &&
        (ENT(uVar12)[2] != 0)) {
        DAT_006db7ac = FUN_0040ca00(iStack_14, iStack_18, iStack_1c);
        DAT_006db7b0 = FUN_0040ca30(iStack_14, iStack_18, iStack_1c);
        DAT_006db7b4 = FUN_0040ca60(iStack_14, iStack_18, iStack_1c);
        iVar13 = FUN_0040ca90(DAT_006db7ac, DAT_006db7b4);
        iVar14 = FUN_0040cad0(DAT_006db7b0, DAT_006db7b4);
        FUN_00477c90((int)((unsigned int)iVar13 - 2U), (int)((unsigned int)iVar14 - 2U), 4, 4, 0xce, 1);
    }
    FUN_00465c40();
    DAT_00525548 = DAT_00525508;
    FUN_00426970();
    iVar13 = DAT_0071ecf4;
    DAT_006fedd0 = uVar8;
    __builtin_memcpy(&DAT_0071bb1b[PIDX], &puVar4, 4);
    __builtin_memcpy(&DAT_0071bb1f[(unsigned int)iVar13 * 0x5bdU], &uVar5, 4);
    __builtin_memcpy(&DAT_0071bb23[(unsigned int)iVar13 * 0x5bdU], &uVar6, 4);
    DAT_00709ffc = uVar9;
    return;
}
