/* SPDX-License-Identifier: MIT */
/* z_bop 004305d0 */
unsigned short __cdecl FUN_00425d10(void);

unsigned short __cdecl FUN_004305d0(unsigned short param_1)
{
  unsigned short uVar1;
  
  uVar1 = FUN_00425d10();
  if ((unsigned short)((unsigned int)uVar1 / (unsigned int)param_1) == 0) {
    return 0;
  }
  return (unsigned short)((unsigned int)uVar1 % ((unsigned int)uVar1 / (unsigned int)param_1));
}
