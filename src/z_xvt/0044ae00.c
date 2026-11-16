/* SPDX-License-Identifier: MIT */
/* z_xvt 0044ae00 */
extern int DAT_0070a190[16];
extern int DAT_006db750[16];
extern int DAT_00703bf0[16];
extern int DAT_007307f0[16];
extern int DAT_0071b6c0[16];
extern int DAT_0071e540[16];
extern int DAT_00732fa0[16];
extern int DAT_00732590[16];
extern int DAT_007329d0[16];
extern int DAT_006da1b0[125];
extern int DAT_006d9da0[125];
extern int DAT_006d9fa0[125];
extern int DAT_0073253c;
extern int DAT_00732548;
extern int DAT_00732538;
extern int DAT_00732530;
extern int DAT_00732578;
extern int DAT_00732584;
extern int DAT_00732568;
extern int DAT_00732558;
extern int DAT_0073255c;
extern char DAT_00732e4c;
extern unsigned char DAT_00525670[];
extern unsigned short DAT_005256b0;
extern unsigned short DAT_005256b4;
extern unsigned short DAT_005256b8;
extern unsigned short DAT_005256bc;
extern unsigned short DAT_005256c0;
extern unsigned short DAT_005256c4;

unsigned int FUN_00419100(int a, int b);
void FUN_0044b5d0(int a, int b, unsigned int c, short d, unsigned int e);

void __stdcall FUN_0044ae00(void)
{
  unsigned char bVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  unsigned int uVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  int local_c;
  int local_8;
  int local_4;

  iVar13 = DAT_0073253c;
  iVar8 = 0;
  iVar11 = 0;
  iVar9 = 0;
  iVar6 = 0;
  local_20 = 0;
  local_10 = 0;
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  iVar7 = 0;
  do {
    iVar2 = iVar7 + 4;
    DAT_0070a190[iVar7 / 4] = iVar6 >> 5;
    DAT_006db750[iVar7 / 4] = local_14 >> 5;
    DAT_00703bf0[iVar7 / 4] = local_18 >> 5;
    DAT_007307f0[iVar7 / 4] = local_1c >> 5;
    DAT_0071b6c0[iVar7 / 4] = local_10 >> 5;
    DAT_0071e540[iVar7 / 4] = local_20 >> 5;
    iVar12 = iVar11 >> 5;
    iVar11 = (int)((unsigned int)iVar11 + (unsigned int)iVar13);
    DAT_00732fa0[iVar7 / 4] = iVar12;
    DAT_00732590[iVar7 / 4] = iVar8 >> 5;
    DAT_007329d0[iVar7 / 4] = iVar9 >> 5;
    iVar9 = (int)((unsigned int)iVar9 + (unsigned int)DAT_00732548);
    iVar8 = (int)((unsigned int)iVar8 + (unsigned int)DAT_00732538);
    local_20 = (int)((unsigned int)local_20 + (unsigned int)DAT_00732530);
    local_10 = (int)((unsigned int)local_10 + (unsigned int)DAT_00732578);
    local_1c = (int)((unsigned int)local_1c + (unsigned int)DAT_00732584);
    local_18 = (int)((unsigned int)local_18 + (unsigned int)DAT_00732568);
    local_14 = (int)((unsigned int)local_14 + (unsigned int)DAT_00732558);
    iVar6 = (int)((unsigned int)iVar6 + (unsigned int)DAT_0073255c);
    iVar7 = iVar2;
  } while (iVar2 < 0x40);
  iVar7 = 0;
  local_c = (int)((unsigned int)(int)((unsigned int)DAT_00732568 + (unsigned int)DAT_00732530 + (unsigned int)DAT_00732548) * 0xFFFFFFFEu);
  local_8 = (int)((unsigned int)(int)((unsigned int)DAT_00732578 + (unsigned int)DAT_00732538 + (unsigned int)DAT_00732558) * 0xFFFFFFFEu);
  local_10 = 5;
  local_4 = (int)((unsigned int)(int)((unsigned int)DAT_00732584 + (unsigned int)iVar13 + (unsigned int)DAT_0073255c) * 0xFFFFFFFEu);
  do {
    local_1c = local_4;
    local_18 = local_8;
    local_14 = local_c;
    local_20 = 5;
    do {
      iVar2 = 5;
      iVar6 = local_14;
      iVar8 = iVar7;
      iVar9 = local_1c;
      iVar11 = local_18;
      do {
        iVar7 = iVar8 + 4;
        iVar12 = iVar9 >> 7;
        iVar9 = (int)((unsigned int)iVar9 + (unsigned int)iVar13);
        DAT_006da1b0[iVar8 / 4] = iVar12;
        DAT_006d9da0[iVar8 / 4] = iVar11 >> 7;
        DAT_006d9fa0[iVar8 / 4] = iVar6 >> 7;
        iVar6 = (int)((unsigned int)iVar6 + (unsigned int)DAT_00732548);
        iVar11 = (int)((unsigned int)iVar11 + (unsigned int)DAT_00732538);
        iVar2 = iVar2 + -1;
        iVar8 = iVar7;
      } while (iVar2 != 0);
      local_1c = (int)((unsigned int)local_1c + (unsigned int)DAT_00732584);
      local_18 = (int)((unsigned int)local_18 + (unsigned int)DAT_00732578);
      local_14 = (int)((unsigned int)local_14 + (unsigned int)DAT_00732530);
      local_20 = local_20 + -1;
    } while (local_20 != 0);
    local_c = (int)((unsigned int)local_c + (unsigned int)DAT_00732568);
    local_8 = (int)((unsigned int)local_8 + (unsigned int)DAT_00732558);
    local_4 = (int)((unsigned int)local_4 + (unsigned int)DAT_0073255c);
    local_10 = local_10 + -1;
  } while (local_10 != 0);
  if (DAT_00732e4c != '\0') {
    iVar13 = 0;
    uVar3 = FUN_00419100(DAT_00732558, DAT_0073255c);
    if (DAT_00732530 < 0) {
      uVar10 = (unsigned int)DAT_005256b0;
      uVar4 = (unsigned int)DAT_005256b4;
      while (uVar4 != 0) {
        uVar4 = uVar4 - 1;
        bVar1 = DAT_00525670[uVar10];
        uVar10 = uVar10 + 1;
        uVar5 = bVar1 & 7;
        iVar13 = DAT_0070a190[uVar5];
        iVar7 = DAT_006db750[uVar5];
        iVar6 = DAT_00703bf0[uVar5];
        if ((bVar1 & 8) != 0) {
          iVar13 = (int)(0u - (unsigned int)iVar13);
          iVar7 = (int)(0u - (unsigned int)iVar7);
          iVar6 = (int)(0u - (unsigned int)iVar6);
        }
        if ((bVar1 & 0x80) == 0) {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar8 = *(int *)((char *)DAT_00732fa0 + uVar5);
          iVar9 = *(int *)((char *)DAT_00732590 + uVar5);
          iVar11 = *(int *)((char *)DAT_007329d0 + uVar5);
        }
        else {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar8 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732fa0 + uVar5));
          iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732590 + uVar5));
          iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_007329d0 + uVar5));
        }
        uVar5 = ((unsigned int)iVar6 + (unsigned int)iVar11) - (unsigned int)(DAT_00732530 >> 2);
        if (-1 < (int)uVar5) {
          FUN_0044b5d0((int)(((unsigned int)iVar13 + (unsigned int)iVar8) - (unsigned int)(DAT_00732584 >> 2)),
                       (int)(((unsigned int)iVar7 + (unsigned int)iVar9) - (unsigned int)(DAT_00732578 >> 2)),
                       uVar5, -(short)uVar3, uVar10);
        }
      }
    }
    else {
      uVar10 = (unsigned int)DAT_005256b0;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00525670[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = DAT_0070a190[uVar4];
        iVar6 = DAT_006db750[uVar4];
        iVar8 = DAT_00703bf0[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar7 = (int)(0u - (unsigned int)iVar7);
          iVar6 = (int)(0u - (unsigned int)iVar6);
          iVar8 = (int)(0u - (unsigned int)iVar8);
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = *(int *)((char *)DAT_00732fa0 + uVar4);
          iVar11 = *(int *)((char *)DAT_00732590 + uVar4);
          iVar2 = *(int *)((char *)DAT_007329d0 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732fa0 + uVar4));
          iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732590 + uVar4));
          iVar2 = (int)(0u - (unsigned int)*(int *)((char *)DAT_007329d0 + uVar4));
        }
        uVar4 = (unsigned int)iVar8 + (unsigned int)iVar2 + (unsigned int)(DAT_00732530 >> 2);
        if (-1 < (int)uVar4) {
          FUN_0044b5d0((int)((unsigned int)iVar7 + (unsigned int)iVar9 + (unsigned int)(DAT_00732584 >> 2)),
                       (int)((unsigned int)iVar6 + (unsigned int)iVar11 + (unsigned int)(DAT_00732578 >> 2)),
                       uVar4, -(short)uVar3, (unsigned int)iVar13);
        }
      }
      uVar10 = (unsigned int)iVar13 + (unsigned int)DAT_005256b4;
    }
    uVar3 = FUN_00419100(DAT_00732578, DAT_00732584);
    if (DAT_00732568 < 0) {
      iVar13 = (int)(uVar10 + (unsigned int)DAT_005256c0);
      uVar10 = (unsigned int)DAT_005256c4;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00525670[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = DAT_007307f0[uVar4];
        iVar6 = DAT_0071b6c0[uVar4];
        iVar8 = DAT_0071e540[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar7 = (int)(0u - (unsigned int)iVar7);
          iVar6 = (int)(0u - (unsigned int)iVar6);
          iVar8 = (int)(0u - (unsigned int)iVar8);
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = *(int *)((char *)DAT_00732fa0 + uVar4);
          iVar11 = *(int *)((char *)DAT_00732590 + uVar4);
          iVar2 = *(int *)((char *)DAT_007329d0 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732fa0 + uVar4));
          iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732590 + uVar4));
          iVar2 = (int)(0u - (unsigned int)*(int *)((char *)DAT_007329d0 + uVar4));
        }
        uVar4 = ((unsigned int)iVar8 + (unsigned int)iVar2) - (unsigned int)(DAT_00732568 >> 2);
        if (-1 < (int)uVar4) {
          FUN_0044b5d0((int)(((unsigned int)iVar7 + (unsigned int)iVar9) - (unsigned int)(DAT_0073255c >> 2)),
                       (int)(((unsigned int)iVar6 + (unsigned int)iVar11) - (unsigned int)(DAT_00732558 >> 2)),
                       uVar4, -(short)uVar3, (unsigned int)iVar13);
        }
      }
    }
    else {
      uVar4 = (unsigned int)DAT_005256c0;
      while (uVar4 != 0) {
        uVar4 = uVar4 - 1;
        bVar1 = DAT_00525670[uVar10];
        uVar10 = uVar10 + 1;
        uVar5 = bVar1 & 7;
        iVar13 = DAT_007307f0[uVar5];
        iVar7 = DAT_0071b6c0[uVar5];
        iVar6 = DAT_0071e540[uVar5];
        if ((bVar1 & 8) != 0) {
          iVar13 = (int)(0u - (unsigned int)iVar13);
          iVar7 = (int)(0u - (unsigned int)iVar7);
          iVar6 = (int)(0u - (unsigned int)iVar6);
        }
        if ((bVar1 & 0x80) == 0) {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar8 = *(int *)((char *)DAT_00732fa0 + uVar5);
          iVar9 = *(int *)((char *)DAT_00732590 + uVar5);
          iVar11 = *(int *)((char *)DAT_007329d0 + uVar5);
        }
        else {
          uVar5 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar8 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732fa0 + uVar5));
          iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00732590 + uVar5));
          iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_007329d0 + uVar5));
        }
        uVar5 = (unsigned int)iVar6 + (unsigned int)iVar11 + (unsigned int)(DAT_00732568 >> 2);
        if (-1 < (int)uVar5) {
          FUN_0044b5d0((int)((unsigned int)iVar13 + (unsigned int)iVar8 + (unsigned int)(DAT_0073255c >> 2)),
                       (int)((unsigned int)iVar7 + (unsigned int)iVar9 + (unsigned int)(DAT_00732558 >> 2)),
                       uVar5, -(short)uVar3, uVar10);
        }
      }
      iVar13 = (int)(uVar10 + (unsigned int)DAT_005256c4);
    }
    uVar3 = FUN_00419100(DAT_00732558, DAT_0073255c);
    if (DAT_00732548 < 0) {
      iVar13 = (int)((unsigned int)iVar13 + (unsigned int)DAT_005256b8);
      uVar10 = (unsigned int)DAT_005256bc;
      while (uVar10 != 0) {
        uVar10 = uVar10 - 1;
        bVar1 = DAT_00525670[iVar13];
        iVar13 = iVar13 + 1;
        uVar4 = bVar1 & 7;
        iVar7 = DAT_007307f0[uVar4];
        iVar6 = DAT_0071b6c0[uVar4];
        iVar8 = DAT_0071e540[uVar4];
        if ((bVar1 & 8) != 0) {
          iVar7 = (int)(0u - (unsigned int)iVar7);
          iVar6 = (int)(0u - (unsigned int)iVar6);
          iVar8 = (int)(0u - (unsigned int)iVar8);
        }
        if ((bVar1 & 0x80) == 0) {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = *(int *)((char *)DAT_0070a190 + uVar4);
          iVar11 = *(int *)((char *)DAT_006db750 + uVar4);
          iVar2 = *(int *)((char *)DAT_00703bf0 + uVar4);
        }
        else {
          uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
          iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_0070a190 + uVar4));
          iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_006db750 + uVar4));
          iVar2 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00703bf0 + uVar4));
        }
        uVar4 = ((unsigned int)iVar8 + (unsigned int)iVar2) - (unsigned int)(DAT_00732548 >> 2);
        if (-1 < (int)uVar4) {
          FUN_0044b5d0((int)(((unsigned int)iVar7 + (unsigned int)iVar9) - (unsigned int)(DAT_0073253c >> 2)),
                       (int)(((unsigned int)iVar6 + (unsigned int)iVar11) - (unsigned int)(DAT_00732538 >> 2)),
                       uVar4, -(short)uVar3, (unsigned int)iVar13);
        }
      }
    }
    else {
      uVar10 = (unsigned int)DAT_005256b8;
      if (uVar10 != 0) {
        do {
          uVar10 = uVar10 - 1;
          bVar1 = DAT_00525670[iVar13];
          iVar13 = iVar13 + 1;
          uVar4 = bVar1 & 7;
          iVar7 = DAT_007307f0[uVar4];
          iVar6 = DAT_0071b6c0[uVar4];
          iVar8 = DAT_0071e540[uVar4];
          if ((bVar1 & 8) != 0) {
            iVar7 = (int)(0u - (unsigned int)iVar7);
            iVar6 = (int)(0u - (unsigned int)iVar6);
            iVar8 = (int)(0u - (unsigned int)iVar8);
          }
          if ((bVar1 & 0x80) == 0) {
            uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
            iVar9 = *(int *)((char *)DAT_0070a190 + uVar4);
            iVar11 = *(int *)((char *)DAT_006db750 + uVar4);
            iVar2 = *(int *)((char *)DAT_00703bf0 + uVar4);
          }
          else {
            uVar4 = (unsigned int)((bVar1 & 0x70) >> 2);
            iVar9 = (int)(0u - (unsigned int)*(int *)((char *)DAT_0070a190 + uVar4));
            iVar11 = (int)(0u - (unsigned int)*(int *)((char *)DAT_006db750 + uVar4));
            iVar2 = (int)(0u - (unsigned int)*(int *)((char *)DAT_00703bf0 + uVar4));
          }
          uVar4 = (unsigned int)iVar8 + (unsigned int)iVar2 + (unsigned int)(DAT_00732548 >> 2);
          if (-1 < (int)uVar4) {
            FUN_0044b5d0((int)((unsigned int)iVar7 + (unsigned int)iVar9 + (unsigned int)(DAT_0073253c >> 2)),
                         (int)((unsigned int)iVar6 + (unsigned int)iVar11 + (unsigned int)(DAT_00732538 >> 2)),
                         uVar4, -(short)uVar3, (unsigned int)iVar13);
          }
        } while (uVar10 != 0);
        return;
      }
    }
  }
  return;
}
