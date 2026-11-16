/* SPDX-License-Identifier: MIT */
/* z_xvt 004c8930 */
/* Unresolved: callee calling conventions and argument types are taken from the
   Ghidra signatures; the real conventions are unconfirmed. */
void FUN_004d9ea0(int *a);
void FUN_004dcfd0(int *a, int b, int c, int d, int e);
char *FUN_004dcfa0(int a);
int FUN_004d9a80(int *a, char *b, char *c, char *d, int e, unsigned int f, int g, char *h);
void FUN_004d5420(void);
void FUN_004d52b0(void);
void FUN_004d1750(char *a, int b, int c);
void FUN_004d0e60(char *a, int b, int c);
void FUN_004d0580(char *a, int b, int c);
void FUN_004d5350(int a);
void FUN_004d9d20(int *a, char *b, char *c, int d);
void FUN_004dd030(int *a, int b, int c);

extern char s_reg7d_005322d4[];
extern char s_reg7u_005322dc[];
extern char s_reg6d_005322c4[];
extern char s_reg6u_005322cc[];
extern char s_reg4d_005322bc[];
extern char s_reg4u_005322b4[];
extern char s_reg3d_005322ac[];
extern char s_reg3u_005322a4[];
extern char s_reg2d_0053229c[];
extern char s_reg2u_00532294[];
extern char s_reg1d_0053228c[];
extern char s_reg1u_00532284[];
extern char s_jewelsound_00531990[];
extern char s_background0_00531e7c[];
extern char s_background1_00531e70[];
extern char s_frame_00531358[];
extern char s_clientactive_00531eac[];
extern char s_allactive_00531ebc[];
extern char s_nameoverlay_00531ea0[];
extern char s_regoverlay_00531e94[];
extern char s_awardoverlay_005320b4[];
extern char s_rebplaque_005320a8[];
extern char s_rebmedal_0053209c[];
extern char s_rebtrophy_00532090[];
extern char s_rebcitation_00532084[];
extern char s_impplaque_00532078[];
extern char s_impmedal_0053206c[];
extern char s_imptrophy_00532060[];
extern char s_impcitation_00532054[];
extern char s_promotionoverlay_00532040[];

extern int DAT_0064ef9c;
extern int DAT_0081bc68;
extern int DAT_0081bcc0;
/* unaligned globals: accessed only via __builtin_memcpy */
extern int DAT_0081fb96;
extern int DAT_0081bfb6;
extern int DAT_0081bfba;
extern int DAT_0082e3c2;
extern int DAT_0082e3c6;
extern int DAT_0081fb9e;
extern int DAT_0081fba2;
extern int DAT_0081c012;
extern int DAT_0081c016;
extern int DAT_0081fbda;
extern int DAT_0081fbde;
extern int DAT_0081bfbe;
extern int DAT_0082e3ca;
extern int DAT_0081fba6;

unsigned int __stdcall FUN_004c8930(void)
{
    char *pcVar1;
    char *puVar2;
    char *puVar3;
    int iVar4;
    unsigned int uVar5;
    int iVar6;
    char *pcVar7;
    int local_2c[4];
    int local_1c[7];
    int t96;
    int tmp;

    local_1c[4] = 0;
    local_1c[0] = 1;
    local_1c[1] = 1;
    local_1c[2] = 1;
    local_1c[3] = 1;
    local_1c[5] = 1;
    local_1c[6] = 1;
    local_1c[DAT_0081bcc0] = 2;
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    local_1c[t96 + 5] = 2;
    FUN_004d9ea0(local_1c);
    FUN_004dcfd0(local_2c, 0x16, 0x14e, 0x2a, 0x166);
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    if (t96 == 1) {
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(0x275);
        FUN_004d9d20(local_2c, s_reg7d_005322d4, pcVar1, iVar4);
        goto LAB_004c8bb3;
    }
    pcVar7 = s_jewelsound_00531990;
    iVar6 = 0x11;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004dcfa0(0x275);
    iVar4 = FUN_004d9a80(local_2c, s_reg7u_005322dc, s_reg7u_005322dc, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 == 0) goto LAB_004c8bb3;
    DAT_0064ef9c = 1;
    tmp = 1;
    __builtin_memcpy(&DAT_0081fb96, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_0082e3c2, 4);
    __builtin_memcpy(&DAT_0081bfb6, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_0082e3c6, 4);
    __builtin_memcpy(&DAT_0081bfba, &tmp, 4);
    puVar2 = (char *)&DAT_0082e3ca;
    puVar3 = (char *)&DAT_0081bfbe;
    for (iVar4 = 5; iVar4 != 0; iVar4 = iVar4 + -1) {
        __builtin_memcpy(&tmp, puVar2, 4);
        __builtin_memcpy(puVar3, &tmp, 4);
        puVar2 = puVar2 + 4;
        puVar3 = puVar3 + 4;
    }
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    __builtin_memcpy(&tmp, (char *)&DAT_0081fbda + (unsigned int)((unsigned int)t96 * 0x3a09u) * 4u, 4);
    __builtin_memcpy(&DAT_0081c012, &tmp, 4);
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    __builtin_memcpy(&tmp, (char *)&DAT_0081fbde + (unsigned int)((unsigned int)t96 * 0x3a09u) * 4u, 4);
    __builtin_memcpy(&DAT_0081c016, &tmp, 4);
    FUN_004d5420();
    FUN_004d52b0();
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    if (t96 == 0) {
        pcVar1 = s_background0_00531e7c;
    } else {
        pcVar1 = s_background1_00531e70;
    }
    FUN_004d1750(pcVar1, 0, 0);
    FUN_004d0e60(s_frame_00531358, 0, 0);
    if (DAT_0081bc68 == 0) {
        pcVar1 = s_clientactive_00531eac;
    } else {
        pcVar1 = s_allactive_00531ebc;
    }
    FUN_004d0e60(pcVar1, 0, 0);
    FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
    switch (DAT_0081bcc0) {
    case 0:
    case 3:
        pcVar1 = s_regoverlay_00531e94;
        goto LAB_004c8b7b;
    case 1:
        FUN_004d0580(s_awardoverlay_005320b4, 0, 0);
        __builtin_memcpy(&t96, &DAT_0081fb96, 4);
        if (t96 == 0) {
            FUN_004d0e60(s_rebplaque_005320a8, 0, 0);
            FUN_004d0e60(s_rebmedal_0053209c, 0, 0);
            FUN_004d0e60(s_rebtrophy_00532090, 0, 0);
            FUN_004d0e60(s_rebcitation_00532084, 0, 0);
        } else {
            FUN_004d0e60(s_impplaque_00532078, 0, 0);
            FUN_004d0e60(s_impmedal_0053206c, 0, 0);
            FUN_004d0e60(s_imptrophy_00532060, 0, 0);
            FUN_004d0e60(s_impcitation_00532054, 0, 0);
        }
        break;
    case 2:
        pcVar1 = s_promotionoverlay_00532040;
LAB_004c8b7b:
        FUN_004d0580(pcVar1, 0, 0);
    }
    FUN_004d5350(1);
LAB_004c8bb3:
    FUN_004dd030(local_2c, 0, -0x1c);
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    if (t96 == 0) {
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(0x274);
        FUN_004d9d20(local_2c, s_reg6d_005322c4, pcVar1, iVar4);
        goto LAB_004c8de4;
    }
    pcVar7 = s_jewelsound_00531990;
    iVar6 = 0x10;
    uVar5 = 0;
    iVar4 = 0xc;
    pcVar1 = FUN_004dcfa0(0x274);
    iVar4 = FUN_004d9a80(local_2c, s_reg6u_005322cc, s_reg6u_005322cc, pcVar1, iVar4, uVar5, iVar6, pcVar7);
    if (iVar4 == 0) goto LAB_004c8de4;
    DAT_0064ef9c = 1;
    tmp = 0;
    __builtin_memcpy(&DAT_0081fb96, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_0081fb9e, 4);
    __builtin_memcpy(&DAT_0081bfb6, &tmp, 4);
    __builtin_memcpy(&tmp, &DAT_0081fba2, 4);
    __builtin_memcpy(&DAT_0081bfba, &tmp, 4);
    puVar2 = (char *)&DAT_0081fba6;
    puVar3 = (char *)&DAT_0081bfbe;
    for (iVar4 = 5; iVar4 != 0; iVar4 = iVar4 + -1) {
        __builtin_memcpy(&tmp, puVar2, 4);
        __builtin_memcpy(puVar3, &tmp, 4);
        puVar2 = puVar2 + 4;
        puVar3 = puVar3 + 4;
    }
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    __builtin_memcpy(&tmp, (char *)&DAT_0081fbda + (unsigned int)((unsigned int)t96 * 0x3a09u) * 4u, 4);
    __builtin_memcpy(&DAT_0081c012, &tmp, 4);
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    __builtin_memcpy(&tmp, (char *)&DAT_0081fbde + (unsigned int)((unsigned int)t96 * 0x3a09u) * 4u, 4);
    __builtin_memcpy(&DAT_0081c016, &tmp, 4);
    FUN_004d5420();
    FUN_004d52b0();
    __builtin_memcpy(&t96, &DAT_0081fb96, 4);
    if (t96 == 0) {
        pcVar1 = s_background0_00531e7c;
    } else {
        pcVar1 = s_background1_00531e70;
    }
    FUN_004d1750(pcVar1, 0, 0);
    FUN_004d0e60(s_frame_00531358, 0, 0);
    if (DAT_0081bc68 == 0) {
        pcVar1 = s_clientactive_00531eac;
    } else {
        pcVar1 = s_allactive_00531ebc;
    }
    FUN_004d0e60(pcVar1, 0, 0);
    FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
    switch (DAT_0081bcc0) {
    case 0:
    case 3:
        pcVar1 = s_regoverlay_00531e94;
        goto LAB_004c8dac;
    case 1:
        FUN_004d0580(s_awardoverlay_005320b4, 0, 0);
        __builtin_memcpy(&t96, &DAT_0081fb96, 4);
        if (t96 == 0) {
            FUN_004d0e60(s_rebplaque_005320a8, 0, 0);
            FUN_004d0e60(s_rebmedal_0053209c, 0, 0);
            FUN_004d0e60(s_rebtrophy_00532090, 0, 0);
            FUN_004d0e60(s_rebcitation_00532084, 0, 0);
        } else {
            FUN_004d0e60(s_impplaque_00532078, 0, 0);
            FUN_004d0e60(s_impmedal_0053206c, 0, 0);
            FUN_004d0e60(s_imptrophy_00532060, 0, 0);
            FUN_004d0e60(s_impcitation_00532054, 0, 0);
        }
        break;
    case 2:
        pcVar1 = s_promotionoverlay_00532040;
LAB_004c8dac:
        FUN_004d0580(pcVar1, 0, 0);
    }
    FUN_004d5350(1);
LAB_004c8de4:
    FUN_004dcfd0(local_2c, 0x16, 0xc6, 0x2a, 0xde);
    if (DAT_0081bcc0 == 3) {
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(10);
        FUN_004d9d20(local_2c, s_reg4d_005322bc, pcVar1, iVar4);
    } else {
        pcVar7 = s_jewelsound_00531990;
        iVar6 = 0xe;
        uVar5 = 0;
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(10);
        iVar4 = FUN_004d9a80(local_2c, s_reg4u_005322b4, s_reg4u_005322b4, pcVar1, iVar4, uVar5, iVar6, pcVar7);
        if (iVar4 != 0) {
            DAT_0064ef9c = 1;
            DAT_0081bcc0 = 3;
            FUN_004d5420();
            FUN_004d52b0();
            __builtin_memcpy(&t96, &DAT_0081fb96, 4);
            if (t96 == 0) {
                pcVar1 = s_background0_00531e7c;
            } else {
                pcVar1 = s_background1_00531e70;
            }
            FUN_004d1750(pcVar1, 0, 0);
            FUN_004d0e60(s_frame_00531358, 0, 0);
            if (DAT_0081bc68 == 0) {
                pcVar1 = s_clientactive_00531eac;
            } else {
                pcVar1 = s_allactive_00531ebc;
            }
            FUN_004d0e60(pcVar1, 0, 0);
            FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
            FUN_004d0580(s_regoverlay_00531e94, 0, 0);
            FUN_004d5350(1);
        }
    }
    FUN_004dd030(local_2c, 0, -0x1c);
    if (DAT_0081bcc0 == 2) {
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(8);
        FUN_004d9d20(local_2c, s_reg3d_005322ac, pcVar1, iVar4);
    } else {
        pcVar7 = s_jewelsound_00531990;
        iVar6 = 0xd;
        uVar5 = 0;
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(8);
        iVar4 = FUN_004d9a80(local_2c, s_reg3u_005322a4, s_reg3u_005322a4, pcVar1, iVar4, uVar5, iVar6, pcVar7);
        if (iVar4 != 0) {
            DAT_0064ef9c = 1;
            DAT_0081bcc0 = 2;
            FUN_004d5420();
            FUN_004d52b0();
            __builtin_memcpy(&t96, &DAT_0081fb96, 4);
            if (t96 == 0) {
                pcVar1 = s_background0_00531e7c;
            } else {
                pcVar1 = s_background1_00531e70;
            }
            FUN_004d1750(pcVar1, 0, 0);
            FUN_004d0e60(s_frame_00531358, 0, 0);
            if (DAT_0081bc68 == 0) {
                pcVar1 = s_clientactive_00531eac;
            } else {
                pcVar1 = s_allactive_00531ebc;
            }
            FUN_004d0e60(pcVar1, 0, 0);
            FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
            FUN_004d0580(s_promotionoverlay_00532040, 0, 0);
            FUN_004d5350(1);
        }
    }
    FUN_004dd030(local_2c, 0, -0x1c);
    if (DAT_0081bcc0 == 1) {
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(9);
        FUN_004d9d20(local_2c, s_reg2d_0053229c, pcVar1, iVar4);
    } else {
        pcVar7 = s_jewelsound_00531990;
        iVar6 = 0xc;
        uVar5 = 0;
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(9);
        iVar4 = FUN_004d9a80(local_2c, s_reg2u_00532294, s_reg2u_00532294, pcVar1, iVar4, uVar5, iVar6, pcVar7);
        if (iVar4 != 0) {
            DAT_0064ef9c = 1;
            DAT_0081bcc0 = 1;
            FUN_004d5420();
            FUN_004d52b0();
            __builtin_memcpy(&t96, &DAT_0081fb96, 4);
            if (t96 == 0) {
                pcVar1 = s_background0_00531e7c;
            } else {
                pcVar1 = s_background1_00531e70;
            }
            FUN_004d1750(pcVar1, 0, 0);
            FUN_004d0e60(s_frame_00531358, 0, 0);
            if (DAT_0081bc68 == 0) {
                pcVar1 = s_clientactive_00531eac;
            } else {
                pcVar1 = s_allactive_00531ebc;
            }
            FUN_004d0e60(pcVar1, 0, 0);
            FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
            FUN_004d0580(s_awardoverlay_005320b4, 0, 0);
            __builtin_memcpy(&t96, &DAT_0081fb96, 4);
            if (t96 == 0) {
                FUN_004d0e60(s_rebplaque_005320a8, 0, 0);
                FUN_004d0e60(s_rebmedal_0053209c, 0, 0);
                FUN_004d0e60(s_rebtrophy_00532090, 0, 0);
                pcVar1 = s_rebcitation_00532084;
            } else {
                FUN_004d0e60(s_impplaque_00532078, 0, 0);
                FUN_004d0e60(s_impmedal_0053206c, 0, 0);
                FUN_004d0e60(s_imptrophy_00532060, 0, 0);
                pcVar1 = s_impcitation_00532054;
            }
            FUN_004d0e60(pcVar1, 0, 0);
            FUN_004d5350(1);
        }
    }
    FUN_004dd030(local_2c, 0, -0x1c);
    if (DAT_0081bcc0 != 0) {
        pcVar7 = s_jewelsound_00531990;
        iVar6 = 0xb;
        uVar5 = 0;
        iVar4 = 0xc;
        pcVar1 = FUN_004dcfa0(7);
        iVar4 = FUN_004d9a80(local_2c, s_reg1u_00532284, s_reg1u_00532284, pcVar1, iVar4, uVar5, iVar6, pcVar7);
        if (iVar4 != 0) {
            DAT_0064ef9c = 1;
            DAT_0081bcc0 = 0;
            FUN_004d5420();
            FUN_004d52b0();
            __builtin_memcpy(&t96, &DAT_0081fb96, 4);
            if (t96 == 0) {
                pcVar1 = s_background0_00531e7c;
            } else {
                pcVar1 = s_background1_00531e70;
            }
            FUN_004d1750(pcVar1, 0, 0);
            FUN_004d0e60(s_frame_00531358, 0, 0);
            if (DAT_0081bc68 == 0) {
                pcVar1 = s_clientactive_00531eac;
            } else {
                pcVar1 = s_allactive_00531ebc;
            }
            FUN_004d0e60(pcVar1, 0, 0);
            FUN_004d0580(s_nameoverlay_00531ea0, 0, 0);
            FUN_004d0580(s_regoverlay_00531e94, 0, 0);
            FUN_004d5350(1);
        }
        return 1;
    }
    iVar4 = 0xc;
    pcVar1 = FUN_004dcfa0(7);
    FUN_004d9d20(local_2c, s_reg1d_0053228c, pcVar1, iVar4);
    return 1;
}
