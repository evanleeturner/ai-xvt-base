/* SPDX-License-Identifier: MIT */
/* z_bop 004193c0 */
extern int DAT_00523420;
extern int DAT_009d8a30;
extern char DAT_009e9680;
extern int DAT_009ecc34;
extern int DAT_00550b98;
extern int DAT_00550880;
extern int DAT_00550b8c;
extern unsigned int DAT_009d77d0;
extern char DAT_009ec468;
extern int DAT_0052341c;

int __cdecl FUN_0046f660(int param_1);
int __cdecl FUN_00465390(int param_1, int param_2, int param_3);
void __cdecl FUN_004651f0(int param_1, int param_2);
int __cdecl FUN_0046e040(void);

void __cdecl FUN_004193c0(int param_1, int param_2)
{
  int iVar3;
  int iVar6;
  int *piVar8;
  unsigned int uVar7;
  unsigned int *puVar5;
  char *pcVar4;
  int bVar1;
  int bVar2;

  if (*(int *)(param_2 + 4) == DAT_00523420) {
    iVar3 = FUN_0046f660(param_1);
    if (((&DAT_009d8a30)[iVar3] == 0) && ((&DAT_009e9680)[iVar3 * 0x5bd] != '\0')) {
      bVar1 = 0;
      if (iVar3 != DAT_009ecc34) {
        piVar8 = (int *)(param_2 + 8);
        iVar6 = 0;
        bVar2 = bVar1;
        do {
          if (*(int *)((int)&DAT_00550b98 + iVar6) != *piVar8) {
            bVar2 = 1;
          }
          iVar6 = iVar6 + 4;
          piVar8 = piVar8 + 1;
        } while (iVar6 < 0x40);
        bVar1 = 0;
        if (bVar2) {
          iVar6 = FUN_00465390(param_1, DAT_00550880, DAT_00550b8c);
          if (iVar6 != 0) {
            FUN_004651f0(param_1, DAT_00550b8c);
          }
          bVar1 = 1;
        }
      }
      iVar6 = FUN_0046e040();
      if (iVar6 != 0) {
        (&DAT_009d77d0)[iVar3] = 2;
        if (!bVar1) {
          (&DAT_009d77d0)[iVar3] = 1;
        }
        uVar7 = 3;
        puVar5 = &DAT_009d77d0;
        pcVar4 = &DAT_009e9680;
        do {
          if (*pcVar4 != '\0') {
            uVar7 = uVar7 & *puVar5;
          }
          puVar5 = puVar5 + 1;
          pcVar4 = pcVar4 + 0x5bd;
        } while (pcVar4 < &DAT_009ec468);
        if ((uVar7 & 1) != 0) {
          DAT_0052341c = 0;
        }
      }
    }
  }
  return;
}
