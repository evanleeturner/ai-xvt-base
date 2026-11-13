/* SPDX-License-Identifier: MIT */
/* z_bop 00458750 */
void __cdecl FUN_00458750(int param_1)
{
  unsigned short uVar1;
  unsigned short uVar2;
  
  uVar1 = 0;
  do {
    uVar2 = uVar1 + 1;
    int local_var = 0;
    __builtin_memcpy((void *)(param_1 + 0x41e + (unsigned int)uVar1 * 4), &local_var, 4);
    uVar1 = uVar2;
  } while (uVar2 < 0x10);
  return;
}
