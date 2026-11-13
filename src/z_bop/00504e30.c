/* SPDX-License-Identifier: MIT */
/* z_bop 00504e30 */
extern uint DAT_00aa6138;
extern uint DAT_00aa6114;
extern int DAT_00b6a5ae;
extern uint DAT_00b6a5aa;
extern char DAT_00b69d20[];
extern char DAT_0051a854[];
extern char *PTR_s_train_0052c1e8;
extern char s__s__0052c300[];

int __cdecl _sprintf(char *dst, const char *fmt, ...);
HFILE __cdecl FUN_004cc4a0(char *path, void *mode);
void __cdecl FUN_004cc660(HFILE f, void *buf);
void __cdecl FUN_004cc5c0(HFILE f, long offset, int whence);
void __cdecl FUN_004cc6c0(HFILE f, void *buf, uint size);
void __cdecl FUN_004cc590(HFILE f);

void __cdecl FUN_00504e30(uint *param_1, int param_2)
{
  HFILE pFVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  uint *puVar5;
  long lVar6;
  short local_2;

  if (param_1 != (uint *)0x0) {
    uVar4 = 0;
    puVar5 = param_1;
    for (iVar2 = 0x400; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar5 = 0;
      puVar5 = puVar5 + 1;
    }
    if (DAT_00aa6138 != 0) {
      piVar3 = (int *)(DAT_00aa6114 + 0x140);
      do {
        if (*piVar3 == (&DAT_00b6a5ae)[DAT_00b6a5aa]) break;
        piVar3 = piVar3 + 0x52;
        uVar4 = uVar4 + 1;
      } while (uVar4 < DAT_00aa6138);
    }
    if (uVar4 < DAT_00aa6138) {
      _sprintf((char *)&DAT_00b69d20, s__s__0052c300, (&PTR_s_train_0052c1e8)[DAT_00b6a5aa],
               uVar4 * 0x148 + DAT_00aa6114);
      pFVar1 = FUN_004cc4a0((char *)&DAT_00b69d20, &DAT_0051a854);
      if (pFVar1 != (HFILE)0x0) {
        if (((DAT_00b6a5aa != 2) && (DAT_00b6a5aa != 4)) && (DAT_00b6a5aa != 5)) {
          FUN_004cc660(pFVar1, &local_2);
          if (local_2 == 0xe) {
            if (param_2 == 0) {
              lVar6 = -0x2000;
            }
            else {
              lVar6 = -0x3000;
            }
            FUN_004cc5c0(pFVar1, lVar6, 2);
            FUN_004cc6c0(pFVar1, param_1, 0x1000);
            *(unsigned char *)((int)param_1 + 0xfff) = 0;
          }
        }
        FUN_004cc590(pFVar1);
      }
    }
  }
  return;
}
