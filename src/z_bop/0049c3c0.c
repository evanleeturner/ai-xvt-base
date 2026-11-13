/* SPDX-License-Identifier: MIT */
/* z_bop 0049c3c0 */
extern void *DAT_00a07bd8;
extern void *DAT_00523434;

void __cdecl FUN_0049e720(char *, void *, int, int);
void __cdecl FUN_0049eb60(int);
unsigned int __cdecl _fread(void *, unsigned int, unsigned int, void *);
void __cdecl FUN_0049e9c0(int);

short __cdecl FUN_0049c3c0(char *param_1, undefined1 *param_2)
{
    void *_File;
    ushort uVar1;
    uint uVar2;
    ushort uVar3;
    ushort uVar4;
    undefined1 *puVar5;
    short sVar6;
    undefined1 local_200[512];

    FUN_0049e720(param_1, &DAT_00523434, 1, 0);
    _File = DAT_00a07bd8;
    if (DAT_00a07bd8 == (void *)0x0) {
        FUN_0049eb60(1);
    }
    sVar6 = 0;
    uVar2 = 0x200;
    do {
        uVar2 = _fread(local_200, 1, uVar2 & 0xffff, _File);
        uVar1 = (ushort)uVar2;
        puVar5 = param_2;
        uVar3 = 0;
        if (uVar1 != 0) {
            do {
                param_2 = puVar5 + 1;
                uVar4 = uVar3 + 1;
                *puVar5 = local_200[uVar3];
                puVar5 = param_2;
                uVar3 = uVar4;
            } while (uVar4 < uVar1);
        }
        sVar6 = sVar6 + uVar1;
    } while (uVar1 == 0x200);
    FUN_0049e9c0(0);
    return sVar6;
}
