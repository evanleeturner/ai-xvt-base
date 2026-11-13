/* SPDX-License-Identifier: MIT */
/* z_bop 00418650 */
extern int DAT_00523428;
extern char DAT_009e9680[];
extern int DAT_009ecc34;
extern int DAT_009ed670[];
extern int DAT_009a8db0[];

void __cdecl FUN_004186e0(int param_1, int *param_2);

void __cdecl FUN_00418650(int param_1)
{
  int *piVar1;
  int iVar2;
  
  if (((DAT_00523428 == 0) && (DAT_009e9680[param_1 * 0x5bd] != '\0')) &&
     (param_1 != DAT_009ecc34)) {
    iVar2 = 0;
    piVar1 = (int *)&DAT_009ed670 + param_1 * 0x7e9;
    if (0 < (int)DAT_009a8db0[param_1]) {
      do {
        if ((*piVar1 == 0) && (piVar1[1] == 2)) {
          FUN_004186e0(param_1, piVar1);
        }
        else {
          piVar1 = (int *)((int)piVar1 + 0x12);
          iVar2 = iVar2 + 1;
        }
      } while (iVar2 < (int)DAT_009a8db0[param_1]);
    }
  }
  return;
}
