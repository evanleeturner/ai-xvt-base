/* SPDX-License-Identifier: MIT */
/* z_bop 004dc7e0 */
extern void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
extern void __cdecl FUN_004d4d20(int *param_1);
extern void __cdecl FUN_004d4e20(void);
extern int __cdecl FUN_004d48e0(void);
extern void __cdecl FUN_004b5e20(int *param_1, int param_2, int param_3);
extern void __cdecl FUN_004d4ec0(int param_1);
extern void __cdecl FUN_004d4970(void);
extern void __cdecl _free(void *param_1);

extern int DAT_00acc131;
extern char DAT_00aa7b0d;
extern int DAT_00aa7b1f;
extern char DAT_00aa7b24;
extern int DAT_00aa6cfc;
extern int DAT_00aa7b13;
extern int DAT_00acc145;
extern int DAT_00acc139;
extern int DAT_00acc13d;
extern int DAT_00acc149;
extern int DAT_00acc54d;
extern int DAT_00acc551;
extern int DAT_00acc55d;
extern int DAT_00acc575;
extern int DAT_00acebbd;

void __stdcall FUN_004dc7e0(void)
{
  int iVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  unsigned short *puVar5;
  int iVar6;
  unsigned short *puVar7;
  int iVar8;
  unsigned short *puVar9;
  unsigned short *local_1c;
  int local_10[4];
  
  if (DAT_00acc131 == 0) {
    return;
  }
  iVar3 = DAT_00acc131 - 1;
  FUN_004d6460(local_10, 0, 0, 0x280, 0x1e0);
  FUN_004d4d20(local_10);
  cVar2 = DAT_00aa7b0d;
  iVar8 = iVar3 * 0x440;
  if ((int)(&DAT_00acc145)[iVar3 * 0x110] < 1) goto LAB_004dc980;
  if (DAT_00aa7b1f == 8) {
    if (DAT_00aa7b24 == '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    else {
      FUN_004d4e20();
    }
    FUN_004b5e20((int *)((char *)&DAT_00acc139 + iVar8), *(int *)((char *)&DAT_00acc54d + iVar8),
                 *(int *)((char *)&DAT_00acc551 + iVar8));
    if (DAT_00aa7b24 != '\0') {
      FUN_004d4ec0(1);
    }
joined_r0x004dc8a9:
    if (cVar2 == '\0') {
      FUN_004d4970();
    }
  }
  else if (DAT_00aa7b1f == 0x10) {
    if (DAT_00aa7b24 == '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
    else {
      FUN_004d4e20();
    }
    iVar1 = *(int *)((char *)&DAT_00acc139 + iVar8);
    iVar6 = *(int *)((char *)&DAT_00acc13d + iVar8);
    local_1c = (unsigned short *)(&DAT_00acc149)[iVar3 * 0x110];
    puVar5 = (unsigned short *)(*(int *)((char *)&DAT_00acc551 + iVar8) * DAT_00aa7b13 + *(int *)((char *)&DAT_00acc54d + iVar8) * 2 +
             DAT_00aa6cfc);
    if (0 < iVar6) {
      do {
        iVar4 = iVar1;
        puVar7 = local_1c;
        puVar9 = puVar5;
        if (0 < iVar1) {
          do {
            *puVar9 = *puVar7;
            iVar4 = iVar4 - 1;
            puVar7 = puVar7 + 1;
            puVar9 = puVar9 + 1;
          } while (iVar4 != 0);
        }
        local_1c = local_1c + iVar1;
        puVar5 = (unsigned short *)((int)puVar5 + (DAT_00aa7b13 & 0xfffffffe));
        iVar6 = iVar6 - 1;
      } while (iVar6 != 0);
    }
    if (DAT_00aa7b24 != '\0') {
      FUN_004d4ec0(1);
    }
    goto joined_r0x004dc8a9;
  }
  _free((void *)(&DAT_00acc149)[iVar3 * 0x110]);
LAB_004dc980:
  FUN_004d4d20((int *)((char *)&DAT_00acc55d + iVar8));
  DAT_00acebbd = *(int *)((char *)&DAT_00acc575 + iVar8);
  DAT_00acc131 = iVar3;
  return;
}
