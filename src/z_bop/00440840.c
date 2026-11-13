/* SPDX-License-Identifier: MIT */
/* z_bop 00440840 */
extern ushort DAT_00a0a1e0;
extern ushort DAT_00a08ca4;
extern short DAT_00a08ca0;
extern short DAT_00a08ca2;
extern short DAT_009ed232;
extern char DAT_009a20ae;
extern void (*DAT_009d1300)(void);
extern int DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern ushort DAT_00a08ca6;

void __cdecl FUN_004a9520(short param_1, short param_2, short param_3, short param_4);
void __cdecl FUN_004a9590(uint param_1);
void __cdecl FUN_004a9500(undefined2 param_1, undefined2 param_2);
void __cdecl FUN_004277f0(ushort param_1, uint param_2, uint param_3);

void __cdecl FUN_00440840(ushort param_1, ushort param_2, uint param_3)
{
  int iVar1;
  ushort uVar2;
  uint uVar3;
  
  uVar3 = (uint)param_1;
  if (*(ushort *)(&DAT_00a0a1e0 + uVar3 * 2) != param_2) {
    *(ushort *)(&DAT_00a0a1e0 + uVar3 * 2) = param_2;
    uVar2 = *(ushort *)(&DAT_00a08ca4 + uVar3 * 0xc);
    iVar1 = uVar3 * 0xc;
    FUN_004a9520(*(short *)(&DAT_00a08ca0 + iVar1),
                 *(short *)(&DAT_00a08ca2 + uVar3 * 0xc),
                 DAT_009ed232 * uVar2 + *(short *)(&DAT_00a08ca0 + iVar1) + 2,
                 (ushort)DAT_009a20ae + *(short *)(&DAT_00a08ca2 + uVar3 * 0xc));
    (*DAT_009d1300)();
    if (0x90 < param_1) {
      param_1 = param_1 + ((ushort)(param_1 - 1) / 0x90) * -0x90;
    }
    if ((param_2 < 0x15) &&
       ((((param_1 == 0x55 || (param_1 == 0x52)) || (param_1 == 0x66)) ||
        ((param_1 == 0x56 || (param_1 == 0x67)))))) {
      uVar3 = 0x4a;
    }
    else if ((param_2 < 0x33) &&
            (((param_1 == 0x55 || (param_1 == 0x52)) ||
             ((param_1 == 0x56 || ((param_1 == 0x66 || (param_1 == 0x67)))))))) {
      uVar3 = 0x4e;
    }
    else if (((param_1 == 0x29) || (param_1 == 0x28)) &&
            (*(short *)(*(int *)(*(int *)(*(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd) * 0x23
                                          + 0x1f + DAT_009a1fe8) + 0xa9) + 0xbf) == 0)) {
      uVar3 = 0x52;
    }
    else {
      uVar3 = (uint)*(ushort *)(&DAT_00a08ca6 + iVar1);
    }
    FUN_004a9590(uVar3);
    FUN_004a9500(*(undefined2 *)(&DAT_00a08ca0 + iVar1),
                 *(undefined2 *)(&DAT_00a08ca2 + iVar1));
    FUN_004277f0(param_2, (uint)uVar2, param_3 & 0xffff);
  }
  return;
}
