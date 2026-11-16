/* SPDX-License-Identifier: MIT */
/* z_bop 0040c430 */
extern void __cdecl FUN_004aba50(void);
extern void __cdecl FUN_004a9520(short, short, short, short);
extern void __cdecl FUN_004abe50(void);
extern void __cdecl FUN_00408140(void);
extern void __cdecl FUN_004b1c10(void);
extern void __cdecl FUN_004b1c90(void);
extern void __cdecl FUN_004b1d10(float *, int);
extern void __cdecl FUN_004b1d70(void);
extern void __cdecl FUN_004b1de0(int *, int);
extern void __cdecl FUN_004b2020(void);
extern void __cdecl FUN_004b1c50(void);
extern void __cdecl FUN_00408110(void);

extern int DAT_009a8e3c;
extern int DAT_009a7b60;
extern unsigned short DAT_009d682c;
extern unsigned short DAT_009a1fe4;
extern unsigned char DAT_00a08100;
extern void (__cdecl *DAT_009eca20)(int, int, int, int, int);
extern int DAT_0054f990;
extern int DAT_0054f994;
extern int DAT_0052f868;
extern int DAT_0054f9a0;
extern float *DAT_0053f970;
extern int *DAT_0054f988;
extern float _DAT_0053f974;
extern float _DAT_0054f9a4;
extern float _DAT_00518060;
extern float _DAT_00518080;
extern int DAT_00a90b20;
extern unsigned char DAT_009a7bc0[];
extern unsigned char DAT_009a7bc1[];
extern unsigned char DAT_009a7bc2[];

#define IADD(a, b) ((int)((unsigned int)(a) + (unsigned int)(b)))
#define ISUB(a, b) ((int)((unsigned int)(a) - (unsigned int)(b)))
#define IMUL(a, b) ((int)((unsigned int)(a) * (unsigned int)(b)))

#define VB(k) IADD(IMUL(DAT_0054f994, 8), (k))

/* straight-line index-buffer emission, no control flow */
#define IDXS() \
    DAT_0054f988[IMUL(DAT_0054f9a0, 5)] = DAT_0054f994; \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 1)] = IADD(DAT_0054f994, 1); \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 2)] = IADD(DAT_0054f994, 2); \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 4)] = 0; \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 3)] = 0x9800; \
    DAT_0054f9a0 = IADD(DAT_0054f9a0, 1); \
    DAT_0054f988[IMUL(DAT_0054f9a0, 5)] = DAT_0054f994; \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 1)] = IADD(DAT_0054f994, 2); \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 2)] = IADD(DAT_0054f994, 3); \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 4)] = 0; \
    DAT_0054f988[IADD(IMUL(DAT_0054f9a0, 5), 3)] = 0x9800; \
    DAT_0054f994 = IADD(DAT_0054f994, 4); \
    DAT_0054f9a0 = IADD(DAT_0054f9a0, 1);

void __cdecl FUN_0040c430(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    float fVar1;
    unsigned char uVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int local_8;
    int local_4;

    if ((param_6 == 1) && (param_3 == 4) && (param_4 == 4)) {
        FUN_004aba50();
        uVar2 = DAT_00a08100;
        iVar4 = IADD(param_1, DAT_009a8e3c);
        DAT_00a08100 = (unsigned char)param_5;
        iVar5 = IADD(param_2, DAT_009a7b60);
        FUN_004a9520((short)DAT_009a8e3c, (short)DAT_009a7b60,
                     (short)(DAT_009d682c + (short)DAT_009a8e3c),
                     (short)(DAT_009a1fe4 + (short)DAT_009a7b60));
        /* upper 16 bits of arg 1 are Ghidra's extraout_var; unverified */
        (*DAT_009eca20)((int)(short)iVar4, iVar5, (int)(short)iVar4 + 4,
                        (int)(short)iVar5 + 4, 1);
        DAT_00a08100 = uVar2;
        FUN_004abe50();
        return;
    }
    if ((DAT_0054f990 < IADD(DAT_0054f994, 0x20)) || (DAT_0052f868 < IADD(DAT_0054f9a0, 0x10))) {
        FUN_00408140();
        FUN_004b1c10();
        FUN_004b1c90();
        FUN_004b1d10(DAT_0053f970, DAT_0054f994);
        FUN_004b1d70();
        FUN_004b1de0(DAT_0054f988, DAT_0054f9a0);
        FUN_004b2020();
        FUN_004b1c50();
        FUN_00408110();
        DAT_0054f9a0 = 0;
        DAT_0054f994 = 0;
    }
    iVar3 = (int)((((((unsigned int)DAT_009a7bc0[IMUL(param_5, 3)] - 0x40u) * 0x100u) +
                    (unsigned int)DAT_009a7bc1[IMUL(param_5, 3)]) * 0x100u +
                   (unsigned int)DAT_009a7bc2[IMUL(param_5, 3)]) * 4u);
    iVar5 = IADD(param_1, param_3);
    iVar4 = IADD(param_2, param_4);
    local_8 = param_3 >> 3;
    local_4 = param_4 >> 3;
    if (local_8 < 3) {
        local_8 = 3;
    }
    if (local_4 < 3) {
        local_4 = 3;
    }
    if (param_3 < local_8) {
        local_8 = param_3;
    }
    if (param_4 < local_4) {
        local_4 = param_4;
    }
    if (param_6 < 1) {
        param_6 = 1;
    }
    fVar1 = _DAT_00518060 / ((float)param_6 * _DAT_00518080 + _DAT_00518060);
    if (DAT_00a90b20 == 2) {
        fVar1 = _DAT_00518060 - fVar1;
    }
    if ((-1 < param_2) && (param_2 < (int)(unsigned int)DAT_009a1fe4)) {
        iVar6 = IADD(local_8, param_1);
        iVar7 = param_1;
        if (param_1 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_009d682c <= iVar6) {
            iVar6 = DAT_009d682c - 1;
        }
        if (iVar7 < iVar6) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)iVar7;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)param_2;
            DAT_0053f970[VB(8)] = _DAT_0054f9a4 + (float)iVar6;
            DAT_0053f970[VB(9)] = (float)param_2 + _DAT_0053f974;
            DAT_0053f970[VB(0x10)] = (float)iVar6 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x11)] = _DAT_0053f974 + (float)IADD(param_2, 1);
            iVar6 = 0;
            DAT_0053f970[VB(0x18)] = (float)iVar7 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x19)] = (float)IADD(param_2, 1) + _DAT_0053f974;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
        iVar6 = ISUB(iVar5, local_8);
        if (iVar6 < 0) {
            iVar6 = 0;
        }
        iVar7 = iVar5;
        if ((int)(unsigned int)DAT_009d682c <= iVar5) {
            iVar7 = DAT_009d682c - 1;
        }
        if (iVar6 < iVar7) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)iVar6;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)param_2;
            DAT_0053f970[VB(8)] = _DAT_0054f9a4 + (float)iVar7;
            DAT_0053f970[VB(9)] = (float)param_2 + _DAT_0053f974;
            DAT_0053f970[VB(0x10)] = (float)iVar7 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x11)] = _DAT_0053f974 + (float)IADD(param_2, 1);
            DAT_0053f970[VB(0x18)] = (float)iVar6 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x19)] = (float)IADD(param_2, 1) + _DAT_0053f974;
            iVar6 = 0;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
    }
    if ((-1 < iVar4) && (iVar4 < (int)(unsigned int)DAT_009a1fe4)) {
        iVar6 = IADD(local_8, param_1);
        iVar7 = param_1;
        if (param_1 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_009d682c <= iVar6) {
            iVar6 = DAT_009d682c - 1;
        }
        if (iVar7 < iVar6) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)iVar7;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)iVar4;
            DAT_0053f970[VB(8)] = _DAT_0054f9a4 + (float)iVar6;
            DAT_0053f970[VB(9)] = (float)iVar4 + _DAT_0053f974;
            DAT_0053f970[VB(0x10)] = (float)iVar6 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x11)] = _DAT_0053f974 + (float)IADD(iVar4, 1);
            DAT_0053f970[VB(0x18)] = (float)iVar7 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x19)] = (float)IADD(iVar4, 1) + _DAT_0053f974;
            iVar6 = 0;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
        local_8 = ISUB(iVar5, local_8);
        iVar6 = IADD(iVar5, 1);
        if (local_8 < 0) {
            local_8 = 0;
        }
        if ((int)(unsigned int)DAT_009d682c <= iVar6) {
            iVar6 = DAT_009d682c - 1;
        }
        if (local_8 < iVar6) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)local_8;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)iVar4;
            DAT_0053f970[VB(8)] = _DAT_0054f9a4 + (float)iVar6;
            DAT_0053f970[VB(9)] = (float)iVar4 + _DAT_0053f974;
            DAT_0053f970[VB(0x10)] = (float)iVar6 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x11)] = _DAT_0053f974 + (float)IADD(iVar4, 1);
            iVar6 = 0;
            DAT_0053f970[VB(0x18)] = (float)local_8 + _DAT_0054f9a4;
            DAT_0053f970[VB(0x19)] = (float)IADD(iVar4, 1) + _DAT_0053f974;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
    }
    if ((-1 < param_1) && (param_1 < (int)(unsigned int)DAT_009d682c)) {
        iVar6 = IADD(local_4, param_2);
        iVar7 = param_2;
        if (param_2 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_009a1fe4 <= iVar6) {
            iVar6 = DAT_009a1fe4 - 1;
        }
        if (iVar7 < iVar6) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)param_1;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)iVar7;
            DAT_0053f970[VB(8)] = (float)param_1 + _DAT_0054f9a4;
            DAT_0053f970[VB(9)] = _DAT_0053f974 + (float)iVar6;
            DAT_0053f970[VB(0x10)] = _DAT_0054f9a4 + (float)IADD(param_1, 1);
            DAT_0053f970[VB(0x11)] = (float)iVar6 + _DAT_0053f974;
            DAT_0053f970[VB(0x18)] = (float)IADD(param_1, 1) + _DAT_0054f9a4;
            DAT_0053f970[VB(0x19)] = (float)iVar7 + _DAT_0053f974;
            iVar6 = 0;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
        iVar6 = ISUB(iVar4, local_4);
        if (iVar6 < 0) {
            iVar6 = 0;
        }
        iVar7 = iVar4;
        if ((int)(unsigned int)DAT_009a1fe4 <= iVar4) {
            iVar7 = DAT_009a1fe4 - 1;
        }
        if (iVar6 < iVar7) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)param_1;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)iVar6;
            DAT_0053f970[VB(8)] = (float)param_1 + _DAT_0054f9a4;
            DAT_0053f970[VB(9)] = _DAT_0053f974 + (float)iVar7;
            DAT_0053f970[VB(0x10)] = _DAT_0054f9a4 + (float)IADD(param_1, 1);
            DAT_0053f970[VB(0x11)] = (float)iVar7 + _DAT_0053f974;
            DAT_0053f970[VB(0x18)] = (float)IADD(param_1, 1) + _DAT_0054f9a4;
            iVar7 = 0;
            DAT_0053f970[VB(0x19)] = (float)iVar6 + _DAT_0053f974;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar7), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar7), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar7), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar7), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar7), 8), 4)], &iVar3, 4);
                iVar6 = IADD(DAT_0054f994, iVar7);
                iVar7 = IADD(iVar7, 1);
                DAT_0053f970[IADD(IMUL(iVar6, 8), 5)] = 0;
            } while (iVar7 < 4);
            IDXS()
        }
    }
    if ((-1 < iVar5) && (iVar5 < (int)(unsigned int)DAT_009d682c)) {
        iVar6 = IADD(local_4, param_2);
        if (param_2 < 0) {
            param_2 = 0;
        }
        if ((int)(unsigned int)DAT_009a1fe4 <= iVar6) {
            iVar6 = DAT_009a1fe4 - 1;
        }
        if (param_2 < iVar6) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)iVar5;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)param_2;
            DAT_0053f970[VB(8)] = (float)iVar5 + _DAT_0054f9a4;
            DAT_0053f970[VB(9)] = _DAT_0053f974 + (float)iVar6;
            DAT_0053f970[VB(0x10)] = _DAT_0054f9a4 + (float)IADD(iVar5, 1);
            DAT_0053f970[VB(0x11)] = (float)iVar6 + _DAT_0053f974;
            DAT_0053f970[VB(0x18)] = (float)IADD(iVar5, 1) + _DAT_0054f9a4;
            iVar6 = 0;
            DAT_0053f970[VB(0x19)] = (float)param_2 + _DAT_0053f974;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar6), 8), 4)], &iVar3, 4);
                iVar7 = IADD(DAT_0054f994, iVar6);
                iVar6 = IADD(iVar6, 1);
                DAT_0053f970[IADD(IMUL(iVar7, 8), 5)] = 0;
            } while (iVar6 < 4);
            IDXS()
        }
        local_4 = ISUB(iVar4, local_4);
        if (local_4 < 0) {
            local_4 = 0;
        }
        if ((int)(unsigned int)DAT_009a1fe4 <= iVar4) {
            iVar4 = DAT_009a1fe4 - 1;
        }
        if (local_4 < iVar4) {
            DAT_0053f970[VB(0)] = _DAT_0054f9a4 + (float)iVar5;
            DAT_0053f970[VB(1)] = _DAT_0053f974 + (float)local_4;
            DAT_0053f970[VB(8)] = (float)iVar5 + _DAT_0054f9a4;
            DAT_0053f970[VB(9)] = _DAT_0053f974 + (float)iVar4;
            DAT_0053f970[VB(0x10)] = _DAT_0054f9a4 + (float)IADD(iVar5, 1);
            DAT_0053f970[VB(0x11)] = (float)iVar4 + _DAT_0053f974;
            DAT_0053f970[VB(0x18)] = (float)IADD(iVar5, 1) + _DAT_0054f9a4;
            iVar4 = 0;
            DAT_0053f970[VB(0x19)] = (float)local_4 + _DAT_0053f974;
            do {
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar4), 8), 2)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar4), 8), 3)] = fVar1;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar4), 8), 6)] = 0;
                DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar4), 8), 7)] = 0;
                __builtin_memcpy(&DAT_0053f970[IADD(IMUL(IADD(DAT_0054f994, iVar4), 8), 4)], &iVar3, 4);
                iVar5 = IADD(DAT_0054f994, iVar4);
                iVar4 = IADD(iVar4, 1);
                DAT_0053f970[IADD(IMUL(iVar5, 8), 5)] = 0;
            } while (iVar4 < 4);
            IDXS()
        }
    }
    return;
}
