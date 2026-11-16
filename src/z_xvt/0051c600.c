/* SPDX-License-Identifier: MIT */
/* z_xvt 0051c600 */
extern unsigned char DAT_00536704;

short __cdecl FUN_0048f3c0(unsigned int param_1, unsigned int param_2);
void *__cdecl FUN_0048f4f0(unsigned int param_1);
void __cdecl FUN_0048f510(unsigned int param_1);
void __cdecl FUN_0051d220(short param_1);

unsigned int __cdecl FUN_0051c600(unsigned int *param_1)
{
    unsigned short sVar1;
    unsigned int *puVar2;
    unsigned int *puVar4;
    int iVar3;
    unsigned int tmp;
    unsigned char *pBytes;

    if (DAT_00536704 == 0) {
        return 0;
    }
    sVar1 = FUN_0048f3c0(0x4a, 2);
    if (sVar1 == 0) {
        return 0;
    }
    puVar2 = (unsigned int *)FUN_0048f4f0((unsigned int)sVar1);
    puVar4 = puVar2;
    for (iVar3 = 0x10; iVar3 != 0; iVar3 = iVar3 - 1) {
        *puVar4 = *param_1;
        param_1 = param_1 + 1;
        puVar4 = puVar4 + 1;
    }
    pBytes = (unsigned char *)puVar2;
    *(unsigned short *)(pBytes + 0x40) = 0;
    tmp = 0;
    __builtin_memcpy(pBytes + 0x42, &tmp, 4);
    *(unsigned char *)(pBytes + 0x46) = 0;
    *(unsigned char *)(pBytes + 0x47) = 0;
    *(unsigned short *)(pBytes + 0x48) = 0;
    FUN_0048f510((unsigned int)puVar2);
    FUN_0051d220(sVar1);
    return 1;
}
