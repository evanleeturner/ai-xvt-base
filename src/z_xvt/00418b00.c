/* SPDX-License-Identifier: MIT */
/* z_xvt 00418b00 */
extern unsigned short DAT_00523c60[];
extern unsigned int DAT_007091a4;
extern unsigned short DAT_007329b8;
extern unsigned int DAT_006db794;
extern unsigned int DAT_0071b70c;
extern unsigned int DAT_007325d0;
extern unsigned short DAT_00703c30;
extern unsigned int DAT_006d4a50;
extern unsigned int DAT_006ffbec;
extern unsigned int DAT_0072f4e8;

void __cdecl FUN_00418b00(uint param_1, short param_2, ushort param_3)
{
  ushort uVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  uint local_4;
  uint uVar5;

  DAT_007091a4 = param_1 & 0xffff;
  DAT_007329b8 = (ushort)(0x4000u - (uint)(int)param_2);
  uVar3 = (DAT_007091a4 * (uint)DAT_00523c60[(uint)(((uint)param_3 >> 5 & 0x3fe) >> 1)] + 0x8000u) >> 0x10;
  if ((param_3 & 0x8000) != 0) {
    uVar3 = 0u - uVar3;
  }
  bVar2 = 0;
  uVar4 = uVar3;
  if ((int)uVar3 < 0) {
    uVar4 = 0u - uVar3;
    bVar2 = 0x80;
  }
  uVar1 = (ushort)(0x8000u - (uint)(int)param_2);
  local_4 = (uint)DAT_00523c60[(uint)(((uint)uVar1 >> 5 & 0x3fe) >> 1)];
  DAT_006db794 = (uVar4 >> 0x10) * local_4 + (((uVar4 & 0xffff) * local_4 + 0x8000u) >> 0x10);
  if (((bVar2 ^ (byte)(uVar1 >> 8)) & 0x80) != 0) {
    DAT_006db794 = 0u - DAT_006db794;
  }
  bVar2 = 0;
  if ((int)uVar3 < 0) {
    uVar3 = 0u - uVar3;
    bVar2 = 0x80;
  }
  uVar5 = (uint)DAT_00523c60[(uint)(((uint)DAT_007329b8 >> 5 & 0x3fe) >> 1)];
  DAT_0071b70c = (uVar3 >> 0x10) * uVar5 + (((uVar3 & 0xffff) * uVar5 + 0x8000u) >> 0x10);
  if (((bVar2 ^ (byte)(DAT_007329b8 >> 8)) & 0x80) != 0) {
    DAT_0071b70c = 0u - DAT_0071b70c;
  }
  DAT_00703c30 = param_3;
  DAT_007325d0 = (DAT_007091a4 *
                  (uint)DAT_00523c60[(uint)((((uint)(ushort)(param_3 + 0x4000) >> 5) & 0x3fe) >> 1)] +
                  0x8000u) >> 0x10;
  if (((param_3 + 0x4000) & 0x8000) != 0) {
    DAT_007325d0 = 0u - DAT_007325d0;
  }
  DAT_006d4a50 = DAT_006db794;
  DAT_006ffbec = DAT_0071b70c;
  DAT_0072f4e8 = DAT_007325d0;
  return;
}
