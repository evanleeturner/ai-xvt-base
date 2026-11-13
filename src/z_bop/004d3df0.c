/* SPDX-License-Identifier: MIT */
/* z_bop 004d3df0 */
extern char DAT_00aa7b0d;
extern void *DAT_00aa7b2e;
extern void *DAT_00aa7b26;
extern int DAT_00aa6cfc;
extern const char s_X_Wing_vs__TIE_Fighter_0052ba90[];

void FUN_004d4970(void);
int FUN_004d48e0(void);
int MessageBoxA(void *hWnd, const char *lpText, const char *lpCaption, unsigned int uType);

int __cdecl FUN_004d3df0(const char *param_1)
{
  char cVar1;
  
  cVar1 = DAT_00aa7b0d;
  FUN_004d4970();
  if (DAT_00aa7b2e != (void *)0x0) {
    ((void (__stdcall *)(void *))(*(int *)DAT_00aa7b2e + 0x28))(DAT_00aa7b2e);
  }
  MessageBoxA(DAT_00aa7b26, param_1, s_X_Wing_vs__TIE_Fighter_0052ba90, 0x30);
  if (cVar1 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
