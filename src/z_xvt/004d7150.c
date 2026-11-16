/* SPDX-License-Identifier: MIT */
/* z_xvt 004d7150 */
/* Unresolved: signatures of FUN_004d66c0, FUN_004bef20 and FUN_004e1770 are not in the evidence.
   They are only passed as code pointers, never called, so they are declared void(void). The
   LAB_ entry points are declared as bytes whose addresses are passed. The decompile shows
   FUN_004dbf00() with no arguments, so no argument is passed; FUN_004dcfa0's result in the
   switch is discarded. */

extern char s_frontres_joinback_bmp_00532a8c[];
extern char s_background_00532a4c[];
extern char s_frame_00531358[];
extern char s_clientactive_00531eac[];
extern char s_allactive_00531ebc[];
extern char s_chatbox_00532a84[];
extern char s_v___d__d_00531e88[];
extern char s__c_s__c_s_00531334[];
extern char s_rebtiny_d_00531328[];
extern char s_imptiny_d_0053131c[];
extern char s_buttonsound_00531310[];
extern char s_leaveup_005312fc[];
extern char s_leavedown_00531304[];

extern int DAT_0081bc68;
extern char DAT_0085e6f0;
extern char DAT_0085e7c6;
extern int DAT_0053308c;
extern uint DAT_00757ce0[];
extern char DAT_0081b740[];
extern char DAT_0081bcf0[];
extern uint DAT_0081bcd0;
extern char DAT_0081e082[];

extern undefined1 LAB_004d6670;
extern undefined1 LAB_004bee00;
extern undefined1 LAB_004e1600;

void __cdecl FUN_004d0290(char *, char *);
void __cdecl FUN_004d52b0(void);
void __cdecl FUN_004d1750(char *, int, int);
void __cdecl FUN_004d0e60(char *, int, int);
void __cdecl FUN_004d0580(char *, int, int);
void __cdecl FUN_004d5350(int);
void __cdecl FUN_004db490(int);
void __cdecl FUN_004dcfd0(int *, int, int, int, int);
char * __cdecl FUN_004dcfa0(int);
void __cdecl FUN_004da920(int, char *, int *, uint);
int __cdecl FUN_004e0750(void);
void __cdecl FUN_004ca210(void);
void __cdecl FUN_004dbf00(void);
void __cdecl FUN_004db670(code *, undefined1 *);
int __cdecl _sprintf(char *, const char *, ...);
int __cdecl FUN_004c0750(int);
void __cdecl FUN_004da030(void);
void __cdecl FUN_004da060(char *);
int __cdecl FUN_004d9a80(int *, char *, char *, char *, int, undefined4, int, char *);
void __cdecl FUN_004da050(void);
void __cdecl FUN_004d0430(char *);
void __cdecl FUN_004d66c0(void);
void __cdecl FUN_004bef20(void);
void __cdecl FUN_004e1770(void);

undefined4 __cdecl FUN_004d7150(uint param_1)
{
  int iVar1;
  uint *puVar2;
  char *pcVar3;
  undefined4 uVar4;
  int *piVar5;
  code *pcVar6;
  int iVar7;
  uint uVar8;
  undefined1 *puVar9;
  char *pcVar10;
  int local_10[4];

  if (param_1 == 0) {
    FUN_004d0290(s_frontres_joinback_bmp_00532a8c, s_background_00532a4c);
    FUN_004d52b0();
    FUN_004d1750(s_background_00532a4c, 0, 0);
    FUN_004d0e60(s_frame_00531358, 0, 0);
    if (DAT_0081bc68 == 0) {
      pcVar3 = s_clientactive_00531eac;
    }
    else {
      pcVar3 = s_allactive_00531ebc;
    }
    FUN_004d0e60(pcVar3, 0, 0);
    FUN_004d0580(s_chatbox_00532a84, 0, 0);
    FUN_004d5350(1);
    FUN_004db490(0x14);
  }
  FUN_004dcfd0(local_10, 0x54, 0x6b, 0x1b2, 0x1b1);
  piVar5 = local_10;
  uVar8 = 0xffff;
  pcVar3 = FUN_004dcfa0(0x14);
  FUN_004da920(0xf, pcVar3, piVar5, uVar8);
  iVar1 = FUN_004e0750();
  if (((((iVar1 == 0x44) || (iVar1 == 0x5e)) || (iVar1 == 0x5f)) ||
      ((iVar1 == 0x60 || (iVar1 == 0x46)))) || (iVar1 == 0x3a)) {
    FUN_004ca210();
    switch (iVar1) {
    case 0x3a:
    case 0x60:
      FUN_004dcfa0(0x21c);
      FUN_004dcfa0(0x21b);
      uVar8 = 0x21a;
      break;
    default:
      goto switchD_004d727f_caseD_3b;
    case 0x44:
      FUN_004dcfa0(0x213);
      FUN_004dcfa0(0x212);
      uVar8 = 0x211;
      break;
    case 0x46:
      FUN_004dcfa0(0x26b);
      FUN_004dcfa0(0x26a);
      uVar8 = 0x269;
      break;
    case 0x5e:
      FUN_004dcfa0(0x216);
      FUN_004dcfa0(0x215);
      uVar8 = 0x214;
      break;
    case 0x5f:
      FUN_004dcfa0(0x219);
      FUN_004dcfa0(0x218);
      uVar8 = 0x217;
    }
    FUN_004dcfa0(uVar8);
    FUN_004dbf00();
switchD_004d727f_caseD_3b:
    if (DAT_0085e6f0 == '\0') {
      DAT_0053308c = 1;
      puVar9 = &LAB_004d6670;
      pcVar6 = (code *)FUN_004d66c0;
    }
    else {
      puVar9 = &LAB_004bee00;
      pcVar6 = (code *)FUN_004bef20;
    }
  }
  else {
    if (iVar1 != 0x45) goto LAB_004d738c;
    puVar9 = &LAB_004e1600;
    puVar2 = DAT_00757ce0;
    for (iVar1 = 0x54; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar2 = 0;
      puVar2 = puVar2 + 1;
    }
    pcVar6 = (code *)FUN_004e1770;
  }
  FUN_004db670(pcVar6, puVar9);
LAB_004d738c:
  FUN_004dcfd0(local_10, 0x1fb, 0x1c4, 0x232, 0x1d0);
  _sprintf(DAT_0081b740, s_v___d__d_00531e88, 1, 1);
  FUN_004da920(0xc, DAT_0081b740, local_10, 0xffff);
  FUN_004dcfd0(local_10, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_0081bcf0[0] != '\0') {
    _sprintf(DAT_0081b740, s__c_s__c_s_00531334, 6, DAT_0081e082, 1, DAT_0081bcf0);
    FUN_004da920(0xc, DAT_0081b740, local_10, DAT_0081bcd0);
    uVar8 = (uint)((int)param_1 >> 0x1f);
    iVar1 = (int)(((((param_1 ^ uVar8) - uVar8) & 0x1f) ^ uVar8) - uVar8) >> 1;
    _sprintf(DAT_0081b740, s_rebtiny_d_00531328, iVar1);
    FUN_004d0e60(DAT_0081b740, 0xcc, 0x1c5);
    _sprintf(DAT_0081b740, s_imptiny_d_0053131c, iVar1);
    FUN_004d0e60(DAT_0081b740, 0x1a4, 0x1c5);
  }
  iVar1 = FUN_004c0750(0);
  if (iVar1 == 1) {
    return 1;
  }
  if (DAT_0085e7c6 != '\0') {
    FUN_004da030();
  }
  pcVar3 = FUN_004dcfa0(0x13);
  FUN_004da060(pcVar3);
  FUN_004dcfd0(local_10, 0x55, 0x1bf, 0xb0, 0x1d7);
  pcVar10 = s_buttonsound_00531310;
  iVar7 = 8;
  uVar4 = 0;
  iVar1 = 0xc;
  pcVar3 = FUN_004dcfa0(0x13);
  iVar1 = FUN_004d9a80(local_10, s_leaveup_005312fc, s_leavedown_00531304, pcVar3, iVar1, uVar4,
                       iVar7, pcVar10);
  FUN_004da050();
  if ((iVar1 != 0) || (param_1 == 0x1e0)) {
    FUN_004ca210();
    DAT_0053308c = 1;
    FUN_004db670((code *)FUN_004d66c0, &LAB_004d6670);
    FUN_004d0430(s_background_00532a4c);
  }
  return 0;
}
