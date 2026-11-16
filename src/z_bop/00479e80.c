/* SPDX-License-Identifier: MIT */
/* z_bop 00479e80 */
typedef struct _iobuf FILE;

extern FILE *DAT_00a07bd8;
extern char DAT_0051ae00[];
extern char DAT_0051a854[];
extern char DAT_005272cc[];
extern undefined1 DAT_005271e0[];

int __cdecl __strcmpi(const char *a, const char *b);
void __cdecl FUN_0049e720(char *p1, char *p2, int p3, int p4);
void __cdecl FUN_004720d0(undefined1 *p1, undefined1 *p2, int p3, int p4);
size_t __cdecl _fread(void *buf, size_t size, size_t count, FILE *f);
int __cdecl _fclose(FILE *f);
uint __cdecl FUN_0040e940(byte *p1, int p2, int p3, uint p4);

byte * __cdecl FUN_00479e80(undefined4 *param_1, char *param_2)
{
  byte *pbVar1;
  char cVar2;
  FILE *pFVar3;
  byte bVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  byte *pbVar10;
  char *pcVar11;
  byte *pbVar12;
  byte *pbVar13;
  char *pcVar14;
  size_t sVar15;
  byte bStack_410;
  byte bStack_40f;
  byte bStack_40e;
  byte *pbStack_40c;
  byte *pbStack_408;
  undefined4 uStack_404;
  char acStack_400[256];
  byte abStack_300[768];

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
    *(undefined4 *)pcVar14 = *(undefined4 *)pcVar11;
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
  iVar5 = __strcmpi(pcVar11, DAT_0051ae00);
  if (iVar5 == 0) {
    *pcVar11 = 't';
    acStack_400[uVar6 - 3] = 'e';
    acStack_400[uVar6 - 2] = 'x';
    FUN_0049e720(acStack_400, DAT_0051a854, 0, 0);
    *pcVar11 = 'r';
    acStack_400[uVar6 - 3] = 'g';
    acStack_400[uVar6 - 2] = 'b';
    if (DAT_00a07bd8 == (FILE *)0x0) {
      FUN_0049e720(acStack_400, DAT_0051a854, 0, 0);
      pFVar3 = DAT_00a07bd8;
      if (DAT_00a07bd8 == (FILE *)0x0) {
        param_1[4] = 8;
        param_1[5] = 8;
        param_1[1] = 0x10;
        *param_1 = 0x100;
        FUN_004720d0((undefined1 *)(param_1 + 6), DAT_005271e0, 8, 8);
        return (byte *)0x3058;
      }
      _fread(param_1, 0x200, 1, DAT_00a07bd8);
      iVar5 = (uint)*(byte *)((int)param_1 + 6) * 0x100 + (uint)*(byte *)((int)param_1 + 7);
      iVar8 = (uint)*(byte *)((int)param_1 + 9) + (uint)*(byte *)(param_1 + 2) * 0x100;
      uStack_404 = (undefined4)param_1;
      pbStack_40c = (byte *)param_1;
      pbVar13 = (byte *)(param_1 + 6);
      sVar15 = (size_t)((uint)iVar8 * (uint)iVar5);
      param_1[4] = iVar5;
      param_1[5] = iVar8;
      _fread(pbVar13, sVar15, 3, pFVar3);
      _fclose(pFVar3);
      pbVar1 = pbVar13 + sVar15;
      abStack_300[0] = *pbVar13 >> 3;
      abStack_300[1] = pbVar13[sVar15] >> 3;
      pbVar12 = (byte *)((int)param_1 + 0x19);
      abStack_300[2] = pbVar13[sVar15 * 2] >> 3;
      *pbVar13 = 0;
      if (1 < (int)sVar15) {
        pbStack_408 = (byte *)(sVar15 - 1);
        uVar6 = 1;
        pbVar13 = pbVar12;
        do {
          bStack_410 = *pbVar13 >> 3;
          bStack_40f = pbVar13[sVar15] >> 3;
          bStack_40e = pbVar13[sVar15 * 2] >> 3;
          uVar7 = FUN_0040e940(&bStack_410, (int)abStack_300, 0, uVar6);
          bVar4 = (byte)uVar7;
          iVar5 = (int)((uVar7 & 0xff) * 3);
          uVar7 = uVar6;
          if ((((abStack_300[iVar5] != bStack_410) || (abStack_300[iVar5 + 1] != bStack_40f)) ||
              (abStack_300[iVar5 + 2] != bStack_40e)) && ((int)uVar6 < 0x100)) {
            bVar4 = (byte)uVar6;
            uVar7 = uVar6 + 1;
            iVar5 = (int)((uVar6 & 0xff) * 3);
            abStack_300[iVar5] = bStack_410;
            abStack_300[iVar5 + 1] = bStack_40f;
            abStack_300[iVar5 + 2] = bStack_40e;
          }
          *pbVar12 = bVar4;
          pbVar12 = pbVar12 + 1;
          pbVar13 = pbVar13 + 1;
          pbStack_408 = (byte *)((int)pbStack_408 + -1);
          uVar6 = uVar7;
        } while (pbStack_408 != (byte *)0x0);
      }
      pbStack_408 = pbVar1 + 0x1000;
      pbVar13 = abStack_300;
      *(undefined4 *)((int)pbStack_40c + 4) = 0x10;
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
            iVar8 = (int)((uint)iVar5 * (uint)*pbVar13 * 0x100U);
            bStack_410 = (byte)((uint)(((int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 0xfU)) >> 4) +
                                       (uint)*pbVar13 * 0x80) >> 8);
            iVar8 = (int)((uint)iVar5 * (uint)pbVar13[1] * 0x100U);
            bStack_40f = (byte)((uint)(((int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 0xfU)) >> 4) +
                                       (uint)pbVar13[1] * 0x80) >> 8);
            iVar8 = (int)((uint)iVar5 * (uint)pbVar13[2] * 0x100U);
            iVar9 = (int)((uint)pbVar13[2] << 7);
            iVar8 = (int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 0xfU)) >> 4;
          }
          else {
            iVar8 = (int)((0x1fU - (uint)*pbVar13) * (uint)(iVar5 + -8) * 0x100U);
            bStack_410 = (byte)((uint)(((int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 7U)) >> 3) +
                                       (uint)*pbVar13 * 0x100) >> 8);
            iVar8 = (int)((0x1fU - (uint)pbVar13[1]) * (uint)(iVar5 + -8) * 0x100U);
            bStack_40f = (byte)((uint)(((int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 7U)) >> 3) +
                                       (uint)pbVar13[1] * 0x100) >> 8);
            iVar8 = (int)((0x1fU - (uint)pbVar13[2]) * (uint)(iVar5 + -8) * 0x100U);
            iVar9 = (int)((uint)pbVar13[2] << 8);
            iVar8 = (int)((uint)iVar8 + ((uint)(iVar8 >> 0x1f) & 7U)) >> 3;
          }
          iVar5 = iVar5 + 1;
          bStack_40e = (byte)((uint)((uint)iVar8 + (uint)iVar9) >> 8);
          /* uVar6 is dead here (reassigned below); used as the 16-bit store temporary */
          uVar6 = (uint)(ushort)(((ushort)bStack_410 * 0x20 + (ushort)bStack_40f) * 0x40 +
                                 (ushort)bStack_40e);
          __builtin_memcpy(pbVar12, &uVar6, 2);
          bStack_410 = bStack_410 * 2;
          bStack_40f = bStack_40f * 2;
          bStack_40e = bStack_40e * 2;
          uVar6 = FUN_0040e940(&bStack_410, 0x9a7bc0, 0x40, 0x100);
          *pbVar10 = (byte)uVar6;
          pbVar12 = pbVar12 + 0x200;
          pbVar10 = pbVar10 + 0x100;
        } while (iVar5 < 0x10);
        pbStack_408 = pbStack_408 + 2;
        pbStack_40c = pbStack_40c + 1;
        pbVar13 = pbVar13 + 3;
        /* original bound is &stack0x00000000, i.e. the end of abStack_300 in the frame */
      } while (pbVar13 < abStack_300 + 0x300);
      return (byte *)((uint)pbVar1 + (0x3000U - (uint)uStack_404));
    }
  }
  else {
    iVar5 = __strcmpi(pcVar11, DAT_005272cc);
    if (iVar5 != 0) {
      param_1[4] = 8;
      param_1[5] = 8;
      param_1[1] = 0x10;
      *param_1 = 0x100;
      FUN_004720d0((undefined1 *)(param_1 + 6), DAT_005271e0, 8, 8);
      return (byte *)0x3058;
    }
    FUN_0049e720(acStack_400, DAT_0051a854, 0, 0);
    if (DAT_00a07bd8 == (FILE *)0x0) {
      param_1[4] = 8;
      param_1[5] = 8;
      param_1[1] = 0x10;
      *param_1 = 0x100;
      FUN_004720d0((undefined1 *)(param_1 + 6), DAT_005271e0, 8, 8);
      return (byte *)0x3058;
    }
  }
  pFVar3 = DAT_00a07bd8;
  _fread(param_1, 0x18, 1, DAT_00a07bd8);
  sVar15 = param_1[4] * param_1[5];
  if (param_1[2] == sVar15) {
    sVar15 = param_1[3];
  }
  *param_1 = 0x100;
  param_1[1] = 0x10;
  _fread(param_1 + 6, sVar15, 1, pFVar3);
  _fread((void *)((int)param_1 + sVar15 + 0x1018), 0x2000, 1, pFVar3);
  _fclose(pFVar3);
  return (byte *)(sVar15 + 0x3018);
}
