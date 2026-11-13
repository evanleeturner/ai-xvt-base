/* SPDX-License-Identifier: MIT */
/* z_xvt 00405fd0 */
extern int DAT_0052552c;
extern unsigned char DAT_006da3d0;
extern int DAT_00525518;
extern unsigned short DAT_00732fe0;
extern unsigned char DAT_00525538;

void __cdecl FUN_00406040(unsigned int param_1);

void __cdecl FUN_00405fd0(unsigned char param_1)
{
    if (DAT_0052552c < 0x102) {
        if (DAT_0052552c != 0x101) {
            DAT_006da3d0 = 0;
            goto LAB_00406003;
        }
    } else {
        if (DAT_0052552c == 0x111) {
            DAT_006da3d0 = 2;
            goto LAB_00406003;
        }
        if (DAT_0052552c != 0x112) {
            DAT_006da3d0 = 0;
            goto LAB_00406003;
        }
    }
    DAT_006da3d0 = 1;
LAB_00406003:
    if (DAT_00525518 == 2) {
        DAT_006da3d0 = 2;
    }
    DAT_00732fe0 = 1;
    FUN_00406040((unsigned int)DAT_006da3d0);
    DAT_00525538 = param_1;
    return;
}
