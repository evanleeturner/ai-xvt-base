/* SPDX-License-Identifier: MIT */
/* z_xvt 004d8fb0 */
extern undefined4 DAT_0064f688;
extern undefined4 DAT_0064f708;
extern undefined4 DAT_00532ae0;
extern undefined4 DAT_00532adc;

undefined4 __stdcall FUN_004d8fb0(void)
{
  int iVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  
  puVar2 = &DAT_0064f688;
  puVar3 = &DAT_0064f708;
  for (iVar1 = 0x20; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  DAT_00532ae0 = DAT_00532adc;
  return 1;
}
