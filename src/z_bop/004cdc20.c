/* SPDX-License-Identifier: MIT */
/* z_bop 004cdc20 */
extern char DAT_00aa7b0d;
extern int **DAT_00acebc5;
extern int DAT_00acec25;
extern int DAT_00acec4d;
extern int DAT_00acebfd;
extern int DAT_00acec45;
extern int DAT_00aa6cfc;
extern int LAB_004cdcf0;

int __stdcall FUN_004d4970(void);
int __stdcall FUN_004d48e0(void);

int __stdcall FUN_004cdc20(void)
{
  char cVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  int *puVar5;
  int *piVar6;
  int local_500[0x140];
  int aiStack_4f0[316];

  cVar1 = DAT_00aa7b0d;
  if (DAT_00acebc5 == 0) {
    return 0;
  }
  FUN_004d4970();
  piVar4 = &DAT_00acec25;
  piVar6 = local_500;
  for (iVar3 = 0x140; iVar3 != 0; iVar3--) {
    *piVar6 = *piVar4;
    piVar4++;
    piVar6++;
  }
  puVar5 = &DAT_00acec4d;
  for (iVar3 = 0x136; iVar3 != 0; iVar3--) {
    *puVar5 = 0;
    puVar5++;
  }
  DAT_00acebfd = 1;
  iVar3 = 0;
  ((void (__stdcall *)(int **, int, int *, int, int))(*DAT_00acebc5)[12])(DAT_00acebc5, 0, &LAB_004cdcf0, 0, 0);
  if (DAT_00acebfd > 0) {
    piVar4 = &DAT_00acec45;
    do {
      iVar2 = 0;
      piVar6 = local_500 + 3;
      do {
        if (*piVar6 == *piVar4) {
          piVar4[1] = aiStack_4f0[iVar2 * 10];
          break;
        }
        piVar6 += 10;
        iVar2++;
      } while (piVar6 < local_500 + 0x140);
      piVar4 += 10;
      iVar3++;
    } while (iVar3 < DAT_00acebfd);
  }
  if (cVar1 != 0) {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
