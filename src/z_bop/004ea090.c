/* SPDX-License-Identifier: MIT */
/* z_bop 004ea090 */
extern int DAT_00665d38;
extern int DAT_00665d3c;
extern int DAT_00665d40;
extern int DAT_00665d44;
extern int DAT_00665d48;
extern int DAT_00665d4c;
extern int DAT_00665d50;
extern int DAT_00665d54;
extern int DAT_00665d58;
extern int DAT_00665d5c;
extern int DAT_00aa6110;
extern unsigned int DAT_00b6a2c0;
extern char DAT_00b69d20[];
extern char DAT_00528118[];
extern char DAT_00523d40[];
extern char s__c_s__c_s_0052a6b0[];
extern char s__c_s__c__1f__s_0052c508[];
extern char s__c_s__c_d_0052c518[];
extern char s__c_s__c_0052c524[];
extern char s__c_s__c_d__s_0052c52c[];
extern double _DAT_00518308;
extern double _DAT_00518310;

void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
char * __cdecl FUN_004ddc10(int param_1);
void __cdecl FUN_004d6480(int *param_1, int *param_2);
int __cdecl FUN_004dbbd0(int param_1, char *param_2, int *param_3, int param_4, int param_5, int param_6);
__int64 __cdecl FUN_0042b270(void);

unsigned int __stdcall FUN_004ea090(void)
{
  char cVar1;
  char *pcVar2;
  char *pcVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  int iVar7;
  char *pcVar8;
  __int64 lVar9;
  int *piVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned int uVar13;
  int iVar14;
  int local_28[4];
  /* Ghidra: undefined8 uStack_18. Held as a double; uVar13 (low dword) and
     uVar12 (high dword) in the decompile are its two halves, passed to the
     %.1f vararg. */
  double uStack_18;
  /* contiguous 16-byte rect at the Ghidra stack slots -0x10, -0xc, -0x8, -0x4 */
  struct {
    int local_10;
    int local_c;
    int local_8;
    int local_4;
  } local_rect;
  unsigned int uTmp;

  FUN_004d6460(local_28, 0x58, 0x6f, 0x14c, 0x7d);
  iVar7 = (int)((unsigned int)DAT_00665d38 - 1u);
  if (iVar7 < 0) {
    iVar7 = 0;
  }
  iVar7 = (int)((unsigned int)iVar7 * 0x200u);
  _sprintf(DAT_00b69d20, DAT_00528118, (int)((unsigned int)DAT_00aa6110 + (unsigned int)iVar7));
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0x1e);
  pcVar2 = FUN_004ddc10((int)((unsigned int)DAT_00665d3c + 0x1f0u));
  uVar13 = 1;
  pcVar3 = FUN_004ddc10(0x1e3);
  _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar3, uVar13, pcVar2);
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0xf);
  iVar4 = (int)((unsigned int)DAT_00aa6110 + (unsigned int)iVar7 + 0x40u);
  uVar13 = 1;
  pcVar2 = FUN_004ddc10(0x1e4);
  _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar2, uVar13, iVar4);
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0xf);
  iVar4 = (int)((unsigned int)DAT_00aa6110 + (unsigned int)iVar7 + 0x80u);
  uVar13 = 1;
  pcVar2 = FUN_004ddc10(0x1e5);
  _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar2, uVar13, iVar4);
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0xf);
  piVar10 = local_28;
  iVar14 = 1;
  iVar4 = 0;
  uVar5 = DAT_00b6a2c0;
  pcVar2 = FUN_004ddc10(0x1e6);
  FUN_004db7d0(0xc, pcVar2, piVar10, iVar4, iVar14, uVar5);
  FUN_004d64a0(local_28, 0, 0xf);
  FUN_004d6480(&local_rect.local_10, local_28);
  local_rect.local_4 = (int)((unsigned int)local_rect.local_c + 0x78u);
  local_rect.local_10 = (int)((unsigned int)local_rect.local_10 + 0x14u);
  iVar4 = FUN_004dbbd0(0xc, (char *)((unsigned int)DAT_00aa6110 + (unsigned int)iVar7 + 0xc0u), &local_rect.local_10, 0xffff, 3, 0);
  FUN_004d64a0(local_28, 0, (int)(((unsigned int)iVar4 * 3u + 6u) * 5u));
  if (DAT_00665d3c == 0) {
    pcVar2 = FUN_004ddc10(0x201);
    uVar12 = 1;
    uVar13 = DAT_00665d40;
    pcVar3 = FUN_004ddc10(0x1e7);
    _sprintf(DAT_00b69d20, s__c_s__c_d__s_0052c52c, 4, pcVar3, uVar12, uVar13, pcVar2);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
    FUN_004d64a0(local_28, 0, 0xf);
    pcVar2 = FUN_004ddc10(0x202);
    uVar12 = 1;
    uVar13 = DAT_00665d44;
    pcVar3 = FUN_004ddc10(0x1e8);
    _sprintf(DAT_00b69d20, s__c_s__c_d__s_0052c52c, 4, pcVar3, uVar12, uVar13, pcVar2);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
    FUN_004d64a0(local_28, 0, 0xf);
    pcVar2 = FUN_004ddc10(0x2ca);
    uVar12 = 1;
    uVar13 = DAT_00665d48;
    pcVar3 = FUN_004ddc10(0x1e9);
    _sprintf(DAT_00b69d20, s__c_s__c_d__s_0052c52c, 4, pcVar3, uVar12, uVar13, pcVar2);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
    FUN_004d64a0(local_28, 0, 0xf);
    uVar13 = 1;
    pcVar2 = FUN_004ddc10(0x1ea);
    _sprintf(DAT_00b69d20, s__c_s__c_0052c524, 4, pcVar2, uVar13);
    if (DAT_00665d4c == 0) {
LAB_004ea58f:
      if (DAT_00665d50 != 0) {
        pcVar2 = FUN_004ddc10((int)((unsigned int)DAT_00665d50 + 0x205u));
        uVar5 = 0xffffffff;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
        uVar5 = 0xffffffff;
        pcVar2 = DAT_00523d40;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
        pcVar2 = FUN_004ddc10(0x205);
        uVar5 = 0xffffffff;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
      }
    }
    else {
      pcVar2 = FUN_004ddc10((int)((unsigned int)DAT_00665d4c + 0x205u));
      uVar5 = 0xffffffff;
      do {
        pcVar3 = pcVar2;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar3 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar3;
      } while (cVar1 != '\0');
      uVar5 = ~uVar5;
      iVar7 = -1;
      pcVar2 = DAT_00b69d20;
      do {
        pcVar8 = pcVar2;
        if (iVar7 == 0) break;
        iVar7 = (int)((unsigned int)iVar7 - 1u);
        pcVar8 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar8;
      } while (cVar1 != '\0');
      pcVar2 = pcVar3 + -uVar5;
      pcVar3 = pcVar8 + -1;
      for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uTmp, pcVar2, 4);
        __builtin_memcpy(pcVar3, &uTmp, 4);
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 4;
      }
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar3 = *pcVar2;
        pcVar2 = pcVar2 + 1;
        pcVar3 = pcVar3 + 1;
      }
      uVar5 = 0xffffffff;
      pcVar2 = DAT_00523d40;
      do {
        pcVar3 = pcVar2;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar3 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar3;
      } while (cVar1 != '\0');
      uVar5 = ~uVar5;
      iVar7 = -1;
      pcVar2 = DAT_00b69d20;
      do {
        pcVar8 = pcVar2;
        if (iVar7 == 0) break;
        iVar7 = (int)((unsigned int)iVar7 - 1u);
        pcVar8 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar8;
      } while (cVar1 != '\0');
      pcVar2 = pcVar3 + -uVar5;
      pcVar3 = pcVar8 + -1;
      for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uTmp, pcVar2, 4);
        __builtin_memcpy(pcVar3, &uTmp, 4);
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 4;
      }
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar3 = *pcVar2;
        pcVar2 = pcVar2 + 1;
        pcVar3 = pcVar3 + 1;
      }
      pcVar2 = FUN_004ddc10(0x204);
      uVar5 = 0xffffffff;
      do {
        pcVar3 = pcVar2;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar3 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar3;
      } while (cVar1 != '\0');
      uVar5 = ~uVar5;
      iVar7 = -1;
      pcVar2 = DAT_00b69d20;
      do {
        pcVar8 = pcVar2;
        if (iVar7 == 0) break;
        iVar7 = (int)((unsigned int)iVar7 - 1u);
        pcVar8 = pcVar2 + 1;
        cVar1 = *pcVar2;
        pcVar2 = pcVar8;
      } while (cVar1 != '\0');
      pcVar2 = pcVar3 + -uVar5;
      pcVar3 = pcVar8 + -1;
      for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&uTmp, pcVar2, 4);
        __builtin_memcpy(pcVar3, &uTmp, 4);
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 4;
      }
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar3 = *pcVar2;
        pcVar2 = pcVar2 + 1;
        pcVar3 = pcVar3 + 1;
      }
      if (DAT_00665d50 != 0) {
        uVar5 = 0xffffffff;
        pcVar2 = DAT_00523d40;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
        pcVar2 = FUN_004ddc10(0x203);
        uVar5 = 0xffffffff;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
        uVar5 = 0xffffffff;
        pcVar2 = DAT_00523d40;
        do {
          pcVar3 = pcVar2;
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          pcVar3 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar3;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5;
        iVar7 = -1;
        pcVar2 = DAT_00b69d20;
        do {
          pcVar8 = pcVar2;
          if (iVar7 == 0) break;
          iVar7 = (int)((unsigned int)iVar7 - 1u);
          pcVar8 = pcVar2 + 1;
          cVar1 = *pcVar2;
          pcVar2 = pcVar8;
        } while (cVar1 != '\0');
        pcVar2 = pcVar3 + -uVar5;
        pcVar3 = pcVar8 + -1;
        for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          __builtin_memcpy(&uTmp, pcVar2, 4);
          __builtin_memcpy(pcVar3, &uTmp, 4);
          pcVar2 = pcVar2 + 4;
          pcVar3 = pcVar3 + 4;
        }
        for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar3 = *pcVar2;
          pcVar2 = pcVar2 + 1;
          pcVar3 = pcVar3 + 1;
        }
        goto LAB_004ea58f;
      }
    }
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
    FUN_004d64a0(local_28, 0, 0xf);
    uVar12 = 1;
    uVar13 = DAT_00665d54;
    pcVar2 = FUN_004ddc10(0x1eb);
    _sprintf(DAT_00b69d20, s__c_s__c_d_0052c518, 4, pcVar2, uVar12, uVar13);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  }
  else {
    if ((DAT_00665d3c == 8) || (DAT_00665d3c == 9)) goto LAB_004ea7e8;
    lVar9 = FUN_0042b270();
    uStack_18 = (double)(int)lVar9;
    if ((double)(int)lVar9 < _DAT_00518308) {
      uVar5 = 0x2c8;
    }
    else {
      uStack_18 = uStack_18 * _DAT_00518310;
      uVar5 = 0x20a;
    }
    pcVar2 = FUN_004ddc10((int)uVar5);
    uVar11 = 1;
    pcVar3 = FUN_004ddc10(0x1ec);
    _sprintf(DAT_00b69d20, s__c_s__c__1f__s_0052c508, 4, pcVar3, uVar11, uStack_18, pcVar2);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
    FUN_004d64a0(local_28, 0, 0xf);
    iVar7 = (int)((unsigned int)DAT_00aa6110 + (unsigned int)iVar7 + 0x1c0u);
    uVar13 = 1;
    pcVar2 = FUN_004ddc10(0x1ed);
    _sprintf(DAT_00b69d20, s__c_s__c_s_0052a6b0, 4, pcVar2, uVar13, iVar7);
    FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  }
  FUN_004d64a0(local_28, 0, 0xf);
LAB_004ea7e8:
  pcVar2 = FUN_004ddc10(0x2cb);
  uVar12 = 1;
  uVar13 = DAT_00665d58;
  pcVar3 = FUN_004ddc10(0x1ee);
  _sprintf(DAT_00b69d20, s__c_s__c_d__s_0052c52c, 4, pcVar3, uVar12, uVar13, pcVar2);
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0xf);
  pcVar2 = FUN_004ddc10(0x2c9);
  uVar12 = 1;
  uVar13 = DAT_00665d5c;
  pcVar3 = FUN_004ddc10(0x1ef);
  _sprintf(DAT_00b69d20, s__c_s__c_d__s_0052c52c, 4, pcVar3, uVar12, uVar13, pcVar2);
  FUN_004db7d0(0xc, DAT_00b69d20, local_28, 0, 1, 0xffff);
  FUN_004d64a0(local_28, 0, 0xf);
  return 1;
}
