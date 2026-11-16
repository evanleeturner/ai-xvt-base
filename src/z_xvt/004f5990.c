/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5990 */
extern unsigned int DAT_0081bfb6;
extern int DAT_00757870;
extern int DAT_0081b850;
extern unsigned int DAT_00758620;
extern unsigned int DAT_00758628;
extern unsigned int DAT_00758624;
extern unsigned int DAT_00757680;
extern int DAT_0075766c;
extern char DAT_007438e4;
extern int DAT_00757874;

extern int _rand(void);
extern int FUN_004cb830(int param_1, unsigned int *param_2, unsigned int param_3);

unsigned int FUN_004f5990(void)
{
  unsigned int uVar1 = DAT_0081bfb6;
  int iVar8 = 0;
  int iVar10 = DAT_0081bfb6 * 0x20;
  int iVar2 = *(int *)((int)&DAT_00757870 + DAT_0081bfb6 * 0x20);
  int iVar4 = iVar10;

  do {
    if (iVar2 == 0) {
      if (DAT_0081b850 != 2) {
        DAT_00758620 = 0x57;
        unsigned int *puVar9 = (unsigned int *)((int)&DAT_00757680 + DAT_0081bfb6 * 0x20);
        unsigned int *puVar11 = (unsigned int *)&DAT_00758628;
        DAT_00758624 = DAT_0081bfb6;
        int iVar2_loop = 8;
        while (iVar2_loop != 0) {
          *puVar11 = *puVar9;
          puVar9++;
          puVar11++;
          iVar2_loop--;
        }
        FUN_004cb830(0, &DAT_00758620, 0x28);
      }
      return 1;
    }
    iVar2 = _rand();
    iVar2 = iVar2 % 9 + 1;
    *(unsigned int *)((int)&DAT_00757680 + iVar4) = 0xffffffff;
    do {
      int iVar5 = 0;
      if (0 < DAT_0075766c) {
        char *pcVar6 = (char *)&DAT_007438e4;
        do {
          if ((*pcVar6 != '\0') && ((unsigned char)pcVar6[-0xc] == uVar1)) {
            int iVar7 = 0;
            if (0 < iVar8) {
              int *piVar3 = (int *)((int)&DAT_00757680 + iVar10);
              do {
                if (*piVar3 == iVar5) break;
                piVar3++;
                iVar7++;
              } while (iVar7 < iVar8);
            }
            if ((iVar7 == iVar8) && (iVar2 = iVar2 - 1, iVar2 == 0)) {
              *(int *)((int)&DAT_00757680 + iVar4) = iVar5;
              break;
            }
          }
          pcVar6 += 0x562;
          iVar5++;
        } while (iVar5 < DAT_0075766c);
      }
    } while (*(int *)((int)&DAT_00757680 + iVar4) == -1);
    iVar2 = *(int *)((int)&DAT_00757874 + iVar4);
    iVar4 = iVar4 + 4;
    iVar8 = iVar8 + 1;
  } while (1);
}
