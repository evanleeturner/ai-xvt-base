/* SPDX-License-Identifier: MIT */
/* z_bop 004728d0 */
extern unsigned int _DAT_005233a0;
extern unsigned short DAT_009a7ed0[];
extern int DAT_009ecc34;
extern unsigned char DAT_009e9a5b[];
extern unsigned char DAT_009e9a5f[];
extern unsigned char DAT_009e9a63[];
extern float _DAT_005181d8;
extern int DAT_00a004a8, DAT_00a004a4, DAT_00a004b0, DAT_00a004b8, DAT_00a004cc;
extern int DAT_00a00488, DAT_009fe7f4, DAT_00a00490, DAT_00a00480;
extern int DAT_009d77b8, DAT_009d77b4, DAT_009d77b0, DAT_009d77ac;
extern int DAT_009d7690, DAT_009d768c, _DAT_009d7688, _DAT_009d77a4, _DAT_009d77a0;
extern int DAT_0060f1c0;
extern unsigned int *DAT_005271d8;
extern unsigned int DAT_0060f210;
extern int _DAT_0060f224, DAT_0060f220, DAT_0060f214;
extern unsigned char DAT_0060f228[];
extern unsigned char DAT_005271e0[];
extern unsigned int *DAT_0060f1ec;
extern int DAT_0060f1e8, _DAT_0060f208, _DAT_0060f1f0, DAT_0060f1d8, DAT_0060f204;
extern int DAT_005271d4;

extern int * __fastcall FUN_004ac720(unsigned int param);
extern void __cdecl FUN_00471ff0(int *param_1);
extern void __cdecl FUN_00420fe0(float *param_1, float *param_2);
extern void __cdecl FUN_004720d0(void *param_1, void *param_2, int param_3, int param_4);
extern void __cdecl FUN_00472c90(int param_1, unsigned int *param_2, void *param_3);
extern void __cdecl FUN_004ac740(void);

/* local_d0 is one 0x34-dword stack block (cleared by the original loop);
   the float locals are slots in it, index = (offset from local_d0) / 4. */
#define local_c8 local_d0[2]
#define local_c4 local_d0[3]
#define local_c0 local_d0[4]
#define local_bc local_d0[5]
#define local_b8 local_d0[6]
#define local_b4 local_d0[7]
#define local_b0 local_d0[8]
#define local_ac local_d0[9]
#define local_a8 local_d0[10]
#define local_a4 local_d0[11]
#define local_a0 local_d0[12]
#define local_9c local_d0[13]
#define local_98 local_d0[14]
#define local_94 local_d0[15]
#define local_90 local_d0[16]
#define local_8c local_d0[17]
#define local_88 local_d0[18]
#define local_84 local_d0[19]
#define local_80 local_d0[20]
#define local_7c local_d0[21]
#define local_78 local_d0[22]
#define local_74 local_d0[23]
#define local_70 local_d0[24]
#define local_6c local_d0[25]

void __cdecl FUN_004728d0(int param_1, int param_2)
{
    unsigned int *puVar1;
    int *piVar2;
    unsigned int uVar3;
    int iVar4;
    float *piVar5;
    int tmp;
    int cnt;
    int arr;
    int a, b, c;
    float local_d0[0x34];

    uVar3 = (unsigned int)*(unsigned char *)(param_1 + 3);
    __builtin_memcpy(&tmp, (unsigned char *)param_1 + 0x1f, 4);
    if (uVar3 == 0x59) {
        if (tmp != 0) {
            uVar3 = (unsigned int)*(unsigned char *)(tmp + 0x87);
            goto LAB_004728fa;
        }
    } else {
LAB_004728fa:
        __builtin_memcpy(&tmp, (unsigned char *)param_1 + 0x1f, 4);
        if (tmp != 0) {
            _DAT_005233a0 = (unsigned int)*(unsigned char *)(tmp + 0x8a);
            goto LAB_0047291b;
        }
    }
    _DAT_005233a0 = 0;
LAB_0047291b:
    piVar2 = FUN_004ac720((unsigned int)DAT_009a7ed0[uVar3]);
    if ((int *)*piVar2 != piVar2) {
        FUN_00471ff0(piVar2);
    }
    piVar5 = local_d0;
    for (iVar4 = 0x34; iVar4 != 0; iVar4 = (int)((unsigned int)iVar4 - 1U)) {
        __builtin_memset(piVar5, 0, 4);
        piVar5 = piVar5 + 1;
    }
    __builtin_memcpy(&local_d0[0], &param_1, 4);
    __builtin_memcpy(&a, DAT_009e9a5b + (unsigned int)DAT_009ecc34 * 0x5bdU, 4);
    __builtin_memcpy(&b, DAT_009e9a5f + (unsigned int)DAT_009ecc34 * 0x5bdU, 4);
    local_c8 = (float)(int)((unsigned int)*(int *)(param_1 + 4) - (unsigned int)a);
    local_c4 = (float)(int)((unsigned int)*(int *)(param_1 + 8) - (unsigned int)b);
    local_bc = (float)DAT_00a004a8 * _DAT_005181d8;
    local_b8 = (float)DAT_00a004a4 * _DAT_005181d8;
    local_b4 = (float)DAT_00a004b0 * _DAT_005181d8;
    local_ac = (float)DAT_00a004b8 * _DAT_005181d8;
    local_b0 = (float)DAT_00a004cc * _DAT_005181d8;
    local_a4 = (float)DAT_00a00488 * _DAT_005181d8;
    local_a8 = (float)DAT_009fe7f4 * _DAT_005181d8;
    local_9c = (float)DAT_00a00490 * _DAT_005181d8;
    local_a0 = (float)DAT_00a00480 * _DAT_005181d8;
    __builtin_memcpy(&c, DAT_009e9a63 + (unsigned int)DAT_009ecc34 * 0x5bdU, 4);
    local_c0 = (float)(int)((unsigned int)*(int *)(param_1 + 0xc) - (unsigned int)c);
    FUN_00420fe0(&local_c8, &local_bc);
    local_b8 = (float)DAT_009d77b8 * _DAT_005181d8;
    local_b4 = (float)DAT_009d77b4 * _DAT_005181d8;
    local_b0 = (float)DAT_009d77b0 * _DAT_005181d8;
    local_bc = (float)DAT_009d77ac * _DAT_005181d8;
    local_ac = (float)DAT_009d7690 * _DAT_005181d8;
    local_a8 = (float)DAT_009d768c * _DAT_005181d8;
    local_a4 = (float)_DAT_009d7688 * _DAT_005181d8;
    local_a0 = (float)_DAT_009d77a4 * _DAT_005181d8;
    local_9c = (float)_DAT_009d77a0 * _DAT_005181d8;
    local_98 = -local_c8;
    local_94 = -local_c4;
    local_90 = -local_c0;
    local_8c = local_bc;
    local_88 = local_b0;
    local_84 = local_a4;
    local_80 = local_b8;
    local_7c = local_ac;
    local_78 = local_a0;
    local_74 = local_b4;
    local_70 = local_a8;
    local_6c = local_9c;
    FUN_00420fe0(&local_98, &local_8c);
    DAT_0060f1c0 = 0;
    if (DAT_005271d8 == (unsigned int *)0) {
        DAT_005271d8 = &DAT_0060f210;
        _DAT_0060f224 = 8;
        DAT_0060f220 = 8;
        DAT_0060f214 = 0x10;
        DAT_0060f210 = 0x100;
        FUN_004720d0(DAT_0060f228, DAT_005271e0, 8, 8);
    }
    iVar4 = 0;
    DAT_0060f1ec = DAT_005271d8;
    DAT_0060f1e8 = 0;
    _DAT_0060f208 = 0;
    _DAT_0060f1f0 = 0;
    DAT_0060f1d8 = 0;
    DAT_0060f204 = 0;
    __builtin_memcpy(&cnt, (unsigned char *)piVar2 + 6, 4);
    if (0 < cnt) {
        do {
            __builtin_memcpy(&arr, (unsigned char *)piVar2 + 10, 4);
            puVar1 = *(unsigned int **)((unsigned int)arr + (unsigned int)iVar4 * 4U);
            if (puVar1[1] == 0x14) {
                DAT_005271d4 = (int)((unsigned int)DAT_005271d4 + 1U);
                param_2 = (int)((unsigned int)param_2 + 1U);
                __builtin_memcpy(&arr, (unsigned char *)piVar2 + 10, 4);
                FUN_00472c90((int)piVar2, *(unsigned int **)((unsigned int)arr + (unsigned int)iVar4 * 4U), local_d0);
            } else if (iVar4 == param_2) {
                DAT_005271d4 = (int)((unsigned int)DAT_005271d4 + 1U);
                FUN_00472c90((int)piVar2, puVar1, local_d0);
            }
            iVar4 = (int)((unsigned int)iVar4 + 1U);
            __builtin_memcpy(&cnt, (unsigned char *)piVar2 + 6, 4);
        } while (iVar4 < cnt);
    }
    FUN_004ac740();
    return;
}

#undef local_c8
#undef local_c4
#undef local_c0
#undef local_bc
#undef local_b8
#undef local_b4
#undef local_b0
#undef local_ac
#undef local_a8
#undef local_a4
#undef local_a0
#undef local_9c
#undef local_98
#undef local_94
#undef local_90
#undef local_8c
#undef local_88
#undef local_84
#undef local_80
#undef local_7c
#undef local_78
#undef local_74
#undef local_70
#undef local_6c
