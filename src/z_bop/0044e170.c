/* SPDX-License-Identifier: MIT */
/* z_bop 0044e170 */
extern int DAT_009e9640;
extern int DAT_00a00854;
extern int DAT_009a1fe0;
extern int DAT_009a1fe8;
extern int DAT_009ecc34;
extern char DAT_009d8e25[];
extern char DAT_009d8e29[];
extern char DAT_009d8c80[];
extern char DAT_009a2196[];
extern char DAT_009a210a[];
extern char DAT_009a8080[];
extern char DAT_009a8081[];
extern char DAT_009a8082[];
extern char DAT_009e967c[];

short __cdecl FUN_00432fa0(unsigned short, unsigned short, unsigned short);

unsigned int __cdecl FUN_0044e170(unsigned int param_1)
{
  int iVar1;
  int iVar5;
  unsigned char bVar3;
  char cVar4;
  _Bool bVar6, bVar7, bVar8, bVar9, bVar10, bVar11, bVar12;
  short sVar13;
  unsigned int uVar14;
  char *pcVar15;
  unsigned int uVar16;
  char *pcVar17;
  unsigned int uVar18;
  int local_18;
  unsigned short wVal;
  unsigned char *pbPuVar2;
  int iEnt;
  short sTmp;

  uVar14 = param_1 & 0xffff;
  if ((DAT_009e9640 <= (int)uVar14) && ((int)uVar14 < DAT_00a00854)) {
    return 0;
  }
  if ((int)uVar14 < DAT_009a1fe0) {
    __builtin_memcpy(&iEnt, (unsigned char *)DAT_009a1fe8 + (0x1fU + uVar14 * 0x23U), 4);
    __builtin_memcpy(&local_18, (unsigned char *)iEnt + 0xa9, 4);
  }
  else {
    local_18 = 0;
  }
  bVar7 = 0;
  bVar11 = 0;
  bVar10 = 0;
  bVar12 = 0;
  bVar8 = 0;
  bVar9 = 0;
  uVar16 = 0;
  bVar3 = *((unsigned char *)DAT_009a1fe8 + (0x16U + uVar14 * 0x23U));
  bVar6 = 0;
  iVar5 = (int)((unsigned int)bVar3 * 0x566U);
  pcVar15 = DAT_009d8e25 + iVar5;
  pbPuVar2 = (unsigned char *)DAT_009e967c + (unsigned int)DAT_009ecc34 * 0x5bdU;
  __builtin_memcpy(&wVal, pbPuVar2, 2);
  pcVar17 = DAT_009d8e29 + ((unsigned int)wVal + (unsigned int)iVar5);
  do {
    if (*pcVar17 != '\0' && *pcVar15 == '\0') {
      iVar1 = (int)((unsigned int)bVar3 * 0x126U);
      __builtin_memcpy(&wVal, pbPuVar2, 2);
      if (DAT_009a2196[(unsigned int)iVar1 + uVar16 + (unsigned int)wVal * 8U] == '\x04') {
        if (pcVar15[2] == '\x06') {
          if (DAT_009a210a[iVar1] == '\0') {
            bVar7 = 1;
          }
          bVar6 = 1;
        }
        cVar4 = pcVar15[1];
        if (cVar4 == '\x02') {
          bVar11 = 1;
        }
        else if (cVar4 == '\x03') {
          bVar12 = 1;
        }
        else if (cVar4 == '\b') {
          bVar10 = 1;
        }
        else if (cVar4 == '\x05') {
          bVar7 = 1;
        }
        else if (cVar4 == '\x04') {
          bVar8 = 1;
        }
        else if (cVar4 == '\x06') {
          bVar9 = 1;
        }
      }
    }
    pcVar15 = pcVar15 + 0x4e;
    pcVar17 = pcVar17 + 0x4e;
    uVar16 = uVar16 + 1;
  } while (uVar16 < 8);
  uVar16 = 0;
  do {
    uVar18 = 0;
    do {
      __builtin_memcpy(&wVal, pbPuVar2, 2);
      iVar1 = (int)(uVar18 + (unsigned int)wVal * 0x126U);
      cVar4 = DAT_009a8080[uVar16 + (unsigned int)iVar1];
      iVar1 = (int)((unsigned int)iVar1 + uVar16);
      if (cVar4 != '\n') {
        sVar13 = FUN_00432fa0((unsigned short)bVar3,
                              (unsigned short)(unsigned char)DAT_009a8081[iVar1],
                              (unsigned short)(unsigned char)DAT_009a8082[iVar1]);
        if (sVar13 != 0) {
          if (cVar4 == '\x02') {
            bVar11 = 1;
          }
          else if (cVar4 == '\x03') {
            bVar12 = 1;
          }
          else if (cVar4 == '\b') {
            bVar10 = 1;
          }
          else if (cVar4 == '\x05') {
            bVar7 = 1;
          }
          else if (cVar4 == '\x04') {
            bVar8 = 1;
          }
          else if (cVar4 == '\x06') {
            bVar9 = 1;
          }
        }
      }
      uVar18 = uVar18 + 4;
    } while (uVar18 < 8);
    uVar16 = uVar16 + 0xb;
  } while (uVar16 < 0x16);
  if ((int)uVar14 < DAT_009a1fe0) {
    if (bVar7) {
      __builtin_memcpy(&wVal, pbPuVar2, 2);
      if (*(char *)((unsigned int)wVal + 0x1f6U + (unsigned int)local_18) != '\0') {
        bVar7 = 0;
      }
    }
    if ((bVar8) || (bVar9)) {
      __builtin_memcpy(&iEnt, (unsigned char *)DAT_009a1fe8 + (0x1fU + uVar14 * 0x23U), 4);
      __builtin_memcpy(&sTmp, (unsigned char *)iEnt + 0x79, 2);
      if (sTmp != 0) {
        bVar10 = 1;
      }
      if ((bVar6) && (DAT_009d8c80[iVar5] != *(char *)((unsigned int)local_18 + 0xb0U))) {
        bVar8 = 0;
        bVar9 = 0;
      }
    }
    if (((bVar10) && (bVar6)) &&
        (DAT_009d8c80[iVar5] != *(char *)((unsigned int)local_18 + 0xb0U))) {
      bVar10 = 0;
    }
    if (((bVar11) && (bVar6)) &&
        (DAT_009d8c80[iVar5] != *(char *)((unsigned int)local_18 + 0xb0U))) {
      bVar11 = 0;
    }
  }
  if (bVar7) {
    return 0xf;
  }
  if (bVar10) {
    if (bVar8) {
      return 0x13;
    }
    return ((0U - (unsigned int)(!bVar9)) & 0xfffffffdU) + 0x14U;
  }
  if (bVar8) {
    return 0x13;
  }
  if (bVar9) {
    return 0x14;
  }
  if (bVar11) {
    return 0x10;
  }
  return ((unsigned int)(!bVar12) - 1U) & 0xffff0012U;
}
