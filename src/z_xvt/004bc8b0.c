/* SPDX-License-Identifier: MIT */
/* z_xvt 004bc8b0 */
extern int DAT_0064ef60;
extern uint DAT_0081bcd0;
extern char s_offslot_005313a0[];
extern char s_onslot_00531398[];
extern char s_3conbtn_00531384[];

void __cdecl FUN_004ddc70(uint *, uint *);
void __cdecl FUN_004d0500(char *, uint *);
void __cdecl FUN_004dd000(int *, int *);
void __stdcall FUN_004d52b0(void);
void __cdecl FUN_004d0580(char *, int, int);
void __cdecl FUN_004d5350(int);
uint __cdecl FUN_004d0e60(char *, int, int);
char * __cdecl FUN_004dcfa0(int);
void __cdecl FUN_004daac0(int, char *, int *, int, int, uint);

void __cdecl FUN_004bc8b0(byte *param_1, int *param_2, int param_3)
{
  uint uVar1;
  uint uVar2;
  int *piVar3;
  char *pcVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  int local_28;
  int local_24 = 0;
  int local_20;
  uint local_18;
  uint local_14;
  uint local_10[2];
  int local_8 = 0;
  
  FUN_004ddc70(&local_18, &local_14);
  FUN_004d0500(s_offslot_005313a0, local_10);
  FUN_004dd000(&local_28, param_2);
  uVar1 = 0;
  do {
    local_20 = (local_8 - local_10[0]) + local_28 + 1;
    if (DAT_0064ef60 != 0) {
      FUN_004d52b0();
      if (uVar1 == 0) {
        pcVar4 = s_offslot_005313a0;
      }
      else {
        pcVar4 = s_onslot_00531398;
      }
      FUN_004d0580(pcVar4, local_28, local_24);
      FUN_004d5350(0);
      if (uVar1 == 0) {
        pcVar4 = s_offslot_005313a0;
      }
      else {
        pcVar4 = s_onslot_00531398;
      }
      FUN_004d0580(pcVar4, local_28, local_24);
    }
    if (*param_1 == uVar1) {
      FUN_004d0e60(s_3conbtn_00531384, local_28, local_24);
    }
    local_28 = local_20 + 5;
    piVar3 = &local_28;
    iVar6 = 1;
    iVar5 = 0;
    uVar2 = uVar1 + 1;
    local_20 = local_20 + 0x69;
    uVar7 = DAT_0081bcd0;
    pcVar4 = FUN_004dcfa0(uVar1 + param_3);
    FUN_004daac0(0xc, pcVar4, piVar3, iVar5, iVar6, uVar7);
    local_28 = *param_2 + 0x82;
    uVar1 = uVar2;
  } while ((int)uVar2 < 2);
  return;
}
