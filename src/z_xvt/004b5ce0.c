/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5ce0 */
extern unsigned int *DAT_0064ee60;
extern unsigned char *DAT_0064ef40;
extern unsigned int DAT_0052f204;

unsigned int __stdcall FUN_004b5ce0(void) {
    DAT_0064ef40 = (unsigned char *)DAT_0064ee60;
    *(unsigned char *)DAT_0064ee60 = 9;
    *(unsigned char *)((unsigned int)DAT_0064ee60 + 1) = 0x10;
    *(unsigned short *)((unsigned int)DAT_0064ee60 + 2) = 1;
    DAT_0064ee60 = (unsigned int *)((unsigned int)DAT_0064ee60 + 4);
    *DAT_0064ee60 = 2;
    *(unsigned short *)(DAT_0064ee60 + 1) = 0;
    *(unsigned short *)((unsigned int)DAT_0064ee60 + 6) = 0;
    DAT_0064ee60[2] = DAT_0052f204;
    DAT_0064ee60[3] = 0;
    DAT_0064ee60 = DAT_0064ee60 + 4;
    return 1;
}
