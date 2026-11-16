/* SPDX-License-Identifier: MIT */
/* z_bop 00430610 */
extern int DAT_00523410;
extern char DAT_009a8c10;
extern short DAT_00a0fe80;
extern char DAT_00bb737d;
extern unsigned char DAT_009a20a4;
extern char DAT_00a0aa1f;
extern char DAT_00a0aa9f;
extern short DAT_00a0a90e;
extern short DAT_00a0a910;
extern int DAT_00a0fc80;
extern char DAT_00a0aa20;
extern char DAT_00a0aaa0;
extern char DAT_00a0a880;

int __cdecl FUN_00430610(int param_1, char param_2, char param_3, char param_4, short param_5)
{
    unsigned int uVar1;

    if (DAT_00523410 != 0) {
        return 0;
    }
    if (DAT_009a8c10 == '\0') {
        return 0;
    }
    if (*(short *)((int)&DAT_00a0fe80 + param_1 * 2) == 0) {
        return 0;
    }
    if (DAT_00bb737d == '\0') {
        if (DAT_009a20a4 == 0x80) {
            return 0;
        }
        if ((((DAT_009a20a4 != 0) && (param_2 == '\x02')) && (param_3 == '\x01')) &&
            (((param_4 == '\0' && (uVar1 = (unsigned int)DAT_009a20a4, (&DAT_00a0aa1f)[uVar1] == '\x02')) &&
             (((&DAT_00a0aa9f)[uVar1] == '\x01' && (*(short *)(&DAT_00a0a90e + uVar1 * 2) == param_5)))))
           ) {
            return 0;
        }
        uVar1 = (unsigned int)DAT_009a20a4;
        (&DAT_00a0a910)[uVar1] = param_5;
        (&DAT_00a0fc80)[uVar1] = param_1;
        (&DAT_00a0aa20)[uVar1] = param_2;
        (&DAT_00a0aaa0)[uVar1] = param_3;
        (&DAT_00a0a880)[uVar1] = param_4;
        DAT_009a20a4 = DAT_009a20a4 + 1;
        return 1;
    }
    return 0;
}
