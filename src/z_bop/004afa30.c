/* SPDX-License-Identifier: MIT */
/* z_bop 004afa30 */
extern int *FUN_004ae1a0(int, int);

int __cdecl FUN_004afa30(int param_1)
{
    int iVar1;
    int *piVar2;
    int iVar3 = 0;
    int iVar4 = 0;
    int iVar5;
    int count;
    int *pArray;

    __builtin_memcpy(&count, (const char *)param_1 + 6, sizeof(int));
    if (0 < count) {
        __builtin_memcpy(&pArray, (const char *)param_1 + 10, sizeof(int *));
        iVar5 = 0;
        do {
            iVar1 = *(int *)((char *)pArray + iVar5);
            if (*(int *)(iVar1 + 4) != 0x14) {
                piVar2 = (int *)FUN_004ae1a0(iVar1, param_1);
                if ((piVar2 != (int *)0x0) && (*piVar2 == 7)) break;
                iVar3 = iVar3 + 1;
            }
            iVar5 = iVar5 + 4;
            iVar4 = iVar4 + 1;
            __builtin_memcpy(&count, (const char *)param_1 + 6, sizeof(int));
        } while (iVar4 < count);
    }
    __builtin_memcpy(&count, (const char *)param_1 + 6, sizeof(int));
    if (count <= iVar4) {
        iVar3 = -1;
    }
    return iVar3;
}
