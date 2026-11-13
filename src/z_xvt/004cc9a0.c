/* SPDX-License-Identifier: MIT */
/* z_xvt 004cc9a0 */
extern int * __cdecl FUN_004cce90(int *param_1, undefined4 *param_2);
extern void __cdecl FUN_004cca00(int param_1, int param_2);
extern int __cdecl FUN_004d4d00(void);
extern void __cdecl FUN_004d4d90(void);
extern char DAT_00759645;
extern int DAT_00758830;

int * __cdecl FUN_004cc9a0(int *param_1, undefined4 *param_2)
{
  char cVar1;
  int *piVar2;
  
  cVar1 = DAT_00759645;
  FUN_004d4d90();
  while (1) {
    piVar2 = FUN_004cce90(param_1, param_2);
    if (piVar2 == 0 || *param_1 == 0) {
      break;
    }
    FUN_004cca00(*piVar2, (int)piVar2);
  }
  if (cVar1 != 0) {
    DAT_00758830 = FUN_004d4d00();
  }
  return piVar2;
}
