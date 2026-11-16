/* SPDX-License-Identifier: MIT */
/* z_bop 004248b0 */
void FUN_0049e720(char *param_1, void *param_2, int param_3, int param_4);
int _fseek(void *_File, long _Offset, int _Origin);
long _ftell(void *_File);
void FUN_004ac6e0(unsigned int param_1);
unsigned int FUN_004ac5f0(unsigned int param_1, int param_2);
unsigned int FUN_0049eb60(int param_1);
char *FUN_004ac720(unsigned int param_1);
int _fclose(void *_File);
int FUN_00425a70(void *param_1, char *param_2);
char *_fgets(char *_Buf, int _MaxCount, void *_File);

extern char s_strings_txt_0051c4b0[];
extern char DAT_0051c4bc;
extern void *DAT_00a07bd8;
extern unsigned short DAT_009d7670;
extern unsigned char DAT_0051bea0[];

extern unsigned int DAT_0099f900;
extern unsigned int DAT_0099f92c;
extern unsigned int DAT_00a60a40[];
extern unsigned int DAT_00a609b0[];
extern unsigned int DAT_00a606f0;
extern unsigned int DAT_00a60704;
extern unsigned int DAT_00a686d4;
extern unsigned int DAT_00a65ce0;
extern unsigned int DAT_00a68600;
extern unsigned int DAT_00a68610;
extern unsigned int DAT_00a68648;
extern unsigned int DAT_00a63380;
extern unsigned int DAT_00a63388;
extern unsigned int DAT_00a63390;
extern unsigned int DAT_00a633b4;
extern unsigned int DAT_00a686b0;
extern unsigned int DAT_00a686d0;
extern unsigned int DAT_00a68730;
extern unsigned int DAT_00a6873c;
extern unsigned int DAT_00a68650;
extern unsigned int DAT_00a6866c;
extern unsigned int DAT_00a68670;
extern unsigned int DAT_00a0a790;
extern unsigned int DAT_00a0a7e0;
extern unsigned int DAT_009a1840;
extern unsigned int DAT_009a1ec4;
extern unsigned int DAT_00a0a0e0;
extern unsigned int DAT_00a0a128;
extern unsigned int DAT_00a08380;
extern unsigned int DAT_00a083b8;
extern unsigned int DAT_00a08bf0;
extern unsigned int DAT_00a08c74;
extern unsigned int DAT_00a0a130;
extern unsigned int DAT_00a0a1d0;
extern unsigned int DAT_00a08350;
extern unsigned int DAT_00a08360;
extern unsigned int DAT_00a607f0;
extern unsigned int DAT_00a60814;
extern unsigned int DAT_00a607ac;
extern unsigned int DAT_00a60820[];
extern unsigned int DAT_00a607b0;
extern char DAT_0051c56a;
extern char DAT_00520eb3;
extern unsigned char DAT_00a686e0[];
extern unsigned int DAT_00a60860;
extern unsigned int DAT_00a60984;
extern unsigned int DAT_00a60990;
extern unsigned int DAT_00a609b8;
extern unsigned int DAT_00a60a60;
extern unsigned int DAT_00a633c0;

void __cdecl FUN_004248b0(int param_1)
{
  char cVar1;
  int bVar2;
  void *_File;
  unsigned int uVar3;
  char *pcVar4;
  long lVar5;
  int iVar6;
  char *pcVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int *puVar10;
  char *pcVar12;
  void *pFStack_414;
  unsigned int *puStack_410;
  int iStack_40c;
  int iStack_408;
  unsigned int uStack_404;
  char acStack_400[0x400];
  unsigned int uTmp;

  if (param_1 == 0) {
    return;
  }
  FUN_0049e720(s_strings_txt_0051c4b0, &DAT_0051c4bc, 1, 0);
  _File = DAT_00a07bd8;
  pFStack_414 = DAT_00a07bd8;
  if (DAT_00a07bd8 != (void *)0) {
    _fseek(DAT_00a07bd8, 0, 2);
    uVar3 = _ftell(_File);
    if (32000 < uVar3) {
      FUN_004ac6e0((unsigned int)DAT_009d7670);
      uVar3 = FUN_004ac5f0(uVar3, 0);
      DAT_009d7670 = (unsigned short)uVar3;
      if (DAT_009d7670 == 0) {
        uVar3 = FUN_0049eb60(0);
      }
    }
    pcVar4 = FUN_004ac720(((uVar3 >> 0x10) << 0x10) | (unsigned int)DAT_009d7670);
    _fseek(_File, 0, 0);
    lVar5 = _ftell(_File);
    if (lVar5 == 0) {
      goto LAB_004249ac;
    }
    _fclose(_File);
    FUN_0049e720(s_strings_txt_0051c4b0, &DAT_0051c4bc, 1, 0);
    pFStack_414 = DAT_00a07bd8;
    if (DAT_00a07bd8 != (void *)0) goto LAB_004249ac;
  }
LAB_00425a4b:
  _fclose(pFStack_414);
  FUN_0049eb60(2);
  return;

LAB_004249ac:
  puVar10 = &DAT_0099f900;
  do {
    iVar6 = FUN_00425a70(pFStack_414, acStack_400);
    if (iVar6 == -1) goto LAB_00425a4b;
    pcVar7 = acStack_400;
    pcVar12 = pcVar4;
    for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
      pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
      pcVar7 = pcVar7 + 4;
      pcVar12 = pcVar12 + 4;
    }
    for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *pcVar12 = *pcVar7;
      pcVar7 = pcVar7 + 1;
      pcVar12 = pcVar12 + 1;
    }
    *puVar10 = (unsigned int)pcVar4;
    pcVar4 = pcVar4 + iVar6 + 1;
    puVar10 = puVar10 + 1;
  } while (puVar10 < &DAT_0099f92c);

  iVar6 = 0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar8 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar8 = uVar3 - 2;
      }
      uVar3 = iVar8 + 1;
      if (iVar6 < 4) {
        pcVar7 = acStack_400;
        pcVar12 = pcVar4;
        for (uVar9 = uVar3 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
          pcVar7 = pcVar7 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar12 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar12 = pcVar12 + 1;
        }
        DAT_00a60a40[iVar6] = (unsigned int)pcVar4;
      }
      else {
        pcVar7 = acStack_400;
        pcVar12 = pcVar4;
        for (uVar9 = uVar3 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
          pcVar7 = pcVar7 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar12 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar12 = pcVar12 + 1;
        }
        DAT_00a609b0[iVar6] = (unsigned int)pcVar4;
      }
      pcVar4 = pcVar4 + iVar8 + 1;
      iVar6 = iVar6 + 1;
    }
  } while (iVar6 < 0x24);

  puVar10 = &DAT_00a606f0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a60704);

  iVar6 = 0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar8 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar8 = uVar3 - 2;
      }
      iVar6 = iVar6 + 1;
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar8 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar8 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      DAT_00a686d4 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar8 + 1;
    }
  } while (iVar6 < 1);

  iStack_40c = 0;
  puStack_410 = &DAT_00a65ce0;
  do {
    uStack_404 = iStack_40c % 0x2f;
    iStack_408 = 0;
    puVar10 = puStack_410;
    if (DAT_0051bea0[uStack_404] != '\0') {
      do {
        pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
        if (pcVar7 == (char *)0) goto LAB_00425a4b;
        acStack_400[0x3ff] = 0;
        if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
          uVar3 = 0xffffffff;
          pcVar7 = acStack_400;
          do {
            if (uVar3 == 0) break;
            uVar3 = uVar3 - 1;
            cVar1 = *pcVar7;
            pcVar7 = pcVar7 + 1;
          } while (cVar1 != '\0');
          uVar3 = ~uVar3;
          iVar6 = uVar3 - 1;
          if (acStack_400[uVar3 - 2] == '\n') {
            acStack_400[uVar3 - 2] = '\0';
            iVar6 = uVar3 - 2;
          }
          pcVar7 = acStack_400;
          pcVar12 = pcVar4;
          for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
            pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
            pcVar7 = pcVar7 + 4;
            pcVar12 = pcVar12 + 4;
          }
          for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
            *pcVar12 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar12 = pcVar12 + 1;
          }
          *puVar10 = (unsigned int)pcVar4;
          pcVar4 = pcVar4 + iVar6 + 1;
          puVar10 = puVar10 + 1;
          iStack_408 = iStack_408 + 1;
        }
      } while (iStack_408 < (int)(unsigned int)DAT_0051bea0[uStack_404]);
    }
    puStack_410 = puStack_410 + 0xe;
    iStack_40c = iStack_40c + 1;
  } while (puStack_410 < &DAT_00a68600);

  puVar10 = &DAT_00a68610;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a68648);

  puVar10 = &DAT_00a63380;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a63388);

  puVar10 = &DAT_00a63390;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a633b4);

  puVar10 = &DAT_00a686b0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a686d0);

  puVar10 = &DAT_00a68730;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a6873c);

  puVar10 = &DAT_00a68650;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a6866c);

  puVar10 = &DAT_00a68670;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a686b0);

  puVar10 = &DAT_00a0a790;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a0a7e0);

  puVar10 = &DAT_009a1840;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      iVar8 = 0;
      *puVar10 = (unsigned int)pcVar4;
      if (0 < iVar6) {
        do {
          if (*pcVar7 == '\\') {
            pcVar12 = pcVar7 + 2;
            if (pcVar7[1] == '0') {
              pcVar7 = pcVar7 + 3;
              iVar8 = iVar8 + 3;
              *pcVar4 = *pcVar12 + -0x30;
            }
            else {
              pcVar7 = pcVar7 + 3;
              iVar8 = iVar8 + 3;
              *pcVar4 = *pcVar12 + -0x28;
            }
          }
          else {
            *pcVar4 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            iVar8 = iVar8 + 1;
          }
          pcVar4 = pcVar4 + 1;
        } while (iVar8 < iVar6);
      }
      *pcVar4 = '\0';
      pcVar4 = pcVar4 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_009a1ec4);

  puVar10 = &DAT_00a0a0e0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a0a128);

  puVar10 = &DAT_00a08380;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a083b8);

  puVar10 = &DAT_00a08bf0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a08c74);

  puVar10 = &DAT_00a0a130;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a0a1d0);

  puVar10 = &DAT_00a08350;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a08360);

  puVar10 = &DAT_00a607f0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a60814);

  iVar6 = 0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar8 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar8 = uVar3 - 2;
      }
      uVar3 = iVar8 + 1;
      if (iVar6 == 0xd) {
        pcVar7 = acStack_400;
        pcVar12 = pcVar4;
        for (uVar9 = uVar3 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
          pcVar7 = pcVar7 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar12 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar12 = pcVar12 + 1;
        }
        DAT_00a607ac = (unsigned int)pcVar4;
      }
      else {
        pcVar7 = acStack_400;
        pcVar12 = pcVar4;
        for (uVar9 = uVar3 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
          pcVar7 = pcVar7 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
          *pcVar12 = *pcVar7;
          pcVar7 = pcVar7 + 1;
          pcVar12 = pcVar12 + 1;
        }
        DAT_00a60820[iVar6] = (unsigned int)pcVar4;
      }
      pcVar4 = pcVar4 + iVar8 + 1;
      iVar6 = iVar6 + 1;
    }
  } while (iVar6 < 0xe);

  puVar10 = &DAT_00a607b0;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a607f0);

  iStack_40c = 0;
  puStack_410 = (unsigned int *)&DAT_0051c56a;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      uTmp = (unsigned int)pcVar4;
      __builtin_memcpy(puStack_410, &uTmp, 4);
      if (acStack_400[0] == 'm') {
        DAT_00a686e0[iStack_40c] = 0;
      }
      else if (acStack_400[0] == 'f') {
        DAT_00a686e0[iStack_40c] = 1;
      }
      else if (acStack_400[0] == 'n') {
        DAT_00a686e0[iStack_40c] = 2;
      }
      else {
        FUN_0049eb60(3);
      }
      pcVar12 = acStack_400 + 2;
      pcVar7 = pcVar4;
      for (uVar3 = (iVar6 - 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar7[0] = pcVar12[0]; pcVar7[1] = pcVar12[1]; pcVar7[2] = pcVar12[2]; pcVar7[3] = pcVar12[3];
        pcVar12 = pcVar12 + 4;
        pcVar7 = pcVar7 + 4;
      }
      for (uVar3 = (iVar6 - 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar7 = *pcVar12;
        pcVar12 = pcVar12 + 1;
        pcVar7 = pcVar7 + 1;
      }
      pcVar4 = pcVar4 + iVar6 + -1;
      puStack_410 = (unsigned int *)((int)puStack_410 + 0x101);
      iStack_40c = iStack_40c + 1;
    }
  } while ((char *)puStack_410 < &DAT_00520eb3);

  puVar10 = &DAT_00a60860;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a60984);

  puVar10 = &DAT_00a60990;
  do {
    pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
    if (pcVar7 == (char *)0) goto LAB_00425a4b;
    acStack_400[0x3ff] = 0;
    if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
      uVar3 = 0xffffffff;
      pcVar7 = acStack_400;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar7;
        pcVar7 = pcVar7 + 1;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      iVar6 = uVar3 - 1;
      if (acStack_400[uVar3 - 2] == '\n') {
        acStack_400[uVar3 - 2] = '\0';
        iVar6 = uVar3 - 2;
      }
      pcVar7 = acStack_400;
      pcVar12 = pcVar4;
      for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
        pcVar7 = pcVar7 + 4;
        pcVar12 = pcVar12 + 4;
      }
      for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar12 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar12 = pcVar12 + 1;
      }
      *puVar10 = (unsigned int)pcVar4;
      pcVar4 = pcVar4 + iVar6 + 1;
      puVar10 = puVar10 + 1;
    }
  } while (puVar10 < &DAT_00a609b8);

  iVar6 = 0;
  bVar2 = 0;
  do {
    if (DAT_00a686e0[iVar6] == 1) {
      bVar2 = 1;
      break;
    }
    iVar6 = iVar6 + 1;
  } while (iVar6 < 0x49);
  if (bVar2) {
    iStack_40c = 0;
    puStack_410 = &DAT_00a60a60;
    do {
      uStack_404 = iStack_40c % 0x2f;
      iStack_408 = 0;
      puVar10 = puStack_410;
      if (DAT_0051bea0[uStack_404] != '\0') {
        do {
          pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
          if (pcVar7 == (char *)0) goto LAB_00425a4b;
          acStack_400[0x3ff] = 0;
          if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
            uVar3 = 0xffffffff;
            pcVar7 = acStack_400;
            do {
              if (uVar3 == 0) break;
              uVar3 = uVar3 - 1;
              cVar1 = *pcVar7;
              pcVar7 = pcVar7 + 1;
            } while (cVar1 != '\0');
            uVar3 = ~uVar3;
            iVar6 = uVar3 - 1;
            if (acStack_400[uVar3 - 2] == '\n') {
              acStack_400[uVar3 - 2] = '\0';
              iVar6 = uVar3 - 2;
            }
            pcVar7 = acStack_400;
            pcVar12 = pcVar4;
            for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
              pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
              pcVar7 = pcVar7 + 4;
              pcVar12 = pcVar12 + 4;
            }
            for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
              *pcVar12 = *pcVar7;
              pcVar7 = pcVar7 + 1;
              pcVar12 = pcVar12 + 1;
            }
            *puVar10 = (unsigned int)pcVar4;
            pcVar4 = pcVar4 + iVar6 + 1;
            puVar10 = puVar10 + 1;
            iStack_408 = iStack_408 + 1;
          }
        } while (iStack_408 < (int)(unsigned int)DAT_0051bea0[uStack_404]);
      }
      puStack_410 = puStack_410 + 0xe;
      iStack_40c = iStack_40c + 1;
    } while (puStack_410 < &DAT_00a63380);
  }

  iVar6 = 0;
  bVar2 = 0;
  do {
    if (DAT_00a686e0[iVar6] == 2) {
      bVar2 = 1;
      break;
    }
    iVar6 = iVar6 + 1;
  } while (iVar6 < 0x49);
  if (bVar2) {
    iStack_40c = 0;
    puStack_410 = &DAT_00a633c0;
    do {
      uStack_404 = iStack_40c % 0x2f;
      iStack_408 = 0;
      puVar10 = puStack_410;
      if (DAT_0051bea0[uStack_404] != '\0') {
        do {
          pcVar7 = _fgets(acStack_400, 0x400, pFStack_414);
          if (pcVar7 == (char *)0) goto LAB_00425a4b;
          acStack_400[0x3ff] = 0;
          if ((acStack_400[0] != '/') || (acStack_400[1] != '/')) {
            uVar3 = 0xffffffff;
            pcVar7 = acStack_400;
            do {
              if (uVar3 == 0) break;
              uVar3 = uVar3 - 1;
              cVar1 = *pcVar7;
              pcVar7 = pcVar7 + 1;
            } while (cVar1 != '\0');
            uVar3 = ~uVar3;
            iVar6 = uVar3 - 1;
            if (acStack_400[uVar3 - 2] == '\n') {
              acStack_400[uVar3 - 2] = '\0';
              iVar6 = uVar3 - 2;
            }
            pcVar7 = acStack_400;
            pcVar12 = pcVar4;
            for (uVar3 = (iVar6 + 1U) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
              pcVar12[0] = pcVar7[0]; pcVar12[1] = pcVar7[1]; pcVar12[2] = pcVar7[2]; pcVar12[3] = pcVar7[3];
              pcVar7 = pcVar7 + 4;
              pcVar12 = pcVar12 + 4;
            }
            for (uVar3 = (iVar6 + 1U) & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
              *pcVar12 = *pcVar7;
              pcVar7 = pcVar7 + 1;
              pcVar12 = pcVar12 + 1;
            }
            *puVar10 = (unsigned int)pcVar4;
            pcVar4 = pcVar4 + iVar6 + 1;
            puVar10 = puVar10 + 1;
            iStack_408 = iStack_408 + 1;
          }
        } while (iStack_408 < (int)(unsigned int)DAT_0051bea0[uStack_404]);
      }
      puStack_410 = puStack_410 + 0xe;
      iStack_40c = iStack_40c + 1;
    } while (puStack_410 < &DAT_00a65ce0);
  }
  _fclose(pFStack_414);
  return;
}
