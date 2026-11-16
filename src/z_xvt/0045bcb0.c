/* SPDX-License-Identifier: MIT */
/* z_xvt 0045bcb0 */
extern int DAT_005f0fa4;
extern uint DAT_006a8a60[];
extern uint DAT_006a9a88[];
extern uint DAT_006cca88;
extern char DAT_006a9a80[];
extern char s_0_RAN_OUT_OF_MYCACHETEXTURES____00527a7f[];
void __cdecl FUN_0048fb30(char *);

void * __cdecl FUN_0045bcb0(uint param_1)
{
  int iVar1;
  uint *puVar2;
  uint uVar3;
  uint *puVar4;
  
  if (DAT_005f0fa4 == -1) {
    puVar2 = DAT_006a9a88;
    puVar4 = DAT_006a8a60;
    for (iVar1 = 0x400; iVar1 != 0; iVar1 = iVar1 - 1) {
      *puVar4 = 0;
      puVar4 = puVar4 + 1;
    }
    do {
      *puVar2 = 0;
      puVar2 = puVar2 + 0x23;
    } while (puVar2 < (uint *)&DAT_006cca88);
  }
  iVar1 = 0;
  DAT_005f0fa4 = param_1 & 0x3ff;
  uVar3 = DAT_005f0fa4;
  do {
    if (DAT_006a8a60[uVar3] == param_1) break;
    uVar3 = uVar3 + 1;
    if (uVar3 == 0x400) {
      uVar3 = 0;
    }
    iVar1 = iVar1 + 1;
  } while (iVar1 < 0x400);
  if (iVar1 == 0x400) {
    iVar1 = 0;
    do {
      if (DAT_006a9a88[DAT_005f0fa4 * 0x23] == 0) break;
      DAT_005f0fa4 = DAT_005f0fa4 + 1;
      if (DAT_005f0fa4 == 0x400) {
        DAT_005f0fa4 = 0;
      }
      iVar1 = iVar1 + 1;
    } while (iVar1 < 0x400);
    uVar3 = DAT_005f0fa4;
    if (iVar1 == 0x400) {
      FUN_0048fb30(s_0_RAN_OUT_OF_MYCACHETEXTURES____00527a7f + 1);
      return (void *)(DAT_006a9a80 + DAT_005f0fa4 * 0x8c);
    }
  }
  DAT_005f0fa4 = uVar3;
  DAT_006a8a60[uVar3] = param_1;
  return (void *)(DAT_006a9a80 + uVar3 * 0x8c);
}
