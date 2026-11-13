/* SPDX-License-Identifier: MIT */
/* z_bop 004d8540 */
// Prototypes for called functions
void FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
char *FUN_004ddc10(unsigned int param_1);
void FUN_004db7d0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
void FUN_004d1400(void);
void FUN_004d4cf0(int *param_1);
void FUN_004d4d20(int *param_1);
int _sprintf(char *param_1, const char *param_2, ...);
void FUN_004d64a0(int *param_1, int param_2, int param_3);

// External globals referenced
extern int DAT_00aa6ae8;
extern int DAT_00aa615e;
extern int DAT_00aa62ae;
extern char DAT_00b69d20[];
extern const char s__c_s__c_s_0052a6b0[];

int __stdcall FUN_004d8540(void)
{
  char *pcVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  int uVar5;
  unsigned int uVar6;
  int local_20[4];
  int local_10[4];
  
  FUN_004d6460(local_20, 0x58, 0xda, 0x1ae, 0xe9);
  piVar2 = local_20;
  uVar6 = 0xffff;
  iVar4 = 1;
  iVar3 = 0;
  pcVar1 = FUN_004ddc10(0xc9);
  FUN_004db7d0(0xc, pcVar1, piVar2, iVar3, iVar4, uVar6);
  if (DAT_00aa6ae8 != -1) {
    iVar3 = 0;
    FUN_004d1400();
    piVar2 = &DAT_00aa615e;
    FUN_004d6460(local_20, 0x58, 0xee, 0x102, 0xfc);
    do {
      if (*piVar2 != 0) {
        iVar3 = iVar3 + 1;
        FUN_004d4cf0(local_10);
        FUN_004d4d20(local_20);
        iVar4 = (int)piVar2 + -0xe;
        uVar5 = 4;
        pcVar1 = FUN_004ddc10(piVar2[1] + 0x9a);
        _sprintf((char *)&DAT_00b69d20, s__c_s__c_s_0052a6b0, 6, pcVar1, uVar5, iVar4);
        FUN_004db7d0(0xc, (char *)&DAT_00b69d20, local_20, 0, 1, 0xffff);
        FUN_004d4d20(local_10);
        if (iVar3 == 4) {
          FUN_004d6460(local_20, 0x103, 0xee, 0x1ae, 0xfc);
        }
        else {
          FUN_004d64a0(local_20, 0, 0xf);
        }
      }
      piVar2 = (int *)((int)piVar2 + 0x2a);
    } while (piVar2 < &DAT_00aa62ae);
  }
  return 1;
}
