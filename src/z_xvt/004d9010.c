/* SPDX-License-Identifier: MIT */
/* z_xvt 004d9010 */
/* local_30 / local_10 are 4-int rects (left, top, right, bottom) in the
   original frame: local_30..local_24 and local_10..local_4 (local_8 unused
   by any read). */

extern int DAT_00532ae4;
extern int DAT_00532ae8;
extern int DAT_0064f688;
extern int DAT_0081b844;
extern int DAT_0081bc74;
extern char s_slideuu_00532b0c[];
extern char s_slideud_00532b14[];
extern char s_slidedu_00532afc[];
extern char s_slidedd_00532b04[];

void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

void FUN_004d8e20(int a);
void FUN_004ddc70(int *a, int *b);
uint FUN_004dbe80(void);
void FUN_004dbec0(void);
void FUN_004d8ee0(void);
unsigned char FUN_004db630(int a);
unsigned char FUN_004db570(void);
unsigned char FUN_004db590(void);
void FUN_004dd000(int *a, int *b);
void FUN_004dcfd0(int *a, int b, int c, int d, int e);
void FUN_004dd130(int *a, int b, int c, uint d);
int FUN_004dd7d0(int *a, int b, int c);
unsigned char FUN_004dbe00(int a);
unsigned char FUN_004db530(void);
unsigned char FUN_004db550(void);
void FUN_004d0e60(char *a, int b, int c);
void FUN_004dd3e0(int *a, int b, int c, int d, int e);
unsigned char FUN_004d8e10(int a);
void FUN_004dd070(int *a, int b, int c);
unsigned char FUN_004db640(void);
void FUN_004db510(int a);
unsigned char FUN_004db5f0(int a);
unsigned char FUN_004db610(int a);
void FUN_004db520(void);
void FUN_004db650(void);

int FUN_004d9010(int *param_1, int param_2, int param_3, int param_4, int param_5, uint param_6, int param_7)
{
    unsigned char bVar1;
    unsigned char uVar2;
    uint uVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int local_38;
    int local_34;
    int local_30[4];
    int local_20;
    int local_1c;
    int local_18;
    int local_14;
    int local_10[4];

    FUN_004d8e20(param_7);
    FUN_004ddc70(&local_20, &local_38);
    iVar5 = (int)((unsigned int)param_1[2] - (unsigned int)param_1[0]);
    local_14 = (int)((unsigned int)(int)((unsigned int)param_1[3] - (unsigned int)param_1[1])
                     + (unsigned int)iVar5 * (unsigned int)-2);
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
        uVar3 = FUN_004dbe80();
        if ((char)uVar3 == '\t') {
            FUN_004dbec0();
            FUN_004d8ee0();
        }
        local_1c = (int)((unsigned int)param_7 + 1000u);
        bVar1 = FUN_004db630(local_1c);
        if (bVar1 == 0) {
            uVar2 = FUN_004db570();
            if ((uVar2 != 0) || (uVar2 = FUN_004db590(), uVar2 != 0)) {
                DAT_00532ae4 = 0;
                DAT_00532ae8 = 0;
            }
            FUN_004dd000(local_30, param_1);
            {
                int t = (int)((unsigned int)param_2 * (unsigned int)local_14);
                if (local_18 == 0 || (t == (int)0x80000000 && local_18 == -1)) {
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                }
                iVar4 = (int)((unsigned int)local_30[1] + (unsigned int)(t / local_18) + (unsigned int)iVar5);
            }
            FUN_004dcfd0(local_10, local_30[0], iVar4, local_30[2],
                         (int)((unsigned int)local_34 + (unsigned int)iVar4));
            local_30[1] = (int)((unsigned int)local_30[1] + (unsigned int)iVar5);
            local_30[3] = (int)((unsigned int)local_30[3] - (unsigned int)iVar5);
            FUN_004dd130(local_30, 0, 0, param_6);
            iVar4 = FUN_004dd7d0(local_30, local_20, local_38);
            if ((iVar4 != 0) &&
                ((uVar2 = FUN_004db570(), uVar2 != 0) ||
                 (uVar2 = FUN_004db590(), uVar2 != 0))) {
                if (local_38 < local_10[1]) {
                    iVar6 = (int)((unsigned int)param_2 - (unsigned int)param_5);
                    if (iVar6 < param_4) {
                        iVar6 = param_4;
                    }
                }
                else if ((local_10[3] < local_38) &&
                         (iVar6 = (int)((unsigned int)param_5 + (unsigned int)param_2), param_3 <= iVar6)) {
                    iVar6 = (int)((unsigned int)param_3 - 1u);
                }
            }
            if (param_7 == DAT_0064f688) {
                bVar1 = FUN_004dbe00(0x21);
                if (bVar1 == 0) {
                    bVar1 = FUN_004dbe00(0x22);
                    if ((bVar1 != 0) &&
                        (iVar6 = (int)((unsigned int)param_5 + (unsigned int)param_2), param_3 <= iVar6)) {
                        iVar6 = (int)((unsigned int)param_3 - 1u);
                    }
                }
                else {
                    iVar6 = (int)((unsigned int)param_2 - (unsigned int)param_5);
                    if (iVar6 < param_4) {
                        iVar6 = param_4;
                    }
                }
            }
            FUN_004dcfd0(local_30, *param_1, param_1[1], param_1[2],
                         (int)((unsigned int)iVar5 + (unsigned int)param_1[1]));
            iVar4 = FUN_004dd7d0(local_30, local_20, local_38);
            if ((iVar4 == 0) ||
                ((uVar2 = FUN_004db530(), uVar2 == 0) &&
                 (uVar2 = FUN_004db550(), uVar2 == 0))) {
                FUN_004d0e60(s_slideuu_00532b0c, *param_1, param_1[1]);
            }
            else {
                FUN_004d0e60(s_slideud_00532b14, *param_1, param_1[1]);
                if (DAT_00532ae4 == 0) {
                    if (param_4 < param_2) {
                        iVar6 = (int)((unsigned int)param_2 - 1u);
                    }
                    if (DAT_00532ae8 == 0) {
                        DAT_00532ae8 = 0xc;
                    }
                    else {
                        DAT_00532ae8 = DAT_00532ae8 >> 2;
                        if (DAT_00532ae8 == 0) {
                            DAT_00532ae8 = 1;
                        }
                    }
                    DAT_00532ae4 = DAT_00532ae8;
                }
                else {
                    DAT_00532ae4 = (int)((unsigned int)DAT_00532ae4 - 1u);
                }
            }
            if (((param_7 == DAT_0064f688) &&
                 (bVar1 = FUN_004dbe00(0x26), bVar1 != 0)) && (param_4 < param_2)) {
                iVar6 = (int)((unsigned int)param_2 - 1u);
            }
            FUN_004dcfd0(local_30, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5),
                         param_1[2], param_1[3]);
            iVar4 = FUN_004dd7d0(local_30, local_20, local_38);
            if ((iVar4 == 0) ||
                ((uVar2 = FUN_004db530(), uVar2 == 0) &&
                 (uVar2 = FUN_004db550(), uVar2 == 0))) {
                FUN_004d0e60(s_slidedu_00532afc, *param_1,
                             (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
            }
            else {
                FUN_004d0e60(s_slidedd_00532b04, *param_1,
                             (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
                if (DAT_00532ae4 == 0) {
                    if (param_2 < (int)((unsigned int)param_3 - 1u)) {
                        iVar6 = (int)((unsigned int)param_2 + 1u);
                    }
                    if (DAT_00532ae8 == 0) {
                        DAT_00532ae8 = 0xc;
                    }
                    else {
                        DAT_00532ae8 = DAT_00532ae8 >> 2;
                        if (DAT_00532ae8 == 0) {
                            DAT_00532ae8 = 1;
                        }
                    }
                    DAT_00532ae4 = DAT_00532ae8;
                }
                else {
                    DAT_00532ae4 = (int)((unsigned int)DAT_00532ae4 - 1u);
                }
            }
            if (((param_7 == DAT_0064f688) &&
                 (bVar1 = FUN_004dbe00(0x28), bVar1 != 0)) &&
                (param_2 < (int)((unsigned int)param_3 - 1u))) {
                iVar6 = (int)((unsigned int)param_2 + 1u);
            }
            FUN_004dd3e0(local_10, 0, 0, 0xffff, 0);
            bVar1 = FUN_004d8e10(param_7);
            if (bVar1 != 0) {
                FUN_004dd070(local_10, 2, 2);
                FUN_004dd3e0(local_10, 0, 0, (short)DAT_0081b844, 1);
                FUN_004dd070(local_10, -2, -2);
            }
            bVar1 = FUN_004db640();
            if (((bVar1 != 0) &&
                 (iVar5 = FUN_004dd7d0(local_10, local_20, local_38), iVar5 != 0)) &&
                ((uVar2 = FUN_004db530(), uVar2 != 0) ||
                 (uVar2 = FUN_004db550(), uVar2 != 0))) {
                FUN_004db510(local_1c);
                return iVar6;
            }
        }
        else {
            DAT_00532ae4 = 0;
            DAT_00532ae8 = 0;
            uVar2 = FUN_004db5f0(local_1c);
            if ((uVar2 != 0) || (uVar2 = FUN_004db610(local_1c), uVar2 != 0)) {
                FUN_004db520();
                FUN_004db650();
            }
            FUN_004dd000(local_30, param_1);
            FUN_004dd130(local_30, 0, 0, param_6);
            FUN_004dcfd0(local_30, *param_1, param_1[1], param_1[2],
                         (int)((unsigned int)iVar5 + (unsigned int)param_1[1]));
            FUN_004d0e60(s_slideuu_00532b0c, *param_1, param_1[1]);
            FUN_004dcfd0(local_30, *param_1, (int)((unsigned int)param_1[3] - (unsigned int)iVar5),
                         param_1[2], param_1[3]);
            FUN_004d0e60(s_slidedu_00532afc, *param_1,
                         (int)((unsigned int)param_1[3] - (unsigned int)iVar5));
            {
                int t = (int)((unsigned int)(int)(((unsigned int)local_38 - (unsigned int)param_1[1])
                                                  - (unsigned int)iVar5)
                              * (unsigned int)local_18);
                if (local_14 == 0 || (t == (int)0x80000000 && local_14 == -1)) {
                    RaiseException(0xC0000094UL, 0, 0, NULL);
                }
                iVar6 = t / local_14;
            }
            if (iVar6 < param_4) {
                iVar6 = param_4;
            }
            if (param_3 <= iVar6) {
                iVar6 = (int)((unsigned int)param_3 - 1u);
            }
            iVar4 = (int)((unsigned int)iVar5 + (unsigned int)param_1[1]);
            if (iVar4 <= local_38) {
                iVar5 = (int)((unsigned int)(int)((unsigned int)param_1[3] - (unsigned int)local_34)
                              - (unsigned int)iVar5);
                iVar4 = local_38;
                if (iVar5 <= local_38) {
                    iVar4 = (int)((unsigned int)iVar5 - 1u);
                }
            }
            FUN_004dcfd0(local_30, *param_1, iVar4, param_1[2],
                         (int)((unsigned int)local_34 + (unsigned int)iVar4));
            FUN_004dd3e0(local_30, 0, 0, 0xffff, 0);
            bVar1 = FUN_004d8e10(param_7);
            if (bVar1 != 0) {
                FUN_004dd070(local_30, 2, 2);
                FUN_004dd3e0(local_30, 0, 0, (short)DAT_0081bc74, 1);
                FUN_004dd070(local_30, -2, -2);
            }
        }
    }
    return iVar6;
}
