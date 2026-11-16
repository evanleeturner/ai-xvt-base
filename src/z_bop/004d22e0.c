/* SPDX-License-Identifier: MIT */
/* z_bop 004d22e0 */
extern char DAT_00aa7b0d;
extern void *DAT_00acebc5;
extern unsigned int DAT_00aa6cfc;

void __stdcall FUN_004d4970(void);
unsigned int __stdcall FUN_004d48e0(void);

bool __cdecl FUN_004d22e0(unsigned int param_1, unsigned int param_2, unsigned int param_3)
{
    char cVar1;
    int iVar2;
    unsigned int local_10;
    unsigned int local_c;
    unsigned int local_8;
    unsigned int local_4;

    cVar1 = DAT_00aa7b0d;
    FUN_004d4970();
    if (DAT_00acebc5 == (void *)0) {
        return 0;
    }
    local_c = 0;
    local_8 = param_3;
    local_4 = param_2;
    local_10 = 0x10;
    iVar2 = (*(int (__stdcall **)(void *, unsigned int, unsigned int *, unsigned int))((*(void **)DAT_00acebc5) + 0x78))(DAT_00acebc5, param_1, &local_10, 0);
    if (cVar1 != '\0') {
        DAT_00aa6cfc = FUN_004d48e0();
    }
    (void)local_c;
    (void)local_8;
    (void)local_4;
    return iVar2 == 0;
}
