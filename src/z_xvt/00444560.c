/* SPDX-License-Identifier: MIT */
/* z_xvt 00444560 */
extern char DAT_0071b740;
extern char DAT_0071e528;

void FUN_004443d0(int);

void __stdcall FUN_00444560(void)
{
    int iVar2 = 0;
    char *pcVar1 = &DAT_0071b740;
    
    do {
        if (*pcVar1 != '\0') {
            FUN_004443d0(iVar2);
        }
        pcVar1 = pcVar1 + 0x5bd;
        iVar2 = iVar2 + 1;
    } while (pcVar1 < &DAT_0071e528);
    
    return;
}
