/* SPDX-License-Identifier: MIT */
/* z_bop 004fb350 */
extern int FUN_004de110(void);
extern int FUN_004cfee0(void);
extern int FUN_004d1400(void);
extern unsigned long GetTickCount(void);
extern void FUN_004b4a60(const char *, const char *);
extern void FUN_004d4e20(void);
extern void FUN_004b5de0(const char *, int, int);
extern void FUN_004b5570(const char *, int, int);
extern void FUN_004d4ec0(int);
extern void FUN_004dc170(void);
extern void FUN_004d6460(int *, int, int, int, int);
extern char *FUN_004ddc10(int);
extern void FUN_004db640(int, char *, int *, unsigned int);
extern void FUN_004e6310(int);
extern void FUN_004b4bf0(const char *);
extern void FUN_004dc330(int (*)(void), int *);
extern int FUN_005066a0(void);

extern int _DAT_00b69cdc;
extern int DAT_00aa6144;
extern int DAT_00b69e30;
extern int DAT_00b6a2c8;
extern int DAT_00b6a2fe;
extern unsigned long DAT_00669798;
extern unsigned long _DAT_00669794;
extern const char s_frontres_wait_bmp_0052d0b4[];
extern const char s_background_0052bf74[];
extern const char s_frame_0052a6d4[];
extern const char s_alloff_0052a6cc[];
extern int LAB_00506690;

int FUN_004fb350(int param_1)
{
  int iVar2;
  int *piVar3;
  unsigned int uVar4;
  char *pcVar1;
  int local_10[4];

  if (param_1 == 0) {
    iVar2 = 1;
    FUN_004de110();
    _DAT_00b69cdc = 1;
    DAT_00aa6144 = FUN_004cfee0();
    if (DAT_00b69e30 == 2) {
      DAT_00aa6144 = 1;
    }
    else {
      iVar2 = FUN_004d1400();
    }
    DAT_00b6a2c8 = iVar2;
    DAT_00b6a2fe = iVar2;
    DAT_00669798 = GetTickCount();
    FUN_004b4a60(s_frontres_wait_bmp_0052d0b4, s_background_0052bf74);
    FUN_004d4e20();
    FUN_004b5de0(s_background_0052bf74, 0, 0);
    FUN_004b5570(s_frame_0052a6d4, 0, 0);
    FUN_004b5570(s_alloff_0052a6cc, 0, 0);
    FUN_004d4ec0(1);
    FUN_004dc170();
  }
  FUN_004d6460(local_10, 0, 0, 0x27f, 0x1df);
  piVar3 = local_10;
  uVar4 = 0xffff;
  pcVar1 = FUN_004ddc10(0xcd);
  FUN_004db640(0xf, pcVar1, piVar3, uVar4);
  if (DAT_00aa6144 != 0) {
    _DAT_00669794 = GetTickCount();
    if ((int)(DAT_00669798 + 2000) < (int)_DAT_00669794) {
      FUN_004e6310(0);
      _DAT_00b69cdc = 1;
      FUN_004b4bf0(s_background_0052bf74);
      FUN_004dc330(FUN_005066a0, &LAB_00506690);
      return 0;
    }
  }
  _DAT_00669794 = GetTickCount();
  if ((int)(DAT_00669798 + 4000) < (int)_DAT_00669794) {
    _DAT_00b69cdc = 1;
    FUN_004b4bf0(s_background_0052bf74);
    FUN_004dc330(FUN_005066a0, &LAB_00506690);
    return 0;
  }
  return 0;
}
