/* SPDX-License-Identifier: MIT */
/* z_bop 004d67e0 */
extern int DAT_00aa6cfc;
extern unsigned char DAT_00aa7b13[4];   /* packed int at odd address */
extern unsigned char DAT_00aa7b1f[4];   /* packed int at odd address */

void __cdecl FUN_004d6950(int *param_1, int param_2, int param_3, unsigned short param_4);
void __cdecl FUN_004d6480(int *param_1, int *param_2);
void __cdecl FUN_004d64a0(int *param_1, int param_2, int param_3);
void __cdecl FUN_004d64e0(int *param_1);

void __cdecl FUN_004d67e0(int *param_1, int param_2, int param_3, unsigned short param_4, int param_5)
{
    int iVar1;
    unsigned int uVar2;
    unsigned char uVar3;
    unsigned int uVar4;
    unsigned char *puVar5;
    int local_28[4];   /* local_28, local_24, local_20, local_1c */
    unsigned char *local_8;
    int bpp;
    unsigned int fill32;

    if (param_5 == 0) {
        FUN_004d6950(param_1, param_2, param_3, param_4);
        return;
    }
    if ((*param_1 < param_1[2]) && (param_1[1] < param_1[3])) {
        FUN_004d6480(local_28, param_1);
        FUN_004d64a0(local_28, param_2, param_3);
        if ((local_28[0] < 0x281) && (((-1 < local_28[2] && (local_28[1] < 0x1e1)) && (-1 < local_28[3])))) {
            FUN_004d64e0(local_28);
            __builtin_memcpy(&iVar1, DAT_00aa7b13, 4);
            local_28[3] = (int)((unsigned int)local_28[3] + 1u);
            uVar4 = ((unsigned int)local_28[2] - (unsigned int)local_28[0]) + 1u;
            __builtin_memcpy(&bpp, DAT_00aa7b1f, 4);
            if (bpp == 8) {
                local_8 = (unsigned char *)((unsigned int)iVar1 * (unsigned int)local_28[1]
                                            + (unsigned int)local_28[0] + (unsigned int)DAT_00aa6cfc);
                if (local_28[1] < local_28[3]) {
                    local_28[3] = (int)((unsigned int)local_28[3] - (unsigned int)local_28[1]);
                    do {
                        uVar3 = (unsigned char)param_4;
                        fill32 = ((((unsigned int)uVar3 << 8) | (unsigned int)uVar3) << 16)
                                 | (((unsigned int)uVar3 << 8) | (unsigned int)uVar3);
                        puVar5 = local_8;
                        for (uVar2 = uVar4 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                            *(unsigned int *)puVar5 = fill32;
                            puVar5 = puVar5 + 4;
                        }
                        for (uVar2 = uVar4 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                            *puVar5 = uVar3;
                            puVar5 = puVar5 + 1;
                        }
                        local_8 = local_8 + iVar1;
                        local_28[3] = (int)((unsigned int)local_28[3] - 1u);
                    } while (local_28[3] != 0);
                    return;
                }
            }
            else {
                if (bpp != 0x10) {
                    return;
                }
                local_8 = (unsigned char *)((unsigned int)iVar1 * (unsigned int)local_28[1]
                                            + (unsigned int)local_28[0] * 2u + (unsigned int)DAT_00aa6cfc);
                if (local_28[1] < local_28[3]) {
                    local_28[3] = (int)((unsigned int)local_28[3] - (unsigned int)local_28[1]);
                    do {
                        uVar2 = uVar4;
                        puVar5 = local_8;
                        if (0 < (int)uVar4) {
                            for (; uVar2 != 0; uVar2 = uVar2 - 1) {
                                *(unsigned short *)puVar5 = param_4;
                                puVar5 = puVar5 + 2;
                            }
                        }
                        local_8 = local_8 + iVar1;
                        local_28[3] = (int)((unsigned int)local_28[3] - 1u);
                    } while (local_28[3] != 0);
                }
            }
        }
    }
    return;
}
