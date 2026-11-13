/* SPDX-License-Identifier: MIT */
/* z_xvt 0051d3b0 */
void __cdecl FUN_0051d3b0(int param_1, int param_2, unsigned int *param_3, unsigned int param_4, int param_5, int param_6, int param_7, int param_8)
{
  unsigned char uVar1;
  unsigned short uVar2;
  unsigned int uVar3;
  int iVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  int iVar7;
  unsigned int *puVar8;

  uVar6 = -param_7 & 7U;
  if ((int)param_4 < (int)(-param_7 & 7U)) {
    uVar6 = param_4;
  }
  iVar4 = (int)(param_4 - uVar6) >> 3;
  uVar5 = (param_4 + iVar4 * -8) - uVar6;
  puVar8 = (unsigned int *)(param_8 * param_2 + param_7 + param_1);
  if ((uVar6 == 0) && (uVar5 == 0)) {
    if (param_5 != 0) {
      do {
        param_5 = param_5 - 1;
        for (iVar7 = iVar4; iVar7 != 0; iVar7 = iVar7 - 1) {
          puVar8[1] = param_3[1];
          *puVar8 = *param_3;
          puVar8 = puVar8 + 2;
          param_3 = param_3 + 2;
        }
        param_3 = (unsigned int *)((int)param_3 + (param_6 - param_4));
        puVar8 = (unsigned int *)((int)puVar8 + (param_2 - param_4));
      } while (param_5 != 0);
      return;
    }
  }
  else if (param_5 != 0) {
    do {
      param_5 = param_5 - 1;
      if ((uVar6 & 1) != 0) {
        uVar1 = *(unsigned char *)param_3;
        param_3 = (unsigned int *)((int)param_3 + 1);
        *(unsigned char *)puVar8 = uVar1;
        puVar8 = (unsigned int *)((int)puVar8 + 1);
      }
      if ((uVar6 & 2) != 0) {
        uVar2 = *(unsigned short *)param_3;
        param_3 = (unsigned int *)((int)param_3 + 2);
        *(unsigned short *)puVar8 = uVar2;
        puVar8 = (unsigned int *)((int)puVar8 + 2);
      }
      iVar7 = iVar4;
      if ((uVar6 & 4) != 0) {
        uVar3 = *param_3;
        param_3 = param_3 + 1;
        *puVar8 = uVar3;
        puVar8 = puVar8 + 1;
      }
      for (; iVar7 != 0; iVar7 = iVar7 - 1) {
        puVar8[1] = param_3[1];
        *puVar8 = *param_3;
        puVar8 = puVar8 + 2;
        param_3 = param_3 + 2;
      }
      if ((uVar5 & 4) != 0) {
        uVar3 = *param_3;
        param_3 = param_3 + 1;
        *puVar8 = uVar3;
        puVar8 = puVar8 + 1;
      }
      if ((uVar5 & 2) != 0) {
        uVar2 = *(unsigned short *)param_3;
        param_3 = (unsigned int *)((int)param_3 + 2);
        *(unsigned short *)puVar8 = uVar2;
        puVar8 = (unsigned int *)((int)puVar8 + 2);
      }
      if ((uVar5 & 1) != 0) {
        uVar1 = *(unsigned char *)param_3;
        param_3 = (unsigned int *)((int)param_3 + 1);
        *(unsigned char *)puVar8 = uVar1;
        puVar8 = (unsigned int *)((int)puVar8 + 1);
      }
      param_3 = (unsigned int *)((int)param_3 + (param_6 - param_4));
      puVar8 = (unsigned int *)((int)puVar8 + (param_2 - param_4));
    } while (param_5 != 0);
  }
  return;
}
