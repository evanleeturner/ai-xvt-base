/* SPDX-License-Identifier: MIT */
/* z_bop 00419620 */
extern unsigned int DAT_00550b8c;
extern unsigned int DAT_00550bd8;
extern unsigned int *DAT_00550b88;
extern unsigned int *DAT_00550880;

void __stdcall FUN_00419620(void)
{
  unsigned int uVar1;
  unsigned int *puVar2;
  unsigned int *puVar3;
  
  DAT_00550b8c = DAT_00550bd8;
  puVar2 = DAT_00550b88;
  puVar3 = DAT_00550880;
  for (uVar1 = DAT_00550bd8 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar3 = *puVar2;
    puVar2 = puVar2 + 1;
    puVar3 = puVar3 + 1;
  }
  for (uVar1 = DAT_00550b8c & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(unsigned char *)puVar3 = *(unsigned char *)puVar2;
    puVar2 = (unsigned int *)((int)puVar2 + 1);
    puVar3 = (unsigned int *)((int)puVar3 + 1);
  }
  return;
}
