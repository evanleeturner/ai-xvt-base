/* SPDX-License-Identifier: MIT */
/* z_bop 004452d0 */
undefined4 * __cdecl FUN_004438c0(void);
void * __cdecl _malloc(uint);
void __cdecl FUN_0049eb60(int);
void __cdecl FUN_00443a50(int);
void __cdecl FUN_004441f0(int);
void __cdecl FUN_00443c30(undefined4 *);

void __cdecl FUN_004452d0(byte *param_1, int param_2, undefined4 param_3, int param_4)
{
  undefined4 *puVar1;
  void *pvVar2;
  byte *pbVar3;
  byte *pbVar4;
  byte *pbVar5;
  uint uVar6;
  uint uVar7;
  int field_108e;
  void *field_1086;

  puVar1 = FUN_004438c0();
  if (puVar1 != (undefined4 *)0x0) {
    *(int *)((char *)puVar1 + 0x1028) = 0;
    *(undefined4 *)((char *)puVar1 + 0x1030) = param_3;
    *(undefined4 *)((char *)puVar1 + 0x1034) = param_4;
    
    field_108e = (int)((uint)*(undefined4 *)((char *)puVar1 + 0x1030) * (uint)param_4);
    __builtin_memcpy((char *)puVar1 + 0x108e, &field_108e, 4);
    
    pvVar2 = _malloc((uint)field_108e * 6);
    field_1086 = pvVar2;
    __builtin_memcpy((char *)puVar1 + 0x1086, &field_1086, 4);
    
    if (pvVar2 == (void *)0x0) {
      FUN_0049eb60(0);
    }
    
    __builtin_memcpy(&field_1086, (char *)puVar1 + 0x1086, 4);
    pbVar5 = (byte *)field_1086;
    uVar6 = 0;
    if (*(undefined4 *)((char *)puVar1 + 0x1034) != 0) {
      do {
        uVar7 = 0;
        pbVar3 = param_1;
        pbVar4 = pbVar5;
        if (*(undefined4 *)((char *)puVar1 + 0x1030) != 0) {
          do {
            param_1 = pbVar3 + 1;
            pbVar5 = pbVar4 + 6;
            uVar7 = uVar7 + 1;
            *pbVar4 = (byte)((ushort)*(undefined2 *)(param_2 + (uint)*pbVar3 * 2) >> 8) & 0xf8;
            pbVar4[1] = (char)(*(ushort *)(param_2 + (uint)*pbVar3 * 2) >> 5) << 2;
            pbVar4[2] = (char)*(undefined2 *)(param_2 + (uint)*pbVar3 * 2) << 3;
            pbVar4[4] = 0;
            pbVar4[5] = 0;
            pbVar4[3] = 0;
            pbVar3 = param_1;
            pbVar4 = pbVar5;
          } while (uVar7 < (uint)*(undefined4 *)((char *)puVar1 + 0x1030));
        }
        uVar6 = uVar6 + 1;
      } while (uVar6 < (uint)*(undefined4 *)((char *)puVar1 + 0x1034));
    }
    FUN_00443a50((int)puVar1);
    FUN_004441f0((int)puVar1);
    FUN_00443c30(puVar1);
  }
  return;
}
