/* SPDX-License-Identifier: MIT */
/* z_xvt 0042b2a0 */
extern unsigned char DAT_00524798[];
extern unsigned char DAT_0070ad92[];
extern unsigned char DAT_006d4988[];
extern short DAT_006d9d62;

void __cdecl FUN_004296b0(ushort param_1);
undefined4 __cdecl FUN_0042af80(undefined2 param_1);

undefined4 __cdecl FUN_0042b2a0(int param_1, uint param_2, undefined2 param_3)
{
  undefined4 uVar1;
  short sVar2;
  uint uVar3;
  
  if (param_1 == -1) {
    return 0;
  }
  sVar2 = 0;
  uVar3 = 0;
  FUN_004296b0((ushort)param_1);
  do {
    if (DAT_00524798[DAT_0070ad92[(uint)(unsigned char)DAT_006d4988[0x16 + param_1 * 0x23] * 0x566 + uVar3]] == param_2) {
      DAT_006d9d62 = sVar2;
      uVar1 = FUN_0042af80(param_3);
      if ((short)uVar1 != 0) {
        return 1;
      }
    }
    uVar3 = uVar3 + 0x52;
    sVar2 = sVar2 + 1;
  } while (uVar3 < 0xf6);
  return 0;
}
