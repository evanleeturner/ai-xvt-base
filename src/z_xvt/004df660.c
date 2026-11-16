/* SPDX-License-Identifier: MIT */
/* z_xvt 004df660 */
HFILE __cdecl FUN_004be160(char *param_1, char *param_2);
void __cdecl FUN_004be320(HFILE param_1, void *param_2, int param_3);
undefined __cdecl FUN_004df700(HFILE param_1, undefined4 *param_2, int param_3);
uint __cdecl FUN_004be1f0(HFILE param_1);
extern char DAT_00523570[];

undefined4 __cdecl FUN_004df660(char *param_1, undefined4 *param_2)
{
  HFILE pFVar1;
  undefined4 uVar2;
  int iVar3;
  short local_38[8];
  undefined1 local_28[0x28];
  short local_1c = 0;
  short local_1a = 0;
  
  if (param_2 == (undefined4 *)0x0) {
    return 0;
  }
  uVar2 = 0;
  pFVar1 = FUN_004be160(param_1, DAT_00523570);
  if (pFVar1 == (HFILE)0x0) {
    return 0;
  }
  FUN_004be320(pFVar1, local_38, 0xe);
  if ((local_38[0] == 0x4d42) && (FUN_004be320(pFVar1, local_28, 0x28), local_1c == 1)) {
    if (local_1a == 4) {
      iVar3 = 0x10;
    }
    else {
      if (local_1a != 8) goto LAB_004df6ed;
      iVar3 = 0x100;
    }
    uVar2 = 1;
    FUN_004df700(pFVar1, param_2, iVar3);
  }
LAB_004df6ed:
  FUN_004be1f0(pFVar1);
  return uVar2;
}
