/* SPDX-License-Identifier: MIT */
/* z_xvt 004d5b70 */
extern undefined4 __stdcall FUN_004d4d90(void);
extern undefined4 __stdcall FUN_004de0c0(void);
extern undefined4 __stdcall FUN_004ddec0(void);
extern undefined4 __stdcall FUN_004cfd60(void);

extern void *DAT_0075966b;
extern void *DAT_007596cf;
extern void *DAT_00759667;
extern void *DAT_0075966f;
extern undefined4 DAT_0081b6e4;
extern undefined4 DAT_0081b6f0;

undefined4 __stdcall FUN_004d5b70(void)
{
  FUN_004d4d90();
  FUN_004de0c0();
  FUN_004ddec0();
  FUN_004cfd60();
  if (DAT_0075966b != NULL) {
    ((void (*)(void *))(*(void **)DAT_0075966b + 2))(DAT_0075966b);
    DAT_0075966b = NULL;
  }
  if (DAT_007596cf != NULL) {
    ((void (*)(void *))(*(void **)DAT_007596cf + 2))(DAT_007596cf);
    DAT_007596cf = NULL;
  }
  if (DAT_00759667 != NULL) {
    ((void (*)(void *))(*(void **)DAT_00759667 + 2))(DAT_00759667);
    DAT_00759667 = NULL;
  }
  if (((DAT_0081b6e4 != 0) || (DAT_0081b6f0 != 0)) && (DAT_0075966f != NULL)) {
    ((void (*)(void *))(*(void **)DAT_0075966f + 2))(DAT_0075966f);
    DAT_0075966f = NULL;
  }
  return 1;
}
