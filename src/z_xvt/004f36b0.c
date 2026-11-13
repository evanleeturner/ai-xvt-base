/* SPDX-License-Identifier: MIT */
/* z_xvt 004f36b0 */
extern undefined4 DAT_00650440;
extern void __cdecl FUN_004dd000(int *param_1, undefined4 *param_2);
extern undefined4 __cdecl FUN_004f32e0(int *param_1, short param_2, short param_3);

undefined4 __stdcall FUN_004f36b0(short in_stack_00000014, short in_stack_00000018)
{
  undefined4 uVar1;
  int local_10[4];
  
  FUN_004dd000(local_10, &DAT_00650440);
  uVar1 = FUN_004f32e0(local_10, in_stack_00000014, in_stack_00000018);
  return (uVar1 & 0xFFFF0000) | 1;
}
