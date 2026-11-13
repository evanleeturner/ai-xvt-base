/* SPDX-License-Identifier: MIT */
/* z_xvt 004f5830 */
extern int DAT_0081b850;
extern int DAT_007579b0;
extern int DAT_00757cee;
extern int DAT_00757e3e;
extern int DAT_00757a28;
extern int DAT_00757870;
extern int DAT_00757874;
extern int DAT_00757680;
extern int DAT_00757684;
extern int DAT_0075788c;
extern int DAT_0075769c;

void __stdcall FUN_004cde00(void);

void __stdcall FUN_004f5830(void)
{
    int iVar1;
    int *piVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    int *piStack_8;
    int iStack_4;
    
    if (DAT_0081b850 != 2) {
        FUN_004cde00();
    }
    piStack_8 = &DAT_007579b0;
    do {
        iVar5 = 0;
        piVar2 = &DAT_00757cee;
        iVar1 = *piStack_8;
        do {
            if (*piVar2 == iVar1) break;
            piVar2 = (int *)((int)piVar2 + 0x2a);
            iVar5 = iVar5 + 1;
        } while (piVar2 < &DAT_00757e3e);
        if (iVar5 == 8) {
            if (0 < DAT_00757a28) {
                iVar5 = 0;
                iStack_4 = DAT_00757a28;
                do {
                    iVar7 = 0;
                    iVar4 = iVar5;
                    do {
                        if (*(int *)((int)&DAT_00757870 + iVar4) == iVar1) {
                            if (iVar7 < 7) {
                                iVar6 = 7 - iVar7;
                                iVar3 = iVar4;
                                do {
                                    iVar6 = iVar6 + -1;
                                    *(int *)((int)&DAT_00757870 + iVar3) =
                                         *(int *)((int)&DAT_00757874 + iVar3);
                                    *(int *)(&DAT_00757680 + iVar3) = *(int *)(&DAT_00757684 + iVar3);
                                    iVar3 = iVar3 + 4;
                                } while (iVar6 != 0);
                            }
                            *(int *)((int)&DAT_0075788c + iVar5) = 0;
                            *(int *)((int)&DAT_0075769c + iVar5) = 0xffffffff;
                        }
                        iVar4 = iVar4 + 4;
                        iVar7 = iVar7 + 1;
                    } while (iVar7 < 8);
                    iVar5 = iVar5 + 0x20;
                    iStack_4 = iStack_4 + -1;
                } while (iStack_4 != 0);
            }
            *piStack_8 = 0;
        }
        piStack_8 = piStack_8 + 1;
        if ((int *)0x7579cf < piStack_8) {
            return;
        }
    } while (1);
}
