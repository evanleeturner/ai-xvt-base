/* SPDX-License-Identifier: MIT */
/* z_xvt 0049d810 */
unsigned int __cdecl FUN_0049d8b0(unsigned short, unsigned short, unsigned short, unsigned short, short, unsigned short);

unsigned int __cdecl FUN_0049d810(unsigned char *param_1, short param_2)
{
    unsigned int uVar2;
    unsigned int uVar3;

    if (param_1[4] == 0x27) {
        param_1[5] = 0xc;
    }

    uVar2 = FUN_0049d8b0((unsigned short)param_1[0],
                         (unsigned short)param_1[1],
                         (unsigned short)param_1[2],
                         (unsigned short)param_1[3],
                         param_2,
                         10) & 0xffff;

    if (param_1[4] == 0x27) {
        uVar3 = 0;
    }
    else {
        uVar3 = FUN_0049d8b0((unsigned short)param_1[4],
                             (unsigned short)param_1[5],
                             (unsigned short)param_1[6],
                             (unsigned short)param_1[7],
                             param_2,
                             10) & 0xffff;
    }

    if (param_1[10] == 1) {
        return uVar3 | uVar2;
    }
    return uVar3 & uVar2;
}
