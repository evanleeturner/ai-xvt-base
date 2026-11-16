/* SPDX-License-Identifier: MIT */
/* z_bop 0049e420 */
extern unsigned char  DAT_009a20ae;
extern unsigned short DAT_009a6fe0;
extern int            DAT_009a7bac;
extern unsigned char  DAT_009a807a;
extern int            DAT_009a8c14;
extern unsigned short DAT_009cd270;
extern void         (*DAT_009d1300)(void);
extern char           DAT_009d80c8;
extern unsigned short DAT_009d8c00;
extern unsigned char  DAT_009e8f52;
extern unsigned char  DAT_009ec464;
extern unsigned char  DAT_009ed23b;
extern unsigned short DAT_009fe7e4;
extern unsigned short DAT_00a07c64;
extern unsigned short DAT_00a07ce4;
extern unsigned char  DAT_00a08100;
extern unsigned short DAT_00a08102;
extern unsigned short DAT_00a08108;
extern unsigned short DAT_00a0813c;
extern unsigned char *DAT_00a60a4c;

int          __cdecl FUN_004aba40(void);
void         __cdecl FUN_004abe50(void);
void         __cdecl FUN_004aba50(void);
void         __cdecl FUN_004a9640(char param_1);
void         __cdecl FUN_004a9520(short a, short b, short c, short d);
void         __cdecl FUN_004a9500(short a, short b);
void         __cdecl FUN_004a9a50(unsigned char *a);
void         __cdecl FUN_004ac260(void);
void         __cdecl FUN_004abee0(void);
unsigned int __cdecl FUN_004aa870(void);

int __cdecl FUN_0049e420(char *param_1)
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
  int iVar17;
  short sVar18;
  unsigned int uVar19;
  unsigned int uVar20;
  short sVar21;
  int iVar22;
  unsigned char *pbVar23;
  char *pcVar24;
  unsigned char *pbVar25;
  unsigned char abStack_100[256];

  uVar16 = DAT_00a0813c;
  uVar15 = DAT_00a08108;
  uVar14 = DAT_00a08102;
  uVar13 = DAT_00a08100;
  uVar12 = DAT_00a07ce4;
  uVar11 = DAT_00a07c64;
  uVar10 = DAT_009fe7e4;
  uVar8 = DAT_009ec464;
  uVar7 = DAT_009e8f52;
  uVar6 = DAT_009d8c00;
  cVar5 = DAT_009d80c8;
  uVar4 = DAT_009cd270;
  uVar3 = DAT_009a807a;
  uVar2 = DAT_009a6fe0;
  iVar17 = FUN_004aba40();
  iVar22 = iVar17;
  if (0 < iVar17) {
    do {
      FUN_004abe50();
      iVar22 = iVar22 + -1;
    } while (iVar22 != 0);
  }
  uVar9 = DAT_009ed23b;
  DAT_009ed23b = 1;
  FUN_004aba50();
  FUN_004a9640('\x01');
  sVar18 = (short)(DAT_009a8c14 >> 1);
  sVar21 = (short)(DAT_009a7bac >> 4);
  FUN_004a9520(sVar21,
               (short)((unsigned int)(int)sVar18 + (unsigned int)((int)(unsigned short)DAT_009a20ae * -2)),
               (short)((unsigned int)(int)(short)DAT_009a7bac - (unsigned int)(int)sVar21),
               (short)((unsigned int)((unsigned int)(unsigned short)DAT_009a20ae * 2) + (unsigned int)(int)sVar18));
  DAT_00a08100 = 0xf9;
  (*DAT_009d1300)();
  sVar21 = (short)(DAT_009a8c14 >> 1);
  sVar18 = (short)(DAT_009a7bac >> 4);
  FUN_004a9520((short)(sVar18 + 1),
               (short)((unsigned int)(int)sVar21 + (unsigned int)((int)(unsigned short)DAT_009a20ae * -2) + 1),
               (short)(((unsigned int)(int)(short)DAT_009a7bac - (unsigned int)(int)sVar18) - 1),
               (short)((unsigned int)((unsigned int)(unsigned short)DAT_009a20ae * 2) + (unsigned int)(int)sVar21 - 1));
  DAT_00a08100 = 0;
  (*DAT_009d1300)();
  DAT_009a807a = 0xf9;
  DAT_009e8f52 = 0;
  DAT_009ec464 = 0;
  FUN_004a9500(0, (short)(((unsigned int)(int)(short)(DAT_009a8c14 >> 1) - (unsigned int)(unsigned short)DAT_009a20ae) - 2));
  uVar19 = 0xffffffff;
  do {
    pcVar24 = param_1;
    if (uVar19 == 0) break;
    uVar19 = uVar19 - 1;
    pcVar24 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar24;
  } while (cVar1 != '\0');
  uVar19 = ~uVar19;
  pbVar23 = (unsigned char *)(pcVar24 - uVar19);
  pbVar25 = abStack_100;
  for (uVar20 = uVar19 >> 2; uVar20 != 0; uVar20 = uVar20 - 1) {
    __builtin_memcpy(pbVar25, pbVar23, 4);
    pbVar23 = pbVar23 + 4;
    pbVar25 = pbVar25 + 4;
  }
  for (uVar20 = uVar19 & 3; uVar20 != 0; uVar20 = uVar20 - 1) {
    *pbVar25 = *pbVar23;
    pbVar23 = pbVar23 + 1;
    pbVar25 = pbVar25 + 1;
  }
  FUN_004a9a50(abStack_100);
  FUN_004a9500(0, (short)((unsigned int)(int)(short)(DAT_009a8c14 >> 1) + 2));
  FUN_004a9a50(DAT_00a60a4c);
  FUN_004abe50();
  FUN_004ac260();
  FUN_004abee0();
  uVar19 = FUN_004aa870();
  DAT_009ed23b = uVar9;
  if (0 < iVar17) {
    do {
      FUN_004aba50();
      iVar17 = iVar17 + -1;
    } while (iVar17 != 0);
  }
  FUN_004a9640(cVar5);
  DAT_009a6fe0 = uVar2;
  DAT_009a807a = uVar3;
  DAT_009cd270 = uVar4;
  DAT_009d8c00 = uVar6;
  DAT_009e8f52 = uVar7;
  DAT_009ec464 = uVar8;
  DAT_009fe7e4 = uVar10;
  DAT_00a07c64 = uVar11;
  DAT_00a07ce4 = uVar12;
  DAT_00a08100 = uVar13;
  DAT_00a08102 = uVar14;
  DAT_00a08108 = uVar15;
  DAT_00a0813c = uVar16;
  iVar17 = FUN_004aba40();
  iVar22 = iVar17;
  if (0 < iVar17) {
    do {
      FUN_004abe50();
      iVar22 = iVar22 + -1;
    } while (iVar22 != 0);
  }
  FUN_004ac260();
  FUN_004abee0();
  if (0 < iVar17) {
    do {
      FUN_004aba50();
      iVar17 = iVar17 + -1;
    } while (iVar17 != 0);
  }
  return (int)(char)uVar19;
}
