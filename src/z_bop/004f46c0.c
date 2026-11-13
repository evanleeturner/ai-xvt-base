/* SPDX-License-Identifier: MIT */
/* z_bop 004f46c0 */
extern unsigned int DAT_00aa5cb0;
extern unsigned int DAT_00aa5df0;
extern unsigned int DAT_00b69e30;
extern unsigned int DAT_00aa6af0;

extern unsigned int __cdecl FUN_004cef70(unsigned int param_1, unsigned int *param_2, unsigned int param_3);

unsigned int __stdcall FUN_004f46c0(void)
{
  int iVar1;
  unsigned int *puVar2;
  
  puVar2 = &DAT_00aa5cb0;
  for (iVar1 = 0x50; iVar1 != 0; iVar1--) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  puVar2 = &DAT_00aa5df0;
  for (iVar1 = 8; iVar1 != 0; iVar1--) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  if (DAT_00b69e30 != 2) {
    DAT_00aa6af0 = 0x54;
    FUN_004cef70(0, (unsigned int *)&DAT_00aa6af0, 4);
  }
  return 1;
}
