/* SPDX-License-Identifier: MIT */
/* z_bop 004d1330 */
extern void __stdcall FUN_004d4970(void);
extern int FUN_004d48e0(void);

extern char DAT_00aa7b0d;
extern int DAT_00acebc5;
extern int DAT_00acebfd;
extern int DAT_00acec45;
extern int DAT_00aa6cfc;
extern int DAT_00acec49;

int __cdecl FUN_004d1330(int param_1)
{
  char cVar1;
  int iVar2;
  int *piVar3;
  int bVar4;
  
  cVar1 = DAT_00aa7b0d;
  if (DAT_00acebc5 == 0) {
    return 0;
  }
  FUN_004d4970();
  iVar2 = 0;
  bVar4 = DAT_00acebfd == 0;
  if (DAT_00acebfd > 0) {
    piVar3 = &DAT_00acec45;
    do {
      if (*piVar3 == param_1) {
        break;
      }
      piVar3 = piVar3 + 10;
      iVar2 = iVar2 + 1;
    } while (iVar2 < DAT_00acebfd);
    bVar4 = DAT_00acebfd == iVar2;
  }
  if (bVar4) {
    if (cVar1 != '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    return 0;
  }
  (&DAT_00acec49)[iVar2 * 10] = 1;
  if (cVar1 != '\0') {
    DAT_00aa6cfc = FUN_004d48e0();
  }
  return 1;
}
