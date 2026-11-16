/* SPDX-License-Identifier: MIT */
/* z_bop 004f82a0 */
extern short DAT_006696e4[];
extern short DAT_006696f4[];
extern short DAT_00669704[];
extern short DAT_00a92142[];
extern short DAT_00a9216e[];
extern short DAT_00669716[];
extern short DAT_00669726[];
extern short DAT_00669736[];
extern short DAT_00669746[];
extern short DAT_00669756[];
extern short DAT_00669766[];
extern int DAT_00669582[];
extern int _DAT_00669214;
extern short DAT_00aa5aac;
extern short DAT_00a921c6[];
extern unsigned char DAT_00a91d17[];
extern unsigned char DAT_00a91d12[];
extern unsigned char DAT_00a91d18[];
extern unsigned char DAT_00a91d24[];
extern unsigned int DAT_00b6a5a6;
extern int DAT_0052c908[];
extern char DAT_0052cab0[];
extern char DAT_0052cab4[];
extern char DAT_0052cab8[];
extern char DAT_0052cabc[];
extern char DAT_00b69d20[];
extern char s_mapicon_d_0052d008[];
extern char DAT_0052d004[];

void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004f7b10(int *param_1, short param_2, short param_3, short *param_4, short *param_5);
void __cdecl FUN_004f8b30(int *param_1, undefined4 param_2, short param_3, short param_4);
void __cdecl FUN_004f8910(char *param_1, int param_2, short param_3, short param_4, short param_5, short param_6);
int __cdecl _sprintf(char *param_1, const char *param_2, ...);
void __cdecl FUN_004b50b0(char *param_1, int *param_2, int param_3, int param_4);
unsigned int __cdecl FUN_004db4e0(int param_1, char *param_2, int param_3, int param_4, unsigned int param_5);

void __cdecl FUN_004f82a0(int *param_1, undefined4 param_2)
{
  char cVar2;
  short sVar3;
  short sVar10;
  short local_44;
  short local_42;
  short sStack_40;
  short local_3e;
  int local_3c;
  int local_38[4];
  char local_28[40];
  int iVar1;
  int iVar4;
  int iVar7;
  int iVar8;
  int iVar12;
  unsigned int uVar5;
  unsigned int uVar6;
  char *pcVar9;
  char *pcVar11;
  unsigned int local_cmp;
  unsigned int local_word;

  sVar10 = 0;
  FUN_004d6480(local_38, param_1);
  do {
    if (DAT_006696e4[sVar10] != 0) {
      sVar3 = DAT_006696f4[sVar10];
      FUN_004f7b10(local_38, DAT_00a92142[sVar3 * 0x2b1], DAT_00a9216e[sVar3 * 0x2b1], &sStack_40, &local_42);
      FUN_004f8b30(param_1, param_2, sVar3, DAT_00669704[sVar10]);
    }
    sVar10 = (short)(sVar10 + 1);
  } while (sVar10 < 8);

  local_3e = 0;
  do {
    iVar4 = (int)local_3e;
    if (DAT_00669716[iVar4] != 0) {
      sVar10 = DAT_00669726[iVar4];
      FUN_004f7b10(local_38, DAT_00669736[iVar4], DAT_00669746[iVar4], &sStack_40, &local_42);
      uVar5 = 0xffffffffU;
      pcVar9 = (char *)DAT_00669582[sVar10];
      do {
        pcVar11 = pcVar9;
        if (uVar5 == 0) break;
        uVar5 = uVar5 - 1;
        pcVar11 = pcVar9 + 1;
        cVar2 = *pcVar9;
        pcVar9 = pcVar11;
      } while (cVar2 != '\0');
      uVar5 = ~uVar5;
      pcVar9 = pcVar11 + -uVar5;
      pcVar11 = local_28;
      for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        __builtin_memcpy(&local_word, pcVar9, sizeof(local_word));
        __builtin_memcpy(pcVar11, &local_word, sizeof(local_word));
        pcVar9 = pcVar9 + 4;
        pcVar11 = pcVar11 + 4;
      }
      for (uVar5 = uVar5 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar11 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar11 = pcVar11 + 1;
      }
      sVar10 = 0;
      cVar2 = local_28[0];
      while (cVar2 != '\0') {
        pcVar9 = local_28 + sVar10;
        if (*pcVar9 == '[') {
          *pcVar9 = '\x02';
        }
        if (*pcVar9 == ']') {
          *pcVar9 = '\x01';
        }
        sVar10 = (short)(sVar10 + 1);
        cVar2 = local_28[sVar10];
      }
      FUN_004f8910(local_28, 1, sStack_40, local_42, DAT_00669756[iVar4], DAT_00669766[iVar4]);
    }
    local_3e = (short)(local_3e + 1);
  } while (local_3e <= 7);

  local_3c = 1;
  local_3e = 0;
  if (0 < DAT_00aa5aac) {
    do {
      iVar7 = (int)local_3e;
      iVar1 = (int)((unsigned int)iVar7 * 0x562U);
      iVar4 = (int)((unsigned int)iVar7 * 0x2b1U + (unsigned int)_DAT_00669214);
      if (DAT_00a921c6[iVar4] != 0) {
        switch (DAT_00a91d17[iVar1]) {
        case 0:
          local_44 = 0;
          break;
        case 1:
          local_44 = 1;
          break;
        case 2:
          local_44 = 2;
          break;
        case 3:
          local_44 = 3;
          break;
        case 4:
          local_44 = 1;
          break;
        case 5:
          local_44 = 4;
        }
        iVar8 = (int)((unsigned int)DAT_0052c908[(short)(unsigned short)DAT_00a91d12[iVar1]] * 0x10U);
        iVar12 = (int)((unsigned int)(*(int *)(DAT_0052cab8 + iVar8) - *(int *)(DAT_0052cab0 + iVar8)) + 1U);
        iVar7 = (int)((unsigned int)(*(int *)(DAT_0052cabc + iVar8) - *(int *)(DAT_0052cab4 + iVar8)) + 1U);
        FUN_004f7b10(local_38, DAT_00a92142[iVar4], DAT_00a9216e[iVar4], &sStack_40, &local_42);
        sStack_40 = (short)(sStack_40 - (short)(iVar12 >> 1));
        local_42 = (short)(local_42 - (short)(iVar7 >> 1));
        _sprintf(DAT_00b69d20, s_mapicon_d_0052d008, (int)local_44);
        FUN_004b50b0(DAT_00b69d20, (int *)(DAT_0052cab0 + iVar8), (int)sStack_40, (int)local_42);
        __builtin_memcpy(&local_cmp, &DAT_00b6a5a6, sizeof(local_cmp));
        if ((DAT_00a91d24[iVar1] != 0) && ((unsigned int)DAT_00a91d18[iVar1] == local_cmp)) {
          sStack_40 = (short)(sStack_40 + (short)iVar12);
          local_42 = (short)(local_42 + (short)iVar7);
          _sprintf(DAT_00b69d20, DAT_0052d004, local_3c);
          FUN_004db4e0(10, DAT_00b69d20, (int)sStack_40, (int)local_42, 0xffff);
          local_3c = local_3c + 1;
        }
      }
      local_3e = (short)(local_3e + 1);
    } while (local_3e < DAT_00aa5aac);
  }
  return;
}
