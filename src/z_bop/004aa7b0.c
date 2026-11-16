/* SPDX-License-Identifier: MIT */
/* z_bop 004aa7b0 */
void __cdecl FUN_00449100(void);

LRESULT __cdecl FUN_004aa7b0(HWND param_1, int param_2, WPARAM param_3, LPARAM param_4)
{
  if (param_2 != 0x311) {
    FUN_00449100();
  }
  if (param_2 == 0xf) {
    return 0;
  }
  return DefWindowProcA(param_1, 0xf, param_3, param_4);
}
