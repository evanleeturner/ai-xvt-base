/* SPDX-License-Identifier: MIT */
/* z_bop 0041bba0 */
extern int DAT_009a1fe8;

void __cdecl FUN_0041bb00(unsigned short param_1);

void __cdecl FUN_0041bba0(unsigned int param_1)
{
    int iVar1;
    unsigned int uVar2;
    int iVar3;
    unsigned short local_short;
    
    iVar1 = (param_1 & 0xffff) * 0x23;
    iVar3 = *(int *)(DAT_009a1fe8 + 0x1f + iVar1);
    if ((iVar3 != 0) && (uVar2 = (unsigned int)*(unsigned char *)(iVar3 + 0x12), uVar2 != 0)) {
        iVar3 = 0;
        do {
            iVar3 = iVar3 + 2;
            __builtin_memcpy(&local_short, (void *)(*(int *)(DAT_009a1fe8 + 0x1f + iVar1) + 0x51 + iVar3), 2);
            FUN_0041bb00(local_short);
            uVar2 = uVar2 - 1;
        } while (uVar2 != 0);
    }
    return;
}
