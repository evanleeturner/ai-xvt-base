/* SPDX-License-Identifier: MIT */
/* z_bop 004d0070 */
extern char DAT_00aa7b0d;
extern undefined4 DAT_00aa6cfc;

extern void __cdecl FUN_004d4970(void);
extern int * __cdecl FUN_004d0540(int *param_1, undefined4 *param_2);
extern void __cdecl FUN_004d00d0(int param_1, int param_2);
extern undefined4 __cdecl FUN_004d48e0(void);

int * __cdecl FUN_004d0070(int *param_1, undefined4 *param_2)
{
  char cVar1;
  int *piVar2;
  
  cVar1 = DAT_00aa7b0d;
  FUN_004d4970();
  while (1) {
    piVar2 = FUN_004d0540(param_1, param_2);
    if (piVar2 == 0 || *param_1 == 0) {
      break;
    }
    FUN_004d00d0(*piVar2, (int)piVar2);
  }
  if (cVar1 != 0) {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return piVar2;
}
