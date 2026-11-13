/* SPDX-License-Identifier: MIT */
/* z_bop 0041ff70 */
extern unsigned short DAT_009ed664;
extern int DAT_009a1fe8;
extern char DAT_009e9a5f;
extern int DAT_009ecc34;
extern char DAT_009e9a5b;
extern char DAT_009e9a63;

unsigned int __cdecl FUN_0046abf0(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_00427a60(short param_1, short param_2, short param_3, short param_4, int param_5);
void __cdecl FUN_00472400(int param_1);

void __cdecl FUN_0041ff70(unsigned short param_1)
{
  short sVar1;
  long long lVar2;
  long long lVar3;
  long long lVar4;
  long long lVar5;
  long long lVar6;
  long long lVar7;
  unsigned int uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  
  DAT_009ed664 = param_1;
  iVar11 = (unsigned int)param_1 * 0x23 + DAT_009a1fe8;
  iVar12 = *(int *)(&DAT_009e9a5f + DAT_009ecc34 * 0x5bd) - *(int *)(iVar11 + 8);
  iVar9 = *(int *)(&DAT_009e9a5b + DAT_009ecc34 * 0x5bd) - *(int *)(iVar11 + 4);
  iVar10 = *(int *)(&DAT_009e9a63 + DAT_009ecc34 * 0x5bd) - *(int *)(iVar11 + 0xc);
  lVar2 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0x99) * (long long)iVar9;
  lVar3 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0x9b) * (long long)iVar12;
  lVar4 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0x9d) * (long long)iVar10;
  lVar5 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0x9f) * (long long)iVar9;
  lVar6 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0xa1) * (long long)iVar12;
  lVar7 = (long long)(int)*(short *)(*(int *)(iVar11 + 0x1f) + 0xa3) * (long long)iVar10;
  sVar1 = *(short *)(iVar11 + 0x14);
  uVar8 = FUN_0046abf0(((unsigned int)lVar7 >> 0xf | (int)((unsigned long long)lVar7 >> 0x20) << 0x11) +
                       ((unsigned int)lVar5 >> 0xf | (int)((unsigned long long)lVar5 >> 0x20) << 0x11) +
                       ((unsigned int)lVar6 >> 0xf | (int)((unsigned long long)lVar6 >> 0x20) << 0x11),
                       ((unsigned int)lVar4 >> 0xf | (int)((unsigned long long)lVar4 >> 0x20) << 0x11) +
                       ((unsigned int)lVar2 >> 0xf | (int)((unsigned long long)lVar2 >> 0x20) << 0x11) +
                       ((unsigned int)lVar3 >> 0xf | (int)((unsigned long long)lVar3 >> 0x20) << 0x11));
  *(short *)(iVar11 + 0x14) = (short)uVar8 + sVar1;
  *(short *)(iVar11 + 0x14) = *(short *)(iVar11 + 0x14) + -0x4000;
  *(unsigned char *)(*(int *)(iVar11 + 0x1f) + 0x92) = 1;
  FUN_00427a60(*(short *)(iVar11 + 0x14),*(short *)(iVar11 + 0x12),*(short *)(iVar11 + 0x10),0,
               iVar11);
  FUN_00472400(iVar11);
  *(short *)(iVar11 + 0x14) = sVar1;
  *(unsigned char *)(*(int *)(iVar11 + 0x1f) + 0x92) = 1;
  return;
}
