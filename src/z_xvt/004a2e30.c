/* SPDX-License-Identifier: MIT */
/* z_xvt 004a2e30 */
extern short DAT_006d9990;
extern unsigned char DAT_006d999c;
extern short DAT_00645ae0;
extern unsigned char DAT_00645aec;
extern short DAT_00645b38;
extern unsigned char DAT_00645b44;

void __stdcall FUN_004a2e30(void)
{
    if (DAT_006d9990 != -1) {
        DAT_006d999c = (unsigned char)(DAT_006d999c + 1);
    }
    if (DAT_00645ae0 != -1) {
        DAT_00645aec = (unsigned char)(DAT_00645aec + 1);
    }
    if (DAT_00645b38 != -1) {
        DAT_00645b44 = (unsigned char)(DAT_00645b44 + 1);
    }
    return;
}
