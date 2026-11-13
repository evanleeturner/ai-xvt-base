/* SPDX-License-Identifier: MIT */
/* z_xvt 004db490 */
extern undefined4 DAT_0075dc4e;
extern undefined4 DAT_0075dc46;
extern undefined4 DAT_0075dc4a;

undefined4 __cdecl FUN_004db490(undefined4 param_1)
{
  int iVar1;
  undefined4 *puVar2;
  
  puVar2 = &DAT_0075dc4e;
  for (iVar1 = 0x8000; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  DAT_0075dc46 = param_1;
  DAT_0075dc4a = param_1;
  return 1;
}
