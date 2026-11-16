/* SPDX-License-Identifier: MIT */
/* z_bop 004d5b90 */
HWND __stdcall FindWindowA(const char *lpClassName, const char *lpWindowName);
BOOL __stdcall ShowWindowAsync(HWND hWnd, int nCmdShow);
extern char s_X_Wing_vs__TIE_Fighter_0052ba90[];

unsigned int __stdcall FUN_004d5b90(void)
{
  HWND hWnd;
  
  hWnd = FindWindowA(s_X_Wing_vs__TIE_Fighter_0052ba90, s_X_Wing_vs__TIE_Fighter_0052ba90);
  if (hWnd != (HWND)0x0) {
    ShowWindowAsync(hWnd, 9);
    return 1;
  }
  return 0;
}
