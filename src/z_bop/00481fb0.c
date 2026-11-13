/* SPDX-License-Identifier: MIT */
/* z_bop 00481fb0 */
extern int DAT_009e9670;
extern int DAT_009e9a67;
extern int DAT_009e9a6b;
extern int DAT_009e9a6d;
extern int DAT_009e9a6f;
extern int DAT_009e9a73;
extern int DAT_009e9a75;
extern int DAT_009e9a7a;
extern int DAT_009e9a7c;
extern int DAT_009e9a7e;
extern int DAT_009e9a80;
extern int DAT_009e9a84;
extern int DAT_009e9a8a;
extern int DAT_009e9a8c;
extern int DAT_009e9b04;
extern int DAT_009e9b7c;

void __cdecl FUN_00427720(unsigned int param_1, int param_2);

void __cdecl FUN_00481fb0(int param_1)
{
    int iVar1;
    int iVar2;
    unsigned short uVar3;
    unsigned short uVar4;
    unsigned int uVar5;

    iVar2 = param_1 * 0x5bd;
    if (*(short *)((int)&DAT_009e9a84 + iVar2) != 0) {
        if (*(short *)((int)&DAT_009e9a8a + iVar2) == 0) {
            uVar5 = 0x12;
        }
        else {
            uVar5 = 0x14;
        }
        FUN_00427720(uVar5, param_1);
        uVar3 = 0;
        do {
            uVar4 = uVar3 + 1;
            *(unsigned short *)((int)&DAT_009e9a8c + (unsigned int)uVar3 * 2 + iVar2) =
                 *(unsigned short *)((int)&DAT_009e9a6f + iVar2);
            iVar1 = iVar2 + (unsigned int)uVar3 * 2;
            *(unsigned short *)((int)&DAT_009e9b04 + iVar1) = *(unsigned short *)((int)&DAT_009e9a6b + iVar2);
            *(unsigned short *)((int)&DAT_009e9b7c + iVar1) = *(unsigned short *)((int)&DAT_009e9a6d + iVar2);
            uVar3 = uVar4;
        } while (uVar4 < 0x3c);
        return;
    }
    *(unsigned short *)((int)&DAT_009e9a80 + iVar2) = 0;
    if ((unsigned int)*(unsigned short *)((int)&DAT_009e9a67 + iVar2) == *(unsigned int *)((int)&DAT_009e9670 + iVar2)) {
        *(unsigned short *)((int)&DAT_009e9a73 + iVar2) = *(unsigned short *)((int)&DAT_009e9a7c + iVar2);
        *(unsigned short *)((int)&DAT_009e9a75 + iVar2) = *(unsigned short *)((int)&DAT_009e9a7e + iVar2);
        FUN_00427720((unsigned int)*(unsigned char *)((int)&DAT_009e9a7a + iVar2), param_1);
        return;
    }
    *(unsigned short *)((int)&DAT_009e9a73 + iVar2) = 0;
    *(unsigned short *)((int)&DAT_009e9a75 + iVar2) = 0;
    FUN_00427720(0x12, param_1);
    *(unsigned short *)((int)&DAT_009e9a84 + iVar2) = 1;
    return;
}
