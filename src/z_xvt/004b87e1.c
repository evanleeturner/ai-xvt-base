/* SPDX-License-Identifier: MIT */
/* z_xvt 004b87e1 */
extern int *DAT_0052f1e8;
extern int *DAT_0052f1ec;
extern int _DAT_0052f1e4;
extern int DAT_00742820;

void __cdecl FUN_004b87e1(int *param_1)
{
    if (param_1 == DAT_0052f1e8) {
        DAT_0052f1e8 = (int *)param_1[1];
        if (DAT_0052f1e8 == (int *)0) {
            DAT_0052f1ec = (int *)0;
        }
        else {
            *DAT_0052f1e8 = 0;
            if (DAT_0052f1e8[1] == 0) {
                DAT_0052f1ec = DAT_0052f1e8;
            }
        }
    }
    else if (param_1 == DAT_0052f1ec) {
        DAT_0052f1ec = (int *)*param_1;
        DAT_0052f1ec[1] = 0;
    }
    else {
        *(int *)(*param_1 + 4) = param_1[1];
        *(int *)param_1[1] = *param_1;
    }
    _DAT_0052f1e4 = _DAT_0052f1e4 - 1;
    DAT_00742820 = DAT_00742820 + param_1[4];
    return;
}
