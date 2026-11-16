/* SPDX-License-Identifier: MIT */
/* z_xvt 00409ba0 */
extern int DAT_00740a80;
extern int DAT_0073b7e0;
extern int DAT_006d4980;
extern int DAT_0073b430;
extern int DAT_006d4a44;
extern int DAT_0073b650;

void __stdcall FUN_00409ba0(void)
{
  int iVar1;
  unsigned int uVar2;
  int *puVar3;
  
  puVar3 = &DAT_00740a80;
  for (iVar1 = 0x1a1; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar3 = 0;
    puVar3 = puVar3 + 1;
  }
  *(unsigned short *)puVar3 = 0;
  uVar2 = 0;
  do {
    puVar3 = (int *)((int)&DAT_0073b7e0 + uVar2);
    uVar2 = uVar2 + 0x61;
    for (iVar1 = 0x18; iVar1 != 0; iVar1 = iVar1 - 1) {
      int zero_val = 0;
      __builtin_memcpy(puVar3, &zero_val, 4);
      puVar3 = puVar3 + 1;
    }
    *(unsigned char *)puVar3 = 0;
  } while (uVar2 < 0x246);
  if (DAT_006d4980 != 0) {
    puVar3 = &DAT_0073b430;
    for (iVar1 = DAT_006d4980; iVar1 != 0; iVar1 = iVar1 - 1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
  }
  DAT_006d4a44 = 0;
  DAT_0073b650 = 0;
  return;
}
