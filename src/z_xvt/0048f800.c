/* SPDX-License-Identifier: MIT */
/* z_xvt 0048f800 */
void __cdecl FUN_004a5510(int *, int *, int *, int *, undefined4 *, undefined4 *, undefined4 *);

void __stdcall FUN_0048f800(undefined2 *in_stack_00000004, undefined2 *in_stack_00000008)
{
  undefined2 local_34[2];
  undefined2 local_30[2];
  int local_2c;
  int local_28;
  undefined4 local_24[3];
  undefined4 local_18[3];
  undefined4 local_c[3];

  FUN_004a5510(&local_2c, &local_28, (int *)local_34, (int *)local_30, local_24, local_18, local_c);
  *in_stack_00000004 = local_34[0];
  *in_stack_00000008 = local_30[0];
  return;
}
