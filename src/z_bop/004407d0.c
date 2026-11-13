/* SPDX-License-Identifier: MIT */
/* z_bop 004407d0 */
extern char DAT_00a0a1e0;
extern char DAT_009ed240;
extern char DAT_00a08ca4;
extern char DAT_00a08ca0;
extern char DAT_00a08ca2;
extern char DAT_00a08ca6;
void (*DAT_009d130c)(undefined4, undefined2, undefined2, undefined2, short, undefined4);

void __cdecl FUN_004407d0(uint param_1, short param_2, undefined4 param_3)
{
  uint uVar2;
  int iVar1;
  
  uVar2 = param_1 & 0xffff;
  if (*(short *)(&DAT_00a0a1e0 + uVar2 * 2) != param_2) {
    *(short *)(&DAT_00a0a1e0 + uVar2 * 2) = param_2;
    iVar1 = uVar2 * 0xc;
    (*DAT_009d130c)(*(undefined4 *)(&DAT_009ed240 + (uint)*(ushort *)(&DAT_00a08ca4 + iVar1) * 4),
                    *(undefined2 *)(&DAT_00a08ca0 + iVar1),*(undefined2 *)(&DAT_00a08ca2 + iVar1),
                    *(undefined2 *)(&DAT_00a08ca6 + iVar1),param_2,param_3);
  }
  return;
}
