/* SPDX-License-Identifier: MIT */
/* z_xvt 004b67f9 */
extern undefined4 *DAT_0052f1e8;
extern undefined4 DAT_0052f1ec;
extern undefined4 _DAT_0052f1e4;
extern undefined4 DAT_0052f1e0;
extern undefined4 DAT_00742820;
void __stdcall FUN_0045bbf0(void);

void __stdcall FUN_004b67f9(void)
{
    undefined4 *puVar1;
    undefined4 *local_8;
    
    local_8 = DAT_0052f1e8;
    while (local_8 != (undefined4 *)0x0) {
        if (local_8[6] != 0) {
            ((void (*)(void *))(*(void **)local_8[6] + 8))((void *)local_8[6]);
            local_8[6] = 0;
        }
        if (local_8[5] != 0) {
            ((void (*)(void *))(*(void **)local_8[5] + 8))((void *)local_8[5]);
            local_8[5] = 0;
        }
        local_8[2] = 0;
        DAT_00742820 = DAT_00742820 + local_8[4];
        FUN_0045bbf0();
        puVar1 = (undefined4 *)local_8[1];
        *local_8 = 0;
        local_8[1] = 0;
        local_8 = puVar1;
    }
    DAT_0052f1e8 = (undefined4 *)0x0;
    DAT_0052f1ec = 0;
    _DAT_0052f1e4 = 0;
    DAT_0052f1e0 = 1;
    return;
}
