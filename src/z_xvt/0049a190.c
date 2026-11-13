/* SPDX-License-Identifier: MIT */
/* z_xvt 0049a190 */
unsigned int __cdecl FUN_0049a190(unsigned int *param_1, unsigned char *param_2, int param_3)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned char *pbVar4;
  int local_4;

  iVar1 = param_3 / 0x7c;
  if (iVar1 == 0) {
    iVar1 = param_3;
  }
  local_4 = 0x7d;
  do {
    uVar3 = 0;
    iVar2 = iVar1;
    pbVar4 = param_2;
    if (iVar1 > 0) {
      do {
        if (param_3 != 0) {
          pbVar4 = pbVar4 + 1;
          param_3 = param_3 - 1;
          uVar3 = (uVar3 + *pbVar4) * 2 | (unsigned int)((int)(uVar3 + *pbVar4) < 0);
        }
        iVar2 = iVar2 - 1;
      } while (iVar2 != 0);
    }
    *param_1 = uVar3;
    param_1 = param_1 + 1;
    local_4 = local_4 - 1;
  } while (local_4 != 0);
  return 0x7d;
}
