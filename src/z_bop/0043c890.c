/* SPDX-License-Identifier: MIT */
/* z_bop 0043c890 */
void __cdecl FUN_00440840(int, int, int);
extern unsigned short DAT_00a08374;

void __cdecl FUN_0043c890(int param_1)
{
  unsigned short uVar1;
  
  uVar1 = (unsigned short)((unsigned int)(param_1 * 0xa1) >> 0x10);
  if (9999 < uVar1) {
    uVar1 = 9999;
  }
  FUN_00440840(DAT_00a08374 + 0x53, uVar1 / 100, 1);
  FUN_00440840(DAT_00a08374 + 0x54, uVar1 % 100, 2);
  return;
}
