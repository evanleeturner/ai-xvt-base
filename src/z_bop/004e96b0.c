/* SPDX-License-Identifier: MIT */
/* z_bop 004e96b0 */
extern int DAT_00665d28;
extern int DAT_00665d2c;
extern int DAT_00665d30;
extern unsigned int DAT_00665d38[11];
extern int DAT_00665d64;
extern int DAT_00665d68;
extern int DAT_00665d6c;
extern int DAT_00665d70;
extern int DAT_00665d74;
extern int DAT_00aa60d0;
extern void *DAT_00aa60f4;
extern void *DAT_00aa6110;
extern int DAT_00b69e30;
extern char DAT_00b69d20[];
extern char DAT_00b6a2e0[];
extern char DAT_00b6bc9e[];
extern unsigned int DAT_00b6a2c0;
extern char DAT_00bb73a8;

extern char s_frontres_review_bmp_0052c47c[];
extern char s_backreview_0052b3b0[];
extern char s_frame_0052a6d4[];
extern char s_alloff_0052a6cc[];
extern char s_configoverlay_0052a6bc[];
extern char s__c_s__c_s_0052a6b0[];
extern char s_rebtiny_d_0052a6a4[];
extern char s_imptiny_d_0052a698[];
extern char s_buttonsound_0052a68c[];
extern char s_leaveup_0052a678[];
extern char s_leavedown_0052a680[];

void __cdecl FUN_004de0b0(int, int);
void __cdecl FUN_004eea80(void);
void __cdecl FUN_004ea8c0(void);
void __cdecl FUN_0042b100(void);
void __cdecl FUN_00426a00(void *);
void __cdecl FUN_00429e70(char *);
void __cdecl FUN_0042b010(int, int, int);
void __cdecl FUN_0042b0e0(int, int, int);
void __cdecl FUN_0042b0d0(int);
void __cdecl FUN_004b4a60(char *, char *);
void __cdecl FUN_004d4e20(void);
void __cdecl FUN_004b5de0(char *, int, int);
void __cdecl FUN_004b5570(char *, int, int);
void __cdecl FUN_004b4d40(char *, int, int);
void __cdecl FUN_004d4ec0(int);
void __cdecl FUN_004dc140(int);
void __cdecl FUN_004d6460(int *, int, int, int, int);
void __cdecl FUN_0042b090(int, int, int);
void __cdecl FUN_0042b250(int);
void __cdecl FUN_0042a380(int, int, int, int, int);
char * __cdecl FUN_004ddc10(int);
void __cdecl FUN_004db640(int, char *, int *, unsigned int);
void __cdecl FUN_004ea090(void);
int __cdecl _sprintf(char *, const char *, ...);
int __cdecl FUN_004bf9b0(int);
void __cdecl FUN_004e9af0(void);
void __cdecl FUN_004dad40(void);
void __cdecl FUN_004dad70(char *);
unsigned int __cdecl FUN_004da780(int *, char *, char *, char *, int, unsigned int, int, char *);
void __cdecl FUN_004dad60(void);
unsigned char __cdecl FUN_004dd0f0(void);
unsigned int __cdecl FUN_004cffb0(void);
void __cdecl FUN_004b4bf0(char *);
void __cdecl FUN_004dcb30(void);
void __cdecl FUN_004dc7e0(void);
void __cdecl _free(void *);
void __cdecl FUN_0042b1b0(void);
void __cdecl FUN_004dc170(void);

unsigned int __cdecl FUN_004e96b0(unsigned int param_1)
{
  char *pcVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar6;
  int *piVar7;
  int iVar8;
  unsigned int uVar9;
  char *pcVar10;
  int local_10[4];
  unsigned int *puVar5;

  if (param_1 == 0) {
    FUN_004de0b0(0x20, 0x13f);
    DAT_00665d28 = 0;
    DAT_00665d70 = 0;
    DAT_00665d2c = 1;
    DAT_00665d68 = 1;
    DAT_00665d6c = 1;
    DAT_00665d30 = 0;
    DAT_00665d74 = 0x42dc0000;
    DAT_00665d64 = 0x43610000;
    FUN_004eea80();
    FUN_004ea8c0();
    if (DAT_00aa60d0 != 0) {
      FUN_0042b100();
    }
    puVar5 = DAT_00665d38;
    for (iVar4 = 0xb; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    DAT_00665d38[0] = *(unsigned int *)((int)DAT_00aa60f4 + DAT_00665d28 * 0x44 + 0x40);
    FUN_00426a00(DAT_00665d38);
    FUN_00429e70((char *)(DAT_00665d28 * 0x44 + (int)DAT_00aa60f4));
    /* UNRESOLVED: 12 arg bytes pushed per stack-check; values not recovered (0 is a placeholder) */
    FUN_0042b010(0, 0, 0);
    iVar4 = *(int *)((int)DAT_00aa60f4 + DAT_00665d28 * 0x44 + 0x40);
    if ((iVar4 == 6) || (iVar4 == 7)) {
      uVar6 = 100;
    }
    else {
      uVar6 = 0;
    }
    FUN_0042b0e0(0, uVar6, 0);
    FUN_0042b0d0(0);
    FUN_004b4a60(s_frontres_review_bmp_0052c47c, s_backreview_0052b3b0);
    FUN_004d4e20();
    FUN_004b5de0(s_backreview_0052b3b0, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    FUN_004b5570(s_alloff_0052a6cc, 0, 0);
    FUN_004b4d40(s_configoverlay_0052a6bc, 0, 0);
    FUN_004d4ec0(1);
    FUN_004dc140(0x14);
  }
  FUN_004d6460(local_10, 0x118, 0x6b, 0x25e, 0x1b1);
  /* UNRESOLVED: 12 arg bytes pushed per stack-check; values not recovered (placeholder) */
  FUN_0042b090(0, 0, 0);
  /* UNRESOLVED: 4 arg bytes pushed per stack-check; value not recovered (placeholder) */
  FUN_0042b250(0);
  /* UNRESOLVED: 20 arg bytes pushed per stack-check; the fifth argument's value and
     position are not recovered (placeholder 0 appended after the four known args) */
  FUN_0042a380(local_10[0], local_10[1],
               (int)((unsigned int)(local_10[2] - local_10[0]) + 1u),
               (int)((unsigned int)(local_10[3] - local_10[1]) + 1u), 0);
  FUN_004d6460(local_10, 0x54, 0x5a, 0x25c, 0x6a);
  piVar7 = local_10;
  uVar9 = 0xffff;
  pcVar2 = FUN_004ddc10(1);
  FUN_004db640(0xf, pcVar2, piVar7, uVar9);
  FUN_004ea090();
  FUN_004d6460(local_10, 200, 0x1c4, 0x1b4, 0x1d0);
  if (DAT_00b6a2e0[0] != '\0') {
    _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, DAT_00b6bc9e, 1, DAT_00b6a2e0);
    FUN_004db640(0xc, DAT_00b69d20, local_10, DAT_00b6a2c0);
    uVar9 = (unsigned int)((int)param_1 >> 0x1f);
    iVar4 = (int)((((((param_1 ^ uVar9) - uVar9) & 0x1f) ^ uVar9) - uVar9)) >> 1;
    _sprintf(DAT_00b69d20, s_rebtiny_d_0052a6a4, iVar4);
    FUN_004b5570(DAT_00b69d20, 0xcc, 0x1c5);
    _sprintf(DAT_00b69d20, s_imptiny_d_0052a698, iVar4);
    FUN_004b5570(DAT_00b69d20, 0x1a4, 0x1c5);
  }
  iVar4 = FUN_004bf9b0(3);
  if (iVar4 == 1) {
    return 1;
  }
  FUN_004e9af0();
  FUN_004d6460(local_10, 0x55, 0x1bf, 0xb0, 0x1d7);
  if (DAT_00bb73a8 != '\0') {
    FUN_004dad40();
    pcVar2 = FUN_004ddc10(0xce);
    FUN_004dad70(pcVar2);
  }
  pcVar10 = s_buttonsound_0052a68c;
  iVar8 = 8;
  uVar6 = 0;
  iVar4 = 0xc;
  pcVar2 = FUN_004ddc10(0xce);
  uVar9 = FUN_004da780(local_10, s_leaveup_0052a678, s_leavedown_0052a680, pcVar2, iVar4, uVar6,
                       iVar8, pcVar10);
  FUN_004dad60();
  uVar9 = uVar9 | (unsigned int)FUN_004dd0f0();
  if (DAT_00b69e30 == 4) {
    uVar3 = FUN_004cffb0();
    uVar9 = uVar9 | uVar3;
  }
  if (uVar9 != 0) {
    FUN_004b4bf0(s_backreview_0052b3b0);
    FUN_004dcb30();
    FUN_004dc7e0();
    if (DAT_00aa60d0 == 0) {
      if (DAT_00aa60f4 != (void *)0) {
        _free(DAT_00aa60f4);
        DAT_00aa60f4 = (void *)0;
      }
    }
    else {
      FUN_0042b1b0();
    }
    if (DAT_00aa6110 != (void *)0) {
      _free(DAT_00aa6110);
      DAT_00aa6110 = (void *)0;
    }
    FUN_004dc170();
  }
  return 0;
}
