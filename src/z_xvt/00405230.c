/* SPDX-License-Identifier: MIT */
/* z_xvt 00405230 */
int _fscanf(HFILE, const char *, ...);
extern const char DAT_00523574[];

void __cdecl FUN_00405230(HFILE param_1)
{
  int iVar1;
  char local_1;
  
  do {
    iVar1 = _fscanf(param_1, DAT_00523574, &local_1);
    if (iVar1 != 1) {
      return;
    }
  } while (local_1 != '}');
  return;
}
