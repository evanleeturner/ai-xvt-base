/* SPDX-License-Identifier: MIT */
/* z_bop 00471f00 */
void __cdecl FUN_00471f60(int *param_1, int param_2);

void __cdecl FUN_00471f00(int *param_1)
{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int local_10;
  int local_6;
  int local_ptr;

  iVar2 = 0;
  iVar4 = (int)param_1 - *param_1;
  *param_1 = *param_1 + iVar4;
  
  __builtin_memcpy(&local_10, (char *)param_1 + 10, 4);
  if (local_10 != 0) {
    iVar3 = 0;
    local_10 = local_10 + iVar4;
    __builtin_memcpy((char *)param_1 + 10, &local_10, 4);
    
    __builtin_memcpy(&local_6, (char *)param_1 + 6, 4);
    if (0 < local_6) {
      do {
        __builtin_memcpy(&local_10, (char *)param_1 + 10, 4);
        iVar1 = local_10;
        
        __builtin_memcpy(&local_ptr, (char *)iVar1 + iVar2, 4);
        if (local_ptr != 0) {
          local_ptr = local_ptr + iVar4;
          __builtin_memcpy((char *)iVar1 + iVar2, &local_ptr, 4);
          __builtin_memcpy(&local_10, (char *)param_1 + 10, 4);
          FUN_00471f60((int *)local_ptr, iVar4);
        }
        iVar2 = iVar2 + 4;
        iVar3 = iVar3 + 1;
        __builtin_memcpy(&local_6, (char *)param_1 + 6, 4);
      } while (iVar3 < local_6);
    }
  }
  return;
}
