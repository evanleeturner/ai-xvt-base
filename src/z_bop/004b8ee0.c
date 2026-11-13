/* SPDX-License-Identifier: MIT */
/* z_bop 004b8ee0 */
void __cdecl FUN_004de090(int *param_1, int *param_2);
void __cdecl FUN_004b4cc0(char *param_1, int *param_2);
void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004d4e20(void);
uint __cdecl FUN_004b4d40(char *param_1, int param_2, int param_3);
void __cdecl FUN_004d4ec0(int param_1);
int __cdecl FUN_004d6b90(int *param_1, int param_2, int param_3);
int __cdecl FUN_004dc220(void);
int __cdecl FUN_004dc240(void);
void __cdecl FUN_004de5a0(char *param_1, int param_2, int param_3, int param_4);
void __cdecl FUN_004b5570(char *param_1, int param_2, int param_3);
char *__cdecl FUN_004ddc10(int param_1);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, uint param_6);

extern int DAT_00664e98;
extern byte DAT_00bb73a7;
extern uint DAT_00b6a2c0;
extern char s_offslot_0052a71c[];
extern char s_onslot_0052a714[];
extern char s_configsound_0052a708[];
extern char s_3conbtn_0052a700[];

void __cdecl FUN_004b8ee0(byte *param_1, int *param_2, int param_3, int param_4, int param_5)
{
  int local_28 = 0;
  int local_24 = 0;
  int local_20 = 0;
  int local_18 = 0;
  int local_14 = 0;
  int local_10[2] = {0};
  int local_8 = 0;
  uint uVar3;
  int iVar2;
  char *pcVar5;
  int *piVar4;
  int iVar6;
  uint uVar7;

  FUN_004de090(&local_14, &local_18);
  FUN_004b4cc0(s_offslot_0052a71c, local_10);
  uVar3 = 0;
  FUN_004d6480(&local_28, param_2);
  do {
    local_20 = (local_8 - local_10[0]) + local_28 + 1;
    if (DAT_00664e98 != 0) {
      FUN_004d4e20();
      if (uVar3 == 0) {
        pcVar5 = s_offslot_0052a71c;
      }
      else {
        pcVar5 = s_onslot_0052a714;
      }
      FUN_004b4d40(pcVar5, local_28, local_24);
      FUN_004d4ec0(0);
      if (uVar3 == 0) {
        pcVar5 = s_offslot_0052a71c;
      }
      else {
        pcVar5 = s_onslot_0052a714;
      }
      FUN_004b4d40(pcVar5, local_28, local_24);
    }
    if (((param_5 == 0) && (iVar2 = FUN_004d6b90(&local_28, local_14, local_18), iVar2 != 0)) &&
       ((FUN_004dc220() != 0) || (FUN_004dc240() != 0))) {
      if ((*param_1 != uVar3) && (DAT_00bb73a7 != '\0')) {
        FUN_004de5a0(s_configsound_0052a708, 1, 0, 0xff);
      }
      *param_1 = (byte)uVar3;
    }
    if (*param_1 == uVar3) {
      if (param_4 == 0) {
        FUN_004b5570(s_3conbtn_0052a700, local_28, local_24);
      }
      else {
        FUN_004b4d40(s_3conbtn_0052a700, local_28, local_24);
      }
    }
    local_28 = local_20 + 5;
    piVar4 = &local_28;
    iVar6 = 1;
    iVar2 = 0;
    uVar3 = uVar3 + 1;
    local_20 = local_20 + 0x69;
    uVar7 = DAT_00b6a2c0;
    pcVar5 = FUN_004ddc10(param_3 + -1 + uVar3);
    FUN_004db7d0(0xc, pcVar5, piVar4, iVar2, iVar6, uVar7);
    local_28 = *param_2 + 0x82;
  } while ((int)uVar3 < 2);
  return;
}
