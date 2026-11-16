/* SPDX-License-Identifier: MIT */
/* z_bop 00446700 */
extern int DAT_009a1fe8;
extern int DAT_009a1fe0;
extern int DAT_009a1fd0;
extern int DAT_009a1fd4;
extern int DAT_009a1fd8;
extern int DAT_009d80c0;
extern int DAT_00a07bdc;
extern int DAT_009d8c18;
extern int DAT_00a07c54;
extern int DAT_009d1150;
extern int DAT_00a07c58;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009ec604;
extern int DAT_00a081e8;
extern int DAT_00a08298;
extern unsigned char DAT_00521f1c[];

uint __cdecl FUN_00458e20(uint a, int b);
uint __cdecl FUN_0041f3d0(uint a, uint b, uint c);
uint __cdecl FUN_0041c570(int a);
uint __cdecl FUN_004a5490(ushort a, ushort b);

uint __cdecl FUN_00446700(ushort param_1, ushort param_2)
{
  char cVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  ushort uVar6;
  uint uVar5;
  int iVar7;
  uint uVar8;
  int iVar9;
  uint uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int tmpInt;
  ushort tmpShort;

  __builtin_memcpy(&tmpInt, (char *)((uint)param_1 * 0x23u + 0x1fu + (uint)DAT_009a1fe8), 4);
  __builtin_memcpy(&tmpShort, (char *)((uint)tmpInt + 0x85u), 2);
  uVar4 = (uint)tmpShort;
  uVar5 = (uint)param_2;
  if (uVar5 <= uVar4) {
    return (uint)DAT_009a1fe8 & 0xffff0000u;
  }
  iVar7 = (int)((uint)DAT_009a1fe8 + uVar5 * 0x23u);
  cVar1 = *(char *)((uint)iVar7 + 2u);
  if (cVar1 == '\x0e') {
    return 0;
  }
  if (cVar1 == '\v') {
    return 0;
  }
  if ((cVar1 != '\n') && ((int)uVar4 < DAT_009a1fe0)) {
    iVar3 = (int)(uVar4 * 0x23u + (uint)DAT_009a1fe8);
    __builtin_memcpy(&tmpInt, (char *)((uint)iVar3 + 0x1bu), 4);
    if (tmpInt == -1) {
      __builtin_memcpy(&uVar4, (char *)((uint)iVar3 + 0x1fu), 4);
      __builtin_memcpy(&tmpInt, (char *)(uVar4 + 0xa9u), 4);
      __builtin_memcpy(&tmpShort, (char *)((uint)tmpInt + 0x42u), 2);
      if (tmpShort != param_2) {
        return uVar4 & 0xffff0000u;
      }
    }
  }
  bVar2 = *(byte *)((uint)iVar7 + 3u);
  FUN_00458e20(uVar5, 0);
  DAT_009d80c0 = DAT_00a07cdc;
  DAT_00a07bdc = DAT_00a07cdc;
  DAT_009d8c18 = DAT_00a07cd8;
  DAT_00a07c54 = DAT_00a07cd8;
  DAT_009d1150 = DAT_00a07cd4;
  DAT_00a07c58 = DAT_00a07cd4;
  uVar4 = (uint)DAT_009a1fd4 - (uint)DAT_00a07cdc;
  __builtin_memcpy(&iVar7, &DAT_00521f1c[(uint)bVar2 * 0x1au], 4);
  if ((int)uVar4 < 0) {
    uVar4 = 0u - uVar4;
  }
  if (0x20000 < (int)uVar4) {
    return uVar4 & 0xffff0000u;
  }
  uVar8 = (uint)DAT_009a1fd8 - (uint)DAT_00a07cd8;
  if ((int)uVar8 < 0) {
    uVar8 = 0u - uVar8;
  }
  uVar6 = (ushort)(uVar4 >> 0x10);
  if (0x20000 < (int)uVar8) {
    return (uint)uVar6 << 0x10;
  }
  uVar10 = (uint)DAT_009a1fd0 - (uint)DAT_00a07cd4;
  if ((int)uVar10 < 0) {
    uVar10 = 0u - uVar10;
  }
  if (0x20000 < (int)uVar10) {
    return (uint)uVar6 << 0x10;
  }
  uVar4 = FUN_0041f3d0(uVar4, uVar8, uVar10);
  if (0x20000 < (int)uVar4) {
    return uVar4 & 0xffff0000u;
  }
  iVar3 = (int)((uint)DAT_009a1fd4 - (uint)DAT_009ec604);
  if (iVar3 < 0) {
    iVar3 = (int)(0u - (uint)iVar3);
  }
  iVar9 = (int)((uint)DAT_009a1fd8 - (uint)DAT_00a081e8);
  if (iVar9 < 0) {
    iVar9 = (int)(0u - (uint)iVar9);
  }
  iVar11 = (int)((uint)DAT_009a1fd0 - (uint)DAT_00a08298);
  if (iVar11 < 0) {
    iVar11 = (int)(0u - (uint)iVar11);
  }
  iVar12 = (int)((uint)DAT_00a07bdc - (uint)DAT_009d80c0);
  if (iVar12 < 0) {
    iVar12 = (int)(0u - (uint)iVar12);
  }
  iVar13 = (int)((uint)DAT_00a07c54 - (uint)DAT_009d8c18);
  if (iVar13 < 0) {
    iVar13 = (int)(0u - (uint)iVar13);
  }
  iVar14 = (int)((uint)DAT_00a07c58 - (uint)DAT_009d1150);
  if (iVar14 < 0) {
    iVar14 = (int)(0u - (uint)iVar14);
  }
  uVar8 = FUN_0041f3d0((uint)iVar3 + (uint)iVar12 + (uint)iVar7,
                       (uint)iVar9 + (uint)iVar13 + (uint)iVar7,
                       (uint)iVar11 + (uint)iVar14 + (uint)iVar7);
  if (uVar8 < uVar4) {
    return uVar8 & 0xffff0000u;
  }
  if ((iVar7 < 0x447) &&
      (*(char *)((uint)DAT_009a1fe8 + 3u + uVar5 * 0x23u) != ':')) {
    uVar5 = FUN_0041c570((int)((uint)(iVar7 >> 3) + (uint)(iVar7 >> 2)));
    return uVar5;
  }
  uVar5 = FUN_004a5490(param_1, param_2);
  return uVar5;
}
