/* SPDX-License-Identifier: MIT */
/* z_xvt 004df030 */
extern char DAT_00758ee8;
extern char DAT_00758ee9;

char __stdcall FUN_004df030(void)
{
    char cVar1;

    cVar1 = (char)(DAT_00758ee8 != '\0');
    if (DAT_00758ee9 != '\0') {
        cVar1 = (char)(cVar1 + '\x01');
    }
    return cVar1;
}
