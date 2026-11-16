/* SPDX-License-Identifier: MIT */
/* z_bop 004c8990 */
/* Globals: the evidence gives only addresses, so the types are inferred.
   DAT_00b6d962 (0xb6d962), DAT_00b6d9b2, DAT_00b6d9ca, DAT_00b6d9e2 and
   DAT_00b6d9fa are all 4-byte values at addresses that are not multiples
   of 4. They are read by memcpy. */
extern char DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern char DAT_00b69d20[];
extern char DAT_00b6d962[];
extern char DAT_00b6d9ca[];
extern char DAT_00b6d9b2[];
extern char DAT_00b6d9fa[];
extern char DAT_00b6d9e2[];
extern char DAT_00528118[];
extern char DAT_0052b460[];
extern char s__s___c_s__c_s_0052b48c[];
extern char s_medlvl_d_0052b49c[];
extern char s_citlvl_d_0052b4a8[];
extern char s_rcitlvl_d_0052b4b4[];

extern int _sprintf(char *, const char *, ...);
extern void FUN_004d6460(int *, int, int, int, int);
extern char *FUN_004ddc10(int);
extern void FUN_004db640(int, char *, int *, unsigned int);
extern void FUN_004b5570(char *, int, int);
extern void FUN_004db4e0(int, char *, int, int, int);
extern void FUN_004d64a0(int *, int, int);
extern void FUN_004daa20(int *, char *, char *, int);

unsigned int __stdcall FUN_004c8990(void)
{
  char *pcVar1;
  int iVar2;
  int iVar3;
  int local_10[4];
  int iDat;
  int iVal;

  FUN_004d6460(local_10, 0x54, 0x5a, 0x194, 0x6a);
  if (DAT_00b6a2e0[0] == '\0') {
    pcVar1 = FUN_004ddc10(9);
    _sprintf(DAT_00b69d20, DAT_00528118, pcVar1);
  }
  else {
    pcVar1 = FUN_004ddc10(9);
    _sprintf(DAT_00b69d20, s__s___c_s__c_s_0052b48c, pcVar1, 6, DAT_00b6bc9e, 4, DAT_00b6a2e0);
  }
  FUN_004db640(0xf, DAT_00b69d20, local_10, 0xffff);
  if (DAT_00b6a2e0[0] == '\0') {
    return 0;
  }

  FUN_004d6460(local_10, 0x10c, 0x76, 0x1a8, 0x84);
  pcVar1 = FUN_004ddc10(0x17b);
  iVar3 = 0x85;
  FUN_004db640(0xc, pcVar1, local_10, 0xffff);
  iVar2 = 0;
  FUN_004d6460(local_10, 0x161, 0x85, 0x183, 0x92);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9ca + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      _sprintf(DAT_00b69d20, s_medlvl_d_0052b49c, iVar2 + 1);
      FUN_004b5570(DAT_00b69d20, 0x161, iVar3);
      __builtin_memcpy(&iDat, DAT_00b6d962, 4);
      __builtin_memcpy(&iVal, DAT_00b6d9ca + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
      _sprintf(DAT_00b69d20, DAT_0052b460, iVal);
      FUN_004db4e0(10, DAT_00b69d20, 0x193, iVar3 + 1, 0xffff);
    }
    iVar3 = iVar3 + 0x16;
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  FUN_004d6460(local_10, 0x61, 0x76, 0xfd, 0x84);
  pcVar1 = FUN_004ddc10(0x17a);
  iVar3 = 0x85;
  FUN_004db640(0xc, pcVar1, local_10, 0xffff);
  iVar2 = 0;
  FUN_004d6460(local_10, 0xb6, 0x85, 0xd8, 0x92);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9b2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      _sprintf(DAT_00b69d20, s_medlvl_d_0052b49c, iVar2 + 1);
      FUN_004b5570(DAT_00b69d20, 0xb6, iVar3);
      __builtin_memcpy(&iDat, DAT_00b6d962, 4);
      __builtin_memcpy(&iVal, DAT_00b6d9b2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
      _sprintf(DAT_00b69d20, DAT_0052b460, iVal);
      FUN_004db4e0(10, DAT_00b69d20, 0xe8, iVar3 + 1, 0xffff);
    }
    iVar3 = iVar3 + 0x16;
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  FUN_004d6460(local_10, 0x10c, 0x10d, 0x1a8, 0x11b);
  pcVar1 = FUN_004ddc10(0x17d);
  iVar3 = 0x11c;
  FUN_004db640(0xc, pcVar1, local_10, 0xffff);
  iVar2 = 0;
  FUN_004d6460(local_10, 0x161, 0x11c, 0x183, 0x129);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9fa + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      _sprintf(DAT_00b69d20, s_medlvl_d_0052b49c, iVar2 + 1);
      FUN_004b5570(DAT_00b69d20, 0x161, iVar3);
      __builtin_memcpy(&iDat, DAT_00b6d962, 4);
      __builtin_memcpy(&iVal, DAT_00b6d9fa + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
      _sprintf(DAT_00b69d20, DAT_0052b460, iVal);
      FUN_004db4e0(10, DAT_00b69d20, 0x193, iVar3 + 1, 0xffff);
    }
    iVar3 = iVar3 + 0x16;
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  FUN_004d6460(local_10, 0x61, 0x10d, 0xfd, 0x11b);
  pcVar1 = FUN_004ddc10(0x17c);
  iVar3 = 0x11c;
  FUN_004db640(0xc, pcVar1, local_10, 0xffff);
  iVar2 = 0;
  FUN_004d6460(local_10, 0xb6, 0x11c, 0xd8, 0x129);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9e2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      __builtin_memcpy(&iDat, DAT_00b6d962, 4);
      if (iDat == 0) {
        pcVar1 = s_rcitlvl_d_0052b4b4;
      }
      else {
        pcVar1 = s_citlvl_d_0052b4a8;
      }
      _sprintf(DAT_00b69d20, pcVar1, iVar2 + 1);
      FUN_004b5570(DAT_00b69d20, 0xb6, iVar3);
      __builtin_memcpy(&iDat, DAT_00b6d962, 4);
      __builtin_memcpy(&iVal, DAT_00b6d9e2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
      _sprintf(DAT_00b69d20, DAT_0052b460, iVal);
      FUN_004db4e0(10, DAT_00b69d20, 0xe8, iVar3 + 1, 0xffff);
    }
    iVar3 = iVar3 + 0x16;
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  iVar2 = 0;
  FUN_004d6460(local_10, 0x161, 0x85, 0x183, 0x92);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9ca + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      iVar3 = 0xc;
      pcVar1 = FUN_004ddc10(iVar2 + 0x17e);
      FUN_004daa20(local_10, (char *)0, pcVar1, iVar3);
    }
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  iVar2 = 0;
  FUN_004d6460(local_10, 0xb6, 0x85, 0xd8, 0x92);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9b2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      iVar3 = 0xc;
      pcVar1 = FUN_004ddc10(iVar2 + 0x17e);
      FUN_004daa20(local_10, (char *)0, pcVar1, iVar3);
    }
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  iVar2 = 0;
  FUN_004d6460(local_10, 0x161, 0x11c, 0x183, 0x129);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9fa + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      iVar3 = 0xc;
      pcVar1 = FUN_004ddc10(iVar2 + 0x17e);
      FUN_004daa20(local_10, (char *)0, pcVar1, iVar3);
    }
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);

  iVar2 = 0;
  FUN_004d6460(local_10, 0xb6, 0x11c, 0xd8, 0x129);
  do {
    __builtin_memcpy(&iDat, DAT_00b6d962, 4);
    __builtin_memcpy(&iVal, DAT_00b6d9e2 + (unsigned int)(((unsigned int)iVar2 + (unsigned int)iDat * 0x4278U) * 4U), 4);
    if (iVal != 0) {
      iVar3 = 0xc;
      pcVar1 = FUN_004ddc10(iVar2 + 0x294);
      FUN_004daa20(local_10, (char *)0, pcVar1, iVar3);
    }
    iVar2 = iVar2 + 1;
    FUN_004d64a0(local_10, 0, 0x16);
  } while (iVar2 < 6);
  return 1;
}
