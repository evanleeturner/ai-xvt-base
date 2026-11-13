/* SPDX-License-Identifier: MIT */
/* z_bop 004abe50 */
extern int DAT_00527ea0;
extern int DAT_00527f78;
extern int DAT_00527eac;
extern short _DAT_009ed23b;
extern int DAT_00527ed0;
extern void **DAT_0066ddcc;
extern void **DAT_00803f80;
extern void **DAT_0066e700;
extern int DAT_00527ecc;

void __stdcall FUN_004abe50(void)
{
  if (DAT_00527ea0 != 1) {
    if (DAT_00527f78 > 1) {
      DAT_00527f78 = DAT_00527f78 - 1;
      return;
    }
    if (DAT_00527f78 < 1) {
      DAT_00527f78 = 0;
      return;
    }
    DAT_00527f78 = DAT_00527f78 - 1;
    if (DAT_00527eac != 0) {
      if (_DAT_009ed23b != 0) {
        if (DAT_00527ed0 != 0) {
          (*(void (**)(void **))((char *)*DAT_0066ddcc + 0x80))(DAT_0066ddcc);
          return;
        }
        (*(void (**)(void **, int))((char *)*DAT_00803f80 + 0x80))(DAT_00803f80, DAT_00527ecc);
        return;
      }
      (*(void (**)(void **, int))((char *)*DAT_0066e700 + 0x80))(DAT_0066e700, DAT_00527ecc);
    }
  }
  return;
}
