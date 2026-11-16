/* SPDX-License-Identifier: MIT */
/* z_xvt 004604c0 */
void FUN_004a6a80(void);
void FUN_004a6f90(void);
void FUN_0048ed00(short a, short b, short c, short d);
void FUN_004b5bc2(void);
void FUN_004b5c5c(char *vb, int n);
void FUN_004b5ce0(void);
void FUN_004b5d64(int *ib, unsigned int n);
void FUN_004b6054(void);

extern unsigned char DAT_00732e50;
extern int _DAT_006db7bc;
extern int _DAT_006da4fc;
extern unsigned short DAT_007091ae;
extern unsigned short DAT_006d4984;
extern void (__cdecl *DAT_0071eae0)(int, int, int, int, int);
extern int DAT_005f10d0;
extern int DAT_005f10d4;
extern int DAT_005f0fa8;
extern unsigned int DAT_005f10e0;
extern char *DAT_005f10b0;
extern int *DAT_005f10c8;
extern float _DAT_005f10e4;
extern float _DAT_005f10b4;
extern float _DAT_005201b8;
extern unsigned char DAT_006da540[];
extern unsigned char DAT_006da541[];
extern unsigned char DAT_006da542[];

void __cdecl FUN_004604c0(int param_1, int param_2, int param_3, int param_4, int param_5, int param_6)
{
    float fVar1;
    unsigned char uVar2;
    unsigned short extraout_var = 0; /* upper half of ECX left by FUN_0048ed00; unresolved register residue */
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int local_8;
    int local_4;
    char *vp;

    if (((param_6 == 1) && (param_3 == 4)) && (param_4 == 4)) {
        FUN_004a6a80();
        uVar2 = DAT_00732e50;
        iVar4 = (int)((unsigned int)param_1 + (unsigned int)_DAT_006db7bc);
        DAT_00732e50 = (unsigned char)param_5;
        iVar5 = (int)((unsigned int)param_2 + (unsigned int)_DAT_006da4fc);
        FUN_0048ed00((short)_DAT_006db7bc, (short)_DAT_006da4fc,
                     (short)((unsigned int)DAT_007091ae + (unsigned int)(short)_DAT_006db7bc),
                     (short)((unsigned int)DAT_006d4984 + (unsigned int)(short)_DAT_006da4fc));
        (*DAT_0071eae0)((int)(((unsigned int)extraout_var << 16) | (unsigned int)(unsigned short)(short)iVar4),
                        iVar5,
                        (int)(short)iVar4 + 4,
                        (int)(short)iVar5 + 4,
                        1);
        DAT_00732e50 = uVar2;
        FUN_004a6f90();
        return;
    }
    if ((DAT_005f10d0 < (int)((unsigned int)DAT_005f10d4 + 0x20u)) ||
        (DAT_005f0fa8 < (int)(DAT_005f10e0 + 0x10u))) {
        FUN_004b5bc2();
        FUN_004b5c5c(DAT_005f10b0, DAT_005f10d4);
        FUN_004b5ce0();
        FUN_004b5d64(DAT_005f10c8, DAT_005f10e0);
        FUN_004b6054();
        DAT_005f10e0 = 0;
        DAT_005f10d4 = 0;
    }
    iVar3 = (int)(((((unsigned int)((int)DAT_006da540[(unsigned int)param_5 * 3u] - 0x40) * 0x100u +
                    (unsigned int)DAT_006da541[(unsigned int)param_5 * 3u]) * 0x100u) +
                   (unsigned int)DAT_006da542[(unsigned int)param_5 * 3u]) * 4u);
    iVar5 = (int)((unsigned int)param_1 + (unsigned int)param_3);
    iVar4 = (int)((unsigned int)param_2 + (unsigned int)param_4);
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
    fVar1 = (float)param_6 * _DAT_005201b8;
    if ((-1 < param_2) && (param_2 < (int)(unsigned int)DAT_006d4984)) {
        iVar6 = (int)((unsigned int)local_8 + (unsigned int)param_1);
        iVar7 = param_1;
        if (param_1 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_007091ae <= iVar6) {
            iVar6 = (int)((unsigned int)DAT_007091ae - 1u);
        }
        if (iVar7 < iVar6) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)iVar7;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)param_2;
            *(float *)(vp + 0x20) = _DAT_005f10e4 + (float)iVar6;
            *(float *)(vp + 0x24) = (float)param_2 + _DAT_005f10b4;
            *(float *)(vp + 0x40) = (float)iVar6 + _DAT_005f10e4;
            *(float *)(vp + 0x44) = _DAT_005f10b4 + (float)(int)((unsigned int)param_2 + 1u);
            *(float *)(vp + 0x60) = (float)iVar7 + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)(int)((unsigned int)param_2 + 1u) + _DAT_005f10b4;
            iVar6 = 0;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
        iVar6 = (int)((unsigned int)iVar5 - (unsigned int)local_8);
        if (iVar6 < 0) {
            iVar6 = 0;
        }
        iVar7 = iVar5;
        if ((int)(unsigned int)DAT_007091ae <= iVar5) {
            iVar7 = (int)((unsigned int)DAT_007091ae - 1u);
        }
        if (iVar6 < iVar7) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)iVar6;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)param_2;
            *(float *)(vp + 0x20) = _DAT_005f10e4 + (float)iVar7;
            *(float *)(vp + 0x24) = (float)param_2 + _DAT_005f10b4;
            *(float *)(vp + 0x40) = (float)iVar7 + _DAT_005f10e4;
            *(float *)(vp + 0x44) = _DAT_005f10b4 + (float)(int)((unsigned int)param_2 + 1u);
            *(float *)(vp + 0x60) = (float)iVar6 + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)(int)((unsigned int)param_2 + 1u) + _DAT_005f10b4;
            iVar6 = 0;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
    }
    if ((-1 < iVar4) && (iVar4 < (int)(unsigned int)DAT_006d4984)) {
        iVar6 = (int)((unsigned int)local_8 + (unsigned int)param_1);
        iVar7 = param_1;
        if (param_1 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_007091ae <= iVar6) {
            iVar6 = (int)((unsigned int)DAT_007091ae - 1u);
        }
        if (iVar7 < iVar6) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)iVar7;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)iVar4;
            *(float *)(vp + 0x20) = _DAT_005f10e4 + (float)iVar6;
            *(float *)(vp + 0x24) = (float)iVar4 + _DAT_005f10b4;
            *(float *)(vp + 0x40) = (float)iVar6 + _DAT_005f10e4;
            *(float *)(vp + 0x44) = _DAT_005f10b4 + (float)(int)((unsigned int)iVar4 + 1u);
            *(float *)(vp + 0x60) = (float)iVar7 + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)(int)((unsigned int)iVar4 + 1u) + _DAT_005f10b4;
            iVar6 = 0;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
        }
        local_8 = (int)((unsigned int)iVar5 - (unsigned int)local_8);
        iVar6 = (int)((unsigned int)iVar5 + 1u);
        if (local_8 < 0) {
            local_8 = 0;
        }
        if ((int)(unsigned int)DAT_007091ae <= iVar6) {
            iVar6 = (int)((unsigned int)DAT_007091ae - 1u);
        }
        if (local_8 < iVar6) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)local_8;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)iVar4;
            *(float *)(vp + 0x20) = _DAT_005f10e4 + (float)iVar6;
            *(float *)(vp + 0x24) = (float)iVar4 + _DAT_005f10b4;
            *(float *)(vp + 0x40) = (float)iVar6 + _DAT_005f10e4;
            *(float *)(vp + 0x44) = _DAT_005f10b4 + (float)(int)((unsigned int)iVar4 + 1u);
            iVar6 = 0;
            *(float *)(vp + 0x60) = (float)local_8 + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)(int)((unsigned int)iVar4 + 1u) + _DAT_005f10b4;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
    }
    if ((-1 < param_1) && (param_1 < (int)(unsigned int)DAT_007091ae)) {
        iVar6 = (int)((unsigned int)local_4 + (unsigned int)param_2);
        iVar7 = param_2;
        if (param_2 < 0) {
            iVar7 = 0;
        }
        if ((int)(unsigned int)DAT_006d4984 <= iVar6) {
            iVar6 = (int)((unsigned int)DAT_006d4984 - 1u);
        }
        if (iVar7 < iVar6) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)param_1;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)iVar7;
            *(float *)(vp + 0x20) = (float)param_1 + _DAT_005f10e4;
            *(float *)(vp + 0x24) = _DAT_005f10b4 + (float)iVar6;
            *(float *)(vp + 0x40) = _DAT_005f10e4 + (float)(int)((unsigned int)param_1 + 1u);
            *(float *)(vp + 0x44) = (float)iVar6 + _DAT_005f10b4;
            *(float *)(vp + 0x60) = (float)(int)((unsigned int)param_1 + 1u) + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)iVar7 + _DAT_005f10b4;
            iVar6 = 0;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
        iVar6 = (int)((unsigned int)iVar4 - (unsigned int)local_4);
        if (iVar6 < 0) {
            iVar6 = 0;
        }
        iVar7 = iVar4;
        if ((int)(unsigned int)DAT_006d4984 <= iVar4) {
            iVar7 = (int)((unsigned int)DAT_006d4984 - 1u);
        }
        if (iVar6 < iVar7) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)param_1;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)iVar6;
            *(float *)(vp + 0x20) = (float)param_1 + _DAT_005f10e4;
            *(float *)(vp + 0x24) = _DAT_005f10b4 + (float)iVar7;
            *(float *)(vp + 0x40) = _DAT_005f10e4 + (float)(int)((unsigned int)param_1 + 1u);
            *(float *)(vp + 0x44) = (float)iVar7 + _DAT_005f10b4;
            *(float *)(vp + 0x60) = (float)(int)((unsigned int)param_1 + 1u) + _DAT_005f10e4;
            *(float *)(vp + 0x64) = (float)iVar6 + _DAT_005f10b4;
            iVar6 = 0;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
    }
    if ((-1 < iVar5) && (iVar5 < (int)(unsigned int)DAT_007091ae)) {
        iVar6 = (int)((unsigned int)local_4 + (unsigned int)param_2);
        if (param_2 < 0) {
            param_2 = 0;
        }
        if ((int)(unsigned int)DAT_006d4984 <= iVar6) {
            iVar6 = (int)((unsigned int)DAT_006d4984 - 1u);
        }
        if (param_2 < iVar6) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)iVar5;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)param_2;
            *(float *)(vp + 0x20) = (float)iVar5 + _DAT_005f10e4;
            *(float *)(vp + 0x24) = _DAT_005f10b4 + (float)iVar6;
            *(float *)(vp + 0x40) = _DAT_005f10e4 + (float)(int)((unsigned int)iVar5 + 1u);
            *(float *)(vp + 0x44) = (float)iVar6 + _DAT_005f10b4;
            *(float *)(vp + 0x60) = (float)(int)((unsigned int)iVar5 + 1u) + _DAT_005f10e4;
            iVar6 = 0;
            *(float *)(vp + 0x64) = (float)param_2 + _DAT_005f10b4;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar6) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar6 = (int)((unsigned int)iVar6 + 1u);
            } while (iVar6 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
        }
        local_4 = (int)((unsigned int)iVar4 - (unsigned int)local_4);
        if (local_4 < 0) {
            local_4 = 0;
        }
        if ((int)(unsigned int)DAT_006d4984 <= iVar4) {
            iVar4 = (int)((unsigned int)DAT_006d4984 - 1u);
        }
        if (local_4 < iVar4) {
            vp = DAT_005f10b0 + (int)((unsigned int)DAT_005f10d4 * 0x20u);
            *(float *)(vp + 0x00) = _DAT_005f10e4 + (float)iVar5;
            *(float *)(vp + 0x04) = _DAT_005f10b4 + (float)local_4;
            *(float *)(vp + 0x20) = (float)iVar5 + _DAT_005f10e4;
            *(float *)(vp + 0x24) = _DAT_005f10b4 + (float)iVar4;
            *(float *)(vp + 0x40) = _DAT_005f10e4 + (float)(int)((unsigned int)iVar5 + 1u);
            *(float *)(vp + 0x44) = (float)iVar4 + _DAT_005f10b4;
            *(float *)(vp + 0x60) = (float)(int)((unsigned int)iVar5 + 1u) + _DAT_005f10e4;
            iVar4 = 0;
            *(float *)(vp + 0x64) = (float)local_4 + _DAT_005f10b4;
            do {
                vp = DAT_005f10b0 + (int)(((unsigned int)DAT_005f10d4 + (unsigned int)iVar4) * 0x20u);
                *(float *)(vp + 8) = fVar1;
                *(float *)(vp + 0xc) = fVar1;
                *(int *)(vp + 0x18) = 0;
                *(int *)(vp + 0x1c) = 0;
                *(int *)(vp + 0x10) = iVar3;
                *(int *)(vp + 0x14) = 0;
                iVar4 = (int)((unsigned int)iVar4 + 1u);
            } while (iVar4 < 4);
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 1u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10e0 = DAT_005f10e0 + 1u;
            DAT_005f10c8[DAT_005f10e0 * 5u] = DAT_005f10d4;
            DAT_005f10c8[DAT_005f10e0 * 5u + 1u] = (int)((unsigned int)DAT_005f10d4 + 2u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 2u] = (int)((unsigned int)DAT_005f10d4 + 3u);
            DAT_005f10c8[DAT_005f10e0 * 5u + 3u] = 0;
            DAT_005f10c8[DAT_005f10e0 * 5u + 4u] = 0x3000;
            DAT_005f10d4 = (int)((unsigned int)DAT_005f10d4 + 4u);
            DAT_005f10e0 = DAT_005f10e0 + 1u;
        }
    }
    return;
}
