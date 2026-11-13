/* SPDX-License-Identifier: MIT */
/* z_bop 00451d00 */
extern uint DAT_009a1fe8;
extern uint DAT_009d8c87;
extern uint DAT_009d8c88;
extern uint DAT_009d8c30;
extern uint DAT_0051c560;
extern uint DAT_009e967a;
extern uint DAT_009e967c;
extern uint DAT_009ecc34;
extern ushort DAT_00a08292;
extern short DAT_00a07be2;
extern short DAT_00a07be4;
extern short DAT_00a07be6;

void __cdecl FUN_00451bf0(int, int);
undefined4 __cdecl FUN_0043c740(int, undefined4 *);
void __cdecl FUN_00450650(int, int);
void __cdecl FUN_0042fb70(int, int, int, int, int, int);

void __cdecl FUN_00451d00(uint param_1, undefined4 *param_2, short param_3, undefined4 param_4, short param_5, uint param_6)
{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;

  uVar3 = param_1 & 0xffff;
  uVar2 = (uint)*(byte *)(uVar3 * 0x23 + 0x16 + &DAT_009a1fe8);
  iVar4 = uVar2 * 0x566;
  DAT_00a08292 = (ushort)(byte)(&DAT_009d8c87)[uVar2 * 0x566];
  if (*(ushort *)((int)&DAT_009e967a + DAT_009ecc34 * 0x5bd) == DAT_00a08292 &&
      (ushort)(byte)(&DAT_009d8c88)[iVar4] == *(ushort *)((int)&DAT_009e967c + DAT_009ecc34 * 0x5bd)) {
    if (param_5 == 0) {
      FUN_00451bf0(0, (int)&DAT_0051c560 + (uint)*(byte *)(param_2 + 1) * 0x101);
      FUN_00451bf0(1, (int)&DAT_009d8c30 + iVar4);
      uVar1 = FUN_0043c740(uVar2, param_2);
      DAT_00a07be4 = (short)uVar1;
      if (DAT_00a07be4 == 0) {
        DAT_00a07be4 = param_3;
        iVar4 = 0x94;
      }
      else {
        DAT_00a07be6 = param_3;
        iVar4 = 0x93;
      }
    }
    else {
      FUN_00451bf0(0, (int)&DAT_009d8c30 + iVar4);
      DAT_00a07be2 = param_3;
      iVar4 = 0x10d;
    }
    FUN_00450650(iVar4, DAT_009ecc34);
    FUN_0042fb70(DAT_009ecc34, uVar3, 1, param_4 & 0xffff, uVar3, 0xffff);
  }
  return;
}
