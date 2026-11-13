/* SPDX-License-Identifier: MIT */
/* z_xvt 004171d0 */
extern int __cdecl FUN_00461ff0(int);
extern void __stdcall FUN_004651f0(void);

extern char DAT_00523c39[];
extern char DAT_00523c38[];
extern unsigned short DAT_00656c84;
extern unsigned short DAT_00656c82;
extern unsigned int DAT_0052552c;
extern unsigned int DAT_00525534;
extern unsigned int DAT_0052550c;
extern unsigned short DAT_0073277c;
extern unsigned short DAT_005369e0[];

void __stdcall FUN_004171d0(void)
{
    unsigned short uVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned short *puVar4;
    unsigned int uVar5;
    unsigned short uVar6;
    short local_2;
    
    local_2 = 10;
    uVar5 = 0;
    uVar6 = 0;
    do {
        iVar2 = FUN_00461ff0((int)(char)DAT_00523c39[uVar5] + (unsigned int)DAT_00656c84);
        uVar3 = iVar2 + ((int)(char)DAT_00523c38[uVar5] + (unsigned int)DAT_00656c82) * 2;
        if ((DAT_0052552c != 0x13) && (DAT_00525534 == 0xa0000)) {
            uVar3 = uVar3 % DAT_0052550c;
            FUN_004651f0();
            FUN_004651f0();
        }
        uVar1 = DAT_0073277c;
        uVar5 += 2;
        puVar4 = (unsigned short *)(uVar3 + (DAT_00525534 & 0xfffffffe));
        DAT_005369e0[uVar6] = *puVar4;
        *puVar4 = uVar1;
        local_2--;
        uVar6++;
    } while (local_2 != 0);
    return;
}
