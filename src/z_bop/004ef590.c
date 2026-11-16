/* SPDX-License-Identifier: MIT */
/* z_bop 004ef590 */
extern unsigned char DAT_00665da0;
extern unsigned char DAT_00665da3;
extern unsigned char DAT_00665dcb;
extern unsigned char DAT_0066617b;
extern unsigned char DAT_006661a0;

HDC __stdcall GetDC(HWND hWnd);
UINT __stdcall GetSystemPaletteEntries(HDC hdc, UINT iStartIndex, UINT nEntries, LPPALETTEENTRY lppe);
int __stdcall ReleaseDC(HWND hWnd, HDC hdc);

void __cdecl FUN_004ef590(HWND param_1)
{
  HDC hdc;
  unsigned char *puVar1;
  
  hdc = GetDC(param_1);
  GetSystemPaletteEntries(hdc, 0, 0x100, (LPPALETTEENTRY)&DAT_00665da0);
  puVar1 = &DAT_00665da3;
  do {
    *puVar1 = 0;
    puVar1 = puVar1 + 4;
  } while (puVar1 < &DAT_00665dcb);
  puVar1 = &DAT_00665dcb;
  do {
    *puVar1 = 4;
    puVar1 = puVar1 + 4;
  } while (puVar1 < &DAT_0066617b);
  puVar1 = &DAT_0066617b;
  do {
    *puVar1 = 0;
    puVar1 = puVar1 + 4;
  } while (puVar1 < (unsigned char *)((int)&DAT_006661a0 + 3));
  ReleaseDC(param_1, hdc);
  return;
}
