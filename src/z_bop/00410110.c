/* SPDX-License-Identifier: MIT */
/* z_bop 00410110 */
extern short DAT_00a08108;
extern short DAT_009d8c00;
extern short DAT_0054f9ac;
extern short DAT_0054f9b4;
extern short DAT_00a08102;
extern unsigned char DAT_009a20ae;
extern short DAT_009a6fe0;
extern short DAT_00a0813c;
extern short DAT_0054f9b8;
extern short DAT_0054f9a8;
extern short DAT_00a07ce4;

void __cdecl FUN_0040fa30(int);

void __stdcall FUN_00410110(void)
{
    unsigned int uVar1;

    if ((short)DAT_00a08108 < (short)DAT_009d8c00) {
        DAT_0054f9ac = DAT_009d8c00;
        DAT_0054f9b4 = DAT_00a08108;
        DAT_0054f9b8 = DAT_00a08102;
        uVar1 = (unsigned int)DAT_009a20ae + (unsigned int)DAT_00a08102;
        if ((int)(short)DAT_00a08108 < (int)(short)DAT_009a6fe0) {
            DAT_0054f9b4 = DAT_009a6fe0;
        }
        if ((int)(short)DAT_00a08102 < (int)(short)DAT_00a0813c) {
            DAT_0054f9b8 = DAT_00a0813c;
        }
        DAT_0054f9a8 = (unsigned short)uVar1;
        if ((int)(short)DAT_00a07ce4 < (int)(uVar1 & 0xffff)) {
            DAT_0054f9a8 = DAT_00a07ce4;
        }
        if ((unsigned short)DAT_0054f9b8 < (unsigned short)DAT_0054f9a8) {
            FUN_0040fa30(0);
        }
    }
    return;
}
