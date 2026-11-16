/* SPDX-License-Identifier: MIT */
/* z_bop 0046aed0 */
extern char DAT_00527010[];
extern char DAT_00527008[];
extern char DAT_00527004;
extern char DAT_0051c4bc;
extern void *DAT_00a07bd8;
extern unsigned char DAT_009a8e40[];
extern char DAT_009d1320[];
extern char DAT_009d136f;
extern char DAT_009d1370;
extern char DAT_009d1371;
extern unsigned char *DAT_00a07cf0[];
extern int DAT_009a8068;
extern char s_LOCATARGET_00525280[];
extern char DAT_005252d0;
extern char s_NULLMANR_005255b8[];
extern char DAT_00525608;
extern char s_NULLORDR_00526050[];
extern char DAT_005260a0;

void __cdecl FUN_0049e720(char *, void *, int, int);
int __cdecl FUN_0046adf0(char *, void *);
unsigned int __cdecl FUN_0046ac80(char *);
int __cdecl FUN_0046acb0(void);
int __cdecl FUN_0046acd0(unsigned char *);
int __cdecl FUN_0046ad30(unsigned char *);
int __cdecl FUN_0046ad90(unsigned char *);
int __cdecl _fclose(void *);
char * __cdecl _strncpy(char *, const char *, unsigned int);
unsigned int __cdecl _fwrite(const void *, unsigned int, unsigned int, void *);

unsigned int __cdecl FUN_0046aed0(char *param_1)
{
  char cVar1;
  void *pFVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  int iVar5;
  char *pcVar6;
  unsigned char *puVar7;
  unsigned char *puVar8;
  char *pcVar9;
  char *pcVar10;
  unsigned int sStack_204;
  unsigned char abStack_200[256];
  char acStack_100[256];
  int iOff;

  uVar3 = 0xffffffff;
  pcVar6 = param_1;
  do {
    pcVar10 = pcVar6;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar10 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar10;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar6 = pcVar10 - uVar3;
  pcVar10 = acStack_100;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    __builtin_memcpy(pcVar10, pcVar6, 4);
    pcVar6 = pcVar6 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar10 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pcVar10 = pcVar10 + 1;
  }
  uVar3 = 0xffffffff;
  pcVar6 = DAT_00527010;
  do {
    pcVar10 = pcVar6;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar10 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar10;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  iVar5 = -1;
  pcVar6 = acStack_100;
  do {
    pcVar9 = pcVar6;
    if (iVar5 == 0) break;
    iVar5 = (int)((unsigned int)iVar5 - 1U);
    pcVar9 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar9;
  } while (cVar1 != '\0');
  pcVar6 = pcVar10 - uVar3;
  pcVar10 = pcVar9 - 1;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    __builtin_memcpy(pcVar10, pcVar6, 4);
    pcVar6 = pcVar6 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar10 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pcVar10 = pcVar10 + 1;
  }
  FUN_0049e720(acStack_100, &DAT_0051c4bc, 0, 0);
  pFVar2 = DAT_00a07bd8;
  if (DAT_00a07bd8 == (void *)0) {
    return 0;
  }
  puVar7 = DAT_009a8e40;
LAB_0046af70:
  iVar5 = FUN_0046adf0((char *)abStack_200, pFVar2);
  if (iVar5 == 0) {
    _fclose(pFVar2);
    return 0;
  }
  if (abStack_200[0] != 0x2a) {
    sStack_204 = FUN_0046ac80((char *)abStack_200);
    if (sStack_204 == 0x100) {
      sStack_204 = (unsigned int)FUN_0046acb0();
      if (sStack_204 == 0x100) {
        _fclose(pFVar2);
        return 0;
      }
    }
    else if ((&DAT_009d1370)[sStack_204 * 0x55U] == '\x01') {
      _fclose(pFVar2);
      return 0;
    }
    _strncpy(DAT_009d1320 + sStack_204 * 0x55U, (char *)abStack_200, 0x50);
    iVar5 = (int)(sStack_204 * 0x55U);
    (&DAT_009d136f)[iVar5] = 0;
    (&DAT_009d1370)[iVar5] = 1;
    iOff = (int)(puVar7 - DAT_009a8e40);
    __builtin_memcpy(&DAT_009d1371 + iVar5, &iOff, 4);
    DAT_00a07cf0[sStack_204] = puVar7;
    iVar5 = FUN_0046adf0((char *)abStack_200, pFVar2);
    if (iVar5 == 0) {
      _fclose(pFVar2);
      return 0;
    }
    iVar5 = FUN_0046acd0(abStack_200);
    if (s_LOCATARGET_00525280[(int)((unsigned int)iVar5 * 0x52U)] == '\0') {
      _fclose(pFVar2);
      return 0;
    }
    *puVar7 = (unsigned char)(&DAT_005252d0)[(int)((unsigned int)iVar5 * 0x52U)];
    iVar5 = FUN_0046adf0((char *)abStack_200, pFVar2);
    if (iVar5 == 0) {
      _fclose(pFVar2);
      return 0;
    }
    iVar5 = FUN_0046ad30(abStack_200);
    if (s_NULLMANR_005255b8[(int)((unsigned int)iVar5 * 0x52U)] == '\0') {
      _fclose(pFVar2);
      return 0;
    }
    uVar3 = (unsigned int)(unsigned char)(&DAT_00525608)[(int)((unsigned int)iVar5 * 0x52U)];
    puVar7 = puVar7 + 1;
    do {
      *puVar7 = (unsigned char)uVar3;
      puVar8 = puVar7;
      while (1) {
        iVar5 = FUN_0046adf0((char *)abStack_200, pFVar2);
        if (iVar5 == 0) {
          _fclose(pFVar2);
          return 0;
        }
        iVar5 = FUN_0046ad90(abStack_200);
        if (s_NULLORDR_00526050[(int)((unsigned int)iVar5 * 0x52U)] == '\0') {
          _fclose(pFVar2);
          return 0;
        }
        puVar7 = puVar8 + 2;
        puVar8[1] = (unsigned char)(&DAT_005260a0)[(int)((unsigned int)iVar5 * 0x52U)];
        if (iVar5 == 0) {
          DAT_009a8068 = (int)((unsigned int)DAT_009a8068 + 1U);
          goto LAB_0046af70;
        }
        iVar5 = FUN_0046adf0((char *)abStack_200, pFVar2);
        if (iVar5 == 0) {
          _fclose(pFVar2);
          return 0;
        }
        uVar3 = FUN_0046ac80((char *)abStack_200);
        if (uVar3 != 0x100) break;
        iVar5 = FUN_0046acb0();
        if (iVar5 == 0x100) {
          _fclose(pFVar2);
          return 0;
        }
        _strncpy(DAT_009d1320 + (int)((unsigned int)iVar5 * 0x55U), (char *)abStack_200, 0x50);
        (&DAT_009d136f)[(int)((unsigned int)iVar5 * 0x55U)] = 0;
        (&DAT_009d1370)[(int)((unsigned int)iVar5 * 0x55U)] = 0;
        *puVar7 = (unsigned char)iVar5;
        puVar8 = puVar7;
      }
    } while (1);
  }
  _fclose(pFVar2);
  sStack_204 = 0;
  do {
    if ((DAT_009d1320[sStack_204 * 0x55U] != '\0') &&
        ((&DAT_009d1370)[sStack_204 * 0x55U] != '\x01')) {
      return 0;
    }
    sStack_204 = sStack_204 + 1;
  } while ((int)sStack_204 < 0x100);
  uVar3 = 0xffffffff;
  do {
    pcVar6 = param_1;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar6 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar6;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar6 = pcVar6 - uVar3;
  pcVar10 = acStack_100;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    __builtin_memcpy(pcVar10, pcVar6, 4);
    pcVar6 = pcVar6 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar10 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pcVar10 = pcVar10 + 1;
  }
  uVar3 = 0xffffffff;
  pcVar6 = DAT_00527008;
  do {
    pcVar10 = pcVar6;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar10 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar10;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  iVar5 = -1;
  pcVar6 = acStack_100;
  do {
    pcVar9 = pcVar6;
    if (iVar5 == 0) break;
    iVar5 = (int)((unsigned int)iVar5 - 1U);
    pcVar9 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar9;
  } while (cVar1 != '\0');
  pcVar6 = pcVar10 - uVar3;
  pcVar10 = pcVar9 - 1;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    __builtin_memcpy(pcVar10, pcVar6, 4);
    pcVar6 = pcVar6 + 4;
    pcVar10 = pcVar10 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar10 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pcVar10 = pcVar10 + 1;
  }
  FUN_0049e720(acStack_100, &DAT_00527004, 0, 1);
  pFVar2 = DAT_00a07bd8;
  if (DAT_00a07bd8 != (void *)0) {
    sStack_204 = 0x5500;
    _fwrite(&sStack_204, 4, 1, DAT_00a07bd8);
    _fwrite(DAT_009d1320, sStack_204, 1, pFVar2);
    sStack_204 = 0xffff;
    _fwrite(&sStack_204, 4, 1, pFVar2);
    _fwrite(DAT_009a8e40, sStack_204, 1, pFVar2);
    _fclose(pFVar2);
  }
  return 1;
}
