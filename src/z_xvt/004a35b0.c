/* SPDX-License-Identifier: MIT */
/* z_xvt 004a35b0 */
extern unsigned short DAT_00732fe2;
extern unsigned short DAT_0071b73a;
extern unsigned int DAT_0071ecf4;
extern unsigned char DAT_0070ad47;
extern unsigned char DAT_0070ad48;
extern unsigned short DAT_0071b73c;
extern unsigned short _DAT_00732950;
extern int _DAT_00732e74;
extern short _DAT_00732952;
extern unsigned char DAT_0052b070;
extern unsigned int DAT_006d4988;
extern short _DAT_00732954;
extern short DAT_00732956;
extern void *_DAT_00732e70;
extern unsigned char DAT_0070acf0;

undefined4 __cdecl FUN_004ac6f0(int param_1, undefined4 *param_2);
void __cdecl FUN_004a1d70(int param_1, uint param_2);
void __cdecl FUN_0040b730(uint param_1, uint param_2, uint param_3, uint param_4, uint param_5, uint param_6);

void __cdecl FUN_004a35b0(uint param_1, undefined4 *param_2, short param_3, undefined4 param_4, short param_5, uint param_6)
{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;

  uVar3 = param_1 & 0xffff;
  uVar2 = (uint)*(byte *)(uVar3 * 0x23 + 0x16 + DAT_006d4988);
  iVar4 = uVar2 * 0x566;
  DAT_00732fe2 = (ushort)(byte)(&DAT_0070ad47)[uVar2 * 0x566];
  if ((*(ushort *)((int)&DAT_0071b73a + DAT_0071ecf4 * 0x5bd) == DAT_00732fe2) &&
     ((ushort)(byte)(&DAT_0070ad48)[iVar4] == *(ushort *)((int)&DAT_0071b73c + DAT_0071ecf4 * 0x5bd))) {
    if (param_5 == 0) {
      _DAT_00732950 = 0x8000;
      _DAT_00732e74 = (int)&DAT_0070acf0 + iVar4;
      _DAT_00732952 = -0x7fff;
      _DAT_00732e70 = &DAT_0052b070 + (uint)*(byte *)(param_2 + 1) * 0xef;
      uVar1 = FUN_004ac6f0(uVar2, param_2);
      _DAT_00732954 = (short)uVar1;
      if (_DAT_00732954 == 0) {
        _DAT_00732954 = param_3;
        iVar4 = 0x93;
      }
      else {
        DAT_00732956 = param_3;
        iVar4 = 0x92;
      }
    }
    else {
      _DAT_00732e70 = (undefined *)((int)&DAT_0070acf0 + iVar4);
      _DAT_00732952 = param_3;
      _DAT_00732950 = 0x8000;
      iVar4 = 0x109;
    }
    FUN_004a1d70(iVar4, DAT_0071ecf4);
    FUN_0040b730(DAT_0071ecf4, uVar3, 1, param_4 & 0xffff, uVar3, 0xffff);
  }
  return;
}
