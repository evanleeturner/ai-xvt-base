/* SPDX-License-Identifier: MIT */
/* z_bop 0046f660 */
int __cdecl FUN_0046f660(int param_1)
{
  extern int DAT_00999570;
  extern int DAT_009996b0;
  int iVar1;
  int *piVar2;
  
  iVar1 = 0;
  piVar2 = &DAT_00999570;
  do {
    if (*piVar2 == param_1) {
      return iVar1;
    }
    piVar2 = piVar2 + 10;
    iVar1 = iVar1 + 1;
  } while (piVar2 < &DAT_009996b0);
  return iVar1;
}
