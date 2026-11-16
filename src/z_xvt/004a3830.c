/* SPDX-License-Identifier: MIT */
/* z_xvt 004a3830 */
/* Prototypes */
void FUN_004a3e40(uint param_1, int param_2, char *param_3);
_Bool FUN_00444040(uint param_1, uint param_2);
short FUN_0049ec30(ushort param_1, ushort param_2, ushort param_3);
int FUN_0042b2a0(int param_1, int param_2, short param_3);
void FUN_004a1d70(int param_1, int param_2);

/* Globals */
extern ushort DAT_00732956;
extern int DAT_0071b704;
extern int DAT_00732904;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0070ad40[];
extern unsigned char DAT_00709b08[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b73e[];
extern unsigned char DAT_0071b730[];
extern char DAT_006d4870;
extern char *_DAT_00732e70;
extern ushort _DAT_00732950;
extern ushort _DAT_00732952;
extern ushort _DAT_00732954;
extern char DAT_0070aee5[];
extern char DAT_0070aee9[];
extern unsigned char DAT_006d4b36[];
extern unsigned char DAT_006d4aaa[];
extern unsigned char DAT_006daa00[];
extern unsigned char DAT_006daa01[];
extern unsigned char DAT_006daa02[];
extern int DAT_0071ecf4;
extern unsigned char DAT_0070ac4a[];
extern uint DAT_0070a014;

uint __cdecl FUN_004a3830(uint param_1, int param_2, int param_3, int param_4, undefined4 param_5, int param_6)
{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  char cVar4;
  int iVar5;
  _Bool bVar6;
  _Bool bVar7;
  _Bool bVar8;
  _Bool bVar9;
  _Bool bVar10;
  _Bool bVar11;
  short sVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  char *pcVar16;
  uint uVar17;
  uint uVar18;
  char *pcVar19;
  int local_24;
  uint local_8;
  ushort uVarW;
  ushort uVarW2;
  int iInner;
  ushort sTmp;
  int iTmp;

  DAT_00732956 = 0x141;
  uVar13 = param_1 & 0xffff;
  local_8 = 0;
  if ((DAT_0071b704 <= (int)uVar13) && ((int)uVar13 < DAT_00732904)) {
    return 0;
  }
  if ((int)uVar13 < DAT_006d4980) {
    __builtin_memcpy(&iInner, (char *)(DAT_006d4988 + 0x1f + uVar13 * 0x23), 4);
    __builtin_memcpy(&local_24, (char *)(iInner + 0xa7), 4);
  }
  else {
    local_24 = 0;
  }
  iVar15 = (int)(DAT_006d4988 + uVar13 * 0x23);
  bVar1 = *(byte *)(iVar15 + 0x16);
  uVar18 = (uint)bVar1;
  iVar5 = (int)((uint)param_2 * 0x5bdU);
  iVar14 = (int)(uVar18 * 0x566U);
  bVar2 = DAT_0070ad48[iVar14];
  __builtin_memcpy(&uVarW, &DAT_0071b73c[iVar5], 2);
  uVar17 = (uint)DAT_00709b08[uVar18 + (uint)uVarW * 0x30];
  if (uVar17 == 0) {
    if (local_24 == 0) {
      if (*(char *)(iVar15 + 2) == '\b') {
        uVar17 = 0x11;
      }
      else {
        bVar3 = *(byte *)(iVar15 + 3);
        if ((bVar3 < 0x46) || (0x4a < bVar3)) {
          if ((bVar3 < 0x50) || (0x52 < bVar3)) {
            if ((0x52 < bVar3) && (bVar3 < 0x55)) {
              uVar17 = 0x14;
            }
          }
          else {
            uVar17 = 0x13;
          }
        }
        else {
          uVar17 = 0x12;
        }
      }
    }
    else {
      __builtin_memcpy(&uVarW2, &DAT_0071b73e[iVar5], 2);
      if (uVarW2 == uVar18) {
        uVar17 = 0xe;
      }
      else if ((uint)uVarW == (uint)bVar2) {
        uVar17 = 0xd;
      }
      else if ((*(char *)(iVar15 + 2) != '\x03') || (uVar17 = 0x10, *(short *)((char *)(local_24 + 0x92)) != 0)) {
        uVar17 = 0xf;
      }
    }
  }
  FUN_004a3e40(param_1, 2, &DAT_006d4870);
  _DAT_00732e70 = &DAT_006d4870;
  _DAT_00732950 = 0x8000;
  _DAT_00732952 = 0x141;
  if (uVar17 == 0) {
    _DAT_00732954 = 0x141;
  }
  else {
    if (*(char *)(uVar17 + 0x52adf8) != '\0') {
      if ((uint)uVarW == (uint)bVar2) {
        _DAT_00732952 = 0x144;
      }
      else {
        bVar11 = FUN_00444040(param_1, (uint)uVarW);
        if (!bVar11) {
          _DAT_00732952 = (ushort)(_DAT_00732952 + 2);
        }
        else {
          _DAT_00732952 = (ushort)(_DAT_00732952 + 1);
        }
      }
    }
    _DAT_00732954 = (ushort)((short)uVar17 + 299);
  }
  bVar6 = 0;
  bVar10 = 0;
  bVar9 = 0;
  uVar17 = 0;
  pcVar16 = &DAT_0070aee5[iVar14];
  bVar7 = 0;
  bVar8 = 0;
  bVar11 = 0;
  pcVar19 = &DAT_0070aee9[(uint)uVarW + iVar14];
  do {
    if (((*pcVar19 != '\0') && (*pcVar16 == '\0')) &&
        (DAT_006d4b36[uVar18 * 0x126 + uVar17 + (uint)uVarW * 8] == '\x04')) {
      if (pcVar16[2] == '\x06') {
        if (DAT_006d4aaa[uVar18 * 0x126] == '\0') {
          bVar6 = 1;
        }
        bVar11 = 1;
      }
      cVar4 = pcVar16[1];
      if (cVar4 == '\x02') {
        bVar10 = 1;
      }
      else if (cVar4 != '\x03') {
        if (cVar4 == '\b') {
          bVar9 = 1;
        }
        else if (cVar4 == '\x05') {
          bVar6 = 1;
        }
        else if ((cVar4 == '\x04') || (cVar4 == ',')) {
          bVar7 = 1;
        }
        else if (cVar4 == '\x06') {
          bVar8 = 1;
        }
      }
    }
    pcVar16 = pcVar16 + 0x4e;
    pcVar19 = pcVar19 + 0x4e;
    uVar17 = uVar17 + 1;
  } while (uVar17 < 8);
  uVar18 = 0;
  do {
    uVar17 = 0;
    do {
      iVar15 = (int)(uVar18 + (uint)uVarW * 0x126);
      cVar4 = (char)DAT_006daa00[uVar17 + iVar15];
      iVar15 = (int)(iVar15 + uVar17);
      if ((cVar4 != '\n') &&
          (sVar12 = FUN_0049ec30((ushort)bVar1, (ushort)DAT_006daa01[iVar15],
                                 (ushort)DAT_006daa02[iVar15]), sVar12 != 0)) {
        if (cVar4 == '\x02') {
          bVar10 = 1;
        }
        else if (cVar4 != '\x03') {
          if (cVar4 == '\b') {
            bVar9 = 1;
          }
          else if (cVar4 == '\x05') {
            bVar6 = 1;
          }
          else if ((cVar4 == '\x04') || (cVar4 == ',')) {
            bVar7 = 1;
          }
          else if (cVar4 == '\x06') {
            bVar8 = 1;
          }
        }
      }
      uVar17 = uVar17 + 4;
    } while (uVar17 < 8);
    uVar18 = uVar18 + 0xb;
  } while (uVar18 < 0x16);
  if ((int)uVar13 < DAT_006d4980) {
    if ((bVar6) && (*(char *)((uint)uVarW + 0x1f0 + local_24) != '\0')) {
      bVar6 = 0;
    }
    if ((bVar7) || (bVar8)) {
      __builtin_memcpy(&iInner, (char *)(DAT_006d4988 + 0x1f + uVar13 * 0x23), 4);
      __builtin_memcpy(&sTmp, (char *)(iInner + 0x79), 2);
      if (sTmp != 0) {
        bVar9 = 1;
      }
      if ((bVar11) && ((char)DAT_0070ad40[iVar14] != *(char *)(local_24 + 0xb0))) {
        bVar7 = 0;
        bVar8 = 0;
      }
    }
    if (((bVar9) && (bVar11)) && ((char)DAT_0070ad40[iVar14] != *(char *)(local_24 + 0xb0))) {
      bVar9 = 0;
    }
    if (((bVar10) && (bVar11)) && ((char)DAT_0070ad40[iVar14] != *(char *)(local_24 + 0xb0))) {
      bVar10 = 0;
    }
  }
  if (bVar6) {
    DAT_00732956 = 0x14e;
  }
  else {
    if (bVar9) {
      if (bVar7) {
        DAT_00732956 = 0x14b;
      }
      else {
        DAT_00732956 = 0x14f;
        if (!bVar8) {
          DAT_00732956 = 0x151;
        }
      }
      __builtin_memcpy(&iTmp, &DAT_0071b730[iVar5], 4);
      iVar14 = FUN_0042b2a0(iTmp, 0x13, (short)param_1);
    }
    else {
      if (bVar7) {
        DAT_00732956 = 0x14b;
        goto LAB_004a3dda;
      }
      if (bVar8) {
        DAT_00732956 = 0x14f;
        goto LAB_004a3dda;
      }
      if (!bVar10) goto LAB_004a3dda;
      DAT_00732956 = 0x147;
      __builtin_memcpy(&iTmp, &DAT_0071b730[iVar5], 4);
      iVar14 = FUN_0042b2a0(iTmp, 0x45, (short)param_1);
    }
    if (iVar14 != 1) goto LAB_004a3dda;
    DAT_00732956 = (ushort)(DAT_00732956 + 1);
  }
  local_8 = 1;
LAB_004a3dda:
  if ((param_3 != 0) && (param_2 == DAT_0071ecf4)) {
    FUN_004a1d70(299, param_2);
    *(ushort *)(&DAT_0070ac4a[(uint)DAT_0071ecf4 * 0x10U]) = 0x49c;
    DAT_0070a014 = (uint)DAT_00732956;
  }
  if (param_4 != 0) {
    return local_8;
  }
  return (uint)DAT_00732956;
}
