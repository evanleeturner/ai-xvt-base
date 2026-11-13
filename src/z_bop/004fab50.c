/* SPDX-License-Identifier: MIT */
/* z_bop 004fab50 */
extern uint DAT_00b6a5aa;
extern uint DAT_00aa6118;
extern uint DAT_0052c208;
extern uint DAT_00bb2810;

void __cdecl FUN_004d6460(uint *param_1, uint param_2, uint param_3, uint param_4, uint param_5);
char * __cdecl FUN_004ddc10(uint param_1);
uint __cdecl FUN_004db7d0(uint param_1, char *param_2, uint *param_3, uint param_4, uint param_5, uint param_6);
int __cdecl FUN_004dbbd0(uint param_1, uint param_2, uint *param_3, uint param_4, uint param_5, uint param_6);
uint __cdecl FUN_004d9d70(uint *param_1, uint param_2, uint param_3, uint param_4, uint param_5, uint param_6, uint param_7);

uint __stdcall FUN_004fab50(void)
{
  char *pcVar1;
  int iVar2;
  uint uVar3;
  uint *piVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  uint local_10[4];
  
  FUN_004d6460(local_10, 0x58, 0x5a, 0x1ae, 0x6b);
  if (DAT_00b6a5aa == 2) {
    uVar3 = 0x1d7;
  }
  else if (DAT_00b6a5aa == 4) {
    uVar3 = 0x1d8;
  }
  else if (DAT_00b6a5aa == 5) {
    uVar3 = 0x309;
  }
  else {
    uVar3 = 0xc0;
  }
  piVar4 = local_10;
  uVar7 = 0xffff;
  iVar5 = 1;
  iVar2 = 0;
  pcVar1 = FUN_004ddc10(uVar3);
  FUN_004db7d0(0xf, pcVar1, piVar4, iVar2, iVar5, uVar7);
  FUN_004d6460(local_10, 0x58, 0x6c, 0x1a4, 0xfc);
  iVar2 = FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, 0x1000);
  uVar6 = 0x1ae;
  if (9 < iVar2 + 1) {
    FUN_004d6460(local_10, 0x1a5, 0x6c, 0x1ae, 0xfc);
    DAT_0052c208 = FUN_004d9d70(local_10, DAT_0052c208, iVar2 + 1, 0, 5, DAT_00bb2810, 9);
    uVar6 = 0x1a4;
  }
  FUN_004d6460(local_10, 0x58, 0x6c, uVar6, 0xfc);
  FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, DAT_0052c208);
  return 1;
}
