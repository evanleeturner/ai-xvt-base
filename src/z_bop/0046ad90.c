/* SPDX-License-Identifier: MIT */
/* z_bop 0046ad90 */
extern char s_NULLORDR_00526050[];

int __cdecl FUN_0046ad90(unsigned char *param_1)
{
  unsigned char *pbVar1;
  unsigned char bVar2;
  int iVar3;
  int iVar4;
  char *pcVar5;
  unsigned char *pbVar6;
  unsigned char *pbVar7;
  int bVar8;
  
  iVar3 = 0;
  if (s_NULLORDR_00526050[0] != '\0') {
    pcVar5 = s_NULLORDR_00526050;
    pbVar7 = param_1;
    pbVar6 = (unsigned char *)pcVar5;
LAB_0046adac:
    {
      bVar2 = *pcVar5;
      bVar8 = bVar2 < *pbVar7;
      if (bVar2 == *pbVar7) {
        if (bVar2 != 0) {
          bVar2 = pcVar5[1];
          bVar8 = bVar2 < pbVar7[1];
          if (bVar2 != pbVar7[1]) goto LAB_0046adcc;
          pcVar5 = pcVar5 + 2;
          pbVar7 = pbVar7 + 2;
          if (bVar2 != 0) goto LAB_0046adac;
        }
        iVar4 = 0;
      }
      else {
LAB_0046adcc:
        iVar4 = (1 - (unsigned int)bVar8) - (unsigned int)(bVar8 != 0);
      }
      if (iVar4 == 0) {
        return iVar3;
      }
      pbVar1 = pbVar6 + 0x52;
      pcVar5 = (char *)(pbVar6 + 0x52);
      iVar3 = iVar3 + 1;
      pbVar7 = param_1;
      pbVar6 = (unsigned char *)pcVar5;
    }
    if (*pbVar1 != 0) goto LAB_0046adac;
  }
  return iVar3;
}
