/* SPDX-License-Identifier: MIT */
/* z_xvt 004df890 */
extern ushort DAT_0075dc4e;
extern char DAT_0075965b;
extern int DAT_0075dc4a;
extern int DAT_0075dc46;

uint __cdecl FUN_004df890(uint param_1)
{
  ushort uVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  int iVar5;
  
  uVar1 = *(ushort *)((int)&DAT_0075dc4e + param_1 * 2);
  if (uVar1 != 0) {
    return (uint)uVar1;
  }
  if (DAT_0075965b == 0) {
    uVar4 = param_1 & 0x7e0;
    uVar3 = (param_1 & 0xf800) >> 0xb;
  }
  else {
    uVar4 = param_1 & 0x3e0;
    uVar3 = (param_1 & 0x7c00) >> 10;
  }
  iVar5 = DAT_0075dc4a - DAT_0075dc46;
  uVar4 = ((uVar4 >> 5) * iVar5) / DAT_0075dc4a;
  uVar3 = (uVar3 * iVar5) / DAT_0075dc4a;
  if (DAT_0075965b == 0) {
    uVar4 = uVar4 & 0x3f;
    iVar2 = (uVar3 & 0x1f) << 6;
  }
  else {
    uVar4 = uVar4 & 0x1f;
    iVar2 = (uVar3 & 0x1f) << 5;
  }
  uVar3 = (iVar2 + uVar4) * 0x20 + ((iVar5 * (param_1 & 0x1f)) / DAT_0075dc4a & 0x1f);
  if (uVar3 == 0) {
    uVar3 = 1;
  }
  *(short *)((int)&DAT_0075dc4e + param_1 * 2) = (short)uVar3;
  return uVar3;
}
