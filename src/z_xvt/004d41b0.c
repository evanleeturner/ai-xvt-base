/* SPDX-License-Identifier: MIT */
/* z_xvt 004d41b0 */
extern const char *s_DirectDraw_Init_FAILED_at__d_005329c4;
extern const char *s_X_Wing_vs__TIE_Fighter_005329ac;

int __cdecl _sprintf(char *dst, const char *fmt, ...);
int __stdcall MessageBoxA(HWND hWnd, const char *lpText, const char *lpCaption, unsigned int uType);
void __cdecl FUN_004d3ca0(int param);

int __cdecl FUN_004d41b0(HWND param_1, int param_2)
{
    char local_100[256];
    
    _sprintf(local_100, s_DirectDraw_Init_FAILED_at__d_005329c4, param_2);
    MessageBoxA(param_1, local_100, s_X_Wing_vs__TIE_Fighter_005329ac, 0);
    FUN_004d3ca0(1);
    return 0;
}
