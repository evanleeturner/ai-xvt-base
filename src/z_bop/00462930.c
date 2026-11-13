/* SPDX-License-Identifier: MIT */
/* z_bop 00462930 */
extern unsigned short DAT_009a2000;
extern char DAT_009a20c0[];
extern char DAT_009d9192[];
extern unsigned short FUN_00432fa0(unsigned short, unsigned short, unsigned short);

unsigned int FUN_00462930(unsigned short param_1, unsigned short param_2, short param_3, unsigned short param_4, unsigned short param_5)
{
    unsigned short uVar4;
    unsigned int uVar3;
    unsigned short uVar1;
    unsigned short uVar2;
    unsigned int in_EAX = (unsigned int)DAT_009a2000;

    uVar4 = 0;
    if (DAT_009a2000 > 0) {
        do {
            uVar3 = (unsigned int)uVar4;
            if ((*(char *)((char *)&DAT_009a20c0 + uVar3 * 0x126 + 3) != '\0') ||
                (*(int *)((char *)&DAT_009d9192 + uVar3 * 0x566) != -1)) {
                uVar1 = FUN_00432fa0(uVar4, param_1, param_2);
                uVar2 = FUN_00432fa0(uVar4, param_4, param_5);
                if (param_3 == 1) {
                    uVar1 = uVar1 | uVar2;
                }
                else {
                    uVar1 = uVar1 & uVar2;
                }
                if (uVar1 != 0) {
                    if (*(char *)((char *)&DAT_009a20c0 + uVar3 * 0x126) == '\0') {
                        return 1;
                    }
                    if (*(char *)((char *)&DAT_009a20c0 + uVar3 * 0x126 + 1) != '\0') {
                        return 1;
                    }
                }
            }
            in_EAX = (unsigned int)DAT_009a2000;
            uVar4 = uVar4 + 1;
        } while ((int)(unsigned int)uVar4 < (int)in_EAX);
    }
    return in_EAX & 0xffff0000;
}
