/* SPDX-License-Identifier: MIT */
/* z_bop 00419570 */
extern unsigned int DAT_00550880;

void __cdecl FUN_00419570(unsigned int *param_1, int param_2, unsigned int param_3)
{
  unsigned int uVar1;
  unsigned int *puVar2;
  
  puVar2 = (unsigned int *)(DAT_00550880 + param_2);
  for (uVar1 = param_3 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = *param_1;
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 1;
  }
  for (uVar1 = param_3 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(unsigned char *)puVar2 = *(unsigned char *)param_1;
    param_1 = (unsigned int *)((int)param_1 + 1);
    puVar2 = (unsigned int *)((int)puVar2 + 1);
  }
  return;
}
