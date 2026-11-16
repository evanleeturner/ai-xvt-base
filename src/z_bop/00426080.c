/* SPDX-License-Identifier: MIT */
/* z_bop 00426080 */
extern int DAT_00a00488;
extern int DAT_00a00490;
extern int DAT_00a00480;
extern int DAT_009fe7f4;
extern int DAT_00a004b8;
extern int DAT_00a004cc;
extern int DAT_00a004b0;
extern int DAT_00a004a4;
extern int DAT_00a004a8;
extern int DAT_009d80d0[16];
extern int DAT_009a8dd0[16];
extern int DAT_009d1270[16];
extern int DAT_009fe740[16];
extern int DAT_009e9600[16];
extern int DAT_009ec480[16];
extern int DAT_00a08250[16];
extern int DAT_00a004e0[16];
extern int DAT_00a07c80[16];
extern int DAT_009a7810[128];
extern int DAT_009a7400[128];
extern int DAT_009a7600[128];
extern char DAT_00a080fc;
extern unsigned char DAT_00523530[];
extern unsigned short DAT_00523570;
extern unsigned short DAT_00523574;
extern unsigned short DAT_00523578;
extern unsigned short DAT_0052357c;
extern unsigned short DAT_00523580;
extern unsigned short DAT_00523584;

/* Calling conventions of these two are not evidenced; left as default. */
int FUN_0046abf0(int a, int b);
void FUN_00426860(int a, int b, unsigned int c, short d, unsigned int e);

void FUN_00426080(void)
{
  unsigned char bVar1;
  unsigned int iVar2;
  int uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  unsigned int iVar6;
  unsigned int iVar7;
  unsigned int iVar8;
  unsigned int iVar9;
  unsigned int uVar10;
  unsigned int iVar11;
  unsigned int iVar12;
  unsigned int iVar13;
  unsigned int local_20;
  unsigned int local_1c;
  unsigned int local_18;
  unsigned int local_14;
  unsigned int local_10;
  unsigned int local_c;
  unsigned int local_8;
  unsigned int local_4;

  iVar13 = (unsigned int)DAT_00a00488;
  iVar9 = 0;
  iVar12 = 0;
  iVar8 = 0;
  local_20 = 0;
  iVar11 = 0;
  local_10 = 0;
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  iVar7 = 0;
  do {
    iVar2 = iVar7 + 4;
    *(int *)((char *)DAT_009d80d0 + iVar7) = (int)iVar11 >> 5;
    *(int *)((char *)DAT_009a8dd0 + iVar7) = (int)local_14 >> 5;
    *(int *)((char *)DAT_009d1270 + iVar7) = (int)local_18 >> 5;
    *(int *)((char *)DAT_009fe740 + iVar7) = (int)iVar8 >> 5;
    *(int *)((char *)DAT_009e9600 + iVar7) = (int)local_1c >> 5;
    *(int *)((char *)DAT_009ec480 + iVar7) = (int)local_10 >> 5;
    iVar6 = (unsigned int)((int)iVar12 >> 5);
    iVar12 = iVar12 + iVar13;
    *(int *)((char *)DAT_00a08250 + iVar7) = (int)iVar6;
    *(int *)((char *)DAT_00a004e0 + iVar7) = (int)local_20 >> 5;
    *(int *)((char *)DAT_00a07c80 + iVar7) = (int)iVar9 >> 5;
    iVar9 = iVar9 + (unsigned int)DAT_00a00490;
    local_20 = local_20 + (unsigned int)DAT_00a00480;
    local_10 = local_10 + (unsigned int)DAT_009fe7f4;
    local_1c = local_1c + (unsigned int)DAT_00a004b8;
    iVar8 = iVar8 + (unsigned int)DAT_00a004cc;
    local_18 = local_18 + (unsigned int)DAT_00a004b0;
    local_14 = local_14 + (unsigned int)DAT_00a004a4;
    iVar11 = iVar11 + (unsigned int)DAT_00a004a8;
    iVar7 = iVar2;
  } while ((int)iVar2 < 0x40);
  iVar7 = 0;
  local_c = ((unsigned int)DAT_009fe7f4 + (unsigned int)DAT_00a00490 + (unsigned int)DAT_00a004b0) * 0xfffffffeu;
  local_8 = ((unsigned int)DAT_00a004b8 + (unsigned int)DAT_00a00480 + (unsigned int)DAT_00a004a4) * 0xfffffffeu;
  local_10 = 5;
  local_4 = ((unsigned int)DAT_00a004a8 + iVar13 + (unsigned int)DAT_00a004cc) * 0xfffffffeu;
  do {
    local_1c = local_4;
    local_18 = local_8;
    local_14 = local_c;
    local_20 = 5;
    do {
      iVar2 = 5;
      iVar8 = iVar7;
      iVar9 = local_18;
      iVar11 = local_1c;
      iVar12 = local_14;
      do {
        iVar7 = iVar8 + 4;
        iVar6 = (unsigned int)((int)iVar11 >> 7);
        iVar11 = iVar11 + iVar13;
        *(int *)((char *)DAT_009a7810 + iVar8) = (int)iVar6;
        *(int *)((char *)DAT_009a7400 + iVar8) = (int)iVar9 >> 7;
        *(int *)((char *)DAT_009a7600 + iVar8) = (int)iVar12 >> 7;
        iVar12 = iVar12 + (unsigned int)DAT_00a00490;
        iVar9 = iVar9 + (unsigned int)DAT_00a00480;
        iVar2 = iVar2 - 1;
        iVar8 = iVar7;
      } while (iVar2 != 0);
      local_1c = local_1c + (unsigned int)DAT_00a004cc;
      local_18 = local_18 + (unsigned int)DAT_00a004b8;
      local_14 = local_14 + (unsigned int)DAT_009fe7f4;
      local_20 = local_20 - 1;
    } while (local_20 != 0);
    local_c = local_c + (unsigned int)DAT_00a004b0;
    local_8 = local_8 + (unsigned int)DAT_00a004a4;
    local_4 = local_4 + (unsigned int)DAT_00a004a8;
    local_10 = local_10 - 1;
  } while (local_10 != 0);
  if (DAT_00a080fc != '\0') {
    iVar13 = 0;
    uVar3 = FUN_0046abf0(DAT_00a004a4, DAT_00a004a8);
    if (DAT_009fe7f4 < 0) {
      uVar10 = (unsigned int)DAT_00523570;
      uVar4 = (unsigned int)DAT_00523574;
      while (uVar4 != 0) {
        uVar4 = uVar4 - 1;
        bVar1 = DAT_00523530[uVar10];
        uVar10 = uVar10 + 1;
        uVar5 = bVar1 & 7;
        iVar13 = (unsigned int)DAT_009d80d0[uVar5];
        iVar7 = (unsigned int)DAT_009a8dd0[uVar5];
        iVar8 = (unsigned int)DAT_009d1270[uVar5];
        if ((bVar1 & 8) != 0) {
          iVar13 = -iVar13;
          iVar7 = -iVar7;
          iVar8 = -iVar8;
        }
        if ((bVar1 & 0x80) == 0) {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = (unsigned int)*(int *)((char *)DAT_00a08250 + uVar5);
          iVar11 = (unsigned int)*(int *)((char *)DAT_00a004e0 + uVar5);
          iVar12 = (unsigned int)*(int *)((char *)DAT_00a07c80 + uVar5);
        }
        else {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = -(unsigned int)*(int *)((char *)DAT_00a08250 + uVar5);
          iVar11 = -(unsigned int)*(int *)((char *)DAT_00a004e0 + uVar5);
          iVar12 = -(unsigned int)*(int *)((char *)DAT_00a07c80 + uVar5);
        }
        uVar5 = (iVar8 + iVar12) - (unsigned int)(DAT_009fe7f4 >> 2);
        if (-1 < (int)uVar5) {
          FUN_00426860((int)((iVar13 + iVar9) - (unsigned int)(DAT_00a004cc >> 2)),
                       (int)((iVar7 + iVar11) - (unsigned int)(DAT_00a004b8 >> 2)),
                       uVar5, (short)(-(short)uVar3), uVar10);
        }
      }
    }
    else {
      uVar10 = (unsigned int)DAT_00523570;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00523530[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = (unsigned int)DAT_009a8dd0[uVar4];
        iVar8 = (unsigned int)DAT_009d1270[uVar4];
        iVar9 = (unsigned int)DAT_009d80d0[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar9 = -iVar9;
          iVar7 = -iVar7;
          iVar8 = -iVar8;
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = (unsigned int)*(int *)((char *)DAT_00a08250 + uVar4);
          iVar12 = (unsigned int)*(int *)((char *)DAT_00a004e0 + uVar4);
          iVar2 = (unsigned int)*(int *)((char *)DAT_00a07c80 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = -(unsigned int)*(int *)((char *)DAT_00a08250 + uVar4);
          iVar12 = -(unsigned int)*(int *)((char *)DAT_00a004e0 + uVar4);
          iVar2 = -(unsigned int)*(int *)((char *)DAT_00a07c80 + uVar4);
        }
        uVar4 = iVar8 + iVar2 + (unsigned int)(DAT_009fe7f4 >> 2);
        if (-1 < (int)uVar4) {
          FUN_00426860((int)(iVar9 + iVar11 + (unsigned int)(DAT_00a004cc >> 2)),
                       (int)(iVar7 + iVar12 + (unsigned int)(DAT_00a004b8 >> 2)),
                       uVar4, (short)(-(short)uVar3), iVar13);
        }
      }
      uVar10 = iVar13 + (unsigned int)DAT_00523574;
    }
    uVar3 = FUN_0046abf0(DAT_00a004b8, DAT_00a004cc);
    if (DAT_00a004b0 < 0) {
      iVar13 = uVar10 + (unsigned int)DAT_00523580;
      uVar10 = (unsigned int)DAT_00523584;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00523530[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = (unsigned int)DAT_009fe740[uVar4];
        iVar8 = (unsigned int)DAT_009e9600[uVar4];
        iVar9 = (unsigned int)DAT_009ec480[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar7 = -iVar7;
          iVar8 = -iVar8;
          iVar9 = -iVar9;
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = (unsigned int)*(int *)((char *)DAT_00a08250 + uVar4);
          iVar12 = (unsigned int)*(int *)((char *)DAT_00a004e0 + uVar4);
          iVar2 = (unsigned int)*(int *)((char *)DAT_00a07c80 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = -(unsigned int)*(int *)((char *)DAT_00a08250 + uVar4);
          iVar12 = -(unsigned int)*(int *)((char *)DAT_00a004e0 + uVar4);
          iVar2 = -(unsigned int)*(int *)((char *)DAT_00a07c80 + uVar4);
        }
        uVar4 = (iVar9 + iVar2) - (unsigned int)(DAT_00a004b0 >> 2);
        if (-1 < (int)uVar4) {
          FUN_00426860((int)((iVar7 + iVar11) - (unsigned int)(DAT_00a004a8 >> 2)),
                       (int)((iVar8 + iVar12) - (unsigned int)(DAT_00a004a4 >> 2)),
                       uVar4, (short)(-(short)uVar3), iVar13);
        }
      }
    }
    else {
      uVar4 = (unsigned int)DAT_00523580;
      while (uVar4 != 0) {
        uVar4 = uVar4 - 1;
        bVar1 = DAT_00523530[uVar10];
        uVar10 = uVar10 + 1;
        uVar5 = bVar1 & 7;
        iVar13 = (unsigned int)DAT_009fe740[uVar5];
        iVar7 = (unsigned int)DAT_009e9600[uVar5];
        iVar8 = (unsigned int)DAT_009ec480[uVar5];
        if ((bVar1 & 8) != 0) {
          iVar13 = -iVar13;
          iVar7 = -iVar7;
          iVar8 = -iVar8;
        }
        if ((bVar1 & 0x80) == 0) {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = (unsigned int)*(int *)((char *)DAT_00a08250 + uVar5);
          iVar11 = (unsigned int)*(int *)((char *)DAT_00a004e0 + uVar5);
          iVar12 = (unsigned int)*(int *)((char *)DAT_00a07c80 + uVar5);
        }
        else {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = -(unsigned int)*(int *)((char *)DAT_00a08250 + uVar5);
          iVar11 = -(unsigned int)*(int *)((char *)DAT_00a004e0 + uVar5);
          iVar12 = -(unsigned int)*(int *)((char *)DAT_00a07c80 + uVar5);
        }
        uVar5 = iVar8 + iVar12 + (unsigned int)(DAT_00a004b0 >> 2);
        if (-1 < (int)uVar5) {
          FUN_00426860((int)(iVar13 + iVar9 + (unsigned int)(DAT_00a004a8 >> 2)),
                       (int)(iVar7 + iVar11 + (unsigned int)(DAT_00a004a4 >> 2)),
                       uVar5, (short)(-(short)uVar3), uVar10);
        }
      }
      iVar13 = uVar10 + (unsigned int)DAT_00523584;
    }
    uVar3 = FUN_0046abf0(DAT_00a004a4, DAT_00a004a8);
    if (DAT_00a00490 < 0) {
      iVar13 = iVar13 + (unsigned int)DAT_00523578;
      uVar10 = (unsigned int)DAT_0052357c;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00523530[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = (unsigned int)DAT_009fe740[uVar4];
        iVar8 = (unsigned int)DAT_009e9600[uVar4];
        iVar9 = (unsigned int)DAT_009ec480[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar7 = -iVar7;
          iVar8 = -iVar8;
          iVar9 = -iVar9;
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = (unsigned int)*(int *)((char *)DAT_009d80d0 + uVar4);
          iVar12 = (unsigned int)*(int *)((char *)DAT_009a8dd0 + uVar4);
          iVar2 = (unsigned int)*(int *)((char *)DAT_009d1270 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar11 = -(unsigned int)*(int *)((char *)DAT_009d80d0 + uVar4);
          iVar12 = -(unsigned int)*(int *)((char *)DAT_009a8dd0 + uVar4);
          iVar2 = -(unsigned int)*(int *)((char *)DAT_009d1270 + uVar4);
        }
        uVar4 = (iVar9 + iVar2) - (unsigned int)(DAT_00a00490 >> 2);
        if (-1 < (int)uVar4) {
          FUN_00426860((int)((iVar7 + iVar11) - (unsigned int)(DAT_00a00488 >> 2)),
                       (int)((iVar8 + iVar12) - (unsigned int)(DAT_00a00480 >> 2)),
                       uVar4, (short)(-(short)uVar3), iVar13);
        }
      }
    }
    else {
      uVar10 = (unsigned int)DAT_00523578;
      if (uVar10 != 0) {
        do {
          uVar10 = uVar10 - 1;
          bVar1 = DAT_00523530[iVar13];
          iVar13 = iVar13 + 1;
          uVar4 = bVar1 & 7;
          iVar7 = (unsigned int)DAT_009fe740[uVar4];
          iVar8 = (unsigned int)DAT_009e9600[uVar4];
          iVar9 = (unsigned int)DAT_009ec480[uVar4];
          if ((bVar1 & 8) != 0) {
            iVar7 = -iVar7;
            iVar8 = -iVar8;
            iVar9 = -iVar9;
          }
          if ((bVar1 & 0x80) == 0) {
            uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
            iVar11 = (unsigned int)*(int *)((char *)DAT_009d80d0 + uVar4);
            iVar12 = (unsigned int)*(int *)((char *)DAT_009a8dd0 + uVar4);
            iVar2 = (unsigned int)*(int *)((char *)DAT_009d1270 + uVar4);
          }
          else {
            uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
            iVar11 = -(unsigned int)*(int *)((char *)DAT_009d80d0 + uVar4);
            iVar12 = -(unsigned int)*(int *)((char *)DAT_009a8dd0 + uVar4);
            iVar2 = -(unsigned int)*(int *)((char *)DAT_009d1270 + uVar4);
          }
          uVar4 = iVar9 + iVar2 + (unsigned int)(DAT_00a00490 >> 2);
          if (-1 < (int)uVar4) {
            FUN_00426860((int)(iVar7 + iVar11 + (unsigned int)(DAT_00a00488 >> 2)),
                         (int)(iVar8 + iVar12 + (unsigned int)(DAT_00a00480 >> 2)),
                         uVar4, (short)(-(short)uVar3), iVar13);
          }
        } while (uVar10 != 0);
        return;
      }
    }
  }
  return;
}
