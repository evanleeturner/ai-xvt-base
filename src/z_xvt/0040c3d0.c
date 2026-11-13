/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c3d0 */
extern uint FUN_004784d0(void);
extern byte DAT_00523a80[];
extern byte DAT_00523a68[];
extern byte DAT_00523a98[];
extern byte DAT_0073b7e0[];

uint __cdecl FUN_0040c3d0(int param_1, int param_2)
{
  byte bVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  
  uVar3 = (uint)(byte)DAT_00523a80[param_1];
  bVar1 = DAT_00523a68[param_1];
  uVar4 = FUN_004784d0();
  uVar5 = (uVar4 & 0xffff) / uVar3;
  if ((short)uVar5 == 0) {
    uVar5 = 0;
  }
  else {
    uVar5 = (uVar4 & 0xffff) % uVar5;
  }
  bVar2 = DAT_00523a98[param_1];
  if ((bVar2 != 0) &&
     (uVar4 = uVar3, bVar2 <= DAT_0073b7e0[bVar1 + uVar5 + param_2 * 0x61]))
  {
    while (uVar4 != 0) {
      uVar5 = uVar5 + 1;
      if ((int)uVar3 <= (int)uVar5) {
        uVar5 = 0;
      }
      uVar4 = uVar4 - 1;
      if (DAT_0073b7e0[bVar1 + uVar5 + param_2 * 0x61] < bVar2) {
        return uVar5;
      }
    }
    uVar5 = 0xffffffff;
  }
  return uVar5;
}
