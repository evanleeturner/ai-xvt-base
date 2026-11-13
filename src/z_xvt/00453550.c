/* SPDX-License-Identifier: MIT */
/* z_xvt 00453550 */
void * __cdecl FUN_0048e480(int param_1, char *param_2);
void __cdecl FUN_00453550(void *param_1, int param_2, float *param_3);

void __cdecl FUN_00453550(void *param_1, int param_2, float *param_3)
{
  unsigned int *puVar1;
  int iVar2;
  float *pfVar3;
  int iVar4;
  unsigned int *pParam1;

  if (param_1 != 0) {
    pParam1 = (unsigned int *)param_1;
    while (pParam1[1] == 7) {
      param_1 = FUN_0048e480(param_2, (char *)pParam1[5]);
      if (param_1 == 0) {
        return;
      }
      pParam1 = (unsigned int *)param_1;
    }
    if (pParam1[1] == 3) {
      iVar2 = pParam1[4];
      pfVar3 = (float *)pParam1[5];
      if (0 < iVar2) {
        while (1) {
          iVar2 = iVar2 - 1;
          *pfVar3 = *param_3 + *pfVar3;
          pfVar3[1] = pfVar3[1] + param_3[1];
          if (iVar2 == 0) break;
          pfVar3[2] = pfVar3[2] + param_3[2];
          pfVar3 = pfVar3 + 3;
        }
        pfVar3[2] = pfVar3[2] + param_3[2];
      }
    }
    iVar2 = 0;
    iVar4 = 0;
    if (0 < (int)pParam1[2]) {
      do {
        puVar1 = (unsigned int *)(pParam1[3] + iVar2);
        iVar2 = iVar2 + 4;
        iVar4 = iVar4 + 1;
        FUN_00453550(*(void **)puVar1, param_2, param_3);
      } while (iVar4 < (int)pParam1[2]);
    }
  }
  return;
}
