/* SPDX-License-Identifier: MIT */
/* z_xvt 004de0c0 */
void __cdecl FUN_004de0e0(char *param_1);
extern int DAT_00759af4;

void __stdcall FUN_004de0c0(void)
{
  int iVar1;
  
  iVar1 = 0xa13b;
  do {
    FUN_004de0e0((char *)(DAT_00759af4 + iVar1));
    iVar1 = iVar1 + -0x145;
  } while (-1 < iVar1);
  return;
}
