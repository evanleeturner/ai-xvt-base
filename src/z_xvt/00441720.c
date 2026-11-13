/* SPDX-License-Identifier: MIT */
/* z_xvt 00441720 */
extern uint DAT_0071b730;
extern int DAT_006d4988;
extern short __cdecl FUN_00446fa0(uint param_1);

void __cdecl FUN_00441720(uint param_1, uint param_2, int param_3)
{
  uint *puVar2 = (uint *)((int)&DAT_0071b730 + param_3 * 0x5bd);
  int iVar1 = (param_2 & 0xffff) * 2;
  
  if (0 < *(short *)(*(int *)(*(int *)(*puVar2 * 0x23 + 0x1f + DAT_006d4988) + 0xa7) + 0x20b + iVar1)) {
    short sVar7 = FUN_00446fa0(*puVar2);
    int iVar6 = *(int *)(*(int *)(*puVar2 * 0x23 + 0x1f + DAT_006d4988) + 0xa7);
    short *psVar3 = (short *)(iVar6 + 0x20b + (param_1 & 0xffff) * 2);
    short sVar4 = *psVar3;
    sVar7 = sVar7 - sVar4;
    if (0 < sVar7) {
      short sVar5 = *(short *)(iVar1 + 0x20b + iVar6);
      if (sVar7 < sVar5) {
        *psVar3 = sVar4 + sVar7;
        psVar3 = (short *)(*(int *)(*(int *)(*puVar2 * 0x23 + 0x1f + DAT_006d4988) + 0xa7) + 0x20b + iVar1);
        *psVar3 = *psVar3 - sVar7;
        return;
      }
      *psVar3 = sVar4 + sVar5;
      *(short *)(*(int *)(*(int *)(*puVar2 * 0x23 + 0x1f + DAT_006d4988) + 0xa7) + 0x20b + iVar1) = 0;
    }
  }
  return;
}
