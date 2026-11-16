/* SPDX-License-Identifier: MIT */
/* z_xvt 004629c0 */
extern byte DAT_006da540[];

void __cdecl FUN_004629c0(int param_1, uint param_2, uint param_3)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  byte *pbVar6;
  int local_14;
  uint local_10;
  
  local_14 = 0;
  do {
    local_10 = param_2;
    uVar5 = local_10;
    iVar1 = 0x7fffffff;
    if (param_2 < param_3) {
      pbVar6 = DAT_006da540 + param_2 * 3;
      do {
        iVar2 = (uint)(byte)(((byte)((uint)(local_14 >> 5) >> 6) & 0x1f) * 2) - (uint)*pbVar6;
        iVar3 = (uint)((byte)(local_14 >> 5) & 0x3f) - (uint)pbVar6[1];
        iVar4 = (uint)(byte)(((byte)local_14 & 0x1f) * 2) - (uint)pbVar6[2];
        iVar2 = iVar4 * iVar4 + iVar3 * iVar3 + iVar2 * iVar2;
        if (iVar2 < iVar1) {
          iVar1 = iVar2;
          local_10 = uVar5;
        }
        pbVar6 = pbVar6 + 3;
        uVar5 = uVar5 + 1;
      } while (uVar5 < param_3);
    }
    local_14 = local_14 + 1;
    *(byte *)(param_1 + local_14 - 1) = (byte)local_10;
  } while (local_14 < 0x10000);
  return;
}
