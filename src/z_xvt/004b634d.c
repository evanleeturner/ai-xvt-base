/* SPDX-License-Identifier: MIT */
/* z_xvt 004b634d */
extern int * __cdecl FUN_004b88a0(unsigned int, unsigned int, int);
extern void * __cdecl _memset(void *, int, unsigned int);

int * __cdecl FUN_004b634d(int param_1, unsigned int * param_2)
{
  int * piVar1;
  unsigned int local_70[0x1b];
  unsigned int local_68 = 0;
  int local_60 = 0;
  unsigned int local_4c = 0;

  piVar1 = FUN_004b88a0(param_2[1], param_2[2], param_1);
  _memset(local_70, 0, 0x6c);
  local_70[0] = 0x6c;
  (**(void (__cdecl **)(int, int, unsigned int *, int, int))(*(int *)piVar1[3] + 0x64))(piVar1[3], 0, local_70, 1, 0);
  param_2[1] = (unsigned int)(local_60 << 3) / *(unsigned int *)(param_1 + 4);
  param_2[2] = local_68;
  param_2[3] = *(unsigned int *)(param_1 + 4) >> 3;
  *param_2 = local_4c;
  return piVar1;
}
