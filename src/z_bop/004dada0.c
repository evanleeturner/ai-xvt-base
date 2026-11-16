/* SPDX-License-Identifier: MIT */
/* z_bop 004dada0 */
extern unsigned char DAT_00aabd11[];
extern unsigned char DAT_00aa80a3[];
extern char DAT_00aa86ab;
extern char DAT_00aac319;
extern unsigned char DAT_00aa7b0d;
extern unsigned char DAT_00aa7b13[];
extern unsigned char DAT_00aa7b1f[];
extern unsigned char DAT_00aa7b32[];
extern unsigned char DAT_00aa7b3a[];
extern unsigned int *DAT_00aa6cfc;
extern unsigned char DAT_00aa6d00[];
extern char DAT_0051a854;
extern char s_times_u_abp_0052c068[];
extern char s_times_new_roman_0052bb00[];

extern int __cdecl _sprintf(char *, const char *, ...);
extern void *__cdecl FUN_004cc4a0(char *, char *);
extern void __cdecl FUN_004cc590(void *);
extern int __cdecl FUN_004dc0a0(char *, int);
extern void __cdecl FUN_004d4970(void);
extern void *__cdecl _malloc(unsigned int);
extern void __cdecl _free(void *);
extern void *__cdecl _realloc(void *, unsigned int);
extern unsigned int *__cdecl FUN_004d48e0(void);
extern int __cdecl FUN_004d37e0(void);
extern unsigned int __cdecl FUN_004b7840(unsigned char *, unsigned char *, unsigned int);
extern void __cdecl FUN_004dc020(char *, void *, unsigned int);

unsigned int __cdecl FUN_004dada0(int param_1)
{
  unsigned char bVar1;
  short sVar2;
  char *pcVar3;
  void *pFVar4;
  HGDIOBJ pvVar5;
  unsigned int uVar6;
  void *pvVar7;
  unsigned int *puVar8;
  unsigned int uVar9;
  int iVar10;
  unsigned char *piVar11;
  int iVar12;
  unsigned int *puVar13;
  unsigned int *puVar14;
  char local_452[2];
  HDC local_450;
  int iStack_44c;
  unsigned int sStack_448;
  unsigned int local_444;
  unsigned int *local_440;
  HFONT local_43c;
  unsigned char *iStack_438;
  unsigned int local_434;
  long local_430[2];
  unsigned int uStack_428;
  unsigned char *local_424;
  int local_420[4];
  int tStack_410[4];
  unsigned int local_400[256];
  void *pvObjA;
  void *pvObjB;
  void *pvTmp;
  int iTmp;
  unsigned int uTmp;

  if (param_1 < 0x100) {
    if (param_1 < 1) {
      param_1 = 1;
    }
  }
  else {
    param_1 = 0xff;
  }
  piVar11 = (unsigned char *)0;
  local_444 = 0;
  local_424 = DAT_00aabd11 + (unsigned int)param_1 * 4;
  __builtin_memcpy(&iTmp, local_424, 4);
  if (iTmp != 0) {
    return 1;
  }
  iVar12 = 0;
  pcVar3 = &DAT_00aa86ab;
  do {
    if (*pcVar3 == '\0') {
      piVar11 = DAT_00aa80a3 + (unsigned int)iVar12 * 0x60b;
      break;
    }
    pcVar3 = pcVar3 + 0x60b;
    iVar12 = iVar12 + 1;
  } while (pcVar3 < &DAT_00aac319);
  if (piVar11 == (unsigned char *)0) {
    return 0;
  }
  _sprintf((char *)local_400, s_times_u_abp_0052c068, param_1);
  pFVar4 = FUN_004cc4a0((char *)local_400, &DAT_0051a854);
  if (pFVar4 != (void *)0) {
    FUN_004cc590(pFVar4);
    iVar12 = FUN_004dc0a0((char *)local_400, iVar12);
    if (iVar12 != 0) {
      return 1;
    }
  }
  local_43c = CreateFontA(-param_1, 0, 0, 0, 400, 0, 0, 0, 0, 4, 0, 3, 0x12, s_times_new_roman_0052bb00);
  if (local_43c == (HFONT)0) {
    return 0;
  }
  local_434 = (unsigned int)DAT_00aa7b0d;
  FUN_004d4970();
  local_450 = GetDC((HWND)0);
  pvVar5 = SelectObject(local_450, local_43c);
  SetMapMode(local_450, 1);
  SetTextCharacterExtra(local_450, 0);
  *(unsigned char *)(piVar11 + 0x504) = 0;
  iVar12 = 1;
  do {
    local_452[0] = (char)iVar12;
    local_452[1] = 0;
    GetTextExtentPoint32A(local_450, local_452, 1, (LPSIZE)local_430);
    if (local_430[0] < 0) {
      local_430[0] = 0;
    }
    if (local_430[1] < 0) {
      local_430[1] = 0;
    }
    iVar10 = iVar12 + 1;
    *(unsigned char *)(piVar11 + iVar12 + 0x504) = (unsigned char)local_430[0];
    *(unsigned char *)(piVar11 + iVar12 + 0x404) = (unsigned char)local_430[1];
    local_444 = local_444 + (unsigned int)local_430[1] * (unsigned int)local_430[0];
    iVar12 = iVar10;
  } while (iVar10 < 0x100);
  *(unsigned char *)(piVar11 + 0x404) = *(unsigned char *)(piVar11 + 0x405);
  SelectObject(local_450, pvVar5);
  ReleaseDC((HWND)0, local_450);
  local_440 = (unsigned int *)_malloc(local_444);
  if (local_440 == (unsigned int *)0) {
    DeleteObject(local_43c);
    if (local_434 != 0) {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    return 0;
  }
  pvTmp = local_440;
  __builtin_memcpy(piVar11, &pvTmp, 4);
  iTmp = 0;
  __builtin_memcpy(piVar11 + 4, &iTmp, 4);
  local_420[0] = 0;
  local_420[1] = 0;
  local_420[2] = 0x280;
  local_420[3] = 0x1e0;
  for (;;) {
    __builtin_memcpy(&pvObjA, DAT_00aa7b3a, 4);
    __builtin_memcpy(&pvObjB, DAT_00aa7b32, 4);
    iVar12 = ((int (__stdcall *)(void *, unsigned int, unsigned int, void *, RECT *, unsigned int))(*(void ***)pvObjA)[7])(pvObjA, 0, 0, pvObjB, (RECT *)local_420, 0);
    if (iVar12 == 0) {
      break;
    }
    if (iVar12 == -0x7789fe3e) {
      iVar12 = FUN_004d37e0();
      if (iVar12 != 0) {
        DeleteObject(local_43c);
        if (local_434 != 0) {
          DAT_00aa6cfc = FUN_004d48e0();
        }
        _free(local_440);
        return 0;
      }
    }
    else if (iVar12 != -0x7789fde4) {
      _free(local_440);
      DeleteObject(local_43c);
      if (local_434 != 0) {
        DAT_00aa6cfc = FUN_004d48e0();
      }
      return 0;
    }
  }
  sStack_448 = 0;
  iStack_44c = 1;
  uVar6 = local_400[0];
LAB_004db0e2:
  do {
    __builtin_memcpy(&pvObjB, DAT_00aa7b32, 4);
    iVar12 = ((int (__stdcall *)(void *, HDC *))(*(void ***)pvObjB)[0x11])(pvObjB, &local_450);
    if (iVar12 != 0) {
      if (iVar12 == -0x7789fe3e) {
        iVar10 = FUN_004d37e0();
        iVar12 = 0;
        if (iVar10 != 0) break;
      }
      if (iVar12 == -0x7789fde4) goto LAB_004db0e2;
      if (iVar12 != 0) break;
    }
    SetMapMode(local_450, 1);
    pvVar5 = SelectObject(local_450, local_43c);
    SetTextColor(local_450, 0xffffff);
    SetBkColor(local_450, 0);
    SetBkMode(local_450, 2);
    SetRect((RECT *)tStack_410, 0, 0, 0x280, 0x1e0);
    local_452[1] = 0;
    local_452[0] = (char)iStack_44c;
    ExtTextOutA(local_450, 0, 0, 2, (RECT *)tStack_410, local_452, 1, (int *)0);
    SelectObject(local_450, pvVar5);
    __builtin_memcpy(&pvObjB, DAT_00aa7b32, 4);
    ((int (__stdcall *)(void *, HDC))(*(void ***)pvObjB)[0x1a])(pvObjB, local_450);
    DAT_00aa6cfc = FUN_004d48e0();
    uStack_428 = 0;
    __builtin_memcpy(piVar11 + ((unsigned int)iStack_44c + 1) * 4, &sStack_448, 4);
    iStack_438 = piVar11 + iStack_44c;
    if (*(char *)(iStack_438 + 0x404) != '\0') {
      do {
        __builtin_memcpy(&iTmp, DAT_00aa7b1f, 4);
        if (iTmp == 8) {
          bVar1 = *(unsigned char *)(iStack_438 + 0x504);
          puVar8 = DAT_00aa6cfc;
LAB_004db267:
          uVar6 = FUN_004b7840(DAT_00aa6d00, (unsigned char *)puVar8, (unsigned int)bVar1);
        }
        else if (iTmp == 0x10) {
          iVar12 = 0;
          if (*(char *)(iStack_438 + 0x504) != '\0') {
            do {
              sVar2 = *(short *)((int)DAT_00aa6cfc + iVar12 * 2);
              *(unsigned char *)((int)local_400 + iVar12) = 0xff;
              if (sVar2 == 0) {
                *(unsigned char *)((int)local_400 + iVar12) = 0;
              }
              iVar12 = iVar12 + 1;
            } while (iVar12 < (int)(unsigned int)*(unsigned char *)(iStack_438 + 0x504));
          }
          bVar1 = *(unsigned char *)(iStack_438 + 0x504);
          puVar8 = local_400;
          goto LAB_004db267;
        }
        if ((int)local_444 <= (int)(sStack_448 + uVar6)) {
          local_444 = (unsigned int)((int)(local_444 * 3u) / 2);
          __builtin_memcpy(&pvTmp, piVar11, 4);
          pvVar7 = _realloc(pvTmp, local_444);
          __builtin_memcpy(piVar11, &pvVar7, 4);
          if (pvVar7 == (void *)0) break;
          local_440 = (unsigned int *)((int)pvVar7 + sStack_448);
        }
        __builtin_memcpy(&iTmp, piVar11, 4);
        if (iTmp == 0) break;
        puVar13 = (unsigned int *)DAT_00aa6d00;
        puVar14 = local_440;
        for (uVar9 = uVar6 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          *puVar14 = *puVar13;
          puVar13 = puVar13 + 1;
          puVar14 = puVar14 + 1;
        }
        for (uVar9 = uVar6 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
          *(unsigned char *)puVar14 = *(unsigned char *)puVar13;
          puVar13 = (unsigned int *)((int)puVar13 + 1);
          puVar14 = (unsigned int *)((int)puVar14 + 1);
        }
        sStack_448 = sStack_448 + uVar6;
        local_440 = (unsigned int *)((int)local_440 + uVar6);
        __builtin_memcpy(&iTmp, DAT_00aa7b13, 4);
        DAT_00aa6cfc = (unsigned int *)((int)DAT_00aa6cfc + iTmp);
        uStack_428 = uStack_428 + 1;
      } while (uStack_428 < (unsigned int)*(unsigned char *)(iStack_438 + 0x404));
    }
    FUN_004d4970();
    __builtin_memcpy(&iTmp, piVar11, 4);
    if ((iTmp == 0) || (iStack_44c = iStack_44c + 1, 0xff < iStack_44c)) break;
  } while (1);
  while (iVar12 == -0x7789fde4) {
    __builtin_memcpy(&pvObjB, DAT_00aa7b32, 4);
    __builtin_memcpy(&pvObjA, DAT_00aa7b3a, 4);
    iVar12 = ((int (__stdcall *)(void *, unsigned int, unsigned int, void *, RECT *, unsigned int))(*(void ***)pvObjB)[7])(pvObjB, 0, 0, pvObjA, (RECT *)local_420, 0);
    if (iVar12 == 0) goto LAB_004db358;
    if (iVar12 == -0x7789fe3e) {
      iVar10 = FUN_004d37e0();
      iVar12 = 0;
      if (iVar10 != 0) break;
    }
  }
  iStack_44c = 0;
LAB_004db358:
  if (local_434 != 0) {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  DeleteObject(local_43c);
  if ((int)sStack_448 < (int)local_444) {
    __builtin_memcpy(&pvTmp, piVar11, 4);
    pvVar7 = _realloc(pvTmp, sStack_448);
    __builtin_memcpy(piVar11, &pvVar7, 4);
    if (pvVar7 == (void *)0) {
      iStack_44c = 0;
    }
  }
  if (iStack_44c != 0x100) {
    __builtin_memcpy(&pvTmp, piVar11, 4);
    if (pvTmp != (void *)0) {
      _free(pvTmp);
    }
    return 0;
  }
  __builtin_memcpy(piVar11 + 0x604, &param_1, 4);
  *(unsigned char *)(piVar11 + 0x608) = 1;
  *(unsigned char *)(piVar11 + 0x609) = 0;
  *(unsigned char *)(piVar11 + 0x60a) = 1;
  uTmp = (unsigned int)piVar11;
  __builtin_memcpy(local_424, &uTmp, 4);
  _sprintf((char *)local_400, s_times_u_abp_0052c068, param_1);
  FUN_004dc020((char *)local_400, piVar11, sStack_448);
  return 1;
}
