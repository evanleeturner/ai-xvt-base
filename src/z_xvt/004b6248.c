/* SPDX-License-Identifier: MIT */
/* z_xvt 004b6248 */
extern void *DAT_00646e18;

int __cdecl FUN_004b6248(int param_1)
{
  int uVar1;
  int iVar2;
  int local_40c;
  unsigned char local_408[1028];
  
  if (DAT_00646e18 == NULL) {
    uVar1 = 0;
  }
  else {
    local_408[3] = 0;
    local_408[0] = 0;
    local_408[1] = 0;
    local_408[2] = 0;
    for (local_40c = 1; local_40c < 0x100; local_40c = local_40c + 1) {
      local_408[local_40c * 4 + 3] = 0;
      local_408[local_40c * 4] = *(unsigned char *)(param_1 + local_40c * 4);
      local_408[local_40c * 4 + 1] = (unsigned char)((unsigned int)*(unsigned int *)(param_1 + local_40c * 4) >> 8);
      local_408[local_40c * 4 + 2] = (unsigned char)((unsigned int)*(unsigned int *)(param_1 + local_40c * 4) >> 0x10);
    }
    iVar2 = (*(int (__stdcall **)(void *, int, int, int, void *))(*(void **)DAT_00646e18 + 0x18))(DAT_00646e18, 0, 0, 0x100, local_408);
    if (iVar2 == 0) {
      uVar1 = 1;
    }
    else {
      uVar1 = 0;
    }
  }
  return uVar1;
}
