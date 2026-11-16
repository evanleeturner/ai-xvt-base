/* SPDX-License-Identifier: MIT */
/* z_bop 004d7e70 */
extern char s_frontres_joinback_bmp_0052bfb4[];
extern char s_background_0052bf74[];
extern char s_frame_0052a6d4[];
extern char s_clientactive_0052b69c[];
extern char s_allactive_0052b6ac[];
extern char s_chatbox_0052bfac[];
extern char s_v___d__d_0052b1b4[];
extern char s__c_s__c_s_0052a6b0[];
extern char s_rebtiny_d_0052a6a4[];
extern char s_imptiny_d_0052a698[];
extern char s_buttonsound_0052a68c[];
extern char s_leaveup_0052a678[];
extern char s_leavedown_0052a680[];

extern int DAT_00b6a250;
extern int DAT_0052c184;
extern char DAT_00bb72d0;
extern char DAT_00bb73a8;
extern unsigned int DAT_00aa6150[];
extern char DAT_00b69d20[];
extern char DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern unsigned int DAT_00b6a2c0;

extern unsigned char LAB_004d7370;
extern unsigned char LAB_004be050;
extern unsigned char LAB_004e1470;

void __cdecl FUN_004d73c0(void);
void __cdecl FUN_004be1b0(void);
void __cdecl FUN_004e15d0(void);
void __cdecl FUN_004b7f30(void);

void __cdecl FUN_004b4a60(char *a, char *b);
void __cdecl FUN_004d4e20(void);
void __cdecl FUN_004b5de0(char *a, int b, int c);
void __cdecl FUN_004b5570(char *a, int b, int c);
void __cdecl FUN_004b4d40(char *a, int b, int c);
void __cdecl FUN_004d4ec0(int a);
void __cdecl FUN_004dc140(int a);
void __cdecl FUN_004d6460(int *r, int a, int b, int c, int d);
char * __cdecl FUN_004ddc10(int a);
void __cdecl FUN_004db640(int a, char *b, int *c, unsigned int d);
int __cdecl FUN_004e0490(void);
void __cdecl FUN_004cd9e0(void);
void __cdecl FUN_004dcb90(char *a, char *b, char *c, char *d, char *e);
void __cdecl FUN_004dc380(code *f, void *p);
void __cdecl FUN_004dc330(code *f, void *p);
int __cdecl _sprintf(char *buf, const char *fmt, ...);
int __cdecl FUN_004bf9b0(int a);
void __cdecl FUN_004dad40(void);
void __cdecl FUN_004dad70(char *a);
int __cdecl FUN_004da780(int *r, char *a, char *b, char *c, int d, int e, int f, char *g);
void __cdecl FUN_004dad60(void);
void __cdecl FUN_004b4bf0(char *a);

unsigned int __cdecl FUN_004d7e70(unsigned int param_1)
{
  int iVar1;
  char *pcVar2;
  unsigned int *puVar3;
  char *pcVar4;
  unsigned int uVar5;
  int *piVar6;
  code *pcVar7;
  char *pcVar8;
  int iVar9;
  unsigned int uVar10;
  void *puVar11;
  char *pcVar12;
  char *pcVar13;
  int local_20[4];
  int local_10[4];

  if (param_1 == 0) {
    FUN_004b4a60(s_frontres_joinback_bmp_0052bfb4, s_background_0052bf74);
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    if (DAT_00b6a250 == 0) {
      pcVar4 = s_clientactive_0052b69c;
    }
    else {
      pcVar4 = s_allactive_0052b6ac;
    }
    FUN_004b5570(pcVar4, 0, 0);
    FUN_004b4d40(s_chatbox_0052bfac, 0, 0);
    FUN_004d4ec0(1);
    FUN_004dc140(0x14);
  }
  FUN_004d6460(local_20, 0x54, 0x6b, 0x1b2, 0x1b1);
  piVar6 = local_20;
  uVar10 = 0xffff;
  pcVar4 = FUN_004ddc10(0x14);
  FUN_004db640(0xf, pcVar4, piVar6, uVar10);
  iVar1 = FUN_004e0490();
  if (((((iVar1 == 0x44) || (iVar1 == 0x5e)) || (iVar1 == 0x5f)) ||
      ((iVar1 == 0x60 || (iVar1 == 0x46)))) || (iVar1 == 0x3a)) {
    FUN_004cd9e0();
    switch (iVar1) {
    case 0x3a:
    case 0x60:
      pcVar12 = (char *)0;
      pcVar8 = (char *)0;
      pcVar4 = FUN_004ddc10(0x22a);
      pcVar13 = FUN_004ddc10(0x229);
      pcVar2 = FUN_004ddc10(0x228);
      FUN_004dcb90(pcVar2, pcVar13, pcVar4, pcVar8, pcVar12);
      break;
    case 0x44:
      pcVar12 = (char *)0;
      pcVar8 = (char *)0;
      pcVar4 = FUN_004ddc10(0x221);
      pcVar13 = FUN_004ddc10(0x220);
      pcVar2 = FUN_004ddc10(0x21f);
      FUN_004dcb90(pcVar2, pcVar13, pcVar4, pcVar8, pcVar12);
      break;
    case 0x46:
      pcVar12 = (char *)0;
      pcVar8 = (char *)0;
      pcVar4 = FUN_004ddc10(0x279);
      pcVar13 = FUN_004ddc10(0x278);
      pcVar2 = FUN_004ddc10(0x277);
      FUN_004dcb90(pcVar2, pcVar13, pcVar4, pcVar8, pcVar12);
      break;
    case 0x5e:
      pcVar12 = (char *)0;
      pcVar8 = (char *)0;
      pcVar4 = FUN_004ddc10(0x224);
      pcVar13 = FUN_004ddc10(0x223);
      pcVar2 = FUN_004ddc10(0x222);
      FUN_004dcb90(pcVar2, pcVar13, pcVar4, pcVar8, pcVar12);
      break;
    case 0x5f:
      pcVar12 = (char *)0;
      pcVar8 = (char *)0;
      pcVar4 = FUN_004ddc10(0x227);
      pcVar13 = FUN_004ddc10(0x226);
      pcVar2 = FUN_004ddc10(0x225);
      FUN_004dcb90(pcVar2, pcVar13, pcVar4, pcVar8, pcVar12);
      FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
      FUN_004dc380(FUN_004b7f30, local_10);
    }
    if (DAT_00bb72d0 == '\0') {
      DAT_0052c184 = 1;
      puVar11 = &LAB_004d7370;
      pcVar7 = FUN_004d73c0;
    }
    else {
      puVar11 = &LAB_004be050;
      pcVar7 = FUN_004be1b0;
    }
  }
  else {
    if (iVar1 != 0x45) goto LAB_004d811e;
    puVar11 = &LAB_004e1470;
    puVar3 = DAT_00aa6150;
    for (iVar1 = 0x54; iVar1 != 0; iVar1 = iVar1 + -1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    pcVar7 = FUN_004e15d0;
  }
  FUN_004dc330(pcVar7, puVar11);
LAB_004d811e:
  FUN_004d6460(local_20, 0x1fb, 0x1c4, 0x232, 0x1d0);
  _sprintf(DAT_00b69d20, s_v___d__d_0052b1b4, 2, 0);
  FUN_004db640(0xc, DAT_00b69d20, local_20, 0xffff);
  FUN_004d6460(local_20, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_00b6a2e0[0] != '\0') {
    _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, DAT_00b6bc9e, 1, DAT_00b6a2e0);
    FUN_004db640(0xc, DAT_00b69d20, local_20, DAT_00b6a2c0);
    uVar10 = (unsigned int)((int)param_1 >> 0x1f);
    iVar1 = (int)(((((param_1 ^ uVar10) - uVar10) & 0x1f) ^ uVar10) - uVar10) >> 1;
    _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, iVar1);
    FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
    _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, iVar1);
    FUN_004b5570(DAT_00b69d20, 0x1a4, 0x1c5);
  }
  iVar1 = FUN_004bf9b0(0);
  if (iVar1 == 1) {
    return 1;
  }
  if (DAT_00bb73a8 != '\0') {
    FUN_004dad40();
  }
  pcVar4 = FUN_004ddc10(0x13);
  FUN_004dad70(pcVar4);
  FUN_004d6460(local_20, 0x55, 0x1bf, 0xb0, 0x1d7);
  pcVar13 = s_buttonsound_0052a68c;
  iVar9 = 8;
  uVar5 = 0;
  iVar1 = 0xc;
  pcVar4 = FUN_004ddc10(0x13);
  iVar1 = FUN_004da780(local_20, s_leaveup_0052a678, s_leavedown_0052a680, pcVar4, iVar1, (int)uVar5, iVar9,
                       pcVar13);
  FUN_004dad60();
  if ((iVar1 != 0) || (param_1 == 0x1e0)) {
    FUN_004cd9e0();
    DAT_0052c184 = 1;
    FUN_004dc330(FUN_004d73c0, &LAB_004d7370);
    FUN_004b4bf0(s_background_0052bf74);
  }
  return 0;
}
