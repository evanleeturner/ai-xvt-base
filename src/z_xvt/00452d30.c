/* SPDX-License-Identifier: MIT */
/* z_xvt 00452d30 */
/* Globals: widths from P-code where available; others assumed int. */
extern unsigned short DAT_006da850;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006ffbf2;
extern short DAT_00732e44;
extern unsigned short DAT_006d4984;
extern unsigned short DAT_00703be4;
extern int _DAT_006da4fc;
extern int DAT_00732f3c;
extern int _DAT_006d4994;
extern int DAT_0070acc4;
extern int DAT_0071ed00;
extern short DAT_006d4a42;
extern int _DAT_0071f2e4;
extern int DAT_0070acc8;
extern int _DAT_006db7bc;
extern int DAT_0071ecf4;

/* packed record array, base bytes at odd addresses */
extern unsigned char DAT_0071bb2f[];
extern unsigned char DAT_0071bb2b[];
extern unsigned char DAT_0071bb2d[];
extern unsigned char DAT_0071bb1b[];
extern unsigned char DAT_0071bb1f[];
extern unsigned char DAT_0071bb23[];

extern short DAT_005f0e34;
extern short DAT_005f0e32;
extern short DAT_005f0e30;
extern short DAT_00525e08;
extern int DAT_005f0e24;
extern int DAT_005f0e28;
extern int DAT_005f0e2c;
extern float _DAT_00520168;

extern int DAT_0073255c, DAT_00732558, DAT_00732568, DAT_00732578, DAT_00732584;
extern int DAT_0073253c, DAT_00732530, DAT_00732548, DAT_00732538;
extern int DAT_0070a138, DAT_0070a134, DAT_0070a130, DAT_0070a12c, DAT_0070a010, DAT_0070a00c;
extern int _DAT_0070a008, _DAT_0070a124, _DAT_0070a120;

extern float _DAT_006ccae0, _DAT_006ccae4, _DAT_006ccae8, _DAT_006ccaec;
extern float _DAT_006ccaf0, _DAT_006ccaf4, _DAT_006ccaf8, _DAT_006ccafc;
extern float _DAT_006ccb00, _DAT_006ccb04, _DAT_006ccb08, _DAT_006ccb0c;
extern float _DAT_006ccb10, _DAT_006ccb14, _DAT_006ccb18, _DAT_006ccb1c;
extern float _DAT_006ccb20, _DAT_006ccb24, _DAT_006ccb28, _DAT_006ccb2c;
extern float _DAT_006ccb30, _DAT_006ccb34, _DAT_006ccb38, _DAT_006ccb3c;
#define DAT_006ccae0 _DAT_006ccae0
#define DAT_006ccaec _DAT_006ccaec
#define DAT_006ccb10 _DAT_006ccb10
#define DAT_006ccb1c _DAT_006ccb1c

extern int DAT_00525e14;
extern unsigned short DAT_00525e18;
extern int DAT_00525e1c;
extern unsigned short DAT_00527e74;
extern unsigned int DAT_006da9f4;
extern int DAT_006db7b4;
extern unsigned char DAT_005254e0;
extern unsigned int DAT_005f0e3f; /* packed (odd address) */
extern int DAT_005254f8;

void __cdecl FUN_00483180(short a, short b, short c, int d, int e, int f, int g);
void __cdecl FUN_004832a0(short a, short b, short c, short d, int e);
void __cdecl FUN_00414fe0(float *a, float *b);
void __cdecl FUN_00465740(void);
void __cdecl FUN_0048f4b0(unsigned int a);
unsigned int __cdecl FUN_0048f3c0(unsigned int a, int b);
unsigned int __cdecl FUN_0048f4f0(unsigned int a);
long long __cdecl __ftol(void); /* FINDING: operand is on x87 stack, not visible in decompile */
void __fastcall FUN_00465970(unsigned int a);
void __cdecl FUN_00486310(int a);
void __cdecl FUN_00466020(void);
void __cdecl FUN_00465c40(void);

unsigned int __cdecl FUN_00452d30(int param_1, int param_2, int param_3, int param_4)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned char *puVar4;
    unsigned char *puVar5;
    unsigned int uVar6;
    unsigned int uVar7;
    long long lVar8;
    unsigned int idx;
    short s0, s1, s2;
    int i0, i1, i2;
    unsigned int p;

    if (DAT_006da850 == 0) {
        return 0;
    }
    if (param_1 < 0) {
        param_3 = (int)((unsigned int)param_3 + (unsigned int)param_1);
        param_1 = 0;
    }
    if (param_2 < 0) {
        param_4 = (int)((unsigned int)param_4 + (unsigned int)param_2);
    }
    if (0x3ff < param_1) {
        return 0;
    }
    if (0x2ff < param_2) {
        return 0;
    }
    if (0x300 < (int)((unsigned int)param_4 + (unsigned int)param_2)) {
        param_4 = (int)(0x300u - (unsigned int)param_2);
    }
    if (0x400 < (int)((unsigned int)param_3 + (unsigned int)param_1)) {
        param_3 = (int)(0x400u - (unsigned int)param_1);
    }
    DAT_007091ae = (unsigned short)param_3;
    DAT_006ffbf2 = (unsigned short)(DAT_007091ae - 1);
    DAT_00732e44 = (short)(param_3 / 2);
    DAT_006d4984 = (unsigned short)param_4;
    DAT_00703be4 = (unsigned short)(DAT_006d4984 - 1);
    _DAT_006da4fc = param_2;
    DAT_00732f3c = 0x200;
    _DAT_006d4994 = 0x100;
    DAT_0070acc4 = 9;
    DAT_0071ed00 = 0;
    DAT_006d4a42 = (short)(param_4 / 2);
    _DAT_0071f2e4 = (int)((unsigned int)DAT_0070acc8 * (unsigned int)param_2 + (unsigned int)param_1);
    _DAT_006db7bc = param_1;

    idx = (unsigned int)DAT_0071ecf4 * 0x5bdu;
    __builtin_memcpy(&s0, DAT_0071bb2f + idx, 2);
    __builtin_memcpy(&s1, DAT_0071bb2b + idx, 2);
    __builtin_memcpy(&s2, DAT_0071bb2d + idx, 2);
    FUN_00483180(s0, s1, s2, 0, 0, 0, 0);
    FUN_004832a0(DAT_005f0e34, DAT_005f0e32, DAT_005f0e30, DAT_00525e08, 0);

    idx = (unsigned int)DAT_0071ecf4 * 0x5bdu;
    __builtin_memcpy(&i0, DAT_0071bb1b + idx, 4);
    _DAT_006ccae0 = (float)(int)((unsigned int)DAT_005f0e24 - (unsigned int)i0);
    idx = (unsigned int)DAT_0071ecf4 * 0x5bdu;
    __builtin_memcpy(&i1, DAT_0071bb1f + idx, 4);
    _DAT_006ccae4 = (float)(int)((unsigned int)DAT_005f0e28 - (unsigned int)i1);
    _DAT_006ccaec = (float)DAT_0073255c * _DAT_00520168;
    _DAT_006ccaf0 = (float)DAT_00732558 * _DAT_00520168;
    _DAT_006ccaf4 = (float)DAT_00732568 * _DAT_00520168;
    _DAT_006ccafc = (float)DAT_00732578 * _DAT_00520168;
    _DAT_006ccaf8 = (float)DAT_00732584 * _DAT_00520168;
    _DAT_006ccb04 = (float)DAT_0073253c * _DAT_00520168;
    _DAT_006ccb00 = (float)DAT_00732530 * _DAT_00520168;
    _DAT_006ccb0c = (float)DAT_00732548 * _DAT_00520168;
    _DAT_006ccb08 = (float)DAT_00732538 * _DAT_00520168;
    idx = (unsigned int)DAT_0071ecf4 * 0x5bdu;
    __builtin_memcpy(&i2, DAT_0071bb23 + idx, 4);
    _DAT_006ccae8 = (float)(int)((unsigned int)DAT_005f0e2c - (unsigned int)i2);
    FUN_00414fe0((float *)&_DAT_006ccae0, (float *)&_DAT_006ccaec);

    _DAT_006ccaf0 = (float)DAT_0070a138 * _DAT_00520168;
    _DAT_006ccaf4 = (float)DAT_0070a134 * _DAT_00520168;
    _DAT_006ccaf8 = (float)DAT_0070a130 * _DAT_00520168;
    _DAT_006ccaec = (float)DAT_0070a12c * _DAT_00520168;
    _DAT_006ccafc = (float)DAT_0070a010 * _DAT_00520168;
    _DAT_006ccb00 = (float)DAT_0070a00c * _DAT_00520168;
    _DAT_006ccb04 = (float)_DAT_0070a008 * _DAT_00520168;
    _DAT_006ccb08 = (float)_DAT_0070a124 * _DAT_00520168;
    _DAT_006ccb0c = (float)_DAT_0070a120 * _DAT_00520168;
    _DAT_006ccb10 = -_DAT_006ccae0;
    _DAT_006ccb14 = -_DAT_006ccae4;
    _DAT_006ccb18 = -_DAT_006ccae8;
    _DAT_006ccb1c = _DAT_006ccaec;
    _DAT_006ccb20 = _DAT_006ccaf8;
    _DAT_006ccb24 = _DAT_006ccb04;
    _DAT_006ccb28 = _DAT_006ccaf0;
    _DAT_006ccb2c = _DAT_006ccafc;
    _DAT_006ccb30 = _DAT_006ccb08;
    _DAT_006ccb34 = _DAT_006ccaf4;
    _DAT_006ccb38 = _DAT_006ccb00;
    _DAT_006ccb3c = _DAT_006ccb0c;
    FUN_00414fe0((float *)&_DAT_006ccb10, (float *)&_DAT_006ccb1c);

    if (DAT_00525e14 == 0) {
        FUN_00465740();
        if (DAT_00525e1c < (int)((((unsigned int)DAT_007091ae >> 7) + 2) * (unsigned int)DAT_006d4984 + (unsigned int)DAT_00527e74)) {
            if (DAT_00525e18 != 0) {
                FUN_0048f4b0((unsigned int)DAT_00525e18);
            }
            DAT_00525e18 = (unsigned short)FUN_0048f3c0((((unsigned int)DAT_007091ae >> 7) + 2) * (unsigned int)DAT_006d4984 + (unsigned int)DAT_00527e74, 0);
            DAT_00525e1c = (int)((((unsigned int)DAT_007091ae >> 7) + 2) * (unsigned int)DAT_006d4984 + (unsigned int)DAT_00527e74);
        }
        DAT_006da9f4 = FUN_0048f4f0((unsigned int)DAT_00525e18);
        uVar2 = 0;
        puVar4 = (unsigned char *)((unsigned int)DAT_00527e74 + DAT_006da9f4);
        if (DAT_006d4984 != 0) {
            do {
                *puVar4 = 1;
                puVar5 = puVar4 + 1;
                uVar6 = (unsigned int)DAT_007091ae;
                uVar7 = uVar6;
                if (0xff < uVar6) {
                    *puVar5 = 0;
                    puVar5 = puVar4 + 2;
                    uVar7 = uVar6 - 0xff;
                    if (0xff < uVar7) {
                        *puVar5 = 0;
                        puVar5 = puVar4 + 3;
                        uVar7 = uVar6 - 0x1ff;
                    }
                }
                *puVar5 = (unsigned char)uVar7;
                puVar4 = puVar5 + 1;
                uVar2 = uVar2 + 1;
            } while (uVar2 < DAT_006d4984);
        }
        DAT_00525e14 = 1;
    }
    lVar8 = __ftol();
    DAT_006db7b4 = (int)lVar8;
    uVar3 = (unsigned int)DAT_005254e0; /* upper ECX bits (extraout_ECX) not reproducible */
    __builtin_memcpy(&p, &DAT_005f0e3f, 4);
    *(unsigned char *)(p + 0x88) = DAT_005254e0;
    uVar1 = (unsigned int)DAT_005254f8;
    DAT_005254f8 = 0;
    FUN_00465970(uVar3);
    FUN_00486310(0x5f0e20);
    FUN_00466020();
    FUN_00465c40();
    DAT_005254f8 = (int)uVar1;
    return 1;
}
