/* SPDX-License-Identifier: MIT */
/* z_bop 004dc3b0 */
extern unsigned int DAT_00acebbd;
extern unsigned int DAT_00aa73f6;
extern unsigned int DAT_00aa7416;
extern unsigned int DAT_00aac111;
extern unsigned char DAT_00aa73a0;
extern unsigned char DAT_00aa73a1;
extern unsigned int DAT_00aa6cfc;

extern void __stdcall FUN_004d4970(void);
extern void FUN_004dc450(unsigned int param_1, int *param_2);
extern int FUN_004d5420(void);
extern void FUN_004dcb30(void);
extern void FUN_004dc7e0(void);
extern unsigned int __stdcall FUN_004d48e0(void);
extern void _exit(int code);

unsigned int __cdecl FUN_004dc3b0(unsigned int param_1, int *param_2)
{
  int iVar1;
  unsigned int *puVar2;
  
  FUN_004d4970();
  FUN_004dc450(param_1, param_2);
  DAT_00acebbd = 0;
  puVar2 = &DAT_00aa73f6;
  for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  puVar2 = &DAT_00aa7416;
  for (iVar1 = 8; iVar1 != 0; iVar1 = iVar1 - 1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  if (DAT_00aac111 != 0) {
    DAT_00aac111 = DAT_00aac111 - 1;
  }
  DAT_00aa73a0 = 0;
  DAT_00aa73a1 = 0;
  do {
    iVar1 = FUN_004d5420();
    if (iVar1 == 1) {
      FUN_004dcb30();
      FUN_004dc7e0();
      DAT_00aa6cfc = FUN_004d48e0();
      DAT_00aa73a0 = 0;
      DAT_00aa73a1 = 0;
      return 1;
    }
  } while (iVar1 != 2);
  _exit(0);
  return 0;
}
