/* SPDX-License-Identifier: MIT */
/* z_bop 004d9d70 */
extern int DAT_0052c00c;
extern int DAT_0052c010;
extern int DAT_00665470;
extern int DAT_00b69e24;
extern int DAT_00b6a264;
extern char s_slidedu_0052c024[];
extern char s_slidedd_0052c02c[];
extern char s_slideuu_0052c034[];
extern char s_slideud_0052c03c[];

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

int __cdecl FUN_004d9ba0(int a);
int __cdecl FUN_004de090(int *a, int *b);
unsigned int __cdecl FUN_004dcb10(void);
void __cdecl FUN_004dcb50(void);
void __cdecl FUN_004d9c50(void);
unsigned char __cdecl FUN_004dc2e0(int a);
unsigned char __cdecl FUN_004dc220(void);
unsigned char __cdecl FUN_004dc240(void);
void __cdecl FUN_004d6480(int *a, int *b);
void __cdecl FUN_004d6460(int *a, int b, int c, int d, int e);
void __cdecl FUN_004d6550(int *a, int b, int c, unsigned int d);
int __cdecl FUN_004d6b90(int *a, int b, int c);
unsigned char __cdecl FUN_004dca90(int a);
unsigned char __cdecl FUN_004dc1e0(void);
unsigned char __cdecl FUN_004dc200(void);
void __cdecl FUN_004b5570(char *a, int b, int c);
void __cdecl FUN_004d67e0(int *a, int b, int c, int d, int e);
unsigned char __cdecl FUN_004d9b80(int a);
void __cdecl FUN_004d64c0(int *a, int b, int c);
unsigned char __cdecl FUN_004dc300(void);
void __cdecl FUN_004dc1c0(int a);
unsigned char __cdecl FUN_004dc2a0(int a);
unsigned char __cdecl FUN_004dc2c0(int a);
void __cdecl FUN_004dc1d0(void);
void __cdecl FUN_004dc310(void);

int __cdecl
FUN_004d9d70(int *param_1, int param_2, int param_3, int param_4, int param_5, unsigned int param_6, int param_7)
{
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int local_38;
    int local_34;
    int local_30[4];   /* local_30, local_2c, local_28, local_24 */
    int local_20;
    int local_1c;
    int local_18;
    int local_14;
    int local_10[4];   /* local_10, local_c, (unused), local_4 */

    FUN_004d9ba0(param_7);
    FUN_004de090(&local_20, &local_38);
    iVar5 = (int)((unsigned int)param_1[2] - (unsigned int)*param_1);
    local_14 = (int)((unsigned int)(int)((unsigned int)param_1[3] - (unsigned int)param_1[1]) + (unsigned int)iVar5 * (unsigned int)-2);
    local_18 = (int)((unsigned int)param_3 - (unsigned int)param_4);
    if (local_18 == 0 || (local_14 == (int)0x80000000 && local_18 == -1)) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    }
    local_34 = local_14 / local_18;
    if (local_34 < 1) {
        local_34 = 1;
    }
    iVar6 = param_2;
    if (iVar5 < (int)((unsigned int)param_1[3] - (unsigned int)param_1[1])) {
        uVar3 = FUN_004dcb10();
        if ((char)uVar3 == '\t') {
            FUN_004dcb50();
            FUN_004d9c50();
        }
        local_1c = (int)((unsigned int)param_7 + 1000U);
        if (FUN_004dc2e0(local_1c) == 0) {
            if (FUN_004dc220() != 0 || FUN_004dc240() != 0) {
                DAT_0052c00c = 0;
                DAT_0052c010 = 0;
            }
            FUN_004d6480(local_30, param_1);
            {
                int t = (int)((unsigned int)param_2 * (unsigned int)local_14);
                if (local_18 == 0 || (t == (int)0x80000000 && local_18 == -1)) {
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                }
                iVar4 = (int)((unsigned int)local_30[1] + (unsigned int)(t / local_18) + (unsigned int)iVar5);
            }
            FUN_004d6460(local_10, local_30[0], iVar4, local_30[2], (int)((unsigned int)local_34 + (unsigned int)iVar4));
            local_30[1] = (int)((unsigned int)local_30[1] + (unsigned int)iVar5);
            local_30[3] = (int)((unsigned int)local_30[3] - (unsigned int)iVar5);
            FUN_004d6550(local_30, 0, 0, param_6);
            iVar4 = FUN_004d6b90(local_30, local_20, local_38);
            if ((iVar4 != 0) && (FUN_004dc220() != 0 || FUN_004dc240() != 0)) {
                if (local_38 < local_10[1]) {
                    iVar6 = (int)((unsigned int)param_2 - (unsigned int)param_5);
                    if (iVar6 < param_4) {
                        iVar6 = param_4;
                    }
                } else if (local_10[3] < local_38) {
                    iVar6 = (int)((unsigned int)param_5 + (unsigned int)param_2);
                    if (param_3 <= iVar6) {
                        iVar6 = (int)((unsigned int)param_3 - 1U);
                    }
                }
            }
            if (DAT_00665470 == param_7) {
                if (FUN_004dca90(0x21) == 0) {
                    if (FUN_004dca90(0x22) != 0) {
                        iVar6 = (int)((unsigned int)param_5 + (unsigned int)param_2);
                        if (param_3 <= iVar6) {
                            iVar6 = (int)((unsigned int)param_3 - 1U);
                        }
                    }
                } else {
                    iVar6 = (int)((unsigned int)param_2 - (unsigned int)param_5);
                    if (iVar6 < param_4) {
                        iVar6 = param_4;
                    }
                }
            }
            FUN_004d6460(local_30, *param_1, param_1[1], param_1[2], (int)((unsigned int)param_1[1] + (unsigned int)iVar5));
            iVar4 = FUN_004d6b90(local_30, local_20, local_38);
            if (iVar4 == 0 || (FUN_004dc1e0() == 0 && FUN_004dc200() == 0)) {
                FUN_004b5570(s_slideuu_0052c034, *param_1, param_1[1]);
            } else {
                FUN_004b5570(s_slideud_0052c03c, *param_1, param_1[1]);
                if (DAT_0052c00c == 0) {
                    if (param_4 < param_2) {
                        iVar6 = (int)((unsigned int)param_2 - 1U);
                    }
                    if (DAT_0052c010 == 0) {
                        DAT_0052c010 = 0xc;
                    } else {
                        DAT_0052c010 = DAT_0052c010 >> 2;
                        if (DAT_0052c010 == 0) {
                            DAT_0052c010 = 1;
                        }
                    }
                    DAT_0052c00c = DAT_0052c010;
                } else {
                    DAT_0052c00c = (int)((unsigned int)DAT_0052c00c - 1U);
                }
            }
            if (DAT_00665470 == param_7 && FUN_004dca90(0x26) != 0 && param_4 < param_2) {
                iVar6 = (int)((unsigned int)param_2 - 1U);
            }
            FUN_004d6460(local_30, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5), param_1[2], param_1[3]);
            iVar4 = FUN_004d6b90(local_30, local_20, local_38);
            if (iVar4 == 0 || (FUN_004dc1e0() == 0 && FUN_004dc200() == 0)) {
                FUN_004b5570(s_slidedu_0052c024, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
            } else {
                FUN_004b5570(s_slidedd_0052c02c, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
                if (DAT_0052c00c == 0) {
                    if (param_2 < (int)((unsigned int)param_3 - 1U)) {
                        iVar6 = (int)((unsigned int)param_2 + 1U);
                    }
                    if (DAT_0052c010 == 0) {
                        DAT_0052c010 = 0xc;
                    } else {
                        DAT_0052c010 = DAT_0052c010 >> 2;
                        if (DAT_0052c010 == 0) {
                            DAT_0052c010 = 1;
                        }
                    }
                    DAT_0052c00c = DAT_0052c010;
                } else {
                    DAT_0052c00c = (int)((unsigned int)DAT_0052c00c - 1U);
                }
            }
            if (DAT_00665470 == param_7 && FUN_004dca90(0x28) != 0 && param_2 < (int)((unsigned int)param_3 - 1U)) {
                iVar6 = (int)((unsigned int)param_2 + 1U);
            }
            FUN_004d67e0(local_10, 0, 0, 0xffff, 0);
            if (FUN_004d9b80(param_7) != 0) {
                FUN_004d64c0(local_10, 2, 2);
                FUN_004d67e0(local_10, 0, 0, (short)DAT_00b69e24, 1);
                FUN_004d64c0(local_10, -2, -2);
            }
            if (FUN_004dc300() != 0 &&
                FUN_004d6b90(local_10, local_20, local_38) != 0 &&
                (FUN_004dc1e0() != 0 || FUN_004dc200() != 0)) {
                FUN_004dc1c0(local_1c);
                return iVar6;
            }
        } else {
            DAT_0052c00c = 0;
            DAT_0052c010 = 0;
            if (FUN_004dc2a0(local_1c) != 0 || FUN_004dc2c0(local_1c) != 0) {
                FUN_004dc1d0();
                FUN_004dc310();
            }
            FUN_004d6480(local_30, param_1);
            FUN_004d6550(local_30, 0, 0, param_6);
            FUN_004d6460(local_30, *param_1, param_1[1], param_1[2], (int)((unsigned int)param_1[1] + (unsigned int)iVar5));
            FUN_004b5570(s_slideuu_0052c034, *param_1, param_1[1]);
            FUN_004d6460(local_30, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5), param_1[2], param_1[3]);
            FUN_004b5570(s_slidedu_0052c024, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
            {
                int t = (int)((unsigned int)(int)(((unsigned int)local_38 - (unsigned int)iVar5) - (unsigned int)param_1[1]) * (unsigned int)local_18);
                if (local_14 == 0 || (t == (int)0x80000000 && local_14 == -1)) {
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                }
                iVar6 = t / local_14;
            }
            if (iVar6 < param_4) {
                iVar6 = param_4;
            }
            if (param_3 <= iVar6) {
                iVar6 = (int)((unsigned int)param_3 - 1U);
            }
            iVar4 = (int)((unsigned int)param_1[1] + (unsigned int)iVar5);
            if (iVar4 <= local_38) {
                iVar5 = (int)(((unsigned int)param_1[3] - (unsigned int)local_34) - (unsigned int)iVar5);
                iVar4 = local_38;
                if (iVar5 <= local_38) {
                    iVar4 = (int)((unsigned int)iVar5 - 1U);
                }
            }
            FUN_004d6460(local_30, *param_1, iVar4, param_1[2], (int)((unsigned int)local_34 + (unsigned int)iVar4));
            FUN_004d67e0(local_30, 0, 0, 0xffff, 0);
            if (FUN_004d9b80(param_7) != 0) {
                FUN_004d64c0(local_30, 2, 2);
                FUN_004d67e0(local_30, 0, 0, (short)DAT_00b6a264, 1);
                FUN_004d64c0(local_30, -2, -2);
            }
        }
    }
    return iVar6;
}
