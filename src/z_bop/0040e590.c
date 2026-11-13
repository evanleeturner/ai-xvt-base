/* SPDX-License-Identifier: MIT */
/* z_bop 0040e590 */
extern int DAT_005233d8;
extern char DAT_00a081f4;

void __cdecl FUN_0040e1b0(int param_1, int param_2, int param_3, int param_4);
void __cdecl FUN_004ab890(int param_1, int param_2, int param_3);

void __stdcall FUN_0040e590(void)
{
  char local_300[768];
  
  FUN_0040e1b0(0x9a7bc0, (int)local_300, 0, 0x100);
  if (DAT_005233d8 != 1) {
    FUN_004ab890((int)local_300, 0, 0x100);
  }
  DAT_00a081f4 = DAT_00a081f4 & 0xfe;
  return;
}
