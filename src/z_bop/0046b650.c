/* SPDX-License-Identifier: MIT */
/* z_bop 0046b650 */
unsigned int FUN_0040e0c0(int a, int b, int c, int d);
void FUN_004a9640(char a);
void FUN_004a95c0(unsigned int a);
void FUN_004a9520(short a, short b, short c, short d);
unsigned int FUN_00480530(void);
void FUN_004a9630(int a);
void FUN_004a9590(int a);
void FUN_004a9500(short a, short b);
void FUN_004a9a50(unsigned int a);
unsigned int FUN_0046be10(unsigned short a, short b);
void FUN_0046bf90(unsigned int a, unsigned short b, short c);

extern unsigned char   DAT_009e9670[];
extern char            DAT_009e9683[];
extern int             DAT_009ecc34;
extern int             DAT_009a1fe8;
extern unsigned short  DAT_00a08374;
extern short           DAT_00a0a2ea[];
extern short           DAT_00a08ba6;
extern int             DAT_009a806c;
extern int             DAT_009a7bac;
extern int             DAT_009a8c14;
extern int             DAT_005233d8;
extern unsigned char   DAT_005233c8;
extern short           DAT_00a08c90;
extern short           DAT_00a08c82;
extern short           DAT_00a08c8e;
extern short           DAT_00a08c8a;
extern short           DAT_00521548;
extern short           DAT_0052154c;
extern void          (*DAT_009d1300)(void);
extern void          (*DAT_009eca20)(unsigned int, unsigned int, unsigned int, unsigned int, unsigned int);
extern unsigned int    DAT_0099f928;
extern unsigned char   DAT_009a20ae;
extern short           DAT_00559790;
extern short           DAT_00559794;
extern unsigned short  DAT_00559798;
extern short           _DAT_0055979c;
extern unsigned short  DAT_00a004d4;
extern short           DAT_00a080f8;
extern short           DAT_009d6930;
extern unsigned short  DAT_0051bf60[];

unsigned int __stdcall FUN_0046b650(void)
{
  unsigned char bVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar3a;
  int iVar17;
  unsigned char bVar4;
  short sVar5, sVar6, sVar7, sVar10, sVar14, sVar15, sVar16;
  unsigned short uVar8, uVar9, uVar11;
  unsigned int uVar12, uVar13, uVar18;
  unsigned short extraout_var_00 = 0;
  unsigned short extraout_var_01 = 0;
  unsigned int unaff_ESI = 0;
  short sStack_24;
  unsigned short local_1a;
  unsigned short uStack_16 = 0;
  unsigned int auStack_14[5];
  unsigned short mask;
  unsigned short tmp;

  __builtin_memcpy(&iVar3, &DAT_009e9670[(unsigned int)DAT_009ecc34 * 0x5bd], 4);
  if (iVar3 == -1) {
    uVar12 = (unsigned int)DAT_00a08374;
    if (DAT_00a0a2ea[uVar12] != DAT_00a08ba6) {
      FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
                   (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
      FUN_004a9640('\0');
      sVar7 = DAT_00a08c90;
      sVar5 = DAT_00a08c82;
      sVar6 = (short)(DAT_00a08c82 + DAT_00a08c8e);
      sVar10 = (short)(DAT_00a08c8a + DAT_00a08c90);
      FUN_004a95c0((unsigned int)DAT_005233c8);
      FUN_004a9520(sVar5, sVar7, sVar6, sVar10);
      (*DAT_009d1300)();
      if ((DAT_00a08ba6 == -2) && (DAT_00521548 == 3)) {
        DAT_00521548 = DAT_0052154c;
        uVar12 = FUN_00480530();
        DAT_0052154c = (short)uVar12;
      }
      uVar12 = FUN_0040e0c0(0, 0x140, 200, 0);
    }
    return uVar12 & 0xffff0000;
  }
  __builtin_memcpy(&iVar3a, (char *)(DAT_009a1fe8 + (int)((unsigned int)iVar3 * 0x23u) + 0x1f), 4);
  __builtin_memcpy(&iVar3, (char *)(iVar3a + 0xa9), 4);
  if (iVar3 == 0) {
    return (unsigned int)DAT_009a1fe8 & 0xffff0000;
  }
  sVar6 = 0;
  do {
    *(short *)((int)auStack_14 + (unsigned int)*(unsigned char *)(sVar6 + 0x2a1 + iVar3) * 2) = sVar6;
    sVar6 = (short)(sVar6 + 1);
  } while (sVar6 < 10);
  sVar6 = 0;
  bVar4 = 1;
  uVar12 = unaff_ESI & 0xffff0000;
  do {
    __builtin_memcpy(&tmp, (char *)((int)auStack_14 + sVar6 * 2), 2);
    uVar13 = (unsigned int)tmp;
    __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
    if ((*(short *)(uVar13 * 2 + 0x2ac + iVar3) == 0) && ((mask & DAT_0051bf60[uVar13]) != 0)) {
      uVar12 = (uVar12 & 0xffff0000) | (unsigned short)((short)uVar12 + 1);
      bVar4 = 0;
    }
    sVar6 = (short)(sVar6 + 1);
  } while (sVar6 < 10);
  if (bVar4) {
    DAT_00a08ba6 = 0;
  }
  FUN_0040e0c0(DAT_009a806c, DAT_009a7bac, DAT_009a8c14,
               (int)((unsigned int)DAT_005233d8 * (unsigned int)DAT_009a7bac));
  FUN_004a9640('\0');
  sVar5 = DAT_00a08c90;
  sVar10 = DAT_00a08c82;
  sVar7 = (short)(DAT_00a08c82 + 2);
  sVar14 = (short)(DAT_00a08c90 + 2);
  sVar6 = (short)(DAT_00a08c82 + -2 + DAT_00a08c8e);
  iVar17 = (int)((((unsigned int)extraout_var_01 << 16) | (unsigned short)DAT_00a08c8a) +
                 (unsigned int)-2 +
                 (((unsigned int)extraout_var_00 << 16) | (unsigned short)DAT_00a08c90));
  uVar8 = (unsigned short)(DAT_009a20ae + 1);
  sVar15 = (short)uVar12;
  if (DAT_00a0a2ea[DAT_00a08374] == DAT_00a08ba6) {
    FUN_004a95c0((unsigned int)DAT_005233c8);
    FUN_004a9630(1);
  }
  else {
    FUN_004a95c0((unsigned int)DAT_005233c8);
    sVar16 = (short)((short)iVar17 + 2);
    iVar17 = (int)(((unsigned int)iVar17 & 0xffff0000) | (unsigned short)sVar16);
    FUN_004a9520(sVar10, sVar5, (short)(sVar6 + 2), sVar16);
    (*DAT_009d1300)();
    if ((DAT_00a08ba6 == -2) || bVar4) {
      if (DAT_00521548 == 3) {
        DAT_00521548 = DAT_0052154c;
        uVar12 = FUN_00480530();
        DAT_0052154c = (short)uVar12;
      }
      if (bVar4) {
        DAT_00a0a2ea[DAT_00a08374] = (short)0xfffe;
      }
      uVar12 = FUN_0040e0c0(0, 0x140, 200, 0);
      return uVar12 & 0xffff0000;
    }
    FUN_004a95c0((unsigned int)DAT_005233c8);
    FUN_004a9630(1);
    FUN_004a9590(0x46);
    (*DAT_009d1300)();
    FUN_004a9500(sVar7, sVar14);
    FUN_004a9a50(DAT_0099f928);
    DAT_00559790 = sVar15;
  }
  local_1a = (unsigned short)(sVar14 + uVar8);
  sVar14 = (short)((unsigned int)sVar15 * (unsigned int)uVar8 + 2u + (unsigned int)local_1a);
  if (sVar15 != DAT_00559790) {
    FUN_004a9520(sVar10, (short)local_1a, (short)(sVar6 + 2),
                 (short)((unsigned int)(DAT_00559790 + 1) * (unsigned int)uVar8 + (unsigned int)local_1a));
    (*DAT_009d1300)();
    DAT_00a004d4 = 0xffff;
    DAT_00559790 = sVar15;
  }
  if ((DAT_009e9683[(unsigned int)DAT_009ecc34 * 0x5bd] == '\0') && (DAT_00521548 == -1)) {
    DAT_00521548 = 3;
  }
  if (DAT_00521548 == 3) {
    uVar13 = 0x46;
LAB_0046baa5:
    FUN_004a95c0(uVar13);
    uVar12 = (uVar12 & 0xffff0000) | (unsigned short)(sVar6 + 2);
    FUN_004a9520(sVar10, sVar5, (short)(sVar6 + 2), (short)(sVar14 + 2));
    (*DAT_009eca20)((((unsigned int)iVar17) & 0xffff0000) | (unsigned short)sVar10,
                    (unsigned int)(int)sVar5, uVar12,
                    ((unsigned int)uStack_16 << 16) | (unsigned short)(sVar14 + 2), 1);
  }
  else if (DAT_0052154c == 3) {
    uVar13 = (unsigned int)DAT_005233c8;
    goto LAB_0046baa5;
  }
  FUN_004a9520(sVar7, (short)local_1a, sVar6, sVar14);
  FUN_004a95c0((unsigned int)DAT_005233c8);
  FUN_004a9590(0x43);
  DAT_00559794 = 1;
  if (DAT_00a0a2ea[DAT_00a08374] == DAT_00a08ba6) {
    if ((DAT_00521548 == 3) && (DAT_00a080f8 == 1)) {
      if (DAT_009d6930 == 0xd) {
        uVar12 = uVar12 & 0xffff0000;
        bVar1 = *(unsigned char *)((short)DAT_00a004d4 + 0x2a1 + iVar3);
        do {
          sVar10 = (short)uVar12;
          bVar2 = *(unsigned char *)(sVar10 + 0x2a1 + iVar3);
          if (bVar2 < bVar1) {
            *(unsigned char *)(sVar10 + 0x2a1 + iVar3) = (unsigned char)(bVar2 + 1);
          }
          uVar12 = (uVar12 & 0xffff0000) | (unsigned short)(sVar10 + 1);
        } while ((short)(sVar10 + 1) < 10);
        *(unsigned char *)((short)DAT_00a004d4 + 0x2a1 + iVar3) = 0;
      }
      else if (DAT_009d6930 == 0xa6) {
        uVar13 = (unsigned int)DAT_00a004d4;
        uVar12 = 0;
        do {
          do {
            uVar13 = FUN_0046be10((unsigned short)uVar13, -1);
            uVar9 = (unsigned short)uVar13;
            __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
          } while ((mask & DAT_0051bf60[(short)uVar9]) == 0);
          if (*(short *)((short)uVar9 * 2 + 0x2ac + iVar3) == 0) {
            DAT_00a004d4 = uVar9;
          }
          __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
        } while ((mask & DAT_0051bf60[(short)uVar9]) == 0);
      }
      else {
        if (DAT_009d6930 != 0xa7) goto LAB_0046bc8d;
        uVar13 = (unsigned int)DAT_00a004d4;
        uVar12 = 0;
        do {
          uVar13 = FUN_0046be10((unsigned short)uVar13, 1);
          uVar9 = (unsigned short)uVar13;
          __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
          if (((mask & DAT_0051bf60[(short)uVar9]) != 0) &&
              (*(short *)((short)uVar9 * 2 + 0x2ac + iVar3) == 0)) {
            DAT_00a004d4 = uVar9;
          }
          __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
        } while ((mask & DAT_0051bf60[(short)DAT_00a004d4]) == 0);
      }
      _DAT_0055979c = 1;
    }
  }
  else {
    DAT_00a004d4 = 0xffff;
    _DAT_0055979c = 1;
  }
LAB_0046bc8d:
  sVar10 = 0;
  do {
    *(short *)((int)auStack_14 + (unsigned int)*(unsigned char *)(sVar10 + 0x2a1 + iVar3) * 2) = sVar10;
    sVar10 = (short)(sVar10 + 1);
  } while (sVar10 < 10);
  sStack_24 = 0;
  do {
    uVar9 = *(unsigned short *)((int)auStack_14 + sStack_24 * 2);
    uVar12 = (uVar12 & 0xffff0000) | uVar9;
    uVar13 = (unsigned int)uVar9;
    uVar11 = local_1a;
    __builtin_memcpy(&mask, (char *)(iVar3 + 0x1e3), 2);
    if (((*(short *)(iVar3 + 0x2ac + uVar13 * 2) == 0) &&
         ((mask & DAT_0051bf60[uVar13]) != 0)) &&
        ((int)((unsigned int)uVar8 + (unsigned int)local_1a) < (int)sVar14)) {
      uVar11 = (unsigned short)(uVar8 + local_1a);
      FUN_004a9520(sVar7, (short)local_1a, sVar6, (short)uVar11);
      FUN_004a9500(sVar7, (short)local_1a);
      if (DAT_00a004d4 == 0xffff) {
        DAT_00559798 = uVar9;
        DAT_00a004d4 = uVar9;
      }
      if ((int)(short)DAT_00a004d4 == (int)uVar13) {
        uVar18 = 0x33;
      }
      else {
        uVar18 = (unsigned int)DAT_005233c8;
      }
      FUN_004a95c0(uVar18);
      if (((DAT_00559794 != 0) || ((int)(short)DAT_00a004d4 == (int)uVar13)) ||
          ((int)(short)DAT_00559798 == (int)uVar13)) {
        FUN_0046bf90(uVar12, local_1a, sVar7);
      }
    }
    local_1a = uVar11;
    sStack_24 = (short)(sStack_24 + 1);
  } while (sStack_24 < 10);
  _DAT_0055979c = 0;
  DAT_00559794 = 0;
  DAT_00559798 = DAT_00a004d4;
  uVar12 = FUN_0040e0c0(0, 0x140, 200, 0);
  return (uVar12 & 0xffff0000) | 1;
}
