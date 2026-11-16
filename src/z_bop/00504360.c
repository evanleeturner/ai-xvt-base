/* SPDX-License-Identifier: MIT */
/* z_bop 00504360 */
/* Callee prototypes: the evidence gives no signatures, so the decompiler's
 * argument lists are used. The calling convention of each callee is not
 * confirmed; __cdecl is assumed. */
extern void FUN_004daba0(int *param_1);
extern void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern char *FUN_004ddc10(int param_1);
extern void FUN_004daa20(int *param_1, char *param_2, char *param_3, int param_4);
extern int FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4,
                        int param_5, unsigned int param_6, int param_7, char *param_8);
extern void FUN_004d64a0(int *param_1, int param_2, int param_3);

extern int DAT_00b69e30;
/* 0xb6a5aa and 0xb6a606 are not 4-byte aligned (address % 4 == 2): packed globals */
extern unsigned char DAT_00b6a5aa[4];
extern unsigned char DAT_00b6a606[4];
extern int DAT_0066d9e0;
extern int DAT_0066d918;

extern char s_deb4d_0052d590[];
extern char s_debrief4d_0052d584[];
extern char s_deb4u_0052d57c[];
extern char s_debrief4u_0052d570[];
extern char s_debrief2d_0052d564[];
extern char s_debrief2u_0052d558[];
extern char s_debrief1d_0052d54c[];
extern char s_debrief1u_0052d540[];
extern char s_jewelsound_0052ad74[];

/* Each use performs a fresh unaligned 4-byte read of the global. */
#define RD_DAT_00b6a5aa() (__builtin_memcpy(&iTmp, DAT_00b6a5aa, 4), iTmp)
#define RD_DAT_00b6a606() (__builtin_memcpy(&iTmp, DAT_00b6a606, 4), iTmp)

unsigned int __stdcall FUN_00504360(void)
{
    char *pcVar1;
    unsigned int uVar2;
    int iVar3;
    int iVar4;
    char *pcVar5;
    int iTmp;
    int local_30[4];
    int local_20[8];

    if (((DAT_00b69e30 == 2) && (RD_DAT_00b6a5aa() != 1)) && (RD_DAT_00b6a5aa() != 2)) {
        local_20[0] = 0;
    }
    else {
        local_20[0] = 1;
    }
    local_20[1] = 1;
    if ((RD_DAT_00b6a606() == 1) &&
        (((RD_DAT_00b6a5aa() == 0 || (RD_DAT_00b6a5aa() == 1)) || (RD_DAT_00b6a5aa() == 3))))
    {
        local_20[2] = 1;
    }
    else {
        local_20[2] = 0;
    }
    local_20[3] = 0;
    local_20[4] = 0;
    local_20[5] = 0;
    local_20[6] = 0;
    local_20[7] = 0;
    local_20[DAT_0066d9e0] = 2;
    FUN_004daba0(local_20);
    FUN_004d6460(local_30, 0x16, 0xaa, 0x2a, 0xc2);
    if (DAT_0066d9e0 == 2) {
        if (RD_DAT_00b6a5aa() == 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x336);
            FUN_004daa20(local_30, s_deb4d_0052d590, pcVar1, iVar3);
        }
        else if (RD_DAT_00b6a5aa() == 1) {
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x13b);
            FUN_004daa20(local_30, s_debrief4d_0052d584, pcVar1, iVar3);
        }
        else if (RD_DAT_00b6a5aa() == 3) {
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x13a);
            FUN_004daa20(local_30, s_debrief4d_0052d584, pcVar1, iVar3);
        }
    }
    else if (RD_DAT_00b6a606() == 1) {
        if (RD_DAT_00b6a5aa() == 0) {
            pcVar5 = s_jewelsound_0052ad74;
            iVar4 = 0xe;
            uVar2 = 0;
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x336);
            iVar3 = FUN_004da780(local_30, s_deb4u_0052d57c, s_deb4u_0052d57c, pcVar1, iVar3,
                                 uVar2, iVar4, pcVar5);
        }
        else if (RD_DAT_00b6a5aa() == 1) {
            pcVar5 = s_jewelsound_0052ad74;
            iVar4 = 0xe;
            uVar2 = 0;
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x13b);
            iVar3 = FUN_004da780(local_30, s_debrief4u_0052d570, s_debrief4u_0052d570, pcVar1,
                                 iVar3, uVar2, iVar4, pcVar5);
        }
        else {
            if (RD_DAT_00b6a5aa() != 3) goto LAB_0050459f;
            pcVar5 = s_jewelsound_0052ad74;
            iVar4 = 0xe;
            uVar2 = 0;
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x13a);
            iVar3 = FUN_004da780(local_30, s_debrief4u_0052d570, s_debrief4u_0052d570, pcVar1,
                                 iVar3, uVar2, iVar4, pcVar5);
        }
        if (iVar3 != 0) {
            DAT_0066d918 = 1;
            DAT_0066d9e0 = 2;
        }
    }
LAB_0050459f:
    FUN_004d64a0(local_30, 0, -0x1c);
    if (DAT_0066d9e0 == 1) {
        iVar3 = 0xc;
        pcVar1 = FUN_004ddc10(0x138);
        FUN_004daa20(local_30, s_debrief2d_0052d564, pcVar1, iVar3);
    }
    else {
        pcVar5 = s_jewelsound_0052ad74;
        iVar4 = 0xc;
        uVar2 = 0;
        iVar3 = 0xc;
        pcVar1 = FUN_004ddc10(0x138);
        iVar3 = FUN_004da780(local_30, s_debrief2u_0052d558, s_debrief2u_0052d558, pcVar1, iVar3,
                             uVar2, iVar4, pcVar5);
        if (iVar3 != 0) {
            DAT_0066d918 = 1;
            DAT_0066d9e0 = 1;
        }
    }
    FUN_004d64a0(local_30, 0, -0x1c);
    if (local_20[0] != 0) {
        if (DAT_0066d9e0 == 0) {
            iVar3 = 0xc;
            pcVar1 = FUN_004ddc10(0x137);
            FUN_004daa20(local_30, s_debrief1d_0052d54c, pcVar1, iVar3);
            return 1;
        }
        pcVar5 = s_jewelsound_0052ad74;
        iVar4 = 0xb;
        uVar2 = 0;
        iVar3 = 0xc;
        pcVar1 = FUN_004ddc10(0x137);
        iVar3 = FUN_004da780(local_30, s_debrief1u_0052d540, s_debrief1u_0052d540, pcVar1, iVar3,
                             uVar2, iVar4, pcVar5);
        if (iVar3 != 0) {
            DAT_0066d918 = 1;
            DAT_0066d9e0 = 0;
        }
    }
    return 1;
}
