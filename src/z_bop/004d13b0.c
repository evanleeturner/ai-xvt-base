/* SPDX-License-Identifier: MIT */
/* z_bop 004d13b0 */
extern char DAT_00aa7b0d;
extern int DAT_00aa6cfc;
extern int DAT_00acebc5;

void __stdcall FUN_004d4970(void);
void __cdecl FUN_004d1280(int param_1);
int __cdecl FUN_004d48e0(void);

void __cdecl FUN_004d13b0(int param_1)
{
  char cVar1;
  
  cVar1 = DAT_00aa7b0d;
  if (DAT_00acebc5 != 0) {
    FUN_004d4970();
    FUN_004d1280(param_1);
    if (cVar1 != '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
  }
  return;
}
