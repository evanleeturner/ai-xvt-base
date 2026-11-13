/* SPDX-License-Identifier: MIT */
/* z_bop 004d3400 */
DWORD __stdcall GetTickCount(void);

extern char DAT_00aa7ff7[4];
extern char DAT_00aa7ff3[4];

uint __stdcall FUN_004d3400(void)
{
  uint v1;
  uint v2;
  uint tick;

  __builtin_memcpy(&v1, DAT_00aa7ff7, 4);
  if (v1 == 1) {
    v1 = 2;
    __builtin_memcpy(DAT_00aa7ff7, &v1, 4);
    tick = GetTickCount();
    v2 = tick + 1000;
    __builtin_memcpy(DAT_00aa7ff3, &v2, 4);
  }
  return 1;
}
