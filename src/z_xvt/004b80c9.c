/* SPDX-License-Identifier: MIT */
/* z_xvt 004b80c9 */
extern int DAT_0052f1e8;
extern int DAT_0052f1e0;
extern void __cdecl FUN_0045bbf0(int);
extern void __cdecl FUN_004b87e1(int *);

bool __cdecl FUN_004b80c9(uint param_1)
{
  uint local_c;
  int *local_8;
  
  local_c = 0;
  for (local_8 = (int *)DAT_0052f1e8; (local_8 != (int *)0x0 && (local_8[3] != DAT_0052f1e0));
      local_8 = (int *)local_8[1]) {
    if ((uint)local_8[4] == param_1) {
      ((void (__cdecl *)(void *))(*(int *)local_8[6] + 8))((void *)local_8[6]);
      ((void (__cdecl *)(void *))(*(int *)local_8[5] + 8))((void *)local_8[5]);
      local_8[2] = 0;
      FUN_0045bbf0(0);
      FUN_004b87e1(local_8);
      return 1;
    }
  }
  local_8 = (int *)DAT_0052f1e8;
  while ((local_8 != (int *)0x0 && (local_c < param_1))) {
    if (local_8[3] != DAT_0052f1e0) {
      ((void (__cdecl *)(void *))(*(int *)local_8[6] + 8))((void *)local_8[6]);
      ((void (__cdecl *)(void *))(*(int *)local_8[5] + 8))((void *)local_8[5]);
      local_8[2] = 0;
      FUN_0045bbf0(0);
      local_c = local_c + local_8[4];
      FUN_004b87e1(local_8);
    }
    local_8 = (int *)local_8[1];
  }
  return local_c != 0;
}
