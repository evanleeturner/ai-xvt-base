/* SPDX-License-Identifier: MIT */
/* z_xvt 00497f00 */
extern void __cdecl FUN_0042a140(uint param_1, uint param_2);
extern uint __cdecl FUN_004187e0(uint param_1, uint param_2);
extern uint __cdecl FUN_0042b250(int param_1);
extern uint __cdecl FUN_00478320(uint param_1, uint param_2);

extern short DAT_006d9d50;
extern int DAT_006d4988;
extern int DAT_006d9d56;
extern int DAT_00703ca0;
extern char s_disableldr1pln_00525028[];
extern int DAT_00732e54;
extern short DAT_00525aae;
extern int DAT_006db580;
extern short DAT_0070a1d2;

void __cdecl FUN_00497f00(int param_1)
{
  short sVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  short sVar7;
  uint uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  ushort uVar12;
  ushort uVar13;
  short sVar14;
  char *pcVar15;
  char *pcVar16;
  bool bVar17;
  int iVar6;

  uVar9 = (uint)DAT_006d9d50;
  iVar6 = param_1 * 0x23;
  if (*(short *)(*(int *)(DAT_006d4988 + 0x1f + iVar6) + 0x79) == 0) {
    uVar9 = 0;
  }
  else {
    FUN_0042a140(uVar9, (uint)*(ushort *)(DAT_006d9d56 + 0x18));
    bVar17 = 0;
    iVar10 = 0xf;
    pcVar15 = (char *)((int)&DAT_00703ca0 + (uint)*(unsigned char *)(DAT_006d9d56 + 0xb) * 0x55);
    pcVar16 = s_disableldr1pln_00525028;
    do {
      if (iVar10 == 0) break;
      iVar10 = iVar10 + -1;
      bVar17 = *pcVar15 == *pcVar16;
      pcVar15 = pcVar15 + 1;
      pcVar16 = pcVar16 + 1;
    } while (bVar17);
    if ((bVar17) && (*(char *)(DAT_006d9d56 + 0x1e) == '\x01')) {
      uVar8 = 0x8d;
    }
    else {
      uVar8 = (uint)*(unsigned char *)(DAT_00732e54 + 0x214);
      if (*(unsigned char *)(DAT_00732e54 + 0x214) < 0x8a) {
        uVar8 = 0x89;
      }
    }
    sVar14 = *(short *)(&DAT_00525aae + uVar8 * 2);
    iVar10 = uVar9 * 0x23 + DAT_006d4988;
    sVar1 = *(short *)(*(int *)(iVar10 + 0x1f) + 0x79);
    uVar13 = *(ushort *)(*(int *)(DAT_006d4988 + iVar6 + 0x1f) + 0x79);
    uVar12 = *(short *)(DAT_006d4988 + iVar6 + 0x10) - *(short *)(iVar10 + 0x10);
    if (0x7fff < uVar12) {
      uVar12 = -uVar12;
    }
    if (uVar12 < 0x4000) {
      uVar9 = FUN_004187e0(uVar13, uVar12);
      sVar7 = -(short)uVar9;
    }
    else {
      uVar9 = FUN_004187e0(uVar13, uVar12);
      sVar7 = (short)uVar9;
    }
    uVar13 = sVar14 + sVar1 + sVar7;
    uVar13 = uVar13 * 0x12 + uVar13 / 5;
    if (uVar13 == 0) {
      uVar13 = 0x13;
    }
    iVar10 = DAT_006db580 / (int)(uint)uVar13;
    sVar14 = (short)iVar10 * DAT_0070a1d2;
    uVar9 = FUN_0042b250(DAT_00732e54);
    uVar9 = FUN_00478320(((uint)(short)((uint)iVar10 >> 0x10) << 16) | (uint)sVar14, (ushort)uVar9);
  }
  iVar11 = DAT_006d4988 + iVar6;
  iVar10 = *(int *)(iVar11 + 0x1f);
  iVar2 = *(int *)(iVar11 + 8);
  uVar9 = uVar9 & 0xffff;
  iVar3 = *(int *)(iVar10 + 0xe);
  iVar4 = *(int *)(iVar10 + 10);
  iVar5 = *(int *)(iVar11 + 0xc);
  *(uint *)(DAT_006d9d56 + 0x1f) =
       (*(int *)(iVar11 + 4) - *(int *)(iVar10 + 6)) * uVar9 + *(int *)(iVar11 + 4);
  *(uint *)(DAT_006d9d56 + 0x23) = *(int *)(DAT_006d4988 + 8 + iVar6) + uVar9 * (iVar2 - iVar4);
  *(uint *)(DAT_006d9d56 + 0x27) = *(int *)(DAT_006d4988 + 0xc + iVar6) + uVar9 * (iVar5 - iVar3);
  return;
}
