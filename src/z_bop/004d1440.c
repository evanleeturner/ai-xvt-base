/* SPDX-License-Identifier: MIT */
/* z_bop 004d1440 */
extern unsigned int DAT_005182a0, DAT_005182a4, DAT_005182a8, DAT_005182ac;
extern unsigned int DAT_005182b0, DAT_005182b4, DAT_005182b8, DAT_005182bc;
extern unsigned int DAT_005182c0, DAT_005182c4, DAT_005182c8, DAT_005182cc;
extern unsigned int DAT_005182d0, DAT_005182d4, DAT_005182d8, DAT_005182dc;
extern unsigned int DAT_00518f00, DAT_00518f04, DAT_00518f08, DAT_00518f0c;
extern unsigned int DAT_00518f10, DAT_00518f14, DAT_00518f18, DAT_00518f1c;
extern unsigned int DAT_00518f20, DAT_00518f24, DAT_00518f28, DAT_00518f2c;
extern unsigned int DAT_00518290, DAT_00518294, DAT_00518298, DAT_0051829c;
extern unsigned char DAT_00acebc9[4];   /* packed pointer (odd address) */
extern unsigned char DAT_00acebc5[4];
extern char s_modem_game_0052b9e4[12];

int Ordinal_4(int param_1, void *param_2, int param_3, int param_4, int param_5);
int FUN_004d1840(int *param_1, unsigned int *param_2, char *param_3, char *param_4);

unsigned char FUN_004d1440(unsigned int param_1, int param_2, unsigned int param_3,
                           unsigned int param_4, unsigned int param_5, unsigned int param_6,
                           unsigned int param_7, unsigned int param_8,
                           unsigned int in_stack_00000024, char *in_stack_00000028)
{
    int iVar1;
    unsigned int *puVar2;
    char *pcVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    int *piVar7;
    unsigned int auStack_104[4];
    unsigned int auStack_dc[10];
    unsigned char acStack_b4[16];
    unsigned int auStack_a4[20];
    unsigned int auStack_40[16];
    unsigned int zero_ptr;

#define local_104 auStack_104[0]
#define local_100 auStack_104[1]
#define local_fc  auStack_104[2]
#define local_f8  auStack_104[3]
#define local_dc  ((char *)auStack_dc)
#define local_d4  auStack_dc[2]
#define local_d0  auStack_dc[3]
#define local_cc  auStack_dc[4]
#define uStack_c8 auStack_dc[5]
#define uStack_c4 auStack_dc[6]
#define uStack_b8 auStack_dc[9]
#define local_94  auStack_a4[4]
#define local_90  auStack_a4[5]
#define uStack_8c auStack_a4[6]
#define uStack_88 auStack_a4[7]
#define uStack_84 auStack_a4[8]
#define uStack_80 auStack_a4[9]
#define uStack_7c auStack_a4[10]
#define uStack_78 auStack_a4[11]
#define uStack_74 auStack_a4[12]
#define uStack_70 auStack_a4[13]
#define uStack_6c auStack_a4[14]
#define uStack_68 auStack_a4[15]
#define uStack_64 auStack_a4[16]
#define uStack_60 auStack_a4[17]
#define uStack_5c auStack_a4[18]
#define uStack_58 auStack_a4[19]
#define uStack_10 auStack_40[12]
#define uStack_c  auStack_40[13]
#define uStack_8  auStack_40[14]
#define uStack_4  auStack_40[15]

    auStack_40[0] = *(unsigned int *)s_modem_game_0052b9e4;
    auStack_40[1] = *(unsigned int *)(s_modem_game_0052b9e4 + 4);
    *(unsigned short *)((char *)auStack_40 + 8) = *(unsigned short *)(s_modem_game_0052b9e4 + 8);
    ((char *)auStack_40)[10] = s_modem_game_0052b9e4[10];
    pcVar3 = (char *)auStack_40 + 11;
    for (iVar1 = 0x35; iVar1 != 0; iVar1 = iVar1 + -1) {
        *pcVar3 = 0;
        pcVar3 = pcVar3 + 1;
    }

    switch (in_stack_00000024) {
    case 0:
        local_104 = DAT_005182a0;
        local_100 = DAT_005182a4;
        local_fc = DAT_005182a8;
        local_f8 = DAT_005182ac;
        local_90 = (unsigned int)auStack_104;
        break;
    case 1:
        local_104 = DAT_005182c0;
        local_100 = DAT_005182c4;
        local_fc = DAT_005182c8;
        local_f8 = DAT_005182cc;
        auStack_a4[1] = DAT_00518f20;
        auStack_a4[2] = DAT_00518f24;
        auStack_a4[3] = DAT_00518f28;
        local_94 = DAT_00518f2c;
        local_90 = (unsigned int)auStack_104;
        break;
    case 2:
        local_104 = DAT_005182b0;
        local_100 = DAT_005182b4;
        local_fc = DAT_005182b8;
        local_f8 = DAT_005182bc;
        auStack_a4[1] = DAT_00518f10;
        auStack_a4[2] = DAT_00518f14;
        auStack_a4[3] = DAT_00518f18;
        local_94 = DAT_00518f1c;
        local_90 = (unsigned int)auStack_104;
        break;
    case 3:
        local_104 = DAT_005182d0;
        local_100 = DAT_005182d4;
        local_fc = DAT_005182d8;
        local_f8 = DAT_005182dc;
        local_dc[0] = '\x02';
        local_dc[1] = '\0';
        local_dc[2] = '\0';
        local_dc[3] = '\0';
        local_dc[4] = -0x80;
        local_dc[5] = '%';
        local_dc[6] = '\0';
        local_dc[7] = '\0';
        auStack_a4[1] = DAT_00518f00;
        local_cc = 1;
        auStack_a4[2] = DAT_00518f04;
        auStack_a4[3] = DAT_00518f08;
        in_stack_00000028 = local_dc;
        local_94 = DAT_00518f0c;
        local_d4 = 0;
        local_d0 = 0;
        local_90 = (unsigned int)auStack_104;
    }
    uVar6 = 0;
    uVar5 = 0;
    uVar4 = 0;
    zero_ptr = 0;
    iVar1 = Ordinal_4(0, DAT_00acebc9, 0, 0, 0);
    if (iVar1 != 0) {
        return 0;
    }
    __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
    iVar1 = FUN_004d1840(piVar7, (unsigned int *)local_90, (char *)acStack_b4, in_stack_00000028);
    if (iVar1 != 0) {
        __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
        ((int (__stdcall *)(int *))(*(int **)piVar7)[2])(piVar7);
        __builtin_memcpy(DAT_00acebc9, &zero_ptr, 4);
        return 0;
    }
    puVar2 = auStack_a4;
    for (iVar1 = 0x14; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    auStack_a4[0] = 0x50;
    auStack_a4[1] = 0;
    auStack_a4[2] = DAT_00518290;
    auStack_a4[3] = DAT_00518294;
    local_94 = DAT_00518298;
    local_90 = DAT_0051829c;
    uStack_8c = uStack_10;
    uStack_88 = uStack_c;
    uStack_84 = uStack_8;
    uStack_80 = uStack_4;
    uStack_7c = 0x10;
    uStack_78 = 0;
    uStack_74 = param_3;
    uStack_70 = 0;
    uStack_6c = 0;
    uStack_68 = 0;
    uStack_64 = 0;
    uStack_60 = 0;
    uStack_5c = 0;
    uStack_58 = 0;
    local_100 = 0x10;
    local_fc = 0;
    local_f8 = param_1;
    pcVar3 = local_dc;
    for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
        pcVar3[0] = '\0';
        pcVar3[1] = '\0';
        pcVar3[2] = '\0';
        pcVar3[3] = '\0';
        pcVar3 = pcVar3 + 4;
    }
    local_dc[0] = '(';
    local_dc[1] = '\0';
    local_dc[2] = '\0';
    local_dc[3] = '\0';
    local_dc[4] = '\x02';
    local_dc[5] = '\0';
    local_dc[6] = '\0';
    local_dc[7] = '\0';
    if (param_2 == 0) {
        local_dc[4] = '\x01';
        local_dc[5] = '\0';
        local_dc[6] = '\0';
        local_dc[7] = '\0';
    }
    local_d4 = (unsigned int)auStack_a4;
    local_d0 = (unsigned int)&local_100;
    uStack_b8 = local_104;
    local_cc = uVar4;
    uStack_c8 = uVar5;
    uStack_c4 = uVar6;
    __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
    iVar1 = ((int (__stdcall *)(int *, int, int, char *))(*(int **)piVar7)[12])(piVar7, 0, 0, local_dc);
    if (iVar1 == 0) {
        __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
        iVar1 = ((int (__stdcall *)(int *, int, void *, int))(*(int **)piVar7)[3])(piVar7, 0, DAT_00acebc5, 0);
        __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
        ((int (__stdcall *)(int *))(*(int **)piVar7)[2])(piVar7);
        __builtin_memcpy(DAT_00acebc9, &zero_ptr, 4);
        return iVar1 == 0;
    }
    __builtin_memcpy(&piVar7, DAT_00acebc9, 4);
    ((int (__stdcall *)(int *))(*(int **)piVar7)[2])(piVar7);
    __builtin_memcpy(DAT_00acebc9, &zero_ptr, 4);
    return 0;
}
