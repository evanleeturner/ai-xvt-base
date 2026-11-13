/* SPDX-License-Identifier: MIT */
/* z_xvt 0049bde0 */
extern int DAT_00525560;
extern int DAT_0070aaf0[];
extern char DAT_0071b740[];
extern int DAT_0071ecf4;
extern int DAT_00645a90[];
extern int DAT_00645778;
extern int DAT_00645a84;
extern unsigned int DAT_0070a150[];
extern char DAT_0071e528;
extern int DAT_0052555c;

int __cdecl FUN_00432c10(int);
int __cdecl FUN_0042e080(int, int, int);
void __cdecl FUN_0042dec0(int, int);
int __cdecl FUN_00431d30(void);

void __cdecl FUN_0049bde0(int param_1, int param_2)
{
  int bVar1;
  int bVar2;
  int iVar3;
  char *pcVar4;
  unsigned int *puVar5;
  int iVar6;
  unsigned int uVar7;
  int *piVar8;

  if (*(int *)(param_2 + 4) == DAT_00525560) {
    iVar3 = FUN_00432c10(param_1);
    if (DAT_0070aaf0[iVar3] == 0 && DAT_0071b740[iVar3 * 0x5bd] != '\0') {
      bVar1 = 0;
      if (DAT_0071ecf4 != iVar3) {
        piVar8 = (int *)(param_2 + 8);
        iVar6 = 0;
        bVar2 = bVar1;
        do {
          if (*(int *)((int)DAT_00645a90 + iVar6) != *piVar8) {
            bVar2 = 1;
          }
          iVar6 = iVar6 + 4;
          piVar8 = piVar8 + 1;
        } while (iVar6 < 0x40);
        bVar1 = 0;
        if (bVar2) {
          iVar6 = FUN_0042e080(param_1, DAT_00645778, DAT_00645a84);
          if (iVar6 != 0) {
            FUN_0042dec0(param_1, DAT_00645a84);
          }
          bVar1 = 1;
        }
      }
      iVar6 = FUN_00431d30();
      if (iVar6 != 0) {
        DAT_0070a150[iVar3] = 2;
        if (!bVar1) {
          DAT_0070a150[iVar3] = 1;
        }
        uVar7 = 3;
        puVar5 = DAT_0070a150;
        pcVar4 = DAT_0071b740;
        do {
          if (*pcVar4 != '\0') {
            uVar7 = uVar7 & *puVar5;
          }
          puVar5 = puVar5 + 1;
          pcVar4 = pcVar4 + 0x5bd;
        } while (pcVar4 < &DAT_0071e528);
        if ((uVar7 & 1) != 0) {
          DAT_0052555c = 0;
        }
      }
    }
  }
  return;
}
