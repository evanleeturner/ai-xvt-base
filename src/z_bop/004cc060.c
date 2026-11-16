/* SPDX-License-Identifier: MIT */
/* z_bop 004cc060 */
typedef struct _iobuf FILE;

extern unsigned int *DAT_00b69cd8;
extern unsigned int DAT_00b69cc8;
extern char DAT_00b69d1e[];
extern char DAT_00b69d20[];
extern char DAT_0051c4bc[];

FILE *__cdecl FUN_004cc4a0(char *param_1, char *param_2);
void __cdecl FUN_004cc590(FILE *param_1);
void __cdecl _free(void *param_1);
char *__cdecl _fgets(char *param_1, int param_2, FILE *param_3);
int __cdecl _atoi(const char *param_1);
void *__cdecl _malloc(unsigned int param_1);

unsigned int __cdecl FUN_004cc060(char *param_1)
{
  char cVar1;
  FILE *_File;
  char *pcVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int *puVar6;
  unsigned int *puVar7;
  unsigned int local_4;

  if (DAT_00b69cd8 != (unsigned int *)0) {
    _free(DAT_00b69cd8);
    DAT_00b69cd8 = (unsigned int *)0;
  }
  _File = FUN_004cc4a0(param_1, DAT_0051c4bc);
  if (_File == (FILE *)0) {
    return 0;
  }
  pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
  if (pcVar2 == (char *)0) {
    FUN_004cc590(_File);
    return 0;
  }
  local_4 = (unsigned int)_atoi(DAT_00b69d20);
  DAT_00b69cd8 = (unsigned int *)_malloc(local_4 * 0x424u);
  if (DAT_00b69cd8 == (unsigned int *)0) {
    return 0;
  }
  puVar6 = DAT_00b69cd8;
  for (uVar3 = (local_4 * 0x424u) >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *puVar6 = 0;
    puVar6 = puVar6 + 1;
  }
  for (iVar4 = 0; iVar4 != 0; iVar4 = iVar4 - 1) {
    *(unsigned char *)puVar6 = 0;
    puVar6 = (unsigned int *)((unsigned char *)puVar6 + 1);
  }
  DAT_00b69cc8 = 0;
  do {
    if (local_4 <= DAT_00b69cc8) {
      FUN_004cc590(_File);
      return 1;
    }
    do {
      pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
      if (pcVar2 == (char *)0) {
        FUN_004cc590(_File);
        return 1;
      }
    } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
    uVar3 = 0xffffffff;
    pcVar2 = DAT_00b69d20;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    if (DAT_00b69d1e[~uVar3] == '\n') {
      uVar3 = 0xffffffff;
      pcVar2 = DAT_00b69d20;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar2 + 1;
      } while (cVar1 != '\0');
      DAT_00b69d1e[~uVar3] = 0;
    }
    iVar4 = _atoi(DAT_00b69d20);
    DAT_00b69cd8[DAT_00b69cc8 * 0x109u] = (unsigned int)iVar4;
    do {
      pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
      if (pcVar2 == (char *)0) {
        DAT_00b69cd8[DAT_00b69cc8 * 0x109u] = 0;
        FUN_004cc590(_File);
        return 1;
      }
    } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
    uVar3 = 0xffffffff;
    pcVar2 = DAT_00b69d20;
    do {
      if (uVar3 == 0) break;
      uVar3 = uVar3 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    if (DAT_00b69d1e[~uVar3] == '\n') {
      uVar3 = 0xffffffff;
      pcVar2 = DAT_00b69d20;
      do {
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar2 + 1;
      } while (cVar1 != '\0');
      DAT_00b69d1e[~uVar3] = 0;
    }
    puVar6 = (unsigned int *)DAT_00b69d20;
    puVar7 = DAT_00b69cd8 + DAT_00b69cc8 * 0x109u + 1;
    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 - 1) {
      *puVar7 = *puVar6;
      puVar6 = puVar6 + 1;
      puVar7 = puVar7 + 1;
    }
    uVar3 = 0;
    *(unsigned char *)((unsigned int)DAT_00b69cd8 + DAT_00b69cc8 * 0x424u + 0x23u) = 0;
    while (uVar3 < 0x1e0) {
      do {
        pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
        if (pcVar2 == (char *)0) {
          DAT_00b69cd8[DAT_00b69cc8 * 0x109u] = 0;
          FUN_004cc590(_File);
          return 1;
        }
      } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
      uVar5 = 0xffffffff;
      pcVar2 = DAT_00b69d20;
      do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar2 + 1;
      } while (cVar1 != '\0');
      if (DAT_00b69d1e[~uVar5] == '\n') {
        uVar5 = 0xffffffff;
        pcVar2 = DAT_00b69d20;
        do {
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        DAT_00b69d1e[~uVar5] = 0;
      }
      puVar6 = (unsigned int *)DAT_00b69d20;
      puVar7 = (unsigned int *)((unsigned int)DAT_00b69cd8 + uVar3 + DAT_00b69cc8 * 0x424u + 0x224u);
      for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 - 1) {
        *puVar7 = *puVar6;
        puVar6 = puVar6 + 1;
        puVar7 = puVar7 + 1;
      }
      uVar3 = uVar3 + 0x20;
      *(unsigned char *)((unsigned int)DAT_00b69cd8 + uVar3 + DAT_00b69cc8 * 0x424u + 0x223u) = 0;
    }
    local_4 = 0;
    uVar3 = 0;
    while (uVar3 < 0x1e0) {
      do {
        pcVar2 = _fgets(DAT_00b69d20, 0xff, _File);
        if (pcVar2 == (char *)0) {
          DAT_00b69cd8[DAT_00b69cc8 * 0x109u] = 0;
          FUN_004cc590(_File);
          return 1;
        }
      } while ((*pcVar2 == '/') && (pcVar2[1] == '/'));
      uVar5 = 0xffffffff;
      pcVar2 = DAT_00b69d20;
      do {
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar2 + 1;
      } while (cVar1 != '\0');
      if (DAT_00b69d1e[~uVar5] == '\n') {
        uVar5 = 0xffffffff;
        pcVar2 = DAT_00b69d20;
        do {
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar2 + 1;
        } while (cVar1 != '\0');
        DAT_00b69d1e[~uVar5] = 0;
      }
      puVar6 = (unsigned int *)DAT_00b69d20;
      puVar7 = (unsigned int *)((unsigned int)DAT_00b69cd8 + uVar3 + DAT_00b69cc8 * 0x424u + 0x24u);
      for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 - 1) {
        *puVar7 = *puVar6;
        puVar6 = puVar6 + 1;
        puVar7 = puVar7 + 1;
      }
      uVar3 = uVar3 + 0x20;
      local_4 = local_4 + 1;
      *(unsigned char *)((unsigned int)DAT_00b69cd8 + uVar3 + DAT_00b69cc8 * 0x424u + 0x23u) = 0;
    }
    DAT_00b69cc8 = DAT_00b69cc8 + 1;
  } while (1);
}
