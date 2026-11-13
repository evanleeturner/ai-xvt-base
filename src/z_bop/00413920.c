/* SPDX-License-Identifier: MIT */
/* z_bop 00413920 */
extern const char *DAT_0051be2c;
int __cdecl FUN_00413b80(HFILE param_1);
int __cdecl _fscanf(HFILE stream, const char *format, ...);

void __cdecl FUN_00413920(HFILE param_1)
{
  int iVar1;
  char local_1;
  
  iVar1 = FUN_00413b80(param_1);
  if (iVar1 == 0) {
    do {
      iVar1 = _fscanf(param_1, DAT_0051be2c, &local_1);
      if (iVar1 != 1) {
        return;
      }
    } while (local_1 != ',');
  }
  return;
}
