/* SPDX-License-Identifier: MIT */
/* z_xvt 00443db0 */
extern int DAT_006d4988;
extern char DAT_007092d9;
extern unsigned short DAT_0071b73c;
extern int DAT_0071ecf4;
extern unsigned char DAT_0070ad48;
extern char DAT_0072f508;
extern char DAT_006d4a02;
extern int DAT_006d4870;

extern int * __cdecl FUN_00432cc0(int param_1);
extern void __cdecl FUN_004ac210(unsigned short param_1, unsigned char param_2);
extern void __cdecl FUN_004a3490(unsigned short param_1, unsigned int * param_2);

void __cdecl FUN_00443db0(unsigned short param_1, unsigned int *param_2, unsigned int param_3)
{
  int iVar1;
  int iVar2;
  unsigned int uVar3;
  unsigned int uVar4;
  unsigned int *puVar5;
  int *piVar6;
  int *piVar7;
  unsigned int *puVar8;
  int bVar9;
  unsigned char bVar10;

  iVar2 = param_3 * 0x23 + DAT_006d4988;
  iVar1 = *(int *)(iVar2 + 0x1b);
  
  if ((DAT_007092d9 == '\0') &&
      (uVar3 = (unsigned int)*(unsigned short *)((int)&DAT_0071b73c + DAT_0071ecf4 * 0x5bd),
       *(char *)(uVar3 + 0x1f0 + *(int *)(*(int *)(iVar2 + 0x1f) + 0xa7)) == '\0')) {
    if (uVar3 == (unsigned char)(&DAT_0070ad48)[(unsigned int)*(unsigned char *)((param_3 & 0xffff) * 0x23 + 0x16 + DAT_006d4988) * 0x566]) {
      bVar9 = 0;
    }
    else {
      bVar9 = (&DAT_0072f508)[(unsigned int)(unsigned char)(&DAT_0070ad48)[(unsigned int)*(unsigned char *)((param_3 & 0xffff) * 0x23 + 0x16 + DAT_006d4988) * 0x566] + uVar3 * 0x1e5] == '\0';
    }
    if (!bVar9) {
      goto LAB_00443e7f;
    }
    if (DAT_006d4a02 != '\x02') {
      goto LAB_00443e91;
    }
    bVar10 = 1;
  }
  else {
LAB_00443e7f:
    if (iVar1 != -1) {
      piVar6 = FUN_00432cc0(iVar1);
      goto LAB_00443ea1;
    }
LAB_00443e91:
    bVar10 = 3;
  }
  piVar6 = &DAT_006d4870;
  FUN_004ac210((unsigned short)param_3, bVar10);
LAB_00443ea1:
  uVar3 = 0xffffffff;
  do {
    piVar7 = piVar6;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    piVar7 = (int *)((int)piVar6 + 1);
    iVar1 = *piVar6;
    piVar6 = piVar7;
  } while ((char)iVar1 != '\0');
  uVar3 = ~uVar3;
  puVar5 = (unsigned int *)((int)piVar7 - uVar3);
  puVar8 = param_2;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *puVar8 = *puVar5;
    puVar5 = puVar5 + 1;
    puVar8 = puVar8 + 1;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(unsigned char *)puVar8 = *(unsigned char *)puVar5;
    puVar5 = (unsigned int *)((int)puVar5 + 1);
    puVar8 = (unsigned int *)((int)puVar8 + 1);
  }
  FUN_004a3490(param_1, param_2);
  return;
}
