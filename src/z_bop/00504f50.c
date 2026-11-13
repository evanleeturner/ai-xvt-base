/* SPDX-License-Identifier: MIT */
/* z_bop 00504f50 */
extern undefined4 __cdecl FUN_004d6460(undefined4 *param_1, undefined4 param_2, undefined4 param_3, undefined4 param_4, undefined4 param_5);
extern char * __cdecl FUN_004ddc10(uint param_1);
extern undefined4 __cdecl FUN_004db640(undefined4 param_1, char *param_2, undefined4 *param_3, uint param_4);
extern undefined4 __cdecl FUN_004dbbd0(undefined4 param_1, char *param_2, undefined4 *param_3, uint param_4, undefined4 param_5, undefined4 param_6);
extern undefined4 __cdecl FUN_004d9d70(undefined4 *param_1, undefined4 param_2, undefined4 param_3, undefined4 param_4, undefined4 param_5, undefined4 param_6, undefined4 param_7);

extern char *DAT_00aa6118;
extern int DAT_00b6a5aa;
extern int DAT_00b6a606;
extern int DAT_0052c208;
extern int DAT_00bb2810;

undefined4 __stdcall FUN_00504f50(void)
{
  char *pcVar1;
  int iVar2;
  uint uVar3;
  undefined4 *piVar4;
  undefined4 uVar5;
  uint uVar6;
  undefined4 local_10[4];

  if (DAT_00aa6118 != (char *)0x0) {
    FUN_004d6460(local_10, 0x58, 0x5a, 0x1ae, 0x6a);
    if ((DAT_00b6a5aa == 1) && (DAT_00b6a606 == 1)) {
      uVar3 = 0x337;
    }
    else if ((DAT_00b6a5aa == 3) && (DAT_00b6a606 == 1)) {
      uVar3 = 0x338;
    }
    else if ((DAT_00b6a5aa == 0) && (DAT_00b6a606 == 1)) {
      uVar3 = 0x339;
    }
    else {
      uVar3 = 0x336;
    }
    piVar4 = local_10;
    uVar6 = 0xffff;
    pcVar1 = FUN_004ddc10(uVar3);
    FUN_004db640(0xf, pcVar1, piVar4, uVar6);
    FUN_004d6460(local_10, 0x58, 0x6f, 0x1a4, 0x1af);
    iVar2 = FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, 0x1000);
    uVar5 = 0x1ae;
    if (0x14 < iVar2 + 1) {
      FUN_004d6460(local_10, 0x1a5, 0x6f, 0x1ae, 0x1af);
      DAT_0052c208 = FUN_004d9d70(local_10, DAT_0052c208, iVar2 + 1, 0, 5, DAT_00bb2810, 9);
      uVar5 = 0x1a4;
    }
    FUN_004d6460(local_10, 0x58, 0x6f, uVar5, 0x1af);
    FUN_004dbbd0(0xc, DAT_00aa6118, local_10, 0xffff, 4, DAT_0052c208);
    return 1;
  }
  return 0;
}
