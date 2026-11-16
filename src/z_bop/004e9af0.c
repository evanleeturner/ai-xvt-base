/* SPDX-License-Identifier: MIT */
/* z_bop 004e9af0 */
/* Prototypes (calling conventions and exact parameter types are not confirmed by the evidence;
   __cdecl is assumed for callees, matching Ghidra's output) */
void FUN_004de090(int *param_1, int *param_2);
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
int FUN_004d6b90(int *param_1, int param_2, int param_3);
unsigned char FUN_004dc1e0(void);
unsigned char FUN_004dc200(void);
unsigned char FUN_004dc220(void);
unsigned char FUN_004dc240(void);
void FUN_004d64a0(int *param_1, int param_2, int param_3);
void FUN_004daba0(int *param_1);
char *FUN_004ddc10(int param_1);
int FUN_004da780(int *param_1, char *param_2, char *param_3, char *param_4, int param_5,
                 int param_6, int param_7, char *param_8);
void FUN_0042b010(void);
void FUN_00426a00(unsigned int *param_1);
void FUN_00429e70(char *param_1);
void FUN_0042b0e0(int param_1, int param_2, int param_3);

/* Globals */
extern char s_jewelsound_0052ad74[];
extern char s_review3u_0052c4f0[];
extern char s_review3d_0052c4fc[];
extern char s_review2u_0052c4d8[];
extern char s_review2d_0052c4e4[];
extern char s_review1u_0052c4c0[];
extern char s_review1d_0052c4cc[];
extern char s_review7u_0052c4a8[];
extern char s_review7d_0052c4b4[];
extern char s_review6u_0052c490[];
extern char s_review6d_0052c49c[];

extern int DAT_00665d2c;
extern int DAT_00665d68;
extern int DAT_00665d6c;
extern int DAT_00665d28;
extern float DAT_00665d64;
extern float DAT_00665d74;
extern unsigned int DAT_00665d38[11];
extern int DAT_00aa60f4;
extern int DAT_00aa60fc;
extern float _DAT_005182ec;
extern double _DAT_005182f0;
extern double _DAT_00518300;

unsigned int __stdcall FUN_004e9af0(void)
{
    unsigned char uVar1;
    int iVar2;
    char *pcVar3;
    int *piVar4;
    unsigned int *puVar5;
    int uVar6;
    int iVar7;
    char *pcVar8;
    int local_38;
    int local_34;
    int local_30[4];
    int local_20[8];

    local_20[0] = 1;
    local_20[1] = 1;
    local_20[2] = 1;
    local_20[3] = 0;
    local_20[4] = 0;
    local_20[5] = 1;
    local_20[6] = 1;
    local_20[7] = 0;
    FUN_004de090(&local_34, &local_38);
    FUN_004d6460(local_30, 0x16, 0x72, 0x2a, 0x8a);
    FUN_004de090(&local_34, &local_38);
    piVar4 = local_20;
    do {
        if (*piVar4 != 0) {
            iVar2 = FUN_004d6b90(local_30, local_34, local_38);
            if (iVar2 != 0) {
                uVar1 = FUN_004dc1e0();
                if (uVar1 == 0) {
                    uVar1 = FUN_004dc200();
                    if (uVar1 == 0) {
                        uVar1 = FUN_004dc220();
                        if (uVar1 == 0) {
                            uVar1 = FUN_004dc240();
                            if (uVar1 == 0) goto LAB_004e9ba6;
                        }
                    }
                }
                *piVar4 = 2;
            }
        }
LAB_004e9ba6:
        piVar4 = piVar4 + 1;
        FUN_004d64a0(local_30, 0, 0x1c);
    } while (piVar4 < local_20 + 3);
    FUN_004d6460(local_30, 0x16, 0x132, 0x2a, 0x14a);
    piVar4 = local_20 + 5;
    do {
        if (*piVar4 != 0) {
            iVar2 = FUN_004d6b90(local_30, local_34, local_38);
            if (iVar2 != 0) {
                uVar1 = FUN_004dc1e0();
                if (uVar1 == 0) {
                    uVar1 = FUN_004dc200();
                    if (uVar1 == 0) {
                        uVar1 = FUN_004dc220();
                        if (uVar1 == 0) {
                            uVar1 = FUN_004dc240();
                            if (uVar1 == 0) goto LAB_004e9c2b;
                        }
                    }
                }
                *piVar4 = 2;
            }
        }
LAB_004e9c2b:
        piVar4 = piVar4 + 1;
        FUN_004d64a0(local_30, 0, 0x1c);
        if (local_20 + 7 <= piVar4) {
            FUN_004daba0(local_20);
            FUN_004d6460(local_30, 0x16, 0xaa, 0x2a, 0xc2);
            pcVar8 = s_jewelsound_0052ad74;
            iVar7 = 0xd;
            uVar6 = 0;
            iVar2 = 0xc;
            pcVar3 = FUN_004ddc10(0x126);
            iVar2 = FUN_004da780(local_30, s_review3u_0052c4f0, s_review3d_0052c4fc, pcVar3,
                                 iVar2, uVar6, iVar7, pcVar8);
            if (iVar2 != 0) {
                DAT_00665d2c = (int)((unsigned int)DAT_00665d2c + 0xffffffffu);
                if (DAT_00665d2c == -2) {
                    DAT_00665d68 = (int)((unsigned int)DAT_00665d68 + 0xffffffffu);
                    DAT_00665d2c = 1;
                    if (DAT_00665d68 == -2) {
                        DAT_00665d68 = 1;
                        DAT_00665d6c = (int)((unsigned int)DAT_00665d6c + 0xffffffffu);
                        if (DAT_00665d6c == -2) {
                            DAT_00665d6c = 1;
                        }
                    }
                }
                FUN_0042b010();
            }
            FUN_004d64a0(local_30, 0, -0x1c);
            pcVar8 = s_jewelsound_0052ad74;
            iVar7 = 0xc;
            uVar6 = 0;
            iVar2 = 0xc;
            pcVar3 = FUN_004ddc10(0x124);
            FUN_004da780(local_30, s_review2u_0052c4d8, s_review2d_0052c4e4, pcVar3, iVar2,
                         uVar6, iVar7, pcVar8);
            iVar2 = FUN_004d6b90(local_30, local_34, local_38);
            if (iVar2 != 0) {
                uVar1 = FUN_004dc1e0();
                if (uVar1 == 0) {
                    uVar1 = FUN_004dc200();
                    if ((uVar1 != 0) &&
                        (DAT_00665d64 = DAT_00665d64 + _DAT_005182ec,
                         (float)_DAT_00518300 <= DAT_00665d64)) {
                        DAT_00665d64 = 0.0f;
                    }
                } else {
                    DAT_00665d64 = DAT_00665d64 - _DAT_005182ec;
                    if (DAT_00665d64 <= (float)_DAT_005182f0) {
                        DAT_00665d64 = 360.0f;
                    }
                }
            }
            FUN_004d64a0(local_30, 0, -0x1c);
            pcVar8 = s_jewelsound_0052ad74;
            iVar7 = 0xb;
            uVar6 = 0;
            iVar2 = 0xc;
            pcVar3 = FUN_004ddc10(0x125);
            FUN_004da780(local_30, s_review1u_0052c4c0, s_review1d_0052c4cc, pcVar3, iVar2,
                         uVar6, iVar7, pcVar8);
            iVar2 = FUN_004d6b90(local_30, local_34, local_38);
            if (iVar2 != 0) {
                uVar1 = FUN_004dc1e0();
                if (uVar1 == 0) {
                    uVar1 = FUN_004dc200();
                    if ((uVar1 != 0) &&
                        (DAT_00665d74 = DAT_00665d74 + _DAT_005182ec,
                         (float)_DAT_00518300 <= DAT_00665d74)) {
                        DAT_00665d74 = 0.0f;
                    }
                } else {
                    DAT_00665d74 = DAT_00665d74 - _DAT_005182ec;
                    if (DAT_00665d74 <= (float)_DAT_005182f0) {
                        DAT_00665d74 = 360.0f;
                    }
                }
            }
            FUN_004d6460(local_30, 0x16, 0x14e, 0x2a, 0x166);
            pcVar8 = s_jewelsound_0052ad74;
            iVar7 = 0x11;
            uVar6 = 0;
            iVar2 = 0xc;
            pcVar3 = FUN_004ddc10(0x128);
            iVar2 = FUN_004da780(local_30, s_review7u_0052c4a8, s_review7d_0052c4b4, pcVar3,
                                 iVar2, uVar6, iVar7, pcVar8);
            if (iVar2 != 0) {
                DAT_00665d28 = (int)((unsigned int)DAT_00665d28 + 0xffffffffu);
                if (DAT_00665d28 < 0) {
                    DAT_00665d28 = (int)((unsigned int)DAT_00aa60fc + 0xffffffffu);
                }
                puVar5 = DAT_00665d38;
                for (iVar2 = 0xb; iVar2 != 0; iVar2 = iVar2 - 1) {
                    *puVar5 = 0;
                    puVar5 = puVar5 + 1;
                }
                DAT_00665d38[0] = *(unsigned int *)((unsigned int)DAT_00aa60f4 + 0x40u +
                                                    (unsigned int)DAT_00665d28 * 0x44u);
                FUN_00426a00(DAT_00665d38);
                FUN_00429e70((char *)((unsigned int)DAT_00665d28 * 0x44u +
                                      (unsigned int)DAT_00aa60f4));
                iVar2 = *(int *)((unsigned int)DAT_00aa60f4 + 0x40u +
                                 (unsigned int)DAT_00665d28 * 0x44u);
                if ((iVar2 == 6) || (iVar2 == 7)) {
                    uVar6 = 100;
                } else {
                    uVar6 = 0;
                }
                FUN_0042b0e0(0, uVar6, 0);
            }
            FUN_004d64a0(local_30, 0, -0x1c);
            pcVar8 = s_jewelsound_0052ad74;
            iVar7 = 0x10;
            uVar6 = 0;
            iVar2 = 0xc;
            pcVar3 = FUN_004ddc10(0x127);
            iVar2 = FUN_004da780(local_30, s_review6u_0052c490, s_review6d_0052c49c, pcVar3,
                                 iVar2, uVar6, iVar7, pcVar8);
            if (iVar2 != 0) {
                DAT_00665d28 = (int)((unsigned int)DAT_00665d28 + 1u);
                if (DAT_00aa60fc <= DAT_00665d28) {
                    DAT_00665d28 = 0;
                }
                puVar5 = DAT_00665d38;
                for (iVar2 = 0xb; iVar2 != 0; iVar2 = iVar2 - 1) {
                    *puVar5 = 0;
                    puVar5 = puVar5 + 1;
                }
                DAT_00665d38[0] = *(unsigned int *)((unsigned int)DAT_00aa60f4 + 0x40u +
                                                    (unsigned int)DAT_00665d28 * 0x44u);
                FUN_00426a00(DAT_00665d38);
                FUN_00429e70((char *)((unsigned int)DAT_00665d28 * 0x44u +
                                      (unsigned int)DAT_00aa60f4));
                iVar2 = *(int *)((unsigned int)DAT_00aa60f4 + 0x40u +
                                 (unsigned int)DAT_00665d28 * 0x44u);
                if ((iVar2 != 6) && (iVar2 != 7)) {
                    FUN_0042b0e0(0, 0, 0);
                    return 1;
                }
                FUN_0042b0e0(0, 100, 0);
            }
            return 1;
        }
    } while (1);
}
