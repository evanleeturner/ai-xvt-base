/* SPDX-License-Identifier: MIT */
/* z_xvt 00402840 */
typedef struct _iobuf FILE;

extern char DAT_00536820[];
extern char DAT_00523570[];
extern char DAT_0052356c[];
extern FILE *DAT_00732948;
extern char s__256s_00523564[];
extern char s__256s_00523550[];
extern char s__inventor_00523558[];
extern char s_ascii_00523548[];
extern char s_binary_00523540[];

void __cdecl FUN_0047ed40(char *, char *, int, int);
int __cdecl _fscanf(FILE *, const char *, ...);
int __cdecl __strnicmp(const char *, const char *, unsigned int);
short __cdecl FUN_00402a80(FILE *);
int __cdecl _fclose(FILE *);
unsigned short __cdecl FUN_0048be40(unsigned int);
void __cdecl FUN_0048aa90(char *, unsigned int);
short __cdecl FUN_0048a570(unsigned int);
unsigned short __cdecl FUN_00488490(char *);
void __cdecl FUN_00438dd0(void);

short __cdecl FUN_00402840(char *param_1)
{
  char cVar1;
  FILE *_File;
  unsigned short uVar3;
  short sVar4;
  char *puVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  int iVar8;
  char *pcVar9;
  char *pcVar10;
  char *pcVar11;
  unsigned int tmp;

  uVar6 = 0xffffffff;
  pcVar9 = param_1;
  do {
    pcVar11 = pcVar9;
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    pcVar11 = pcVar9 + 1;
    cVar1 = *pcVar9;
    pcVar9 = pcVar11;
  } while (cVar1 != '\0');
  uVar6 = ~uVar6;
  pcVar9 = pcVar11 - uVar6;
  pcVar11 = DAT_00536820;
  for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    __builtin_memcpy(&tmp, pcVar9, 4);
    __builtin_memcpy(pcVar11, &tmp, 4);
    pcVar9 = pcVar9 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar11 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    pcVar11 = pcVar11 + 1;
  }
  FUN_0047ed40(DAT_00536820, DAT_00523570, 0, 0);
  if (DAT_00732948 != (FILE *)0) {
    _fclose(DAT_00732948);
    uVar3 = FUN_00488490(DAT_00536820);
    FUN_00438dd0();
    sVar4 = FUN_0048a570((unsigned int)uVar3);
    FUN_00438dd0();
    return sVar4;
  }
  puVar5 = DAT_00536820;
  cVar1 = DAT_00536820[0];
  while (cVar1 != '.') {
    pcVar9 = puVar5 + 1;
    puVar5 = puVar5 + 1;
    cVar1 = *pcVar9;
  }
  uVar6 = 0xffffffff;
  *puVar5 = 0;
  pcVar9 = DAT_0052356c;
  do {
    pcVar11 = pcVar9;
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    pcVar11 = pcVar9 + 1;
    cVar1 = *pcVar9;
    pcVar9 = pcVar11;
  } while (cVar1 != '\0');
  uVar6 = ~uVar6;
  iVar8 = -1;
  pcVar9 = DAT_00536820;
  do {
    pcVar10 = pcVar9;
    if (iVar8 == 0) break;
    iVar8 = iVar8 + -1;
    pcVar10 = pcVar9 + 1;
    cVar1 = *pcVar9;
    pcVar9 = pcVar10;
  } while (cVar1 != '\0');
  pcVar9 = pcVar11 - uVar6;
  pcVar11 = pcVar10 - 1;
  for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    __builtin_memcpy(&tmp, pcVar9, 4);
    __builtin_memcpy(pcVar11, &tmp, 4);
    pcVar9 = pcVar9 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar11 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    pcVar11 = pcVar11 + 1;
  }
  FUN_0047ed40(DAT_00536820, DAT_00523570, 1, 0);
  _File = DAT_00732948;
  if (DAT_00732948 == (FILE *)0) {
    return 0;
  }
  iVar8 = _fscanf(DAT_00732948, s__256s_00523564, DAT_00536820);
  if (iVar8 != 1) {
    return 0;
  }
  iVar8 = __strnicmp(DAT_00536820, s__inventor_00523558, 9);
  if (iVar8 != 0) {
    return 0;
  }
  iVar8 = _fscanf(_File, s__256s_00523550, DAT_00536820);
  if (iVar8 != 1) {
    return 0;
  }
  iVar8 = _fscanf(_File, s__256s_00523550, DAT_00536820);
  if (iVar8 != 1) {
    return 0;
  }
  iVar8 = __strnicmp(DAT_00536820, s_ascii_00523548, 5);
  if (iVar8 == 0) {
    uVar6 = (unsigned int)(unsigned short)FUN_00402a80(_File);
  }
  else {
    iVar8 = __strnicmp(DAT_00536820, s_binary_00523540, 6);
    if (iVar8 != 0) {
      _fclose(_File);
      return 0;
    }
    uVar6 = 0;
  }
  _fclose(_File);
  uVar3 = FUN_0048be40(uVar6);
  uVar6 = 0xffffffff;
  do {
    pcVar9 = param_1;
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    pcVar9 = param_1 + 1;
    cVar1 = *param_1;
    param_1 = pcVar9;
  } while (cVar1 != '\0');
  uVar6 = ~uVar6;
  pcVar9 = pcVar9 - uVar6;
  pcVar11 = DAT_00536820;
  for (uVar7 = uVar6 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
    __builtin_memcpy(&tmp, pcVar9, 4);
    __builtin_memcpy(pcVar11, &tmp, 4);
    pcVar9 = pcVar9 + 4;
    pcVar11 = pcVar11 + 4;
  }
  for (uVar6 = uVar6 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
    *pcVar11 = *pcVar9;
    pcVar9 = pcVar9 + 1;
    pcVar11 = pcVar11 + 1;
  }
  FUN_0048aa90(DAT_00536820, (unsigned int)uVar3);
  sVar4 = FUN_0048a570((unsigned int)uVar3);
  return sVar4;
}
