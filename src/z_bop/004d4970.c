/* SPDX-License-Identifier: MIT */
/* z_bop 004d4970 */
extern int DAT_00aa7b2e;
extern void *DAT_00aa7b3a;
extern char DAT_00aa7b0d;

void __stdcall FUN_004d4970(void)
{
  if ((DAT_00aa7b2e != 0) && (DAT_00aa7b3a != 0)) {
    (*(void (__stdcall **)(void *, int))((char *)*(void **)DAT_00aa7b3a + 0x80))(DAT_00aa7b3a, 0);
    DAT_00aa7b0d = 0;
  }
  return;
}
