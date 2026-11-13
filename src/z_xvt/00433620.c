/* SPDX-License-Identifier: MIT */
/* z_xvt 00433620 */
extern DWORD DAT_005251b0;
extern DWORD DAT_005251ac;
extern DWORD DAT_005251b4;
extern DWORD DAT_00739290;
extern DWORD DAT_0073379c;

DWORD __stdcall timeGetTime(void);

void __stdcall FUN_00433620(void)
{
    DWORD DVar1;
    DWORD *puVar2;
    unsigned int uVar3;

    uVar3 = 0;
    DAT_005251b0 = DAT_005251ac;
    DAT_005251b4 = 0;
    if (DAT_00739290 != 0) {
        puVar2 = &DAT_0073379c;
        do {
            uVar3 = uVar3 + 1;
            *puVar2 = puVar2[1];
            puVar2[2] = puVar2[3];
            puVar2[4] = puVar2[5];
            DVar1 = timeGetTime();
            puVar2[0x88] = DVar1;
            puVar2 = puVar2 + 0x8e;
        } while (uVar3 < DAT_00739290);
    }
    return;
}
