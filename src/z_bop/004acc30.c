/* SPDX-License-Identifier: MIT */
/* z_bop 004acc30 */
typedef struct _iobuf FILE;

extern int DAT_00528100;
extern char DAT_000b0000[];
extern int DAT_00528114;
extern char s_mpDump_txt_0052811c[];
extern char DAT_005240e0[];
extern char DAT_00528118[];
extern int DAT_00528104;
extern int DAT_00528108;
extern int DAT_00528110;
extern int DAT_0052810c;

FILE * __cdecl FID_conflict___wfopen(const char *filename, const char *mode);
int __cdecl _fprintf(FILE *stream, const char *format, ...);
int __cdecl _fclose(FILE *stream);
void * __cdecl FID_conflict__memcpy(void *dest, const void *src, unsigned int count);

void __cdecl FUN_004acc30(char *param_1)
{
  char cVar1;
  int iVar2;
  FILE *_File;
  int iVar3;
  int iVar4;
  char *puVar5;
  unsigned int uVar6;
  int iVar7;
  char *pcVar8;

  if (DAT_00528100 == 0) {
    puVar5 = DAT_000b0000;
    iVar2 = 2000;
    do {
      *puVar5 = 0x20;
      puVar5[1] = 7;
      puVar5 = puVar5 + 2;
      iVar2 = iVar2 - 1;
    } while (iVar2 != 0);
    DAT_00528100 = 1;
  }
  if ((DAT_00528114 != 0) &&
     (_File = FID_conflict___wfopen(s_mpDump_txt_0052811c, DAT_005240e0), _File != (FILE *)0)) {
    _fprintf(_File, DAT_00528118, param_1);
    _fclose(_File);
  }
  if (*param_1 == '\n') {
    param_1 = param_1 + 1;
    DAT_00528104 = 0;
    DAT_00528108 = DAT_00528108 + 1;
  }
  do {
    uVar6 = 0xffffffff;
    pcVar8 = param_1;
    do {
      if (uVar6 == 0) break;
      uVar6 = uVar6 - 1;
      cVar1 = *pcVar8;
      pcVar8 = pcVar8 + 1;
    } while (cVar1 != '\0');
    iVar2 = ~uVar6 - 1;
    if (DAT_00528110 < DAT_00528108) {
      FID_conflict__memcpy
        (DAT_000b0000 + DAT_0052810c * 0xa0, (void *)(DAT_0052810c * 0xa0 + 0xb00a0),
         (DAT_00528110 - DAT_0052810c) * 0xa0);
      iVar3 = 0x50;
      puVar5 = DAT_000b0000 + DAT_00528110 * 0xa0;
      do {
        *puVar5 = 0x20;
        puVar5 = puVar5 + 2;
        iVar3 = iVar3 - 1;
      } while (iVar3 != 0);
      DAT_00528108 = DAT_00528110;
    }
    iVar3 = iVar2;
    if (0x50 < DAT_00528104 + iVar2) {
      iVar3 = 0x50 - DAT_00528104;
    }
    iVar4 = DAT_00528108 * 0x50 + DAT_00528104;
    iVar7 = 0;
    if (0 < iVar3) {
      do {
        if (param_1[iVar7] == '\n') {
          param_1 = param_1 + iVar7 + 1;
          DAT_00528108 = DAT_00528108 + 1;
          iVar2 = iVar2 - (iVar7 + 1);
          iVar3 = 0;
          DAT_00528104 = 0;
          break;
        }
        DAT_000b0000[iVar7 * 2 + iVar4 * 2] = param_1[iVar7];
        iVar7 = iVar7 + 1;
      } while (iVar7 < iVar3);
    }
    DAT_00528104 = DAT_00528104 + iVar3;
    if (iVar3 == iVar2) {
      return;
    }
    param_1 = param_1 + iVar3;
    DAT_00528108 = DAT_00528108 + 1;
    DAT_00528104 = 0;
  } while (1);
}
