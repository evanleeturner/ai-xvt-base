/* SPDX-License-Identifier: MIT */
/* z_xvt 0040c4c0 */
extern int DAT_00525550;
extern char DAT_006db590;
extern short DAT_00740a80[];
extern char DAT_0085e79d;
extern char DAT_006d4a44;
extern char DAT_0073b65f[];
extern char DAT_0073b6df[];
extern short DAT_0073b54e[];
extern short DAT_0073b550[];
extern int DAT_00740880[];
extern char DAT_0073b660[];
extern char DAT_0073b6e0[];
extern char DAT_0073b4c0[];

int __cdecl FUN_0040c4c0(int param_1, char param_2, char param_3, char param_4, short param_5)
{
    unsigned int uVar1;

    if (DAT_00525550 != 0) {
        return 0;
    }
    if (DAT_006db590 == '\0') {
        return 0;
    }
    if (DAT_00740a80[param_1] == 0) {
        return 0;
    }
    if (DAT_0085e79d != '\0') {
        if ((unsigned char)DAT_006d4a44 == 0x80) {
            return 0;
        }
        if ((((DAT_006d4a44 != 0) && (param_2 == '\x02')) && (param_3 == '\x01')) &&
           (((param_4 == '\0' && (uVar1 = (unsigned int)DAT_006d4a44, DAT_0073b65f[uVar1] == '\x02')) &&
            ((DAT_0073b6df[uVar1] == '\x01' && (DAT_0073b54e[uVar1] == param_5)))))
           ) {
            return 0;
        }
        uVar1 = (unsigned int)DAT_006d4a44;
        DAT_0073b550[uVar1] = param_5;
        DAT_00740880[uVar1] = param_1;
        DAT_0073b660[uVar1] = param_2;
        DAT_0073b6e0[uVar1] = param_3;
        DAT_0073b4c0[uVar1] = param_4;
        DAT_006d4a44 = DAT_006d4a44 + 1;
        return 1;
    }
    return 0;
}
