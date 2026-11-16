/* SPDX-License-Identifier: MIT */
/* z_bop 00476280 */
typedef unsigned int undefined4;

extern int DAT_0060f1f8;
extern int DAT_0060f1dc;
extern int DAT_005272bc;
extern int DAT_0060f1c8;

void __cdecl FUN_00475d50(int param_1, undefined4 *param_2, undefined4 param_3, int param_4, int param_5);

void __cdecl FUN_00476280(int param_1, undefined4 *param_2, int param_3, int param_4)
{
  int iVar1;
  int iVar2;
  
  DAT_0060f1f8 = DAT_0060f1dc;
  iVar2 = 0;
  DAT_005272bc = 0;
  if (0 < *(int *)(DAT_0060f1c8 + 8)) {
    iVar1 = 0;
    do {
      FUN_00475d50(param_1, param_2, *(undefined4 *)(*(int *)(DAT_0060f1c8 + 0xc) + iVar1), param_3, param_4);
      if (DAT_005272bc == 0) {
        return;
      }
      iVar1 = iVar1 + 4;
      iVar2 = iVar2 + 1;
    } while (iVar2 < *(int *)(DAT_0060f1c8 + 8));
  }
  return;
}
