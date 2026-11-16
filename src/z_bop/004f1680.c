/* SPDX-License-Identifier: MIT */
/* z_bop 004f1680 */
extern unsigned int DAT_00aa6150;
extern unsigned int DAT_00aa615e;
extern unsigned int DAT_00aa62aa;

unsigned int __stdcall FUN_004f1680(void)
{
  unsigned int *piVar1;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  unsigned int *piVar5;
  unsigned int *puVar6;
  unsigned int *puVar7;
  unsigned int *puVar8;

  iVar4 = 0;
  piVar5 = &DAT_00aa615e;
  while (*piVar5 != 0 || (uVar2 = iVar4 + 1, 7 < uVar2)) {
LAB_004f16de:
    piVar5 = (unsigned int *)((unsigned int)piVar5 + 0x2a);
    iVar4 = iVar4 + 1;
    if ((unsigned int)&DAT_00aa62aa + 3 < (unsigned int)piVar5) {
      return 1;
    }
  }
  piVar1 = (unsigned int *)((unsigned int)&DAT_00aa615e + uVar2 * 0x2a);
LAB_004f16a4:
  if (*piVar1 == 0) goto code_r0x004f16a9;
  puVar8 = (unsigned int *)((unsigned int)&DAT_00aa6150 + uVar2 * 0x2a);
  puVar6 = puVar8;
  puVar7 = (unsigned int *)((unsigned int)piVar5 + -0xe);
  for (iVar3 = 10; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar7 = *puVar6;
    puVar6 = puVar6 + 1;
    puVar7 = puVar7 + 1;
  }
  *(unsigned short *)puVar7 = *(unsigned short *)puVar6;
  for (iVar3 = 10; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar8 = 0;
    puVar8 = puVar8 + 1;
  }
  *(unsigned short *)puVar8 = 0;
  goto LAB_004f16de;
code_r0x004f16a9:
  piVar1 = (unsigned int *)((unsigned int)piVar1 + 0x2a);
  uVar2 = uVar2 + 1;
  if ((unsigned int)&DAT_00aa62aa + 3 < (unsigned int)piVar1) goto LAB_004f16de;
  goto LAB_004f16a4;
}
