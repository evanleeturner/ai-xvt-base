/* SPDX-License-Identifier: MIT */
/* z_bop 004ee8e0 */
extern int DAT_00aa6108;
extern int DAT_00aa60f8;
extern int DAT_00aa6104;
extern int DAT_00aa6100;
extern unsigned char DAT_00a92202[];
extern unsigned char DAT_00a92203[];
extern unsigned char DAT_00a91d12[];
extern unsigned int DAT_0052c564[];
extern unsigned int DAT_0052c578[];
extern int DAT_00aa5e10[];
extern int DAT_00aa5cb0[];
extern int DAT_00aa5ac0[];
extern unsigned char DAT_00aa615e[];   /* packed record table, stride 0x2a */
extern unsigned char DAT_00aa6166[];
extern unsigned char DAT_00aa616a[];

unsigned int __cdecl FUN_004ee8e0(int param_1)
{
    unsigned char bVar1;
    unsigned int uVar2;
    int *piVar3;
    int iVar4;
    int iVar5;
    int *piVar6;
    int *local_8;
    int local_4;
    int tmp;

    if (param_1 == -1) {
        if (DAT_00aa6108 == 0) {
            if (DAT_00aa60f8 != 0) {
                return (unsigned int)DAT_00a92202[(unsigned int)DAT_00aa60f8 + (unsigned int)DAT_00aa6104 * 0x562u];
            }
            return (unsigned int)DAT_00a91d12[(unsigned int)DAT_00aa6104 * 0x562u];
        }
        if (DAT_00aa6100 == 0) {
            return (unsigned int)DAT_00a91d12[(unsigned int)DAT_00aa6104 * 0x562u];
        }
        bVar1 = DAT_00a92202[(unsigned int)DAT_00aa6104 * 0x562u];
        uVar2 = (unsigned int)DAT_00aa6100;
        if (bVar1 != 0) {
            if (bVar1 < 3) {
                return DAT_0052c564[DAT_00aa6100];
            }
            if (bVar1 == 3) {
                return DAT_0052c578[DAT_00aa6100];
            }
            return (unsigned int)DAT_00aa6100;
        }
    } else {
        iVar4 = 0;
        piVar6 = DAT_00aa5e10;
        local_8 = DAT_00aa5cb0;
        do {
            iVar5 = 0;
            if (0 < *piVar6) {
                piVar3 = local_8;
                do {
                    __builtin_memcpy(&tmp, &DAT_00aa615e[(unsigned int)param_1 * 0x2au], 4);
                    if (*piVar3 == tmp) {
                        local_4 = DAT_00aa5ac0[iVar5 + iVar4];
                        break;
                    }
                    piVar3 = piVar3 + 1;
                    iVar5 = iVar5 + 1;
                } while (iVar5 < *piVar6);
            }
            iVar4 = iVar4 + 8;
            piVar6 = piVar6 + 1;
            local_8 = local_8 + 8;
        } while (piVar6 < &DAT_00aa5e10[10]);
        __builtin_memcpy(&uVar2, &DAT_00aa6166[(unsigned int)param_1 * 0x2au], 4);
        if (uVar2 == 0) {
            __builtin_memcpy(&uVar2, &DAT_00aa616a[(unsigned int)param_1 * 0x2au], 4);
            if ((uVar2 != 0xffffffffu) && (uVar2 < 10)) {
                return (unsigned int)DAT_00a92203[uVar2 + (unsigned int)local_4 * 0x562u];
            }
            uVar2 = (unsigned int)DAT_00a91d12[(unsigned int)local_4 * 0x562u];
        }
    }
    return uVar2;
}
