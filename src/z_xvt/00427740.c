/* SPDX-License-Identifier: MIT */
/* z_xvt 00427740 */
void *FID_conflict___expand(void *ptr, int size);

int __cdecl FUN_00427740(int param_1)
{
    int iVar1;
    void *pvVar2;
    unsigned int uVar3;
    int *puVar4;
    int *puVar5;
    int *puVar6;
    int field_108e;
    void *field_1086;

    iVar1 = *(int *)(param_1 + 0x1034) * *(int *)(param_1 + 0x1030);
    __builtin_memcpy(&field_108e, (void *)(param_1 + 0x108e), 4);
    if (field_108e == iVar1) {
        return 1;
    }
    __builtin_memcpy(&field_1086, (void *)(param_1 + 0x1086), 4);
    pvVar2 = FID_conflict___expand(field_1086, iVar1 * 6);
    if (pvVar2 != (void *)0) {
        __builtin_memcpy((void *)(param_1 + 0x1086), &pvVar2, 4);
        puVar6 = (int *)((int)pvVar2 + field_108e * 6 - 6);
        puVar5 = (int *)((int)pvVar2 + *(int *)(param_1 + 0x1034) * *(int *)(param_1 + 0x1030) * 6 - 6);
        uVar3 = 0;
        if (field_108e != 0) {
            do {
                iVar1 = *(unsigned char *)((int)puVar6 + 3) + 1;
                puVar4 = puVar5;
                do {
                    puVar5 = (int *)((int)puVar4 - 6);
                    iVar1--;
                    *puVar4 = *puVar6;
                    *(unsigned short *)(puVar4 + 1) = *(unsigned short *)(puVar6 + 1);
                    *(unsigned char *)((int)puVar4 + 3) = 0;
                    puVar4 = puVar5;
                } while (iVar1 != 0);
                puVar6 = (int *)((int)puVar6 - 6);
                uVar3++;
            } while (uVar3 < (unsigned int)field_108e);
        }
        iVar1 = *(int *)(param_1 + 0x1034) * *(int *)(param_1 + 0x1030);
        __builtin_memcpy((void *)(param_1 + 0x108e), &iVar1, 4);
        return 1;
    }
    return 0;
}
