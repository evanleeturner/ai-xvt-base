/* SPDX-License-Identifier: MIT */
/* z_bop 0043e6f0 */
extern char DAT_00a0a1e0[];
extern char DAT_00a08ca0[];
extern char DAT_00a08ca2[];
extern char DAT_00a08ca4[];
extern void (*DAT_009d80c4)(void *, ushort, short, int, int);
extern char DAT_009ed240[];

void __cdecl FUN_0043e6f0(ushort param_1, uint param_2, ushort param_3, short param_4)
{
    uint uVar3;
    ushort uVar6;
    uint iVar4;
    ushort uVar1;
    short sVar5;
    ushort uVar2;
    int bVar7;

    uVar3 = param_2 & 0xffff;
    if (*(ushort *)((char *)DAT_00a0a1e0 + uVar3 * 2) != param_1) {
        uVar6 = 0;
        *(ushort *)((char *)DAT_00a0a1e0 + uVar3 * 2) = param_1;
        iVar4 = uVar3 * 0xc;
        uVar1 = *(ushort *)((char *)DAT_00a08ca0 + iVar4);
        sVar5 = *(short *)((char *)DAT_00a08ca2 + iVar4);
        uVar2 = *(ushort *)((char *)DAT_00a08ca4 + iVar4);
        if (param_3 != 0) {
            do {
                bVar7 = uVar6 < param_1;
                uVar6 = uVar6 + 1;
                (*DAT_009d80c4)(*(void **)((char *)DAT_009ed240 + ((uint)bVar7 + (uint)uVar2) * 4), uVar1, sVar5, 0xfd, 0);
                sVar5 = sVar5 - param_4;
            } while (uVar6 < param_3);
        }
    }
    return;
}
