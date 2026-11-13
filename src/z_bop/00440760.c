/* SPDX-License-Identifier: MIT */
/* z_bop 00440760 */
extern unsigned short DAT_00a0a1e0[];
extern unsigned int DAT_009ed240[];
extern unsigned short DAT_00a08ca0[];
extern unsigned short DAT_00a08ca2[];
extern unsigned short DAT_00a08ca4[];
extern unsigned short DAT_00a08ca6[];
extern void (__cdecl *DAT_009d80c4)(unsigned int, unsigned short, unsigned short, unsigned short, int);

void __cdecl FUN_00440760(int param_1, unsigned int param_2)
{
  int iVar1;
  unsigned int uVar2;
  unsigned short uVar3;
  unsigned short uVar4;
  unsigned short uVar5;
  unsigned short uVar6;

  if (DAT_00a0a1e0[param_1] != param_2) {
    DAT_00a0a1e0[param_1] = (unsigned short)param_2;
    iVar1 = param_1 * 0xc;
    uVar6 = *(unsigned short *)(DAT_00a08ca6 + iVar1);
    uVar5 = *(unsigned short *)(DAT_00a08ca2 + iVar1);
    uVar4 = *(unsigned short *)(DAT_00a08ca0 + iVar1);
    uVar3 = *(unsigned short *)(DAT_00a08ca4 + iVar1);
    uVar2 = DAT_009ed240[uVar3 + param_2];
    DAT_009d80c4(uVar2, uVar4, uVar5, uVar6, 0);
  }
  return;
}
