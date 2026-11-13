/* SPDX-License-Identifier: MIT */
/* z_xvt 00406330 */
extern short DAT_0071e522;
extern short DAT_0071e518;
extern short DAT_00703ad4;
extern int _DAT_0074110c;

void __cdecl FUN_00406330(int *param_1, int param_2, int param_3)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  
  do {
    iVar4 = (int)DAT_0071e522 + *param_1;
    iVar2 = param_1[2] + iVar4;
    iVar1 = (int)DAT_0071e518;
    iVar3 = (int)DAT_00703ad4;
    if (iVar4 < iVar1) {
      iVar4 = iVar1;
    }
    if ((iVar4 <= iVar3) && (iVar1 <= iVar2)) {
      if (iVar3 < iVar2) {
        iVar2 = iVar3;
      }
      iVar2 = iVar2 - iVar4;
      if (iVar2 != 0) {
        iVar1 = param_1[1];
        piVar5 = (int *)(iVar4 * 4 + param_3);
        do {
          iVar4 = *piVar5;
          piVar5 = piVar5 + 1;
          *(char *)(param_2 + iVar4) = (char)iVar1;
          iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
      }
    }
    _DAT_0074110c = _DAT_0074110c + -1;
    param_1 = param_1 + 3;
  } while (_DAT_0074110c != 0);
  return;
}
