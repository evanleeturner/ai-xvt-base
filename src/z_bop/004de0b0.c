/* SPDX-License-Identifier: MIT */
/* z_bop 004de0b0 */
extern int DAT_00aa7209;
extern int DAT_00aa720d;
extern int __stdcall SetCursorPos(int, int);

void __cdecl FUN_004de0b0(int param_1, int param_2)
{
  if (param_1 < 0x281) {
    if (param_1 < 0) {
      param_1 = 0;
    }
  }
  else {
    param_1 = 0x280;
  }
  if (param_2 < 0x1e1) {
    if (param_2 < 0) {
      param_2 = 0;
    }
  }
  else {
    param_2 = 0x1e0;
  }
  DAT_00aa7209 = param_1;
  DAT_00aa720d = param_2;
  SetCursorPos(param_1, param_2);
  return;
}
