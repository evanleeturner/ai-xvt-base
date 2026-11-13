/* SPDX-License-Identifier: MIT */
/* z_xvt 00492a00 */
extern void *DAT_00732e54;
void __cdecl FUN_004979e0(short param_1, int param_2);

unsigned short __stdcall caseD_8(void)
{
  FUN_004979e0(-0x8000, 1);
  unsigned short local_val = 0xffff;
  __builtin_memcpy((char *)DAT_00732e54 + 0xbd, &local_val, 2);
  return 0;
}
