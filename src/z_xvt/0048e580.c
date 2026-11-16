/* SPDX-License-Identifier: MIT */
/* z_xvt 0048e580 */
typedef struct _iobuf FILE;

extern FILE *DAT_00732948;
extern char DAT_005225d0[];
extern char DAT_0052a558[];
extern char DAT_00523570[];
extern unsigned char DAT_0052a478[];

int __cdecl __strcmpi(const char *, const char *);
void __cdecl FUN_0047ed40(char *, char *, int, int);
void __cdecl FUN_00486080(unsigned char *, unsigned char *, int, int);
unsigned int __cdecl _fread(void *, unsigned int, unsigned int, FILE *);
int __cdecl _fclose(FILE *);
unsigned int __cdecl FUN_00462920(unsigned char *, int, int, unsigned int);

unsigned char * __cdecl FUN_0048e580(unsigned int *param_1, char *param_2)
{
  unsigned char *pbVar1;
  char cVar2;
  FILE *pFVar3;
  unsigned char bVar4;
  int iVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  int iVar8;
  int iVar9;
  unsigned char *pbVar10;
  char *pcVar11;
  unsigned char *pbVar12;
  unsigned char *pbVar13;
  char *pcVar14;
  unsigned int sVar15;
  unsigned char bStack_410;
  unsigned char bStack_40f;
  unsigned char bStack_40e;
  unsigned char *pbStack_40c;
  unsigned char *pbStack_408;
  unsigned int *uStack_404;
  char acStack_400[256];
  unsigned char abStack_300[768];
  unsigned short usTmp;

  uVar6 = 0xffffffff;
  do {
    pcVar11 = param_2;
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    pcVar11 = param_2 + 1;
    cVar2 = *param_2;
    param_2 = pcVar11;
  } while (cVar2 != '\0');
  uVar6 = ~uVar6;
  pcVar11 = pcVar11 + -uVar6;
  pcVar14 = acStack_400;
  for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    *(unsigned int *)pcVar14 = *(unsigned int *)pcVar11;
    pcVar11 = pcVar11 + 4;
    pcVar14 = pcVar14 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar14 = *pcVar11;
    pcVar11 = pcVar11 + 1;
    pcVar14 = pcVar14 + 1;
  }
  uVar6 = 0xffffffff;
  pcVar11 = acStack_400;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar2 = *pcVar11;
    pcVar11 = pcVar11 + 1;
  } while (cVar2 != '\0');
  uVar6 = ~uVar6;
  pcVar11 = acStack_400 + (uVar6 - 4);
  iVar5 = __strcmpi(pcVar11, DAT_005225d0);
  if (iVar5 == 0) {
    *pcVar11 = 't';
    acStack_400[uVar6 - 3] = 'e';
    acStack_400[uVar6 - 2] = 'x';
    FUN_0047ed40(acStack_400, DAT_00523570, 0, 0);
    *pcVar11 = 'r';
    acStack_400[uVar6 - 3] = 'g';
    acStack_400[uVar6 - 2] = 'b';
    if (DAT_00732948 == (FILE *)0x0) {
      FUN_0047ed40(acStack_400, DAT_00523570, 0, 0);
      pFVar3 = DAT_00732948;
      if (DAT_00732948 == (FILE *)0x0) {
        param_1[4] = 8;
        param_1[5] = 8;
        param_1[1] = 0x10;
        *param_1 = 0x100;
        FUN_00486080((unsigned char *)(param_1 + 6), DAT_0052a478, 8, 8);
        return (unsigned char *)0x3058;
      }
      _fread(param_1, 0x200, 1, DAT_00732948);
      iVar5 = (unsigned int)*(unsigned char *)((int)param_1 + 6) * 0x100 + (unsigned int)*(unsigned char *)((int)param_1 + 7);
      iVar8 = (unsigned int)*(unsigned char *)((int)param_1 + 9) + (unsigned int)*(unsigned char *)(param_1 + 2) * 0x100;
      uStack_404 = param_1;
      pbStack_40c = (unsigned char *)param_1;
      pbVar13 = (unsigned char *)(param_1 + 6);
      sVar15 = (unsigned int)iVar8 * (unsigned int)iVar5;
      param_1[4] = iVar5;
      param_1[5] = iVar8;
      _fread(pbVar13, sVar15, 3, pFVar3);
      _fclose(pFVar3);
      pbVar1 = pbVar13 + sVar15;
      abStack_300[0] = *pbVar13 >> 3;
      abStack_300[1] = pbVar13[sVar15] >> 3;
      pbVar12 = (unsigned char *)((int)param_1 + 0x19);
      abStack_300[2] = pbVar13[sVar15 * 2] >> 3;
      *pbVar13 = 0;
      if (1 < (int)sVar15) {
        pbStack_408 = (unsigned char *)(sVar15 - 1);
        uVar6 = 1;
        pbVar13 = pbVar12;
        do {
          bStack_410 = *pbVar13 >> 3;
          bStack_40f = pbVar13[sVar15] >> 3;
          bStack_40e = pbVar13[sVar15 * 2] >> 3;
          uVar7 = FUN_00462920(&bStack_410, (int)abStack_300, 0, uVar6);
          bVar4 = (unsigned char)uVar7;
          iVar5 = (uVar7 & 0xff) * 3;
          uVar7 = uVar6;
          if ((((abStack_300[iVar5] != bStack_410) || (abStack_300[iVar5 + 1] != bStack_40f)) ||
              (abStack_300[iVar5 + 2] != bStack_40e)) && ((int)uVar6 < 0x100)) {
            bVar4 = (unsigned char)uVar6;
            uVar7 = uVar6 + 1;
            iVar5 = (uVar6 & 0xff) * 3;
            abStack_300[iVar5] = bStack_410;
            abStack_300[iVar5 + 1] = bStack_40f;
            abStack_300[iVar5 + 2] = bStack_40e;
          }
          *pbVar12 = bVar4;
          pbVar12 = pbVar12 + 1;
          pbVar13 = pbVar13 + 1;
          pbStack_408 = (unsigned char *)((int)pbStack_408 + -1);
          uVar6 = uVar7;
        } while (pbStack_408 != (unsigned char *)0x0);
      }
      pbStack_408 = pbVar1 + 0x1000;
      pbVar13 = abStack_300;
      *(unsigned int *)((int)pbStack_40c + 4) = 0x10;
      pbStack_40c[0] = 0;
      pbStack_40c[1] = 1;
      pbStack_40c[2] = 0;
      pbStack_40c[3] = 0;
      pbStack_40c = pbVar1;
      do {
        iVar5 = 0;
        pbVar12 = pbStack_408;
        pbVar10 = pbStack_40c;
        do {
          if (iVar5 < 8) {
            iVar8 = (int)((unsigned int)iVar5 * (unsigned int)*pbVar13 * 0x100);
            bStack_410 = (unsigned char)((((unsigned int)((iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 0xf)) >> 4)) + (unsigned int)*pbVar13 * 0x80) >> 8);
            iVar8 = (int)((unsigned int)iVar5 * (unsigned int)pbVar13[1] * 0x100);
            bStack_40f = (unsigned char)((((unsigned int)((iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 0xf)) >> 4)) + (unsigned int)pbVar13[1] * 0x80) >> 8);
            iVar8 = (int)((unsigned int)iVar5 * (unsigned int)pbVar13[2] * 0x100);
            iVar9 = (unsigned int)pbVar13[2] << 7;
            iVar8 = (iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 0xf)) >> 4;
          }
          else {
            iVar8 = (int)((0x1f - (unsigned int)*pbVar13) * (unsigned int)(iVar5 + -8) * 0x100);
            bStack_410 = (unsigned char)((((unsigned int)((iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 7)) >> 3)) + (unsigned int)*pbVar13 * 0x100) >> 8);
            iVar8 = (int)((0x1f - (unsigned int)pbVar13[1]) * (unsigned int)(iVar5 + -8) * 0x100);
            bStack_40f = (unsigned char)((((unsigned int)((iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 7)) >> 3)) + (unsigned int)pbVar13[1] * 0x100) >> 8);
            iVar8 = (int)((0x1f - (unsigned int)pbVar13[2]) * (unsigned int)(iVar5 + -8) * 0x100);
            iVar9 = (unsigned int)pbVar13[2] << 8;
            iVar8 = (iVar8 + (int)((unsigned int)(iVar8 >> 0x1f) & 7)) >> 3;
          }
          iVar5 = iVar5 + 1;
          bStack_40e = (unsigned char)((unsigned int)(iVar8 + iVar9) >> 8);
          usTmp = (unsigned short)(((unsigned short)bStack_410 * 0x20 + (unsigned short)bStack_40f) * 0x40 + (unsigned short)bStack_40e);
          __builtin_memcpy(pbVar12, &usTmp, 2);
          bStack_410 = (unsigned char)(bStack_410 * 2);
          bStack_40f = (unsigned char)(bStack_40f * 2);
          bStack_40e = (unsigned char)(bStack_40e * 2);
          uVar6 = FUN_00462920(&bStack_410, 0x6da540, 0x40, 0x100);
          *pbVar10 = (unsigned char)uVar6;
          pbVar12 = pbVar12 + 0x200;
          pbVar10 = pbVar10 + 0x100;
        } while (iVar5 < 0x10);
        pbStack_408 = pbStack_408 + 2;
        pbStack_40c = pbStack_40c + 1;
        pbVar13 = pbVar13 + 3;
      } while (pbVar13 < abStack_300 + 768);
      return pbVar1 + (0x3000 - (int)uStack_404);
    }
  }
  else {
    iVar5 = __strcmpi(pcVar11, DAT_0052a558);
    if (iVar5 != 0) {
      param_1[4] = 8;
      param_1[5] = 8;
      param_1[1] = 0x10;
      *param_1 = 0x100;
      FUN_00486080((unsigned char *)(param_1 + 6), DAT_0052a478, 8, 8);
      return (unsigned char *)0x3058;
    }
    FUN_0047ed40(acStack_400, DAT_00523570, 0, 0);
    if (DAT_00732948 == (FILE *)0x0) {
      param_1[4] = 8;
      param_1[5] = 8;
      param_1[1] = 0x10;
      *param_1 = 0x100;
      FUN_00486080((unsigned char *)(param_1 + 6), DAT_0052a478, 8, 8);
      return (unsigned char *)0x3058;
    }
  }
  pFVar3 = DAT_00732948;
  _fread(param_1, 0x18, 1, DAT_00732948);
  sVar15 = param_1[5] * param_1[4];
  if (param_1[2] == sVar15) {
    sVar15 = param_1[3];
  }
  *param_1 = 0x100;
  param_1[1] = 0x10;
  _fread(param_1 + 6, sVar15, 1, pFVar3);
  _fread((void *)((int)param_1 + sVar15 + 0x1018), 0x2000, 1, pFVar3);
  _fclose(pFVar3);
  return (unsigned char *)(sVar15 + 0x3018);
}
