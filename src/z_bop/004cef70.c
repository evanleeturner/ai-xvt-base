/* SPDX-License-Identifier: MIT */
/* z_bop 004cef70 */
extern char DAT_00aa7b0d;
extern unsigned int DAT_00acebc5;
extern unsigned int DAT_00aa6cfc;

void __stdcall FUN_004d4970(void);
int __cdecl FUN_004cefe0(int param_1, unsigned int *param_2, unsigned int param_3);
unsigned int __cdecl FUN_004d48e0(void);

int __cdecl FUN_004cef70(int param_1, unsigned int *param_2, unsigned int param_3)
{
  char cVar1;
  int bVar2;
  unsigned int local_4;
  
  cVar1 = DAT_00aa7b0d;
  if (DAT_00acebc5 == 0) {
    return 1;
  }
  FUN_004d4970();
  bVar2 = FUN_004cefe0(param_1, param_2, param_3);
  local_4 = 0x39;
  FUN_004cefe0(param_1, &local_4, 4);
  if (cVar1 != 0) {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return bVar2;
}
