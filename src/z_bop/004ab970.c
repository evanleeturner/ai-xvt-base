/* SPDX-License-Identifier: MIT */
/* z_bop 004ab970 */
extern char DAT_0066e200[];
extern void **DAT_0066e700;
extern void **DAT_0066ddd8;
extern int DAT_00527eac;
extern void **DAT_0066ddcc;
extern const char s____CleanupAndExit_err____d_005280bc[];
extern const char s_Game_could_not_start_005280a4[];
extern const char s_ERROR_00521bb8[];

extern int FUN_0046c6a0(void);

int __cdecl FUN_004ab970(int param_1)
{
  wsprintfA(DAT_0066e200, s____CleanupAndExit_err____d_005280bc, param_1);
  OutputDebugStringA(DAT_0066e200);
  if (DAT_0066e700 != 0) {
    (*(void (**)(void *))((unsigned char *)*DAT_0066e700 + 8))(DAT_0066e700);
    DAT_0066e700 = 0;
  }
  if (DAT_0066ddd8 != 0) {
    (*(void (**)(void *))((unsigned char *)*DAT_0066ddd8 + 8))(DAT_0066ddd8);
    DAT_0066ddd8 = 0;
  }
  if (DAT_00527eac != 0 && DAT_0066ddcc != 0) {
    (*(void (**)(void *))((unsigned char *)*DAT_0066ddcc + 8))(DAT_0066ddcc);
    DAT_0066ddcc = 0;
  }
  FUN_0046c6a0();
  MessageBoxA(0, s_Game_could_not_start_005280a4, s_ERROR_00521bb8, 0);
  return 0;
}
