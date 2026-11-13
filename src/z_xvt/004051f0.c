/* SPDX-License-Identifier: MIT */
/* z_xvt 004051f0 */
int __cdecl _fscanf(HFILE *stream, const char *format, ...);
extern char *DAT_00523574;

void __cdecl FUN_004051f0(HFILE *param_1)
{
  int iVar1;
  char local_1;
  
  do {
    iVar1 = _fscanf(param_1, DAT_00523574, &local_1);
    if (iVar1 != 1) {
      return;
    }
  } while (local_1 == '\"');
  return;
}
