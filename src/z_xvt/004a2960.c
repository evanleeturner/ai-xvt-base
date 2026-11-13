/* SPDX-License-Identifier: MIT */
/* z_xvt 004a2960 */
void __cdecl FUN_0048ee10(short param_1);
void __cdecl FUN_004a2e70(void);
void __cdecl FUN_0048ee20(char param_1);

extern unsigned short DAT_006559b4;
extern void (*DAT_006d9984)(unsigned int);
extern unsigned int DAT_0071ecf4;
extern unsigned short DAT_0070ac46;
extern unsigned short DAT_0070ac48;
extern unsigned int DAT_0070ac40;
extern unsigned char DAT_0070a178;

void __cdecl FUN_004a2960(short param_1, char param_2)
{
    if (DAT_006559b4 == 0) {
        if (param_2 != '?' && param_2 != '!' && param_2 != ':' && param_2 != ' ' && param_2 != '.') {
            (*DAT_006d9984)(0x2e);
        }
        FUN_0048ee10(1);
        (*DAT_006d9984)(10);
        FUN_0048ee10(0);
    }
    if (param_1 == 3 || param_1 == 7) {
        *(unsigned short *)((unsigned char *)&DAT_0070ac46 + DAT_0071ecf4 * 0x10) = 0x1d8;
    }
    else if (param_1 == 8) {
        *(unsigned short *)((unsigned char *)&DAT_0070ac48 + DAT_0071ecf4 * 0x10) = 0x760;
    }
    else if (param_1 == 4) {
        *(unsigned short *)((unsigned char *)&DAT_0070ac46 + DAT_0071ecf4 * 0x10) = 0x760;
    }
    else if (DAT_0070a178 == '\0') {
        if (param_1 == 2 || param_1 == 1) {
            *(unsigned int *)((unsigned char *)&DAT_0070ac40 + DAT_0071ecf4 * 8) = 0x588;
        }
        else {
            *(unsigned int *)((unsigned char *)&DAT_0070ac40 + DAT_0071ecf4 * 8) = 0x674;
        }
    }
    else {
        *(unsigned int *)((unsigned char *)&DAT_0070ac40 + DAT_0071ecf4 * 8) = 0x162;
    }
    FUN_004a2e70();
    FUN_0048ee20('\x02');
}
