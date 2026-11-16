/* SPDX-License-Identifier: MIT */
/* z_bop 004186e0 */
extern int DAT_009a8db0[];
extern undefined4 DAT_009ed670[];

void __cdecl FUN_004186e0(int param_1, undefined4 *param_2)
{
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 tmp4;
  undefined2 tmp2;

  if (DAT_009a8db0[param_1] != 0) {
    puVar3 = DAT_009ed670 + param_1 * 0x7e9;
    if (puVar3 <= param_2) {
      iVar2 = DAT_009a8db0[param_1] - 1;
      iVar1 = 0;
      DAT_009a8db0[param_1] = iVar2;
      if (iVar2 > 0) {
        do {
          if (param_2 <= puVar3) {
            __builtin_memcpy(&tmp4, (void *)((int)puVar3 + 0x12), 4);
            *puVar3 = tmp4;
            __builtin_memcpy(&tmp4, (void *)((int)puVar3 + 0x16), 4);
            puVar3[1] = tmp4;
            __builtin_memcpy(&tmp4, (void *)((int)puVar3 + 0x1a), 4);
            puVar3[2] = tmp4;
            __builtin_memcpy(&tmp4, (void *)((int)puVar3 + 0x1e), 4);
            puVar3[3] = tmp4;
            __builtin_memcpy(&tmp2, (void *)((int)puVar3 + 0x22), 2);
            __builtin_memcpy((void *)(puVar3 + 2), &tmp2, 2);
          }
          iVar1 = iVar1 + 1;
          puVar3 = (undefined4 *)((int)puVar3 + 0x12);
        } while (iVar1 < DAT_009a8db0[param_1]);
      }
    }
  }
  return;
}
