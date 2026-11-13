/* SPDX-License-Identifier: MIT */
/* z_xvt 0040efa0 */
extern int DAT_006d4988;
extern int DAT_0052b08d;
extern uint FUN_004a7aa0(uint);

int __cdecl FUN_0040efa0(uint param_1)
{
    char *pcVar1;
    int iVar2;
    int iVar3;
    uint uVar4;
    uint uVar5;

    iVar2 = (param_1 & 0xffff) * 0x23;
    iVar3 = DAT_006d4988 + iVar2;
    __builtin_memcpy(&pcVar1, (void *)(iVar3 + 0x1f), sizeof(pcVar1));
    if (pcVar1 == NULL) {
        return 0;
    }
    if (*pcVar1 == '\0') {
        unsigned char b;
        __builtin_memcpy(&b, (void *)(iVar3 + 3), sizeof(b));
        uint arg = ((uint)iVar3 >> 8) | b;
        arg &= 0xffff00ff;
        uVar4 = FUN_004a7aa0(arg);

        unsigned short tmp;
        __builtin_memcpy(&tmp, (void *)((int)&DAT_0052b08d + (uVar4 & 0xffff) * 0xef), sizeof(tmp));
        uVar4 = (uint)tmp;

        __builtin_memcpy(&tmp, (void *)(pcVar1 + 0x79), sizeof(tmp));
        uVar5 = (uint)tmp;

        if (uVar4 < uVar5) {
            return uVar5 << 0xc;
        }
    }
    else {
        if (*pcVar1 != '\x01') {
            return 0;
        }
        int ptr;
        __builtin_memcpy(&ptr, (void *)(pcVar1 + 0xa3), sizeof(ptr));
        if (ptr != 0) {
            unsigned short tmp;
            __builtin_memcpy(&tmp, (void *)(ptr + 6), sizeof(tmp));
            return (uint)tmp << 0xc;
        }
        unsigned short tmp;
        __builtin_memcpy(&tmp, (void *)(pcVar1 + 0x79), sizeof(tmp));
        uVar4 = (uint)tmp;
    }
    return uVar4 << 0xc;
}
