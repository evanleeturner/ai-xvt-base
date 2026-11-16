/* SPDX-License-Identifier: MIT */
/* z_bop 0045c630 */
extern ushort DAT_009a73c0;
extern char DAT_009d8cd8[];
extern char DAT_009d8cd9[];
extern char DAT_009d8cda[];
extern char DAT_009d8cdb[];
extern char DAT_009d8cdc[];
extern char DAT_009d8cde[];
extern char DAT_009d8cdf[];
extern char DAT_009d8ce0[];
extern char DAT_009d8ce1[];
extern char DAT_009d8ce2[];

extern uint __cdecl FUN_0045c720(ushort, ushort, ushort, ushort, ushort);

void __cdecl FUN_0045c630(uint param_1)
{
    uint local_1;
    uint local_2;
    uint uVar3;

    local_1 = (param_1 & 0xffff) * 0x52;
    local_2 = local_1 + (uint)DAT_009a73c0 * 0x566;
    uVar3 = FUN_0045c720((ushort)DAT_009d8cde[local_2], (ushort)DAT_009d8cdf[local_2],
                         (ushort)DAT_009d8ce2[local_2], (ushort)DAT_009d8ce0[local_2],
                         (ushort)DAT_009d8ce1[local_2]);
    if ((short)uVar3 == -1) {
        local_2 = local_1 + (uint)DAT_009a73c0 * 0x566;
        FUN_0045c720((ushort)DAT_009d8cd8[local_2], (ushort)DAT_009d8cda[local_2],
                     (ushort)DAT_009d8cdc[local_2], (ushort)DAT_009d8cd9[local_2],
                     (ushort)DAT_009d8cdb[local_2]);
    }
    return;
}
