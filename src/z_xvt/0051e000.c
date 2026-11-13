/* SPDX-License-Identifier: MIT */
/* z_xvt 0051e000 */
void __cdecl FUN_0051e000(unsigned int *param_1, int param_2, int param_3, int param_4, int *param_5, int param_6)
{
  unsigned short uVar1;
  int iVar2;
  short sVar3;
  int iVar4;
  unsigned int uVar5;
  int iVar6;
  unsigned int uVar7;
  int *piVar8;
  int *piVar9;
  int *piVar10;
  int *piVar11;
  unsigned int uVar12;
  unsigned int *puVar13;
  unsigned int *puVar14;
  int local_18;
  
  iVar4 = param_2 >> 2;
  uVar12 = (unsigned int)(param_2 + iVar4 * -4);
  iVar2 = (param_6 - param_2) * 2;
  piVar10 = (int *)(param_6 + (int)param_5);
  
  if (uVar12 == 0) {
    while (param_3 != 0) {
      param_3 = param_3 + -1;
      local_18 = iVar4;
      while (local_18 != 0) {
        local_18 = local_18 + -1;
        uVar12 = *param_1;
        param_1 = param_1 + 1;
        uVar7 = uVar12 >> 0x10;
        iVar6 = (uVar7 & 0xff) + (uVar7 & 0xffffff00) * 0x10000 + uVar7 * 0x100;
        *param_5 = iVar6;
        *piVar10 = iVar6;
        iVar6 = (uVar12 & 0xff00) * 0x10000 + (uVar12 & 0xffff) * 0x100 + (uVar12 & 0xff);
        param_5[1] = iVar6;
        piVar10[1] = iVar6;
        param_5 = param_5 + 2;
        piVar10 = piVar10 + 2;
      }
      param_1 = (unsigned int *)((int)param_1 + (param_4 - param_2));
      param_5 = (int *)((int)param_5 + iVar2);
      piVar10 = (int *)((int)piVar10 + iVar2);
    }
  }
  else if (param_3 != 0) {
    do {
      param_3 = param_3 + -1;
      local_18 = iVar4;
      while (local_18 != 0) {
        local_18 = local_18 + -1;
        uVar7 = *param_1;
        param_1 = param_1 + 1;
        uVar5 = uVar7 >> 0x10;
        iVar6 = (uVar5 & 0xff) + (uVar5 & 0xffffff00) * 0x10000 + uVar5 * 0x100;
        *param_5 = iVar6;
        *piVar10 = iVar6;
        iVar6 = (uVar7 & 0xff00) * 0x10000 + (uVar7 & 0xffff) * 0x100 + (uVar7 & 0xff);
        param_5[1] = iVar6;
        piVar10[1] = iVar6;
        param_5 = param_5 + 2;
        piVar10 = piVar10 + 2;
      }
      piVar8 = param_5;
      piVar11 = piVar10;
      puVar13 = param_1;
      if ((uVar12 & 2) != 0) {
        puVar13 = (unsigned int *)((int)param_1 + 2);
        uVar1 = (unsigned short)*param_1;
        piVar8 = param_5 + 1;
        iVar6 = (uVar1 & 0xff) + (uVar1 & 0xffffff00) * 0x10000 + (unsigned int)uVar1 * 0x100;
        piVar11 = piVar10 + 1;
        *param_5 = iVar6;
        *piVar10 = iVar6;
      }
      piVar9 = piVar8;
      piVar10 = piVar11;
      puVar14 = puVar13;
      if ((uVar12 & 1) != 0) {
        puVar14 = (unsigned int *)((int)puVar13 + 1);
        piVar9 = (int *)((int)piVar8 + 2);
        piVar10 = (int *)((int)piVar11 + 2);
        sVar3 = (short)((unsigned short)(unsigned char)*puVar13 + (unsigned short)(unsigned char)*puVar13 * 0x100);
        *(short *)piVar8 = sVar3;
        *(short *)piVar11 = sVar3;
      }
      param_1 = (unsigned int *)((int)puVar14 + (param_4 - param_2));
      param_5 = (int *)((int)piVar9 + iVar2);
      piVar10 = (int *)((int)piVar10 + iVar2);
    } while (param_3 != 0);
    return;
  }
  return;
}
