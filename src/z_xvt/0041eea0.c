/* SPDX-License-Identifier: MIT */
/* z_xvt 0041eea0 */
/* Globals (types inferred from the decompile) */
extern unsigned char DAT_0070ad8b[];
extern unsigned char DAT_0070ad8a[];
extern unsigned char DAT_0071f2d8[4];
extern unsigned char DAT_0071f2dc;
extern unsigned char DAT_0071f2dd;
extern unsigned char DAT_007092c1;
extern unsigned short DAT_0073b374;
extern unsigned char DAT_007394ca;
extern unsigned char DAT_007394cc;
extern unsigned char DAT_006d4a66[];
extern unsigned char DAT_0070b156[];
extern unsigned char DAT_0070b182[];
extern unsigned char DAT_0070b1ae[];
extern unsigned char DAT_0070b15e[];
extern unsigned char DAT_0070b18a[];
extern unsigned char DAT_0070b1b6[];
extern unsigned char DAT_0070b1e2[];
extern unsigned char DAT_0070b252[];
extern int DAT_00732a2c;
extern int DAT_00732a28;
extern int DAT_00732a24;
extern int DAT_0073abc4;
extern int DAT_0073abc8;
extern int DAT_0073abcc;
extern unsigned char DAT_006d4a02;
extern unsigned short DAT_007394c8;
extern unsigned short DAT_00732e40;
extern unsigned short DAT_006da9f0;
extern unsigned short DAT_007394ce;
extern int DAT_006d4a50;
extern int DAT_006ffbec;
extern int DAT_0072f4e8;
extern unsigned char DAT_0073b372;
extern unsigned char DAT_0070ad4d[];
extern unsigned char DAT_007394c5;
extern unsigned char DAT_0070ad4e[];
extern unsigned short DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_00732e54;
extern unsigned char DAT_0052b14b[];
extern unsigned char DAT_0052b14d[];
extern unsigned char DAT_0052b14f[];
extern unsigned char DAT_0052b151[];
extern unsigned char DAT_0052b153[];
extern unsigned char DAT_0052b155[];
extern int DAT_0070ab18;
extern int DAT_0070ab14;
extern int DAT_0070ac20;
extern unsigned char DAT_0070ad43[];
extern unsigned char DAT_007394c4;
extern unsigned char DAT_0070ad47[];
extern unsigned char DAT_007394c3;
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_007394cb;
extern unsigned char DAT_0070ad44[];
extern unsigned char DAT_0073abc1;
extern unsigned char DAT_0070b215[];
extern unsigned char DAT_0073abc0;
extern unsigned char DAT_0073b370;
extern unsigned char DAT_0052686b[];
extern unsigned char DAT_005247e8[];
extern unsigned char DAT_0070ad42[];
extern unsigned char DAT_007394c7;
extern unsigned char DAT_0070ad49[];
extern unsigned short DAT_007394d0;
extern unsigned char DAT_007394c2;
extern unsigned char DAT_006d4a6c[];
extern unsigned short DAT_006d4a6a[];
extern unsigned char DAT_0070ad40[];
extern unsigned char DAT_006d4aa3[];
extern unsigned char DAT_007394d2;

/* Prototypes */
/* Only 4 stack argument words (16 bytes) are pushed; the 3rd is passed zero-extended
   by the first call and sign-extended by the second */
void FUN_0042a230(int, short, int, short);
void FUN_00418cd0(int, int, int);
unsigned short FUN_00478470(void);
void FUN_00418b00(unsigned short, unsigned short, unsigned short);
unsigned long long __stdcall FUN_0041f670(void);
unsigned int FUN_004a7aa0(unsigned int);
unsigned long long FUN_004a31b0(unsigned int, unsigned int);

unsigned long long __cdecl FUN_0041eea0(unsigned short param_1)
{
    int bVar1;
    unsigned char cVar2;
    unsigned char cVar3;
    unsigned char cVar4;
    unsigned short uVar6;
    unsigned int uVar5;
    unsigned int uVar8;
    int iVar7;
    int iVar9;
    int iVar10;
    unsigned short uVar11;
    unsigned short local_2;
    unsigned long long r;
    unsigned char *p;
    int t32;
    short sA;
    short sB;
    short sC;
    unsigned short uA;
    int i32;

    cVar4 = DAT_0071f2dd;
    cVar3 = DAT_0071f2dc;
    cVar2 = DAT_0071f2d8[3];
    bVar1 = 0;
    uVar5 = (unsigned int)DAT_0073b374;
    DAT_007394ca = 0;
    DAT_007394cc = 0;
    if (((((DAT_0070ad8b[uVar5 * 0x566u] == 0) ||
           ((DAT_0071f2d8[3] == 0 && DAT_0071f2dc == 0) && DAT_0071f2dd == 0)) ||
          (DAT_007092c1 != 0)) || (param_1 != 0xffff))) {
        iVar10 = (int)(((unsigned int)*(unsigned short *)(DAT_006d4a66 + uVar5 * 0x126u) - 0x8000u) +
                       uVar5 * 0x2b3u);
        DAT_00732a2c = (int)((unsigned int)(int)*(short *)(DAT_0070b156 + (unsigned int)iVar10 * 2u) << 8);
        iVar10 = (int)((unsigned int)iVar10 * 2u);
        DAT_00732a28 = (int)((unsigned int)(int)*(short *)(DAT_0070b182 + iVar10) * 0xffffff00u);
        DAT_00732a24 = (int)((unsigned int)(int)*(short *)(DAT_0070b1ae + iVar10) << 8);
        DAT_0073abc4 = DAT_00732a2c;
        DAT_0073abc8 = DAT_00732a28;
        DAT_0073abcc = DAT_00732a24;
        __builtin_memcpy(&i32, DAT_0070b252 + uVar5 * 0x566u, 4);
        if ((((DAT_0071f2d8[3] != 0 || DAT_0071f2dc != 0) || DAT_0071f2dd != 0) &&
             (DAT_006d4a02 == 2)) && (i32 != -1)) {
            uVar11 = FUN_00478470();
            uVar8 = FUN_00478470();
            uVar5 = uVar11 & 0x7fff;
            if ((uVar8 & 1) == 0) {
                uVar5 = 0u - uVar5;
            }
            DAT_0073abc4 = (int)((unsigned int)DAT_0073abc4 + uVar5);
            uVar11 = FUN_00478470();
            uVar8 = FUN_00478470();
            uVar5 = uVar11 & 0x7fff;
            if ((uVar8 & 1) == 0) {
                uVar5 = 0u - uVar5;
            }
            DAT_0073abc8 = (int)((unsigned int)DAT_0073abc8 + uVar5);
        }
        if (*(short *)(DAT_0070b1e2 + (unsigned int)DAT_0073b374 * 0x566u) == 0) {
            DAT_007394c8 = 0;
            DAT_00732e40 = 0x4000;
        } else {
            iVar10 = (int)((unsigned int)DAT_0073b374 * 0x566u);
            DAT_00732a2c = (int)((unsigned int)(int)*(short *)(DAT_0070b15e + iVar10) * 0x100u);
            DAT_00732a28 = (int)((unsigned int)(int)*(short *)(DAT_0070b18a + iVar10) * 0xffffff00u);
            DAT_00732a24 = (int)((unsigned int)(int)*(short *)(DAT_0070b1b6 + iVar10) * 0x100u);
            FUN_00418cd0((int)((unsigned int)DAT_00732a2c - (unsigned int)DAT_0073abc4),
                         (int)((unsigned int)DAT_00732a28 - (unsigned int)DAT_0073abc8),
                         (int)((unsigned int)DAT_00732a24 - (unsigned int)DAT_0073abcc));
            DAT_007394c8 = DAT_006da9f0;
        }
        DAT_006da9f0 = DAT_007394c8;
        DAT_007394ce = DAT_00732e40;
        __builtin_memcpy(&i32, DAT_0070b252 + (unsigned int)DAT_0073b374 * 0x566u, 4);
        if ((((DAT_007092c1 == 0) && (DAT_0070ad8b[(unsigned int)DAT_0073b374 * 0x566u] == 0)) &&
             ((cVar2 != 0 || cVar3 != 0) || cVar4 != 0)) &&
            ((param_1 == 0xffff && (i32 == -1)))) {
            DAT_006da9f0 = (unsigned short)(DAT_007394c8 + 0xffff8000u);
            uVar11 = (unsigned short)(0x8000u - DAT_00732e40);
            DAT_00732e40 = uVar11;
            FUN_00418b00(0xffff, DAT_006da9f0, uVar11);
            DAT_006d4a50 = (int)((unsigned int)DAT_006d4a50 * 8u);
            DAT_006ffbec = (int)((unsigned int)DAT_006ffbec * 8u);
            DAT_0072f4e8 = (int)((unsigned int)DAT_0072f4e8 * 8u);
            DAT_0073abc4 = (int)((unsigned int)DAT_0073abc4 + (unsigned int)DAT_006d4a50);
            DAT_0073abc8 = (int)((unsigned int)DAT_0073abc8 + (unsigned int)DAT_006ffbec);
            DAT_0073abcc = (int)((unsigned int)DAT_0073abcc + (unsigned int)DAT_0072f4e8);
            DAT_007394ca = 1;
        }
        DAT_0073b372 = DAT_0070ad4d[(unsigned int)DAT_0073b374 * 0x566u];
        DAT_007394c5 = DAT_0070ad4e[(unsigned int)DAT_0073b374 * 0x566u];
    } else {
        uVar8 = (unsigned int)DAT_00732a3c;
        if ((int)uVar8 < DAT_006d4980) {
            local_2 = 0;
            uVar8 = 0;
            uVar11 = DAT_00732a3c;
            do {
                p = (unsigned char *)((unsigned int)uVar11 * 0x23u + (unsigned int)DAT_006d4988);
                if (p[3] != 0) {
                    __builtin_memcpy(&t32, p + 0x1f, 4);
                    __builtin_memcpy(&DAT_00732e54, (unsigned char *)t32 + 0xa7, 4);
                    if (p[0x16] == DAT_0070ad8a[uVar5 * 0x566u] &&
                        *(signed char *)((unsigned char *)DAT_00732e54 + 5) == -1) {
                        bVar1 = 1;
                        local_2 = uVar11;
                        goto LAB_0041efc5;
                    }
                }
                uVar11 = (unsigned short)(uVar11 + 1);
            } while ((int)(unsigned int)uVar11 < DAT_006d4980);
            bVar1 = 0;
        }
LAB_0041efc5:
        if (!bVar1) {
            return (((unsigned long long)uVar8 << 32) | (unsigned long long)(uVar5 * 0x2b3u)) &
                   0xffffffffffff0000ULL;
        }
        iVar10 = (int)((unsigned int)local_2 * 0x23u);
        iVar7 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar10);
        __builtin_memcpy(&t32, (unsigned char *)iVar7 + 0x1f, 4);
        __builtin_memcpy(&DAT_00732e54, (unsigned char *)t32 + 0xa7, 4);
        iVar9 = (int)((unsigned int)*(unsigned char *)((unsigned char *)DAT_00732e54 + 4) * 0xefu);
        __builtin_memcpy(&sA, DAT_0052b14b + iVar9, 2);
        __builtin_memcpy(&uA, DAT_0052b14d + iVar9, 2);
        __builtin_memcpy(&sB, DAT_0052b14f + iVar9, 2);
        FUN_0042a230(iVar7, sA, (int)(unsigned int)uA, sB);
        iVar7 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar10);
        p = (unsigned char *)iVar7;
        __builtin_memcpy(&i32, p + 4, 4);
        DAT_0073abc4 = (int)((unsigned int)i32 + (unsigned int)DAT_0070ab18);
        __builtin_memcpy(&i32, p + 8, 4);
        DAT_0073abc8 = (int)((unsigned int)i32 + (unsigned int)DAT_0070ab14);
        __builtin_memcpy(&i32, p + 0xc, 4);
        DAT_0073abcc = (int)((unsigned int)i32 + (unsigned int)DAT_0070ac20);
        __builtin_memcpy(&sA, DAT_0052b151 + iVar9, 2);
        __builtin_memcpy(&sB, DAT_0052b153 + iVar9, 2);
        __builtin_memcpy(&sC, DAT_0052b155 + iVar9, 2);
        FUN_0042a230(iVar7, sA, (int)sB, sC);
        iVar10 = (int)((unsigned int)DAT_006d4988 + (unsigned int)iVar10);
        p = (unsigned char *)iVar10;
        __builtin_memcpy(&i32, p + 4, 4);
        DAT_00732a2c = (int)((unsigned int)i32 + (unsigned int)DAT_0070ab18);
        __builtin_memcpy(&i32, p + 8, 4);
        DAT_00732a28 = (int)((unsigned int)i32 + (unsigned int)DAT_0070ab14);
        __builtin_memcpy(&i32, p + 0xc, 4);
        DAT_00732a24 = (int)((unsigned int)i32 + (unsigned int)DAT_0070ac20);
        FUN_00418cd0((int)((unsigned int)DAT_00732a2c - (unsigned int)DAT_0073abc4),
                     (int)((unsigned int)DAT_00732a28 - (unsigned int)DAT_0073abc8),
                     (int)((unsigned int)DAT_00732a24 - (unsigned int)DAT_0073abcc));
        DAT_007394cc = 1;
        DAT_007394c8 = DAT_006da9f0;
        DAT_007394ce = DAT_00732e40;
        DAT_0073b372 = 0;
        if (3 < DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566u]) {
            DAT_0073b372 = 6;
        }
        DAT_007394c5 = 0;
    }
    uVar8 = (unsigned int)DAT_0073b374;
    uVar5 = uVar8 * 0x566u;
    DAT_007394c4 = DAT_0070ad47[uVar5];
    DAT_007394c3 = DAT_0070ad48[uVar5];
    DAT_007394cb = DAT_0070ad44[uVar5];
    DAT_0073abc1 = DAT_0070b215[uVar5];
    iVar9 = 0;
    DAT_0073abc0 = 0;
    DAT_0073b370 = DAT_0052686b[(unsigned int)DAT_005247e8[DAT_0070ad42[uVar5]] * 0x16u];
    DAT_007394c7 = DAT_0070ad49[uVar5];
    iVar10 = (int)(((((unsigned int)DAT_005247e8[DAT_0070ad42[uVar5]] * 0xbu) >> 8) << 8) |
                   (unsigned int)DAT_007394c7);
    if (param_1 == 0xffff) {
        DAT_007394d0 = 0;
        DAT_007394c2 = 0xff;
        if (DAT_0070ad43[uVar5] != 0) {
            do {
                if (*(unsigned short *)(DAT_006d4a6c + (unsigned int)DAT_0073b374 * 0x126u) <
                    DAT_006d4a6a[(unsigned int)DAT_0073b374 * 0x93u]) {
                    r = FUN_0041f670();
                    uVar11 = (unsigned short)r;
                    if (uVar11 == 0xffff) {
                        /* (EDX << 32) | (upper 16 bits of EAX << 16) */
                        return r & 0xffffffffffff0000ULL;
                    }
                    iVar9 = iVar9 + 1;
                    uVar5 = (unsigned int)DAT_0073b374;
                    *(unsigned short *)(DAT_006d4a6c + uVar5 * 0x126u) =
                        (unsigned short)(*(unsigned short *)(DAT_006d4a6c + uVar5 * 0x126u) + 1);
                    if ((unsigned int)DAT_0070ad40[uVar5 * 0x566u] == (unsigned int)DAT_007394d0) {
                        DAT_006d4aa3[uVar5 * 0x126u] = (unsigned char)(DAT_006d4aa3[uVar5 * 0x126u] + 1);
                    }
                }
                DAT_007394d0 = (unsigned short)(DAT_007394d0 + 1);
                iVar10 = (int)((unsigned int)DAT_0073b374 * 0x2b3u);
                uVar5 = (unsigned int)DAT_007394d0;
            } while (uVar5 < (unsigned int)DAT_0070ad43[(unsigned int)DAT_0073b374 * 0x566u]);
        }
    } else {
        uVar5 = uVar8 * 0x126u;
        if (*(unsigned short *)(DAT_006d4a6c + uVar5) < DAT_006d4a6a[uVar8 * 0x93u]) {
            DAT_007394d0 = param_1;
            r = FUN_0041f670();
            uVar11 = (unsigned short)r;
            if (uVar11 == 0xffff) {
                /* (EDX << 32) | (upper 16 bits of EAX << 16) */
                return r & 0xffffffffffff0000ULL;
            }
            iVar9 = 1;
            uVar8 = (unsigned int)DAT_0073b374;
            *(unsigned short *)(DAT_006d4a6c + uVar8 * 0x126u) =
                (unsigned short)(*(unsigned short *)(DAT_006d4a6c + uVar8 * 0x126u) + 1);
            iVar10 = (int)(uVar8 * 0x2b3u);
            uVar5 = (unsigned int)DAT_007394d0;
            if ((unsigned int)DAT_0070ad40[uVar8 * 0x566u] == uVar5) {
                DAT_006d4aa3[uVar8 * 0x126u] = (unsigned char)(DAT_006d4aa3[uVar8 * 0x126u] + 1);
            }
        }
    }
    uVar6 = (unsigned short)(uVar5 >> 0x10);
    if ((((cVar2 != 0 || cVar3 != 0) || cVar4 != 0) && (param_1 == 0xffff)) && (iVar9 != 0)) {
        uVar5 = FUN_004a7aa0(((uVar5 >> 8) << 8 | (unsigned int)DAT_007394d2) & 0xffff00ffu);
        /* extraout_var_02 = high half of EAX from FUN_004a7aa0 */
        r = FUN_004a31b0((((uVar5 >> 16) & 0xffffu) << 16) | (unsigned int)DAT_0073b374, uVar5);
        iVar10 = (int)(unsigned int)(r >> 32);          /* extraout_EDX_01 */
        uVar6 = (unsigned short)((r >> 16) & 0xffffu);  /* extraout_var_01 */
    }
    return ((unsigned long long)(unsigned int)iVar10 << 32) |
           (unsigned long long)(((unsigned int)uVar6 << 16) | 1u);
}
