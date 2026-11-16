/* SPDX-License-Identifier: MIT */
/* z_xvt 00486750 */
extern short DAT_006da850[];
extern unsigned int _DAT_005254e0;
extern float _DAT_005201ec;          /* unverified: assumed float */
extern int DAT_0071ecf4;
extern unsigned char DAT_0071bb1b;   /* base address only, accessed unaligned */
extern unsigned char DAT_0071bb1f;
extern unsigned char DAT_0071bb23;

/* Assumed int: Ghidra writes (float)X (INT2FLOAT). Unverified against disassembly. */
extern int DAT_0073255c, DAT_00732558, DAT_00732568, DAT_00732578, DAT_00732584;
extern int DAT_0073253c, DAT_00732530, DAT_00732548, DAT_00732538;
extern int DAT_0070a138, DAT_0070a134, DAT_0070a130, DAT_0070a12c, DAT_0070a010;
extern int DAT_0070a00c, _DAT_0070a008, _DAT_0070a124, _DAT_0070a120;

extern unsigned int DAT_00602678;
extern unsigned int *DAT_0052a470;
extern unsigned int DAT_006026c8, DAT_006026cc, DAT_006026d8, _DAT_006026dc;
extern unsigned char DAT_006026e0, DAT_0052a478;
extern unsigned int *DAT_006026a4;
extern unsigned int DAT_006026a0, _DAT_006026c0, _DAT_006026a8, DAT_00602690, DAT_006026bc;
extern unsigned int DAT_0052a46c;

int *__cdecl FUN_0048f4f0(unsigned int param_1);
void __cdecl FUN_00486000(int *param_1, int param_2);
void __cdecl FUN_00414fe0(float *param_1, float *param_2);
void __cdecl FUN_00486080(void *param_1, void *param_2, int param_3, int param_4);
void __cdecl FUN_00486b80(int param_1, unsigned int *param_2, int *param_3);
void __cdecl FUN_0048f510(void);

void __cdecl FUN_00486750(int param_1, int param_2)
{
    unsigned int uVar4;
    unsigned int *puVar2;
    int *piVar3;
    int *piVar5;
    int iVar6;
    int iVar7;
    int iVar8;
    unsigned short uStack_dc; /* intentionally uninitialized, as in original */
    volatile unsigned short *pStack_dc;
    union {
        int i[0x34];
        float f[0x34];
    } local_d0;
    unsigned char *pb1;
    unsigned char *pb3;
    int pField;
    int tmp;
    int cnt;
    int ofs;
    int a;

    pb1 = (unsigned char *)param_1;
    uVar4 = (unsigned int)pb1[3];
    __builtin_memcpy(&pField, pb1 + 0x1f, 4);
    if ((uVar4 == 0x59) && (pField != 0)) {
        uVar4 = (unsigned int)*(unsigned char *)(pField + 0x85);
    }
    __builtin_memcpy(&pField, pb1 + 0x1f, 4);
    if (pField == 0) {
        _DAT_005254e0 = 0;
    } else {
        _DAT_005254e0 = (unsigned int)*(unsigned char *)(pField + 0x88);
    }
    pStack_dc = (volatile unsigned short *)&uStack_dc;
    piVar3 = FUN_0048f4f0(((unsigned int)*pStack_dc << 16) | (unsigned int)(unsigned short)DAT_006da850[uVar4]);
    pb3 = (unsigned char *)piVar3;
    piVar5 = (int *)*piVar3;
    if (piVar3 != piVar5) {
        iVar6 = 0;
        iVar8 = (int)((unsigned int)(int)piVar3 - (unsigned int)(int)piVar5);
        *piVar3 = (int)((unsigned int)(int)piVar5 + (unsigned int)iVar8);
        __builtin_memcpy(&tmp, pb3 + 10, 4);
        if (tmp != 0) {
            iVar7 = 0;
            tmp = (int)((unsigned int)tmp + (unsigned int)iVar8);
            __builtin_memcpy(pb3 + 10, &tmp, 4);
            __builtin_memcpy(&cnt, pb3 + 6, 4);
            if (0 < cnt) {
                do {
                    __builtin_memcpy(&ofs, pb3 + 10, 4);
                    piVar5 = (int *)(int)((unsigned int)ofs + (unsigned int)iVar6);
                    if (*piVar5 != 0) {
                        *piVar5 = (int)((unsigned int)*piVar5 + (unsigned int)iVar8);
                        __builtin_memcpy(&ofs, pb3 + 10, 4);
                        FUN_00486000(*(int **)(int)((unsigned int)ofs + (unsigned int)iVar6), iVar8);
                    }
                    iVar6 = (int)((unsigned int)iVar6 + 4u);
                    iVar7 = (int)((unsigned int)iVar7 + 1u);
                    __builtin_memcpy(&cnt, pb3 + 6, 4);
                } while (iVar7 < cnt);
            }
        }
    }

    for (iVar6 = 0; iVar6 != 0x34; iVar6++) {
        local_d0.i[iVar6] = 0;
    }
    local_d0.i[0] = param_1;

    __builtin_memcpy(&a, &DAT_0071bb1b + (int)((unsigned int)DAT_0071ecf4 * 0x5bdu), 4);
    local_d0.f[2] = (float)(int)((unsigned int)*(int *)(param_1 + 4) - (unsigned int)a);
    __builtin_memcpy(&a, &DAT_0071bb1f + (int)((unsigned int)DAT_0071ecf4 * 0x5bdu), 4);
    local_d0.f[3] = (float)(int)((unsigned int)*(int *)(param_1 + 8) - (unsigned int)a);
    local_d0.f[5] = (float)DAT_0073255c * _DAT_005201ec;
    local_d0.f[6] = (float)DAT_00732558 * _DAT_005201ec;
    local_d0.f[7] = (float)DAT_00732568 * _DAT_005201ec;
    local_d0.f[9] = (float)DAT_00732578 * _DAT_005201ec;
    local_d0.f[8] = (float)DAT_00732584 * _DAT_005201ec;
    local_d0.f[11] = (float)DAT_0073253c * _DAT_005201ec;
    local_d0.f[10] = (float)DAT_00732530 * _DAT_005201ec;
    local_d0.f[13] = (float)DAT_00732548 * _DAT_005201ec;
    local_d0.f[12] = (float)DAT_00732538 * _DAT_005201ec;
    __builtin_memcpy(&a, &DAT_0071bb23 + (int)((unsigned int)DAT_0071ecf4 * 0x5bdu), 4);
    local_d0.f[4] = (float)(int)((unsigned int)*(int *)(param_1 + 0xc) - (unsigned int)a);
    FUN_00414fe0(&local_d0.f[2], &local_d0.f[5]);

    local_d0.f[6] = (float)DAT_0070a138 * _DAT_005201ec;
    local_d0.f[7] = (float)DAT_0070a134 * _DAT_005201ec;
    local_d0.f[8] = (float)DAT_0070a130 * _DAT_005201ec;
    local_d0.f[5] = (float)DAT_0070a12c * _DAT_005201ec;
    local_d0.f[9] = (float)DAT_0070a010 * _DAT_005201ec;
    local_d0.f[10] = (float)DAT_0070a00c * _DAT_005201ec;
    local_d0.f[11] = (float)_DAT_0070a008 * _DAT_005201ec;
    local_d0.f[12] = (float)_DAT_0070a124 * _DAT_005201ec;
    local_d0.f[13] = (float)_DAT_0070a120 * _DAT_005201ec;
    local_d0.f[14] = -local_d0.f[2];
    local_d0.f[15] = -local_d0.f[3];
    local_d0.f[16] = -local_d0.f[4];
    local_d0.f[17] = local_d0.f[5];
    local_d0.f[18] = local_d0.f[8];
    local_d0.f[19] = local_d0.f[11];
    local_d0.f[20] = local_d0.f[6];
    local_d0.f[21] = local_d0.f[9];
    local_d0.f[22] = local_d0.f[12];
    local_d0.f[23] = local_d0.f[7];
    local_d0.f[24] = local_d0.f[10];
    local_d0.f[25] = local_d0.f[13];
    FUN_00414fe0(&local_d0.f[14], &local_d0.f[17]);

    DAT_00602678 = 0;
    if (DAT_0052a470 == (unsigned int *)0) {
        DAT_0052a470 = &DAT_006026c8;
        _DAT_006026dc = 8;
        DAT_006026d8 = 8;
        DAT_006026cc = 0x10;
        DAT_006026c8 = 0x100;
        FUN_00486080(&DAT_006026e0, &DAT_0052a478, 8, 8);
    }
    iVar6 = 0;
    DAT_006026a4 = DAT_0052a470;
    DAT_006026a0 = 0;
    _DAT_006026c0 = 0;
    _DAT_006026a8 = 0;
    DAT_00602690 = 0;
    DAT_006026bc = 0;
    __builtin_memcpy(&cnt, pb3 + 6, 4);
    if (0 < cnt) {
        do {
            __builtin_memcpy(&ofs, pb3 + 10, 4);
            puVar2 = *(unsigned int **)(int)((unsigned int)ofs + (unsigned int)iVar6 * 4u);
            if (puVar2[1] == 0x14) {
                DAT_0052a46c = DAT_0052a46c + 1;
                param_2 = (int)((unsigned int)param_2 + 1u);
                __builtin_memcpy(&ofs, pb3 + 10, 4);
                FUN_00486b80((int)piVar3,
                             *(unsigned int **)(int)((unsigned int)ofs + (unsigned int)iVar6 * 4u),
                             local_d0.i);
            } else if (param_2 == iVar6) {
                DAT_0052a46c = DAT_0052a46c + 1;
                FUN_00486b80((int)piVar3, puVar2, local_d0.i);
            }
            iVar6 = (int)((unsigned int)iVar6 + 1u);
            __builtin_memcpy(&cnt, pb3 + 6, 4);
        } while (iVar6 < cnt);
    }
    FUN_0048f510();
    return;
}
