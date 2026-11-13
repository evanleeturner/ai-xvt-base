/* SPDX-License-Identifier: MIT */
/* z_bop 004d2370 */
extern char DAT_00aa7b0d;
extern int DAT_00acebc5;
extern int DAT_00acebfd;
extern int DAT_00aa6cfc;

void __stdcall FUN_004d4970(void);
void __stdcall FUN_004cdc20(void);
int __stdcall FUN_004d48e0(void);

int __stdcall FUN_004d2370(void)
{
  char cVar1;
  
  cVar1 = DAT_00aa7b0d;
  if (DAT_00acebc5 == 0) {
    return 0;
  }
  FUN_004d4970();
  DAT_00acebfd = 1;
  FUN_004cdc20();
  if (cVar1 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
