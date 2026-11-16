/* SPDX-License-Identifier: MIT */
/* z_bop 0042dbd0 */
extern int DAT_00a10514;
extern int DAT_00a604d4;
extern int DAT_00a60534;
extern int __cdecl FUN_0042dda0(char *param_1);

int __cdecl FUN_0042dbd0(char *param_1)
{
    int iVar1;
    int iVar2;
    int iVar3;
    int *puVar4;
    unsigned int uStack_4;

    if (DAT_00a10514 == 0) {
        return 0;
    }
    if (*param_1 == '\0') {
        return 0;
    }
    iVar1 = FUN_0042dda0(param_1);
    if (iVar1 == -1) {
        return 0;
    }
    iVar3 = 0;
    puVar4 = &DAT_00a604d4;
    do {
        if (puVar4[-2] == iVar1) {
            iVar2 = (*(int (__stdcall **)(int *, unsigned int *))((char *)*puVar4 + 0x24))((int *)*puVar4, &uStack_4);
            if ((iVar2 == 0) && ((uStack_4 & 1) != 0 || (uStack_4 & 4) != 0)) {
                iVar3 = iVar3 + 1;
            }
        }
        puVar4 = puVar4 + 3;
    } while (puVar4 < &DAT_00a60534);
    return iVar3;
}
