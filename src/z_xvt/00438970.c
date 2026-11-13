/* SPDX-License-Identifier: MIT */
/* z_xvt 00438970 */
extern char DAT_0071b740;
extern char DAT_0071e528;
extern int DAT_0072f444;

void __stdcall FUN_00438970(void)
{
    char *pcVar1;

    DAT_0072f444 = 0;
    pcVar1 = &DAT_0071b740;
    do {
        if ((*pcVar1 == '\x01') || (*pcVar1 == '\x02')) {
            DAT_0072f444 = (int)((unsigned int)DAT_0072f444 + 1u);
        }
        pcVar1 = pcVar1 + 0x5bd;
    } while (pcVar1 < &DAT_0071e528);
    return;
}
