/* SPDX-License-Identifier: MIT */
/* z_bop 004d1ca0 */
extern int DAT_00b69a89;
extern int DAT_00b63f91;
extern int DAT_00acebf5;
extern int DAT_00b69851;

extern int * FUN_004cfe80(int *);

int __stdcall FUN_004d1ca0(void)
{
  int *puVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int *piVar6;
  int *piVar7;
  int local_4;
  
  iVar5 = 0;
  puVar1 = FUN_004cfe80(&local_4);
  if (0 < DAT_00b69a89) {
    piVar6 = &DAT_00b63f91;
    do {
      iVar3 = 0;
      if (0 < local_4) {
        piVar2 = puVar1 + 8;
        do {
          if (*piVar2 == *piVar6) break;
          piVar2 = piVar2 + 10;
          iVar3 = iVar3 + 1;
        } while (iVar3 < local_4);
      }
      if ((local_4 <= iVar3) && (*piVar6 != DAT_00acebf5)) {
        *piVar6 = 0;
        piVar6[1] = 0x7f;
        piVar6[3] = 0x7f;
        piVar6[5] = 0x7f;
        piVar6[2] = 0x7f;
        piVar6[4] = 0x7f;
        piVar6[6] = 0x7f;
        piVar6[7] = 0;
        *(unsigned char *)(piVar6 + 8) = 0x39;
        piVar6[0x88] = 1;
        piVar6[0x89] = 0;
        piVar6[0x8d] = 0;
        piVar6[0x8a] = 0;
        piVar6[0x8b] = 0;
        piVar6[0x8c] = 0;
      }
      piVar6 = piVar6 + 0x8e;
      iVar5 = iVar5 + 1;
    } while (iVar5 < DAT_00b69a89);
  }
  iVar5 = 0;
  if (DAT_00b69a89 != 1 && -1 < DAT_00b69a89 + -1) {
    piVar6 = &DAT_00b63f91;
    do {
      if ((*piVar6 == 0) && (iVar3 = iVar5 + 1, iVar3 < DAT_00b69a89)) {
        piVar2 = &DAT_00b63f91 + iVar3 * 0x8e;
      LAB_004d1da7:
        if (*piVar2 == 0) goto code_r0x004d1dab;
        piVar2 = &DAT_00b63f91 + iVar3 * 0x8e;
        piVar7 = piVar6;
        for (iVar4 = 0x8e; iVar4 != 0; iVar4 = iVar4 + -1) {
          *piVar7 = *piVar2;
          piVar2 = piVar2 + 1;
          piVar7 = piVar7 + 1;
        }
        (&DAT_00b63f91)[iVar3 * 0x8e] = 0;
        (&DAT_00b63f91)[iVar3 * 0x8e + 1] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 3] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 5] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 2] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 4] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 6] = 0x7f;
        (&DAT_00b63f91)[iVar3 * 0x8e + 7] = 0;
        *(unsigned char *)(&DAT_00b63f91 + iVar3 * 0x8e + 8) = 0x39;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x88] = 1;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x89] = 0;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x8d] = 0;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x8a] = 0;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x8b] = 0;
        (&DAT_00b63f91)[iVar3 * 0x8e + 0x8c] = 0;
      }
      code_r0x004d1dab:
        piVar2 = piVar2 + 0x8e;
        iVar3 = iVar3 + 1;
        if (DAT_00b69a89 <= iVar3) goto LAB_004d1e4f;
        goto LAB_004d1da7;
    LAB_004d1e4f:
      piVar6 = piVar6 + 0x8e;
      iVar5 = iVar5 + 1;
    } while (iVar5 < DAT_00b69a89 + -1);
  }
  DAT_00b69a89 = 0;
  piVar6 = &DAT_00b63f91;
  do {
    if (*piVar6 != 0) {
      DAT_00b69a89 = DAT_00b69a89 + 1;
    }
    piVar6 = piVar6 + 0x8e;
  } while (piVar6 < &DAT_00b69851);
  return 1;
}
