/* SPDX-License-Identifier: MIT */
/* z_xvt 0047ea40 */
void FUN_0048ed00(short param_1, short param_2, short param_3, short param_4);
int FUN_004a6a70(void);
void FUN_004a6f90(void);
void FUN_004a6a80(void);
void FUN_0048ee20(char param_1);
void FUN_0048ece0(short param_1, short param_2);
void __cdecl FUN_0048f270(unsigned char *param_1);
void FUN_004a7130(void);
void FUN_004a7040(void);
unsigned int FUN_004a5470(void);

extern unsigned short DAT_00732e8c;
extern unsigned short DAT_00732e58;
extern unsigned short DAT_00732e52;
extern unsigned char  DAT_00732e50;
extern unsigned short DAT_00732a34;
extern unsigned short DAT_007329c0;
extern unsigned short DAT_00730894;
extern unsigned char  DAT_0071e524;
extern unsigned char  DAT_0071b012;
extern unsigned short DAT_0070acc0;
extern char           DAT_0070a188;
extern unsigned short DAT_006ffbf0;
extern unsigned char  DAT_006da9fa;
extern unsigned short DAT_006d9980;
extern unsigned char  DAT_0071f2eb;
extern int            DAT_006db594;
extern int            DAT_006da538;
extern unsigned char  DAT_006d4a4e;
extern int            DAT_006a89ec;
extern void         (*DAT_00703c80)(void);

int __cdecl FUN_0047ea40(char *param_1)
{
  char cVar1;
  unsigned short uVar2;
  unsigned char uVar3;
  unsigned short uVar4;
  char cVar5;
  unsigned short uVar6;
  unsigned char uVar7;
  unsigned char uVar8;
  unsigned char uVar9;
  unsigned short uVar10;
  unsigned short uVar11;
  unsigned short uVar12;
  unsigned char uVar13;
  unsigned short uVar14;
  unsigned short uVar15;
  unsigned short uVar16;
  short sVar17;
  int iVar18;
  short sVar19;
  unsigned int uVar20;
  unsigned int uVar21;
  int iVar22;
  unsigned char *pbVar23;
  char *pcVar24;
  unsigned char *pbVar25;
  unsigned char abStack_100[256];

  uVar16 = DAT_00732e8c;
  uVar15 = DAT_00732e58;
  uVar14 = DAT_00732e52;
  uVar13 = DAT_00732e50;
  uVar12 = DAT_00732a34;
  uVar11 = DAT_007329c0;
  uVar10 = DAT_00730894;
  uVar8 = DAT_0071e524;
  uVar7 = DAT_0071b012;
  uVar6 = DAT_0070acc0;
  cVar5 = DAT_0070a188;
  uVar4 = DAT_006ffbf0;
  uVar3 = DAT_006da9fa;
  uVar2 = DAT_006d9980;
  iVar18 = FUN_004a6a70();
  iVar22 = iVar18;
  if (0 < iVar18) {
    do {
      FUN_004a6f90();
      iVar22 = iVar22 + -1;
    } while (iVar22 != 0);
  }
  uVar9 = DAT_0071f2eb;
  DAT_0071f2eb = 1;
  FUN_004a6a80();
  FUN_0048ee20('\x01');
  sVar17 = (short)(DAT_006db594 >> 1);
  sVar19 = (short)(DAT_006da538 >> 4);
  FUN_0048ed00(sVar19,
               (short)((unsigned short)sVar17 + (unsigned short)((unsigned int)DAT_006d4a4e * 0xfffffffeU)),
               (short)((unsigned short)(short)DAT_006da538 - (unsigned short)sVar19),
               (short)((unsigned short)((unsigned int)DAT_006d4a4e * 2U) + (unsigned short)sVar17));
  DAT_00732e50 = 0xf9;
  (*DAT_00703c80)();
  sVar19 = (short)(DAT_006db594 >> 1);
  sVar17 = (short)(DAT_006da538 >> 4);
  FUN_0048ed00((short)((unsigned short)sVar17 + 1),
               (short)((unsigned short)sVar19 + (unsigned short)((unsigned int)DAT_006d4a4e * 0xfffffffeU) + 1),
               (short)(((unsigned short)(short)DAT_006da538 - (unsigned short)sVar17) + 0xffffU),
               (short)((unsigned short)((unsigned int)DAT_006d4a4e * 2U) + (unsigned short)sVar19 + 0xffffU));
  DAT_00732e50 = 0;
  (*DAT_00703c80)();
  DAT_006da9fa = 0xf9;
  DAT_0071b012 = 0;
  DAT_0071e524 = 0;
  FUN_0048ece0(0, (short)(((unsigned short)(short)(DAT_006db594 >> 1) - (unsigned short)DAT_006d4a4e) + 0xfffeU));
  uVar20 = 0xffffffff;
  do {
    pcVar24 = param_1;
    if (uVar20 == 0) break;
    uVar20 = uVar20 - 1;
    pcVar24 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar24;
  } while (cVar1 != '\0');
  uVar20 = ~uVar20;
  pbVar23 = (unsigned char *)(pcVar24 - uVar20);
  pbVar25 = abStack_100;
  for (uVar21 = uVar20 >> 2; uVar21 != 0; uVar21 = uVar21 - 1) {
    __builtin_memcpy(pbVar25, pbVar23, 4);
    pbVar23 = pbVar23 + 4;
    pbVar25 = pbVar25 + 4;
  }
  for (uVar21 = uVar20 & 3; uVar21 != 0; uVar21 = uVar21 - 1) {
    *pbVar25 = *pbVar23;
    pbVar23 = pbVar23 + 1;
    pbVar25 = pbVar25 + 1;
  }
  FUN_0048f270(abStack_100);
  FUN_0048ece0(0, (short)((unsigned short)(short)(DAT_006db594 >> 1) + 2));
  FUN_0048f270((unsigned char *)DAT_006a89ec);
  FUN_004a6f90();
  FUN_004a7130();
  FUN_004a7040();
  uVar20 = FUN_004a5470();
  DAT_0071f2eb = uVar9;
  if (0 < iVar18) {
    do {
      FUN_004a6a80();
      iVar18 = iVar18 + -1;
    } while (iVar18 != 0);
  }
  FUN_0048ee20(cVar5);
  DAT_006d9980 = uVar2;
  DAT_006da9fa = uVar3;
  DAT_006ffbf0 = uVar4;
  DAT_0070acc0 = uVar6;
  DAT_0071b012 = uVar7;
  DAT_0071e524 = uVar8;
  DAT_00730894 = uVar10;
  DAT_007329c0 = uVar11;
  DAT_00732a34 = uVar12;
  DAT_00732e50 = uVar13;
  DAT_00732e52 = uVar14;
  DAT_00732e58 = uVar15;
  DAT_00732e8c = uVar16;
  iVar18 = FUN_004a6a70();
  iVar22 = iVar18;
  if (0 < iVar18) {
    do {
      FUN_004a6f90();
      iVar22 = iVar22 + -1;
    } while (iVar22 != 0);
  }
  FUN_004a7130();
  FUN_004a7040();
  if (0 < iVar18) {
    do {
      FUN_004a6a80();
      iVar18 = iVar18 + -1;
    } while (iVar18 != 0);
  }
  return (int)(char)uVar20;
}
