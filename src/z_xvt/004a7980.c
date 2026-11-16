/* SPDX-License-Identifier: MIT */
/* z_xvt 004a7980 */
extern undefined4 DAT_00645bcc;

void __cdecl FUN_004a7980(undefined4 param_1, undefined4 *param_2, undefined4 param_3, int *param_4)
{
    int iVar1;
    undefined4 *puVar2;
    undefined4 uStack_6c;
    undefined4 uStack_68;
    undefined4 *puStack_64;
    undefined4 uStack_60;
    undefined4 local_54[0x15];
    int local_44;
    undefined4 local_38;
    undefined4 local_34;

    puVar2 = local_54;
    for (iVar1 = 0x15; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    local_54[3] = param_3;
    local_44 = *param_4;
    uStack_60 = 0;
    local_34 = *param_2;
    local_38 = param_1;
    local_54[0] = 0x54;
    puStack_64 = local_54;
    uStack_68 = DAT_00645bcc;
    uStack_6c = 0x4a79d2;
    acmStreamPrepareHeader((HACMSTREAM)uStack_68, (LPACMSTREAMHEADER)&uStack_6c, 0);
    uStack_6c = 0;
    acmStreamConvert((HACMSTREAM)uStack_68, (LPACMSTREAMHEADER)&uStack_60, 0);
    acmStreamUnprepareHeader((HACMSTREAM)uStack_68, (LPACMSTREAMHEADER)&uStack_6c, 0);
    *param_4 = (int)puStack_64;
    *param_2 = local_54[0];
    (void)local_44;
    (void)local_38;
    (void)local_34;
    return;
}
