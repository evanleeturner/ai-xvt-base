/* SPDX-License-Identifier: MIT */
/* z_xvt 004cb830 */
extern char DAT_00759645;
extern uint DAT_007806fa;
extern uint DAT_00758830;

extern void __stdcall FUN_004d4d90(void);
extern bool __cdecl FUN_004cb8b0(int param_1, uint *param_2, uint param_3);
extern uint __cdecl FUN_004d4d00(void);

bool __cdecl FUN_004cb830(int param_1, uint *param_2, uint param_3)
{
  char cVar1;
  bool bVar2;
  uint local_4;
  
  cVar1 = DAT_00759645;
  if (DAT_007806fa != 0) {
    return 1;
  }
  FUN_004d4d90();
  bVar2 = FUN_004cb8b0(param_1, param_2, param_3);
  local_4 = 0x39;
  FUN_004cb8b0(param_1, &local_4, 4);
  if (cVar1 != 0) {
    DAT_00758830 = FUN_004d4d00();
  }
  return bVar2;
}
