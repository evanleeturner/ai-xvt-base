/* SPDX-License-Identifier: MIT */
/* z_bop 004453d0 */
undefined4 * __stdcall FUN_004438c0(void);
void * __cdecl _malloc(unsigned int);
void __cdecl FUN_00443a50(int);
void __cdecl FUN_004441f0(int);
void __cdecl FUN_00443c30(void *);
extern byte DAT_00523600[];
extern byte DAT_00523610[];

void __cdecl FUN_004453d0(int param_1, int param_2, undefined4 param_3, int param_4, int param_5)
{
  undefined4 *puVar3;
  byte *pbVar9;
  byte bVar6;
  byte local_4;
  unsigned int uVar7;
  int iVar8;
  int iVar10;
  byte *pbVar1;
  byte *pbVar2;
  undefined1 *puVar4;
  undefined1 *puVar5;
  bool bVar11;
  int local_108e;
  undefined1 **local_1086;

  puVar3 = FUN_004438c0();
  if (puVar3 != (undefined4 *)0x0) {
    puVar3[0x40a] = 0;
    puVar3[0x40c] = param_3;
    puVar3[0x40d] = param_4;
    local_108e = puVar3[0x40c] * param_4;
    __builtin_memcpy((void *)((int)puVar3 + 0x108e), &local_108e, 4);
    
    puVar4 = (undefined1 *)_malloc(puVar3[0x40c] * param_4 * 6);
    pbVar9 = (byte *)(param_1 + 0x10);
    iVar10 = 0;
    local_1086 = (undefined1 **)puVar4;
    __builtin_memcpy((void *)((int)puVar3 + 0x1086), &local_1086, 4);
    
    for (bVar6 = *pbVar9; bVar6 != 0xff; pbVar9 = pbVar9 + 1) {
      while (bVar6 != 0xfe) {
        local_4 = *pbVar9;
        if (local_4 == 0xfb) {
          pbVar1 = pbVar9 + 2;
          pbVar2 = pbVar9 + 1;
          pbVar9 = pbVar9 + 3;
          iVar10 = (unsigned int)*pbVar1 * 0x100 + (unsigned int)*pbVar2;
        }
        else if (local_4 == 0xfc) {
          pbVar1 = pbVar9 + 1;
          pbVar9 = pbVar9 + 2;
          puVar5 = puVar4;
          bVar6 = *pbVar1;
          do {
            *puVar5 = 0x80;
            puVar4 = puVar5 + 6;
            puVar5[1] = 0x80;
            puVar5[2] = 0x80;
            *(undefined2 *)(puVar5 + 4) = 0;
            puVar5[3] = 0;
            bVar11 = bVar6 != 0;
            puVar5 = puVar4;
            bVar6 = bVar6 - 1;
          } while (bVar11);
        }
        else {
          if (local_4 == 0xfd) {
            local_4 = pbVar9[1];
            uVar7 = (unsigned int)pbVar9[2];
            pbVar9 = pbVar9 + 3;
          }
          else {
            pbVar9 = pbVar9 + 1;
            uVar7 = (unsigned int)(local_4 >> (DAT_00523610[param_5] & 0x1f)) + iVar10;
            local_4 = DAT_00523600[param_5] & local_4;
          }
          iVar8 = uVar7 * 4;
          puVar5 = puVar4;
          do {
            *puVar5 = *(undefined1 *)(param_2 + iVar8);
            puVar4 = puVar5 + 6;
            puVar5[1] = *(undefined1 *)(param_2 + 1 + iVar8);
            puVar5[2] = *(undefined1 *)(param_2 + 2 + iVar8);
            *(undefined2 *)(puVar5 + 4) = 0;
            puVar5[3] = 0;
            bVar11 = local_4 != 0;
            puVar5 = puVar4;
            local_4 = local_4 - 1;
          } while (bVar11);
        }
        bVar6 = *pbVar9;
      }
    }
    FUN_00443a50((int)puVar3);
    FUN_004441f0((int)puVar3);
    FUN_00443c30(puVar3);
  }
  return;
}
