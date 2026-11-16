/* SPDX-License-Identifier: MIT */
/* z_xvt 004fdfe0 */
extern int DAT_0081b850;
extern unsigned char DAT_0081bfba[4];
extern unsigned char DAT_0081c012[4];
extern int DAT_00654cd8;
extern int DAT_00654c10;

extern char s_jewelsound_00531990[];
extern char s_debrief1u_00534390[];
extern char s_debrief1d_0053439c[];
extern char s_debrief2u_005343a8[];
extern char s_debrief2d_005343b4[];
extern char s_debrief4u_005343c0[];
extern char s_debrief4d_005343cc[];

void FUN_004d9ea0(int *param_1);
void FUN_004dcfd0(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004dcfa0(int param_1);
void FUN_004d9d20(int *param_1, char *param_2, char *param_3, int param_4);
int FUN_004d9a80(int *param_1, char *param_2, char *param_3, char *param_4, int param_5,
                 undefined4 param_6, int param_7, char *param_8);
void FUN_004dd030(int *param_1, int param_2, int param_3);

undefined4 __stdcall FUN_004fdfe0(void)
{
  char *pcVar1;
  undefined4 uVar2;
  int iVar3;
  int iVar4;
  char *pcVar5;
  int local_2c[4];
  int local_1c[7];
  int local_bfba;
  int local_c012;

  __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
  if (((DAT_0081b850 == 2) && (local_bfba != 1)) && (local_bfba != 2)) {
    local_1c[0] = 0;
  }
  else {
    local_1c[0] = 1;
  }
  local_1c[1] = 1;
  __builtin_memcpy(&local_c012, DAT_0081c012, 4);
  local_1c[2] = (int)(local_c012 == 1);
  local_1c[3] = 0;
  local_1c[4] = 0;
  local_1c[5] = 0;
  local_1c[6] = 0;
  local_1c[DAT_00654cd8] = 2;
  FUN_004d9ea0(local_1c);
  FUN_004dcfd0(local_2c, 0x16, 0xaa, 0x2a, 0xc2);
  if (DAT_00654cd8 == 2) {
    __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
    if (local_bfba == 1) {
      iVar3 = 0xc;
      pcVar1 = FUN_004dcfa0(0x12e);
      FUN_004d9d20(local_2c, s_debrief4d_005343cc, pcVar1, iVar3);
    }
    else {
      __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
      if (local_bfba == 3) {
        iVar3 = 0xc;
        pcVar1 = FUN_004dcfa0(0x12d);
        FUN_004d9d20(local_2c, s_debrief4d_005343cc, pcVar1, iVar3);
      }
    }
  }
  else {
    __builtin_memcpy(&local_c012, DAT_0081c012, 4);
    if (local_c012 == 1) {
      __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
      if (local_bfba == 1) {
        pcVar5 = s_jewelsound_00531990;
        iVar4 = 0xe;
        uVar2 = 0;
        iVar3 = 0xc;
        pcVar1 = FUN_004dcfa0(0x12e);
        iVar3 = FUN_004d9a80(local_2c, s_debrief4u_005343c0, s_debrief4u_005343c0, pcVar1, iVar3,
                             uVar2, iVar4, pcVar5);
      }
      else {
        __builtin_memcpy(&local_bfba, DAT_0081bfba, 4);
        if (local_bfba != 3) goto LAB_004fe18a;
        pcVar5 = s_jewelsound_00531990;
        iVar4 = 0xe;
        uVar2 = 0;
        iVar3 = 0xc;
        pcVar1 = FUN_004dcfa0(0x12d);
        iVar3 = FUN_004d9a80(local_2c, s_debrief4u_005343c0, s_debrief4u_005343c0, pcVar1, iVar3,
                             uVar2, iVar4, pcVar5);
      }
      if (iVar3 != 0) {
        DAT_00654c10 = 1;
        DAT_00654cd8 = 2;
      }
    }
  }
LAB_004fe18a:
  FUN_004dd030(local_2c, 0, -0x1c);
  if (DAT_00654cd8 == 1) {
    iVar3 = 0xc;
    pcVar1 = FUN_004dcfa0(299);
    FUN_004d9d20(local_2c, s_debrief2d_005343b4, pcVar1, iVar3);
  }
  else {
    pcVar5 = s_jewelsound_00531990;
    iVar4 = 0xc;
    uVar2 = 0;
    iVar3 = 0xc;
    pcVar1 = FUN_004dcfa0(299);
    iVar3 = FUN_004d9a80(local_2c, s_debrief2u_005343a8, s_debrief2u_005343a8, pcVar1, iVar3,
                         uVar2, iVar4, pcVar5);
    if (iVar3 != 0) {
      DAT_00654c10 = 1;
      DAT_00654cd8 = 1;
    }
  }
  FUN_004dd030(local_2c, 0, -0x1c);
  if (local_1c[0] != 0) {
    if (DAT_00654cd8 == 0) {
      iVar3 = 0xc;
      pcVar1 = FUN_004dcfa0(0x12a);
      FUN_004d9d20(local_2c, s_debrief1d_0053439c, pcVar1, iVar3);
      return 1;
    }
    pcVar5 = s_jewelsound_00531990;
    iVar4 = 0xb;
    uVar2 = 0;
    iVar3 = 0xc;
    pcVar1 = FUN_004dcfa0(0x12a);
    iVar3 = FUN_004d9a80(local_2c, s_debrief1u_00534390, s_debrief1u_00534390, pcVar1, iVar3,
                         uVar2, iVar4, pcVar5);
    if (iVar3 != 0) {
      DAT_00654c10 = 1;
      DAT_00654cd8 = 0;
    }
  }
  return 1;
}
