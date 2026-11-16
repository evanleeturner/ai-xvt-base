/* SPDX-License-Identifier: MIT */
/* z_xvt 004f0db0 */
extern unsigned int DAT_00758620;
extern unsigned int DAT_00757870;
extern unsigned int DAT_00758624;
extern void __cdecl FUN_004cb830(unsigned int, unsigned int *, unsigned int);

unsigned int __stdcall FUN_004f0db0(void)
{
  unsigned int iVar1;
  unsigned int *puVar2;
  unsigned int *puVar3;
  
  DAT_00758620 = 0x48;
  puVar2 = &DAT_00757870;
  puVar3 = &DAT_00758624;
  for (iVar1 = 0x50; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  FUN_004cb830(0, (unsigned int *)&DAT_00758620, 0x144);
  return 1;
}
