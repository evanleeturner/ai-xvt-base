/* SPDX-License-Identifier: MIT */
/* z_bop 00451f50 */
extern short DAT_00a07be6;
extern short DAT_00a07be2;
extern short DAT_00a07be4;
extern int DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_009a1fe0;
extern unsigned char *DAT_009a1fe8;
extern unsigned char DAT_009d8c88[];
extern unsigned char DAT_009d8c80[];
extern unsigned char DAT_009d7188[];
extern unsigned char DAT_009e967c[];
extern unsigned char DAT_009e967e[];
extern unsigned char DAT_009e9670[];
extern unsigned char DAT_009fd458[];
extern char DAT_009d8e25[];
extern char DAT_009d8e29[];
extern char DAT_009a2196[];
extern char DAT_009a210a[];
extern unsigned char DAT_009a8080[];
extern unsigned char DAT_009a8081[];
extern unsigned char DAT_009a8082[];
extern char DAT_009a1ed0[];
extern unsigned char DAT_009d8b8a[];
extern int DAT_009ecc34;
extern unsigned int DAT_009d7694;

void __cdecl FUN_004525a0(unsigned int, int, char *);
void __cdecl FUN_00451bf0(int, char *);
short __cdecl FUN_00432fa0(unsigned short, unsigned short, unsigned short);
int __cdecl FUN_00404670(int, int, short);
void __cdecl FUN_00450650(int, int);

unsigned int __cdecl
FUN_00451f50(unsigned int param_1, int param_2, int param_3, int param_4,
             unsigned int param_5, int param_6)
{
  unsigned char bVar1;
  unsigned char bVar2;
  unsigned char bVar3;
  char cVar4;
  int iVar5;
  int iVar6;
  int bVar7;
  int bVar8;
  int bVar9;
  int bVar10;
  int bVar11;
  short sVar12;
  unsigned int uVar13;
  char *pcVar14;
  int iVar15;
  unsigned char *iVar16;
  unsigned int uVar17;
  unsigned int uVar18;
  unsigned int uVar19;
  char *pcVar20;
  int bVar21;
  unsigned char *local_24;
  unsigned int local_8;
  unsigned char *pTmp;
  unsigned short us;
  short ss;
  unsigned int idx;

  DAT_00a07be6 = 0x14b;
  uVar13 = param_1 & 0xffff;
  local_8 = 0;
  if ((DAT_009e9640 <= (int)uVar13) && ((int)uVar13 < DAT_00a00854)) {
    return 0;
  }
  if ((int)uVar13 < DAT_009a1fe0) {
    __builtin_memcpy(&pTmp, DAT_009a1fe8 + 0x1f + uVar13 * 0x23, 4);
    __builtin_memcpy(&local_24, pTmp + 0xa9, 4);
  } else {
    local_24 = 0;
  }
  iVar15 = (int)(uVar13 * 0x23);
  iVar16 = DAT_009a1fe8 + iVar15;
  bVar1 = iVar16[0x16];
  uVar19 = (unsigned int)bVar1;
  iVar6 = (int)((unsigned int)param_2 * 0x5bd);
  iVar5 = (int)(uVar19 * 0x566);
  bVar2 = DAT_009d8c88[iVar5];
  __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
  uVar18 = (unsigned int)DAT_009d7188[uVar19 + (unsigned int)us * 0x30];
  if (uVar18 == 0) {
    if (local_24 == 0) {
      if (*(char *)(iVar16 + 2) == '\b') {
        uVar18 = 0x11;
      } else {
        bVar3 = iVar16[3];
        if ((bVar3 < 0x46) || (0x4a < bVar3)) {
          if ((bVar3 < 0x50) || (0x52 < bVar3)) {
            if ((0x52 < bVar3) && (bVar3 < 0x55)) {
              uVar18 = 0x14;
            }
          } else {
            uVar18 = 0x13;
          }
        } else {
          uVar18 = 0x12;
        }
      }
    } else {
      __builtin_memcpy(&us, DAT_009e967e + (int)((unsigned int)param_2 * 0x5bd), 2);
      if ((unsigned int)us == uVar19) {
        uVar18 = 0xe;
      } else {
        __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
        if ((unsigned int)bVar2 == (unsigned int)us) {
          uVar18 = 0xd;
        } else {
          if (*(char *)(iVar16 + 2) != '\x03') {
            uVar18 = 0xf;
          } else {
            uVar18 = 0x10;
            if (*(short *)(local_24 + 0x92) != 0) {
              uVar18 = 0xf;
            }
          }
        }
      }
    }
  }
  FUN_004525a0(param_1, 2, DAT_009a1ed0);
  FUN_00451bf0(0, DAT_009a1ed0);
  DAT_00a07be2 = 0x14b;
  if (uVar18 == 0) {
    DAT_00a07be4 = 0x14b;
  } else {
    if (*(char *)(uVar18 + 0x5240b8) != '\0') {
      __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
      uVar17 = (unsigned int)us;
      if (bVar2 == uVar17) {
        DAT_00a07be2 = 0x14e;
      } else {
        if (uVar17 == DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar15) * 0x566]) {
          bVar21 = 0;
        } else {
          bVar21 = DAT_009fd458[(unsigned int)DAT_009d8c88[(unsigned int)*(unsigned char *)(DAT_009a1fe8 + 0x16 + iVar15) * 0x566] + uVar17 * 0x1e5] == 0;
        }
        DAT_00a07be2 = 0x14d;
        if (bVar21) {
          DAT_00a07be2 = 0x14c;
        }
      }
    }
    DAT_00a07be4 = (short)((short)uVar18 + 0x135);
  }
  bVar7 = 0;
  bVar11 = 0;
  bVar10 = 0;
  uVar18 = 0;
  pcVar14 = DAT_009d8e25 + iVar5;
  bVar8 = 0;
  bVar9 = 0;
  bVar21 = 0;
  __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
  pcVar20 = DAT_009d8e29 + (unsigned int)us + iVar5;
  do {
    __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
    if (((*pcVar20 != '\0') && (*pcVar14 == '\0')) &&
        (DAT_009a2196[uVar19 * 0x126 + uVar18 + (unsigned int)us * 8] == '\x04')) {
      if (pcVar14[2] == '\x06') {
        if (DAT_009a210a[uVar19 * 0x126] == '\0') {
          bVar7 = 1;
        }
        bVar21 = 1;
      }
      cVar4 = pcVar14[1];
      if (cVar4 == '\x02') {
        bVar11 = 1;
      } else if (cVar4 != '\x03') {
        if (cVar4 == '\b') {
          bVar10 = 1;
        } else if (cVar4 == '\x05') {
          bVar7 = 1;
        } else if ((cVar4 == '\x04') || (cVar4 == ',')) {
          bVar8 = 1;
        } else if (cVar4 == '\x06') {
          bVar9 = 1;
        }
      }
    }
    pcVar14 = pcVar14 + 0x4e;
    pcVar20 = pcVar20 + 0x4e;
    uVar18 = uVar18 + 1;
  } while (uVar18 < 8);
  uVar19 = 0;
  do {
    uVar18 = 0;
    do {
      __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
      idx = uVar18 + (unsigned int)us * 0x126 + uVar19;
      cVar4 = (char)DAT_009a8080[idx];
      if ((cVar4 != '\n') &&
          (sVar12 = FUN_00432fa0((unsigned short)bVar1,
                                 (unsigned short)DAT_009a8081[idx],
                                 (unsigned short)DAT_009a8082[idx]),
           sVar12 != 0)) {
        if (cVar4 == '\x02') {
          bVar11 = 1;
        } else if (cVar4 != '\x03') {
          if (cVar4 == '\b') {
            bVar10 = 1;
          } else if (cVar4 == '\x05') {
            bVar7 = 1;
          } else if ((cVar4 == '\x04') || (cVar4 == ',')) {
            bVar8 = 1;
          } else if (cVar4 == '\x06') {
            bVar9 = 1;
          }
        }
      }
      uVar18 = uVar18 + 4;
    } while (uVar18 < 8);
    uVar19 = uVar19 + 0xb;
  } while (uVar19 < 0x16);
  if ((int)uVar13 < DAT_009a1fe0) {
    __builtin_memcpy(&us, DAT_009e967c + iVar6, 2);
    if ((bVar7) && (*(char *)((unsigned int)us + 0x1f6 + local_24) != '\0')) {
      bVar7 = 0;
    }
    if ((bVar8) || (bVar9)) {
      __builtin_memcpy(&pTmp, DAT_009a1fe8 + 0x1f + iVar15, 4);
      __builtin_memcpy(&ss, pTmp + 0x79, 2);
      if (ss != 0) {
        bVar10 = 1;
      }
      if ((bVar21) && ((char)DAT_009d8c80[iVar5] != *(char *)(local_24 + 0xb0))) {
        bVar8 = 0;
        bVar9 = 0;
      }
    }
    if (((bVar10) && (bVar21)) && ((char)DAT_009d8c80[iVar5] != *(char *)(local_24 + 0xb0))) {
      bVar10 = 0;
    }
    if (((bVar11) && (bVar21)) && ((char)DAT_009d8c80[iVar5] != *(char *)(local_24 + 0xb0))) {
      bVar11 = 0;
    }
  }
  if (bVar7) {
    DAT_00a07be6 = 0x158;
  } else {
    if (bVar10) {
      if (bVar8) {
        DAT_00a07be6 = 0x155;
      } else {
        DAT_00a07be6 = 0x159;
        if (!bVar9) {
          DAT_00a07be6 = 0x15b;
        }
      }
      {
        int tmp;
        __builtin_memcpy(&tmp, DAT_009e9670 + iVar6, 4);
        iVar15 = FUN_00404670(tmp, 0x13, (short)param_1);
      }
    } else {
      if (bVar8) {
        DAT_00a07be6 = 0x155;
        goto LAB_00452534;
      }
      if (bVar9) {
        DAT_00a07be6 = 0x159;
        goto LAB_00452534;
      }
      if (!bVar11) goto LAB_00452534;
      DAT_00a07be6 = 0x151;
      {
        int tmp;
        __builtin_memcpy(&tmp, DAT_009e9670 + iVar6, 4);
        iVar15 = FUN_00404670(tmp, 0x45, (short)param_1);
      }
    }
    if (iVar15 != 1) goto LAB_00452534;
    DAT_00a07be6 = (short)(DAT_00a07be6 + 1);
  }
  local_8 = 1;
LAB_00452534:
  if ((param_3 != 0) && (param_2 == DAT_009ecc34)) {
    FUN_00450650(0x135, param_2);
    *(short *)(DAT_009d8b8a + (unsigned int)DAT_009ecc34 * 0x10) = 0x49c;
    DAT_009d7694 = (unsigned int)(unsigned short)DAT_00a07be6;
  }
  if (param_4 != 0) {
    return local_8;
  }
  return (unsigned int)(unsigned short)DAT_00a07be6;
}
