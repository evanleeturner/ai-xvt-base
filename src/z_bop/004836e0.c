/* SPDX-License-Identifier: MIT */
/* z_bop 004836e0 */
extern int DAT_009a1fe8;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;

extern void FUN_00458e20(unsigned int param_1, unsigned int param_2);
extern int FUN_004ae5a0(unsigned int param_1, int param_2);
extern int FUN_004ae6e0(unsigned int param_1, int param_2);
extern int FUN_004ae640(unsigned int param_1, int param_2);
extern void FUN_004037b0(void *param_1, int param_2, int param_3, int param_4);
extern void FUN_0046a8d0(unsigned int param_1, unsigned int param_2, unsigned int param_3);
extern void FUN_0041f410(unsigned int param_1, unsigned int param_2, unsigned int param_3);

void __cdecl FUN_004836e0(unsigned int param_1, unsigned int param_2, int param_3)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  unsigned int uVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  
  iVar8 = (param_1 & 0xffff) * 0x23 + DAT_009a1fe8;
  iVar1 = *(int *)(iVar8 + 4);
  iVar2 = *(int *)(iVar8 + 8);
  iVar8 = *(int *)(iVar8 + 0xc);
  FUN_00458e20(param_2 & 0xffff, 0);
  iVar6 = DAT_00a07cdc;
  iVar5 = DAT_00a07cd8;
  iVar4 = DAT_00a07cd4;
  iVar3 = (param_2 & 0xffff) * 0x23;
  uVar9 = (unsigned int)*(unsigned char *)(DAT_009a1fe8 + 3 + iVar3);
  DAT_009d8a58 = FUN_004ae5a0(uVar9, param_3);
  DAT_009d8a54 = FUN_004ae6e0(uVar9, param_3);
  iVar7 = FUN_004ae640(uVar9, param_3);
  DAT_009d8b60 = -iVar7;
  FUN_004037b0((void *)(DAT_009a1fe8 + iVar3), DAT_009d8a58, DAT_009d8a54, DAT_009d8b60);
  uVar10 = (iVar6 + DAT_009d8a58) - iVar1;
  uVar11 = (iVar5 + DAT_009d8a54) - iVar2;
  uVar9 = (iVar4 + DAT_009d8b60) - iVar8;
  FUN_0046a8d0(uVar10, uVar11, uVar9);
  FUN_0041f410(uVar10, uVar11, uVar9);
  return;
}
