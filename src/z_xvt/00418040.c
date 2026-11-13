/* SPDX-License-Identifier: MIT */
/* z_xvt 00418040 */
int * __cdecl FUN_00418040(undefined4 *param_1)
{
  int *piVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  uint unaff_ESI;
  int *piVar5;
  undefined4 *puVar6;
  int *piVar7;
  undefined4 *puVar8;
  uint puVar9;
  undefined4 *puVar10;
  int *piVar11;
  uint local_14;
  undefined1 auStack_10[16];
  uint local_10;

  unaff_ESI = 0;
  if (param_1 == (undefined4 *)0x0) {
    return (int *)0x0;
  }
  piVar4 = (int *)param_1[param_1[3] + 4];
  if (piVar4 != (int *)0x0) {
    iVar2 = (**(int (__cdecl **)(int *, uint *))(*piVar4 + 0x24))(piVar4, &local_14);
    if (iVar2 < 0) {
      unaff_ESI = 0;
    }
    if ((unaff_ESI & 1) != 0) {
      if ((int)param_1[2] < 2) {
        piVar4 = (int *)0x0;
      }
      else {
        iVar2 = param_1[3];
        param_1[3] = iVar2 + 1;
        if ((int)param_1[2] <= iVar2 + 1) {
          param_1[3] = 0;
        }
        piVar4 = (int *)param_1[param_1[3] + 4];
        iVar2 = (**(int (__cdecl **)(int *, uint *))(*piVar4 + 0x24))(piVar4, &local_14);
        if ((-1 < iVar2) && ((unaff_ESI & 1) != 0)) {
          (**(void (__cdecl **)(int *))(*piVar4 + 0x48))(piVar4);
          (**(void (__cdecl **)(int *, int))(*piVar4 + 0x34))(piVar4, 0);
        }
      }
    }
    if ((piVar4 != (int *)0x0) && ((unaff_ESI & 2) != 0)) {
      piVar11 = piVar4;
      iVar2 = (**(int (__cdecl **)(void))(*piVar4 + 0x50))();
      if (-1 < iVar2) {
        piVar1 = (int *)*param_1;
        if (((piVar4 != (int *)0x0) && (piVar1 != (int *)0x0)) && (param_1[1] != 0)) {
          puVar9 = local_14;
          puVar10 = (undefined4 *)0x0;
          iVar2 = (**(int (__cdecl **)(int *, int, undefined4, undefined1 *, uint *))(*piVar4 + 0x2c))(piVar4, 0, param_1[1], auStack_10, &local_10);
          if (-1 < iVar2) {
            piVar5 = piVar1;
            piVar7 = piVar11;
            for (uVar3 = puVar9 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
              *piVar7 = *piVar5;
              piVar5 = piVar5 + 1;
              piVar7 = piVar7 + 1;
            }
            for (uVar3 = puVar9 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
              *(char *)piVar7 = (char)*piVar5;
              piVar5 = (int *)((int)piVar5 + 1);
              piVar7 = (int *)((int)piVar7 + 1);
            }
            if (local_14 != 0) {
              puVar6 = (undefined4 *)(puVar9 + (int)piVar1);
              puVar8 = puVar10;
              for (uVar3 = local_14 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
                *puVar8 = *puVar6;
                puVar6 = puVar6 + 1;
                puVar8 = puVar8 + 1;
              }
              for (uVar3 = local_14 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
                *(undefined1 *)puVar8 = *(undefined1 *)puVar6;
                puVar6 = (undefined4 *)((int)puVar6 + 1);
                puVar8 = (undefined4 *)((int)puVar8 + 1);
              }
            }
            (**(void (__cdecl **)(int *, int *, uint, undefined4 *, uint))(*piVar4 + 0x4c))(piVar4, piVar11, puVar9, puVar10, local_14);
            return piVar4;
          }
        }
      }
      piVar4 = (int *)0x0;
    }
  }
  return piVar4;
}
