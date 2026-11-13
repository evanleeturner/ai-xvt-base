/* SPDX-License-Identifier: MIT */
/* z_bop 004138e0 */
extern int _fscanf(HFILE, const char *, ...);
extern char DAT_0051be2c[];

void __cdecl FUN_004138e0(HFILE param_1)
{
  int iVar1;
  char local_1;
  
  do {
    iVar1 = _fscanf(param_1, DAT_0051be2c, &local_1);
    if (iVar1 != 1) {
      return;
    }
  } while (local_1 == '[');
  return;
}
