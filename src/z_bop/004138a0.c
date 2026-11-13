/* SPDX-License-Identifier: MIT */
/* z_bop 004138a0 */
extern int __cdecl _fscanf(void *, const char *, ...);
extern const char *DAT_0051be2c;

void __cdecl FUN_004138a0(void *param_1)
{
  int iVar1;
  char local_1;
  
  do {
    iVar1 = _fscanf(param_1, DAT_0051be2c, &local_1);
    if (iVar1 != 1) {
      return;
    }
  } while (local_1 != '{');
  return;
}
