/* SPDX-License-Identifier: MIT */
/* z_bop 004d56b0 */
extern int DAT_00aa7b36;
extern int DAT_00aa7b9a;
extern int DAT_00aa7b32;
extern int DAT_00b69cb0;
extern int DAT_00b69cbc;
extern int DAT_00aa7b3a;

extern void __stdcall FUN_004d4970(void);
extern void __stdcall FUN_004de4d0(void);
extern void __stdcall FUN_004de2e0(void);
extern void __stdcall FUN_004d31a0(void);

int __stdcall FUN_004d56b0(void)
{
  FUN_004d4970();
  FUN_004de4d0();
  FUN_004de2e0();
  FUN_004d31a0();
  if (DAT_00aa7b36 != 0) {
    ((void (__stdcall *)(int))(*(void **)DAT_00aa7b36 + 8))(DAT_00aa7b36);
    DAT_00aa7b36 = 0;
  }
  if (DAT_00aa7b9a != 0) {
    ((void (__stdcall *)(int))(*(void **)DAT_00aa7b9a + 8))(DAT_00aa7b9a);
    DAT_00aa7b9a = 0;
  }
  if (DAT_00aa7b32 != 0) {
    ((void (__stdcall *)(int))(*(void **)DAT_00aa7b32 + 8))(DAT_00aa7b32);
    DAT_00aa7b32 = 0;
  }
  if (((DAT_00b69cb0 != 0) || (DAT_00b69cbc != 0)) && (DAT_00aa7b3a != 0)) {
    ((void (__stdcall *)(int))(*(void **)DAT_00aa7b3a + 8))(DAT_00aa7b3a);
    DAT_00aa7b3a = 0;
  }
  return 1;
}
