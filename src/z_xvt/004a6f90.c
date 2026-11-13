/* SPDX-License-Identifier: MIT */
/* z_xvt 004a6f90 */
extern int DAT_0052ae28;
extern int DAT_0052aef8;
extern int DAT_0052ae30;
extern int DAT_0071f2eb;
extern int *DAT_00658020;
extern int DAT_0052ae54;
extern int *DAT_006572c0;
extern int DAT_0052ae50;
extern int DAT_0052ae2c;
extern int *DAT_00657c00;

void __stdcall FUN_004a6f90(void)
{
    int *piVar1;
    
    if (DAT_0052ae28 != 1) {
        if (1 < DAT_0052aef8) {
            DAT_0052aef8 = DAT_0052aef8 - 1;
            return;
        }
        if (DAT_0052aef8 < 1) {
            DAT_0052aef8 = 0;
            return;
        }
        DAT_0052aef8 = DAT_0052aef8 - 1;
        piVar1 = DAT_00657c00;
        if (DAT_0052ae30 != 0) {
            if (DAT_0071f2eb != 0) {
                piVar1 = DAT_00658020;
                if (DAT_0052ae54 != 0) {
                    piVar1 = DAT_006572c0;
                }
            }
        }
        ((void (__stdcall *)(int *, int))(*(int **)piVar1 + 0x80))(piVar1, DAT_0052ae50);
        if (DAT_0052ae2c == 0 && DAT_00657c00 != 0) {
            ((void (__stdcall *)(int *))(*(int **)DAT_00657c00 + 8))(DAT_00657c00);
            DAT_00657c00 = 0;
        }
    }
    return;
}
