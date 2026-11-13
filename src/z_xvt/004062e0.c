/* SPDX-License-Identifier: MIT */
/* z_xvt 004062e0 */
extern short DAT_0071e522;
extern int _DAT_0074110c;

void __cdecl FUN_004062e0(int *param_1, int param_2, int param_3)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  
  do {
    iVar1 = param_1[1];
    iVar2 = *param_1;
    iVar3 = param_1[2];
    param_1 = param_1 + 3;
    piVar4 = (int *)((DAT_0071e522 + iVar2) * 4 + param_3);
    do {
      iVar2 = *piVar4;
      piVar4 = piVar4 + 1;
      *(char *)(param_2 + iVar2) = (char)iVar1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
    _DAT_0074110c = _DAT_0074110c + -1;
  } while (_DAT_0074110c != 0);
  return;
}
