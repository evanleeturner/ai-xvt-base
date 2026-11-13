/* SPDX-License-Identifier: MIT */
/* z_bop 0046f940 */
extern int DAT_009994fc;
extern undefined4 DAT_00999550;

undefined4 * __cdecl FUN_0046f5e0(int *param_1, undefined4 *param_2, int param_3);

undefined4 __stdcall FUN_0046f940(void)
{
  int *piVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  undefined4 local_8;
  int local_4;
  
  piVar1 = (int *)FUN_0046f5e0(&local_4,&local_8,0x3c);
  if (piVar1 == (int *)0x0) {
    return 0;
  }
  DAT_009994fc = *piVar1;
  iVar4 = 0;
  if (0 < DAT_009994fc) {
    puVar5 = &DAT_00999550;
    do {
      puVar2 = (undefined4 *)FUN_0046f5e0(&local_4,&local_8,0x3c);
      if (puVar2 == (undefined4 *)0x0) {
        return 0;
      }
      puVar6 = puVar5;
      for (iVar3 = 10; iVar3 != 0; iVar3 = iVar3 + -1) {
        *puVar6 = *puVar2;
        puVar2 = puVar2 + 1;
        puVar6 = puVar6 + 1;
      }
      iVar4 = iVar4 + 1;
      puVar5 = puVar5 + 10;
    } while (iVar4 < DAT_009994fc);
  }
  return 1;
}
