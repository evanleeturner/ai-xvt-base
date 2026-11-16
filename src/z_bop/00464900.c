/* SPDX-License-Identifier: MIT */
/* z_bop 00464900 */
extern unsigned int FUN_00411810(int param_1);
extern int FUN_004ac760(void);
extern void *FID_conflict___wfopen(const char *, const char *);
extern int _fprintf(void *, const char *, ...);
extern int _fflush(void *);
extern void FUN_0046d350(int param_1, unsigned int *param_2, int param_3);
extern int FUN_0046f700(void);
extern int *FUN_00418760(int param_1, unsigned int param_2, int *param_3);
extern void FUN_004aa720(void);

extern unsigned char DAT_00a082aa;
extern unsigned char DAT_00a082ab;
extern unsigned short DAT_00a082ac;
extern unsigned char DAT_009a8c12;
extern unsigned short DAT_00a00498;
extern unsigned short DAT_00a004a0;
extern unsigned short DAT_00a08244;
extern unsigned char DAT_00557360[];
extern unsigned int DAT_00557364;
extern unsigned char DAT_00557368;
extern unsigned int DAT_009a8c2c;
extern unsigned int DAT_00559788;
extern unsigned int DAT_00557348;
extern int DAT_00a080f8;
extern int DAT_005242d0;
extern void *DAT_005242d4;
extern char s_inputlog_txt_0052435c[];
extern char DAT_0052436c[];
extern char s__8x__2x__2x__2x__2x_00524344[];
extern int DAT_00523428;
extern char DAT_009e9680[];
extern int DAT_00a07bb0[];
extern int DAT_00a07bd0;
extern int DAT_009ecc34;
extern unsigned char DAT_00557148[];
extern unsigned int DAT_00557354;
extern unsigned int DAT_00557580;
extern unsigned int DAT_00556ef8;
extern int DAT_009fd394;
extern int DAT_005242c4;
extern int DAT_009e9bf8;
extern int DAT_009ec9e0;
extern int DAT_00a082a8;

void __stdcall FUN_00464900(void)
{
  int iVar1;
  unsigned int uVar2;
  int iVar3;
  char *pcVar4;
  int *piVar5;
  unsigned char *puVar6;
  unsigned char *puVar7;
  unsigned char *piVar8;
  unsigned int *puVar9;
  int iTmp;
  int *piVar10;

  FUN_00411810(-2);
  DAT_00a082aa = (unsigned char)DAT_009a8c12;
  DAT_00a082ab = (unsigned char)((unsigned char)DAT_00a00498 & 0xfe);
  *(unsigned int *)&DAT_00557360[0] = 1;
  DAT_00a082ac = (unsigned short)((DAT_00a082ac & 0xff00) |
                 (unsigned char)((unsigned char)DAT_00a004a0 & 0xfe));
  DAT_00a082ac = (unsigned short)((DAT_00a082ac & 0x00ff) |
                 ((unsigned short)((unsigned char)DAT_00a08244 & 3) << 8));
  iVar1 = FUN_004ac760();
  DAT_009a8c2c = DAT_009a8c2c + (unsigned int)iVar1;
  if ((((int)(DAT_009a8c2c - DAT_00559788) < 0x7f) && (-1 < (int)(DAT_009a8c2c - DAT_00559788))) &&
      (DAT_00559788 != 0)) {
    uVar2 = DAT_009a8c2c & 0x7f;
  }
  else {
    uVar2 = 0x7f;
  }
  if (0xec < (int)(DAT_00559788 - DAT_00557348)) {
    uVar2 = 0x7f;
  }
  if (uVar2 == 0x7f) {
    iVar1 = 9;
    DAT_00557364 = (DAT_009a8c2c << 8) | 0x7f;
    DAT_00557368 = (unsigned char)(DAT_009a8c2c >> 0x18);
    DAT_00557348 = DAT_009a8c2c;
  }
  else {
    iVar1 = 5;
    DAT_00557364 = (DAT_00557364 & 0xffffff00) | (unsigned int)(unsigned char)uVar2;
  }
  iVar3 = iVar1;
  DAT_00559788 = DAT_009a8c2c;
  if (DAT_00a082aa != 0) {
    iVar3 = iVar1 + 1;
    DAT_00557364 = DAT_00557364 | 0x80;
    DAT_00557360[iVar1] = DAT_00a082aa;
  }
  DAT_00557360[iVar3] = DAT_00a082ab;
  DAT_00557360[iVar3 + 1] = (unsigned char)DAT_00a082ac;
  if (((DAT_00a082ac >> 8) & 1) != 0) {
    DAT_00557360[iVar3] = DAT_00557360[iVar3] | 1;
  }
  if (((DAT_00a082ac >> 8) & 2) != 0) {
    DAT_00557360[iVar3 + 1] = DAT_00557360[iVar3 + 1] | 1;
  }
  if (1 < DAT_00a080f8) {
    if ((DAT_005242d0 == 1) &&
        ((DAT_005242d4 != (void *)0 ||
         (DAT_005242d4 = FID_conflict___wfopen(s_inputlog_txt_0052435c, DAT_0052436c),
          DAT_005242d4 != (void *)0)))) {
      _fprintf(DAT_005242d4, s__8x__2x__2x__2x__2x_00524344, DAT_00557364,
               (unsigned int)DAT_00a082aa, (unsigned int)DAT_00a082ab,
               (unsigned int)(unsigned char)DAT_00a082ac,
               (unsigned int)(unsigned char)(DAT_00a082ac >> 8));
      _fflush(DAT_005242d4);
    }
    if (DAT_00523428 == 0) {
      iVar1 = 0;
      pcVar4 = DAT_009e9680;
      piVar5 = DAT_00a07bb0;
      do {
        if ((*pcVar4 != '\0') && ((*piVar5 != 0 || (iVar1 == DAT_009ecc34)))) {
          __builtin_memcpy(&iTmp, pcVar4 + 0x578, 4);
          FUN_0046d350(iTmp, (unsigned int *)&DAT_00557360[0], iVar3 + 2);
        }
        pcVar4 = pcVar4 + 0x5bd;
        piVar5 = piVar5 + 1;
        iVar1 = iVar1 + 1;
      } while (piVar5 < &DAT_00a07bd0);
    }
    else {
      *(unsigned int *)&DAT_00557148[0] = 0x16;
      DAT_00557148[4] = (unsigned char)(DAT_00557148[4] + 1);
      puVar6 = (unsigned char *)&DAT_00557364;
      puVar7 = &DAT_00557148[DAT_00557354];
      for (uVar2 = (unsigned int)(iVar3 - 2) >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
        __builtin_memcpy(puVar7, puVar6, 4);
        puVar6 = puVar6 + 4;
        puVar7 = puVar7 + 4;
      }
      for (uVar2 = (unsigned int)(iVar3 - 2) & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
        *puVar7 = *puVar6;
        puVar6 = puVar6 + 1;
        puVar7 = puVar7 + 1;
      }
      DAT_00557354 = DAT_00557354 + (unsigned int)iVar3 + (unsigned int)-2;
      if (DAT_00557580 < DAT_009a8c2c - DAT_00556ef8) {
        puVar9 = (unsigned int *)&DAT_00557148[0];
        DAT_00556ef8 = DAT_009a8c2c;
        uVar2 = DAT_00557354;
        iVar1 = FUN_0046f700();
        FUN_0046d350(iVar1, puVar9, (int)uVar2);
        if (DAT_009fd394 < DAT_005242c4) {
          iVar1 = 0;
          piVar10 = DAT_00a07bb0;
          piVar8 = (unsigned char *)&DAT_009e9bf8;
          do {
            if (*(char *)(piVar8 - 0x578) != '\0' && (iVar1 != DAT_009ecc34)) {
              iVar3 = FUN_0046f700();
              __builtin_memcpy(&iTmp, piVar8, 4);
              if ((iVar3 != iTmp) && (*piVar10 != 0)) {
                FUN_0046d350(iTmp, (unsigned int *)&DAT_00557148[0], (int)DAT_00557354);
              }
            }
            piVar10 = piVar10 + 1;
            piVar8 = piVar8 + 0x5bd;
            iVar1 = iVar1 + 1;
          } while (piVar8 < (unsigned char *)&DAT_009ec9e0);
        }
        DAT_00557354 = 5;
        *(unsigned int *)&DAT_00557148[0] = 0x16;
        DAT_00557148[4] = 0;
      }
    }
  }
  piVar5 = FUN_00418760(DAT_009ecc34, DAT_009a8c2c, &DAT_00a082a8);
  if (piVar5 != (int *)0) {
    *piVar5 = 0;
    piVar5[1] = 1;
  }
  FUN_004aa720();
  return;
}
