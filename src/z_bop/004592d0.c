/* SPDX-License-Identifier: MIT */
/* z_bop 004592d0 */
extern unsigned short __cdecl FUN_00459750(unsigned int param_1);
extern void __cdecl FUN_00459f30(unsigned int param_1, unsigned int param_2);
extern unsigned int __stdcall FUN_00425cb0(void);
extern unsigned char *DAT_009a1fe8;

unsigned short __cdecl FUN_004592d0(unsigned int param_1)
{
  short *psVar1;
  unsigned short *puVar2;
  int iVar3;
  int iVar4;
  unsigned char bVar5;
  unsigned short uVar6;
  unsigned short uVar7;
  unsigned int uVar8;
  short sVar9;
  short sVar10;

  uVar6 = FUN_00459750(0xd);
  if (uVar6 == 0xffff) {
    return 0xffff;
  }
  FUN_00459f30((unsigned int)uVar6, param_1 & 0xffff);
  iVar4 = (unsigned int)uVar6 * 0x23;
  **(unsigned char **)(DAT_009a1fe8 + 0x1f + iVar4) = 5;
  *(unsigned char *)(DAT_009a1fe8 + 2 + iVar4) = 0xd;
  *(unsigned char *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 1) = 0;
  bVar5 = FUN_00425cb0();
  *(unsigned char *)(DAT_009a1fe8 + 3 + iVar4) = (bVar5 & 1) + 0x85;
  *(unsigned char *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0x87) =
       *(unsigned char *)((param_1 & 0xffff) * 0x23 + 3 + DAT_009a1fe8);
  *(unsigned int *)(DAT_009a1fe8 + 0x1b + iVar4) = 0xffffffff;
  uVar7 = FUN_00425cb0();
  sVar10 = (uVar7 & 0x7ff) + 0x100;
  uVar7 = FUN_00425cb0();
  sVar9 = (uVar7 & 0x7ff) + 0x100;
  uVar8 = FUN_00425cb0();
  if ((uVar8 & 1) != 0) {
    sVar10 = -sVar10;
  }
  uVar8 = FUN_00425cb0();
  if ((uVar8 & 1) != 0) {
    sVar9 = -sVar9;
  }
  psVar1 = (short *)(DAT_009a1fe8 + 0x10 + iVar4);
  *psVar1 = *psVar1 + sVar10;
  psVar1 = (short *)(DAT_009a1fe8 + 0x12 + iVar4);
  *psVar1 = *psVar1 + sVar9;
  puVar2 = (unsigned short *)(DAT_009a1fe8 + 0x12 + iVar4);
  uVar7 = *puVar2;
  if (0x7fff < uVar7) {
    *puVar2 = -uVar7;
    psVar1 = (short *)(DAT_009a1fe8 + 0x10 + iVar4);
    *psVar1 = *psVar1 + -0x8000;
  }
  *(unsigned char *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0x92) = 1;
  iVar3 = *(int *)(DAT_009a1fe8 + 0x1f + iVar4);
  *(unsigned char *)(iVar3 + 0x8b) = *(unsigned char *)(iVar3 + 0x92);
  iVar3 = *(int *)(DAT_009a1fe8 + 0x1f + iVar4);
  uVar7 = FUN_00425cb0();
  psVar1 = (short *)(iVar3 + 0x79);
  *psVar1 = *psVar1 + (uVar7 & 0xff) + 0x32;
  *(unsigned short *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0x83) = 0;
  uVar7 = FUN_00425cb0();
  *(unsigned short *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar4) + 0x81) = ((uVar7 & 3) + 1) * 0xec;
  *(unsigned char *)(DAT_009a1fe8 + 0x19 + iVar4) = 0;
  return uVar6;
}
