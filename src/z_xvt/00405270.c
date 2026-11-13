/* SPDX-License-Identifier: MIT */
/* z_xvt 00405270 */
int _fscanf(void *, const char *, ...);
extern char DAT_00523574[];

void __cdecl FUN_00405270(void *param_1)
{
  int iVar1;
  char local_1;
  
  do {
    iVar1 = _fscanf(param_1, DAT_00523574, &local_1);
    if (iVar1 != 1) {
      return;
    }
  } while (local_1 != ']');
  return;
}
