/* SPDX-License-Identifier: MIT */
/* z_bop 00413c60 */
int _fgetc(void *);

void __cdecl FUN_00413c60(void *param_1)
{
  int iVar1;
  
  do {
    iVar1 = _fgetc(param_1);
  } while ((char)iVar1 != '\n');
  return;
}
