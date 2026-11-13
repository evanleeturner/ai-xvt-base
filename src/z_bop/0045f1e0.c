/* SPDX-License-Identifier: MIT */
/* z_bop 0045f1e0 */
extern uint DAT_009a1fe8;
extern int DAT_009a1fe0;
extern uint DAT_00a00870[];
extern short DAT_00a00874[];
extern ushort DAT_009a73b0;
extern int extraout_ECX;

extern int FUN_004adc40(int param_1);
extern uint FUN_004ae2a0(int param_1, int param_2);
extern uint FUN_004836e0(uint param_1, uint param_2, int param_3);
extern ushort FUN_00459b10(ushort param_1);

uint __cdecl FUN_0045f1e0(uint param_1)
{
  byte bVar1;
  byte bVar2;
  short sVar3;
  ushort uVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  ushort local_44;
  ushort local_42;
  ushort uStack_40;
  ushort uStack_3e;
  uint local_3c;
  byte local_34[52];

  local_44 = 0;
  local_3c = 0x1000000;
  uVar10 = param_1 & 0xffff;
  local_42 = 0;
  local_34[0] = 0;
  bVar1 = *(byte *)(DAT_009a1fe8 + 3 + uVar10 * 0x23);
  if (DAT_009a1fe0 <= (int)uVar10) {
    return (uVar10 * 7) & 0xffff0000;
  }
  if (bVar1 < 0x49) {
    uStack_40 = (ushort)DAT_00a00870[(uint)bVar1 * 0x65];
    uStack_3e = (ushort)(DAT_00a00870[(uint)bVar1 * 0x65] >> 0x10);
  }
  else {
    iVar7 = FUN_004adc40((uint)bVar1);
    uStack_40 = (ushort)iVar7;
    uStack_3e = (ushort)(iVar7 >> 0x10);
  }
  uVar9 = 0;
  if (0 < ((uStack_3e << 16) | uStack_40)) {
    do {
      bVar2 = *(byte *)(DAT_009a1fe8 + 3 + uVar10 * 0x23);
      uVar6 = (uint)bVar2;
      if (bVar2 < 0x49) {
        uVar8 = uVar9;
        if ((int)DAT_00a00870[uVar6 * 0x65] <= (int)uVar9) {
          uVar8 = DAT_00a00870[uVar6 * 0x65] - 1;
        }
        iVar7 = uVar6 * 0x65 + uVar8;
        sVar3 = (short)DAT_00a00874[iVar7];
      }
      else {
        uVar5 = FUN_004ae2a0(uVar6, uVar9);
        sVar3 = (short)uVar5;
        iVar7 = extraout_ECX;
      }
      if ((sVar3 == 1) || (sVar3 == 3)) {
        if (bVar1 != 0x36) {
          local_34[local_42] = (byte)uVar9;
          local_42 = local_42 + 1;
        }
        else {
          uVar6 = FUN_004836e0((iVar7 & 0xffff0000) | DAT_009a73b0, param_1, uVar9);
          if (uVar6 < local_3c) {
            local_44 = (ushort)uVar9;
            local_3c = uVar6;
          }
        }
      }
      uVar9 = (uint)(ushort)((ushort)uVar9 + 1);
    } while ((int)uVar9 < ((uStack_3e << 16) | uStack_40));
  }
  if (bVar1 != 0x36) {
    uVar4 = FUN_00459b10(local_42);
    return (uint)local_34[uVar4];
  }
  return (uint)local_44;
}
