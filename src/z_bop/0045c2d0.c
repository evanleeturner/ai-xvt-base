/* SPDX-License-Identifier: MIT */
/* z_bop 0045c2d0 */
void __cdecl FUN_004ac6e0(uint param_1);

ushort DAT_009a2000;
ushort DAT_009d8120;
ushort DAT_009e8f60;

void __stdcall FUN_0045c2d0(void)
{
    ushort *puVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int local_4;
    
    iVar4 = 0;
    if (0 < DAT_009a2000) {
        puVar1 = &DAT_009d8120;
        do {
            iVar3 = 8;
            do {
                iVar2 = 3;
                do {
                    if (*puVar1 != 0) {
                        FUN_004ac6e0((uint)*puVar1);
                    }
                    puVar1 = puVar1 + 1;
                    iVar2 = iVar2 - 1;
                } while (iVar2 != 0);
                iVar3 = iVar3 - 1;
            } while (iVar3 != 0);
            iVar4 = iVar4 + 1;
        } while (iVar4 < DAT_009a2000);
    }
    local_4 = 10;
    puVar1 = &DAT_009e8f60;
    do {
        iVar4 = 7;
        do {
            iVar3 = 4;
            do {
                iVar2 = 3;
                do {
                    if (*puVar1 != 0) {
                        FUN_004ac6e0((uint)*puVar1);
                    }
                    puVar1 = puVar1 + 1;
                    iVar2 = iVar2 - 1;
                } while (iVar2 != 0);
                iVar3 = iVar3 - 1;
            } while (iVar3 != 0);
            iVar4 = iVar4 - 1;
        } while (iVar4 != 0);
        local_4 = local_4 - 1;
    } while (local_4 != 0);
    return;
}
