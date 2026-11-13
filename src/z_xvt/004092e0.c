/* SPDX-License-Identifier: MIT */
/* z_xvt 004092e0 */
extern short DAT_0071b718;
extern int DAT_006db598;
extern int DAT_00525518;
extern short DAT_0070acec;
extern short DAT_0070acea;
extern int DAT_00730884;
extern short DAT_0071e518;
extern short DAT_006da53e;
extern short DAT_00703ad4;
extern short DAT_00732a20;
extern short DAT_0071f2e0;
extern short DAT_0070acd4;

int __stdcall FUN_004092e0(void)
{
    DAT_0071b718 = DAT_0071b718 + 1;
    DAT_006db598 = DAT_006db598 + DAT_00525518;
    if (DAT_0071b718 != 0) {
        if (-1 < DAT_0071b718) {
            if (DAT_0070acec <= DAT_0071b718) {
                return 0;
            }
            goto LAB_00409305;
        }
        DAT_0070acea = DAT_0070acea + -1;
        DAT_0071e518 = DAT_0071e518 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2);
        if (-1 < DAT_0071e518) goto LAB_00409305;
    }
    DAT_0071e518 = 0;
LAB_00409305:
    DAT_006da53e = DAT_006da53e + 1;
    if (DAT_006da53e == 0) {
        DAT_00703ad4 = *(short *)(DAT_00730884 + 0xe) + -1;
        DAT_0070acea = *(short *)(DAT_00730884 + 0x1932) + -1;
        DAT_0071e518 = DAT_00732a20 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acea * 2);
        if (DAT_0071e518 < 0) {
            DAT_0071e518 = 0;
            return 1;
        }
    }
    else if (DAT_0070acec <= DAT_006da53e) {
        if (DAT_0070acec == DAT_006da53e) {
            DAT_00703ad4 = DAT_0071f2e0;
            DAT_0070acd4 = *(short *)(DAT_00730884 + 0x1932);
        }
        DAT_0070acd4 = DAT_0070acd4 + -1;
        DAT_00703ad4 = DAT_00703ad4 - *(short *)(DAT_00730884 + 0x1934 + DAT_0070acd4 * 2);
    }
    return 1;
}
