/* SPDX-License-Identifier: MIT */
/* z_bop 00438330 */
extern unsigned char DAT_009e9a6b;
extern unsigned char DAT_009e9a6d;
extern unsigned char DAT_009e9a6f;
extern int DAT_00a07c68;
extern int DAT_00a07c6c;
extern int DAT_00a07cc4;
extern int DAT_00a004d0;
extern int DAT_00a004ac;
extern int DAT_00a004b4;
extern int DAT_00a0049c;
extern int DAT_00a0048c;
extern int DAT_00a00494;
extern int DAT_009d12e8;
extern int DAT_009d1264;
extern int DAT_009d1154;
extern int DAT_009a8e20;
extern int DAT_009a8e1c;
extern int DAT_009a8e28;
extern int DAT_009a8d80;
extern int DAT_009a8d60;
extern int DAT_009a8d8c;
extern unsigned short DAT_009ec474;

void __cdecl FUN_00427940(short, short, short, short, short, short, int);
void __cdecl FUN_004290b0(int, int, int, short);
short __cdecl FUN_0046a460(short);
int __cdecl FUN_0046abf0(int, int);
long long __cdecl FUN_0046a6d0(short);
long long __cdecl FUN_0046a3c0(short);

void __cdecl FUN_00438330(short param_1, short param_2, int param_3)
{
  int iVar1;
  short sVar2;
  short sVar3;
  int uVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  long long lVar14;
  long long lVar15;
  long long lVar16;
  short sStack_68;
  short sStack_64;
  short sStack_60;
  short sStack_5c;
  short sStack_58;
  short sStack_54;
  short sStack_50;
  short sStack_4c;
  short sStack_48;
  short sStack_44;
  short sStack_3c;
  short sStack_2c;
  short sStack_20;
  short tmpA;
  short tmpB;
  short tmpC;

  iVar1 = (int)((unsigned int)param_3 * 0x5bdu);
  __builtin_memcpy(&tmpA, &DAT_009e9a6f + iVar1, 2);
  __builtin_memcpy(&tmpB, &DAT_009e9a6b + iVar1, 2);
  __builtin_memcpy(&tmpC, &DAT_009e9a6d + iVar1, 2);
  FUN_00427940(tmpA, tmpB, tmpC, 0, 0, 0, 0);
  DAT_00a07c68 = (int)(0u - (unsigned int)DAT_009d12e8);
  DAT_00a07c6c = (int)(0u - (unsigned int)DAT_009d1264);
  DAT_00a004d0 = DAT_009a8e20;
  DAT_00a004ac = DAT_009a8e1c;
  DAT_00a07cc4 = (int)(0u - (unsigned int)DAT_009d1154);
  DAT_00a004b4 = DAT_009a8e28;
  DAT_00a0049c = DAT_009a8d80;
  DAT_00a0048c = DAT_009a8d60;
  DAT_00a00494 = DAT_009a8d8c;
  FUN_004290b0(DAT_009a8d80, DAT_009a8d60, DAT_009a8d8c, param_1);
  if (((unsigned char)DAT_009ec474 & 0xe) != 2) {
    FUN_004290b0(DAT_00a004d0, DAT_00a004ac, DAT_00a004b4, param_2);
  }
  sVar2 = FUN_0046a460((short)(0u - (unsigned int)(int)(short)DAT_00a07cc4));
  uVar4 = FUN_0046abf0(DAT_00a07c68, (int)(0u - (unsigned int)DAT_00a07c6c));
  sVar3 = (short)(0u - (unsigned int)(int)(short)uVar4);
  __builtin_memcpy(&DAT_009e9a6d + iVar1, &sVar3, 2);
  __builtin_memcpy(&DAT_009e9a6b + iVar1, &sVar2, 2);
  lVar14 = FUN_0046a6d0(sVar3);
  lVar15 = FUN_0046a3c0(sVar3);
  sVar3 = (short)lVar15;
  lVar15 = FUN_0046a6d0(sVar2);
  lVar16 = FUN_0046a3c0(sVar2);
  sVar2 = (short)lVar16;
  iVar5 = (int)(short)lVar15;
  sStack_68 = (short)((unsigned int)((unsigned int)(int)sVar3 * (unsigned int)iVar5) >> 0xf);
  iVar13 = (int)(short)lVar14;
  sStack_64 = (short)((unsigned int)((unsigned int)(int)sVar3 * (unsigned int)(int)sVar2) >> 0xf);
  sStack_60 = (short)((unsigned int)((unsigned int)iVar13 * (unsigned int)iVar5) >> 0xf);
  sStack_5c = (short)((unsigned int)((unsigned int)iVar13 * (unsigned int)(int)sVar2) >> 0xf);
  iVar6 = (int)(short)(-sVar3);
  iVar6 = (int)-(int)sVar3;
  iVar7 = (int)((unsigned int)DAT_00a0049c * (unsigned int)iVar13 +
                (unsigned int)DAT_00a0048c * (unsigned int)iVar6);
  if (0x3fffffff < iVar7) {
    iVar7 = 0x3fffffff;
  }
  if (iVar7 < -0x3fffffff) {
    iVar7 = -0x3fff0000;
  }
  iVar10 = -(int)sVar2;
  sStack_58 = (short)(iVar7 >> 0xf);
  iVar7 = (int)sStack_60;
  iVar11 = (int)sStack_68;
  iVar8 = (int)((unsigned int)DAT_00a0049c * (unsigned int)iVar11 +
                (unsigned int)DAT_00a0048c * (unsigned int)iVar7 +
                (unsigned int)DAT_00a00494 * (unsigned int)iVar10);
  if (0x3fffffff < iVar8) {
    iVar8 = 0x3fffffff;
  }
  if (iVar8 < -0x3fffffff) {
    iVar8 = -0x3fff0000;
  }
  sStack_54 = (short)(iVar8 >> 0xf);
  iVar12 = (int)sStack_5c;
  iVar8 = (int)sStack_64;
  iVar9 = (int)((unsigned int)DAT_00a0049c * (unsigned int)iVar8 +
                (unsigned int)DAT_00a0048c * (unsigned int)iVar12 +
                (unsigned int)DAT_00a00494 * (unsigned int)iVar5);
  if (0x3fffffff < iVar9) {
    iVar9 = 0x3fffffff;
  }
  if (iVar9 < -0x3fffffff) {
    iVar9 = -0x3fff0000;
  }
  DAT_00a0049c = (int)sStack_58;
  DAT_00a0048c = (int)sStack_54;
  sStack_50 = (short)(iVar9 >> 0xf);
  DAT_00a00494 = (int)sStack_50;
  iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar13 +
                (unsigned int)DAT_00a004ac * (unsigned int)iVar6);
  if (0x3fffffff < iVar9) {
    iVar9 = 0x3fffffff;
  }
  if (iVar9 < -0x3fffffff) {
    iVar9 = -0x3fff0000;
  }
  sStack_4c = (short)(iVar9 >> 0xf);
  iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar11 +
                (unsigned int)DAT_00a004ac * (unsigned int)iVar7 +
                (unsigned int)DAT_00a004b4 * (unsigned int)iVar10);
  if (0x3fffffff < iVar9) {
    iVar9 = 0x3fffffff;
  }
  if (iVar9 < -0x3fffffff) {
    iVar9 = -0x3fff0000;
  }
  sStack_48 = (short)(iVar9 >> 0xf);
  iVar9 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)iVar8 +
                (unsigned int)DAT_00a004ac * (unsigned int)iVar12 +
                (unsigned int)DAT_00a004b4 * (unsigned int)iVar5);
  if (0x3fffffff < iVar9) {
    iVar9 = 0x3fffffff;
  }
  if (iVar9 < -0x3fffffff) {
    iVar9 = -0x3fff0000;
  }
  DAT_00a004d0 = (int)sStack_4c;
  DAT_00a004ac = (int)sStack_48;
  sStack_44 = (short)(iVar9 >> 0xf);
  DAT_00a004b4 = (int)sStack_44;
  iVar6 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)iVar13 +
                (unsigned int)DAT_00a07c6c * (unsigned int)iVar6);
  if (0x3fffffff < iVar6) {
    iVar6 = 0x3fffffff;
  }
  if (iVar6 < -0x3fffffff) {
    iVar6 = -0x3fff0000;
  }
  sStack_3c = (short)(iVar6 >> 0xf);
  iVar6 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)iVar11 +
                (unsigned int)DAT_00a07c6c * (unsigned int)iVar7 +
                (unsigned int)DAT_00a07cc4 * (unsigned int)iVar10);
  if (0x3fffffff < iVar6) {
    iVar6 = 0x3fffffff;
  }
  if (iVar6 < -0x3fffffff) {
    iVar6 = -0x3fff0000;
  }
  sStack_2c = (short)(iVar6 >> 0xf);
  iVar5 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)iVar8 +
                (unsigned int)DAT_00a07c6c * (unsigned int)iVar12 +
                (unsigned int)DAT_00a07cc4 * (unsigned int)iVar5);
  if (0x3fffffff < iVar5) {
    iVar5 = 0x3fffffff;
  }
  if (iVar5 < -0x3fffffff) {
    iVar5 = -0x3fff0000;
  }
  DAT_00a07c68 = (int)sStack_3c;
  DAT_00a07c6c = (int)sStack_2c;
  sStack_20 = (short)(iVar5 >> 0xf);
  DAT_00a07cc4 = (int)sStack_20;
  uVar4 = FUN_0046abf0(DAT_00a0048c, DAT_00a0049c);
  tmpA = (short)(0u - (unsigned int)(int)(short)uVar4);
  __builtin_memcpy(&DAT_009e9a6f + iVar1, &tmpA, 2);
  if (((unsigned char)DAT_009ec474 & 0xe) == 2) {
    tmpA = (short)((int)(short)(0u - (unsigned int)(int)(short)uVar4) + (int)param_2);
    __builtin_memcpy(&DAT_009e9a6f + iVar1, &tmpA, 2);
  }
  return;
}
