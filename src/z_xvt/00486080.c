/* SPDX-License-Identifier: MIT */
/* z_xvt 00486080 */
uint __cdecl FUN_00462920(byte *param_1, int param_2, int param_3, uint param_4);

/*
 * Ghidra keeps local_30c, local_30b and local_30a as three separate byte
 * locals, and the original passes &local_30c to FUN_00462920 as a 3-byte
 * buffer. C cannot guarantee that three separate locals are contiguous, so one
 * 3-byte array backs them. The macros keep the decompiler's three names.
 */
#define local_30c local_30c_buf[0]
#define local_30b local_30c_buf[1]
#define local_30a local_30c_buf[2]

void __cdecl FUN_00486080(undefined1 *param_1, byte *param_2, int param_3, int param_4)
{
    undefined1 uVar1;
    uint uVar2;
    int iVar3;
    uint uVar4;
    int iVar5;
    short *psVar6;
    uint uVar7;
    byte *pbVar8;
    undefined1 *puVar9;
    byte local_30c_buf[3];
    short *local_308;
    undefined1 *local_304;
    byte local_300[768];

    local_300[0] = param_2[2] >> 3;
    local_300[1] = param_2[1] >> 3;
    uVar7 = (uint)param_4 * (uint)param_3;
    local_300[2] = *param_2 >> 3;
    *param_1 = 0;
    if (1 < uVar7) {
        local_308 = (short *)(uVar7 - 1);
        uVar4 = 1;
        puVar9 = param_1;
        do {
            puVar9 = puVar9 + 1;
            local_30c = param_2[5] >> 3;
            local_30b = param_2[4] >> 3;
            local_30a = param_2[3] >> 3;
            uVar2 = FUN_00462920(&local_30c, (int)local_300, 0, uVar4);
            uVar1 = (undefined1)uVar2;
            iVar5 = (int)((uVar2 & 0xff) * 3);
            uVar2 = uVar4;
            if ((((local_300[iVar5] != local_30c) || (local_300[iVar5 + 1] != local_30b)) ||
                 (local_300[iVar5 + 2] != local_30a)) && (uVar4 < 0x100)) {
                uVar1 = (undefined1)uVar4;
                uVar2 = uVar4 + 1;
                iVar5 = (int)((uVar4 & 0xff) * 3);
                local_300[iVar5] = local_30c;
                local_300[iVar5 + 1] = local_30b;
                local_300[iVar5 + 2] = local_30a;
            }
            *puVar9 = uVar1;
            local_308 = (short *)((int)local_308 + -1);
            uVar4 = uVar2;
            param_2 = param_2 + 3;
        } while (local_308 != (short *)0x0);
    }
    pbVar8 = local_300;
    local_304 = param_1 + uVar7;
    local_308 = (short *)(local_304 + 0x1000);
    do {
        uVar7 = 0;
        psVar6 = local_308;
        puVar9 = local_304;
        do {
            if (uVar7 < 8) {
                local_30c = (byte)((((uVar7 * *pbVar8) & 0xfffff8) * 0x10 + (uint)*pbVar8 * 0x80) >> 8);
                local_30b = (byte)((((uVar7 * pbVar8[1]) & 0xfffff8) * 0x10 + (uint)pbVar8[1] * 0x80) >> 8);
                iVar3 = (int)((uint)pbVar8[2] << 7);
                iVar5 = (int)(((uVar7 * pbVar8[2]) & 0xfffff8) << 4);
            }
            else {
                iVar5 = (int)(uVar7 - 8);
                local_30c = (byte)(((((0x1f - (uint)*pbVar8) * (uint)iVar5) & 0xfffff8) * 0x20 +
                                    (uint)*pbVar8 * 0x100) >> 8);
                local_30b = (byte)(((((0x1f - (uint)pbVar8[1]) * (uint)iVar5) & 0xfffff8) * 0x20 +
                                    (uint)pbVar8[1] * 0x100) >> 8);
                iVar3 = (int)((uint)pbVar8[2] << 8);
                iVar5 = (int)((((0x1f - (uint)pbVar8[2]) * (uint)iVar5) & 0xfffff8) << 5);
            }
            uVar7 = uVar7 + 1;
            local_30a = (byte)(((uint)iVar5 + (uint)iVar3) >> 8);
            *psVar6 = (short)(((ushort)local_30c * 0x20 + (ushort)local_30b) * 0x40 + (ushort)local_30a);
            local_30c = (byte)(local_30c * 2);
            local_30b = (byte)(local_30b * 2);
            local_30a = (byte)(local_30a * 2);
            uVar4 = FUN_00462920(&local_30c, 0x6da540, 0x40, 0x100);
            *puVar9 = (char)uVar4;
            psVar6 = psVar6 + 0x100;
            puVar9 = puVar9 + 0x100;
        } while (uVar7 < 0x10);
        local_308 = local_308 + 1;
        local_304 = local_304 + 1;
        pbVar8 = pbVar8 + 3;
    } while (pbVar8 < local_300 + 0x300);
    return;
}

#undef local_30c
#undef local_30b
#undef local_30a
