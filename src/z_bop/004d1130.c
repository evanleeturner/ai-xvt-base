/* SPDX-License-Identifier: MIT */
/* z_bop 004d1130 */
DWORD __stdcall GetTickCount(void);
int * __cdecl FUN_004d0070(int *param_1, uint *param_2);

int * __cdecl FUN_004d1130(int *param_1, uint *param_2, int param_3)
{
  DWORD DVar1;
  DWORD DVar2;
  int *piVar3;
  int local_8;
  uint local_4;
  
  DVar1 = GetTickCount();
  do {
    DVar2 = GetTickCount();
    if ((uint)(param_3 * 1000) < DVar2 - DVar1) {
      return (int *)0x0;
    }
    piVar3 = FUN_004d0070(&local_8, &local_4);
  } while (piVar3 == (int *)0x0);
  *param_1 = local_8;
  *param_2 = local_4;
  return piVar3;
}
