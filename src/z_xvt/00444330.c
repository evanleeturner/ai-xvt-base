/* SPDX-License-Identifier: MIT */
/* z_xvt 00444330 */
extern int DAT_0071ecf4;
extern char DAT_0071b740;
extern char DAT_0071e528;
extern unsigned short DAT_00732950;

extern int * FUN_00432cc0(int param_1);
extern void FUN_004a3490(int param_1, int *param_2);
extern void FUN_004a1d70(int param_1, int param_2);

void __cdecl FUN_00444330(int param_1)
{
  int *piVar1;
  char *pcVar2;
  int iVar3;
  
  if (DAT_0071ecf4 != param_1) {
    piVar1 = FUN_00432cc0(param_1);
    FUN_004a3490(0, piVar1);
    FUN_004a1d70(0x173, DAT_0071ecf4);
    iVar3 = 0;
    pcVar2 = &DAT_0071b740;
    do {
      if (*pcVar2 == 1) {
        iVar3 = iVar3 + 1;
      }
      pcVar2 = pcVar2 + 0x5bd;
    } while (pcVar2 < &DAT_0071e528);
    if ((&DAT_0071b740)[DAT_0071ecf4 * 0x5bd] == 1) {
      if (iVar3 == 1) {
        FUN_004a1d70(0x175, DAT_0071ecf4);
        return;
      }
      DAT_00732950 = (unsigned short)iVar3;
      FUN_004a1d70(0x174, DAT_0071ecf4);
    }
  }
  return;
}
