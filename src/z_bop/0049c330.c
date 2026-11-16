/* SPDX-License-Identifier: MIT */
/* z_bop 0049c330 */
extern int DAT_00523434;
extern void *DAT_00a07bd8;

int __cdecl FUN_0049e720(char *param_1, void *param_2, int param_3, int param_4);
int __cdecl _fgetc(void *stream);
void __cdecl FUN_0049e9c0(int param_1);

int __cdecl FUN_0049c330(char *param_1, unsigned char *param_2, int param_3)
{
    int *piVar1;
    unsigned int uVar2;
    void *_File;
    int iVar3;
    short sVar4;

    iVar3 = FUN_0049e720(param_1, &DAT_00523434, 1, 0);
    _File = DAT_00a07bd8;
    if (iVar3 != 0) {
        sVar4 = 0;
        piVar1 = (int *)((char *)DAT_00a07bd8 + 0xc);
        uVar2 = *piVar1;
        while ((uVar2 & 0x10) == 0) {
            *(unsigned char **)(param_3 + sVar4 * 4) = param_2;
            iVar3 = _fgetc(_File);
            uVar2 = *piVar1;
            while (((uVar2 & 0x10) == 0) && ((short)iVar3 != 0xff)) {
                *param_2 = (char)iVar3;
                param_2 = param_2 + 1;
                iVar3 = _fgetc(_File);
                uVar2 = *piVar1;
            }
            sVar4 = sVar4 + 1;
            *param_2 = 0xff;
            param_2 = param_2 + 1;
            uVar2 = *piVar1;
        }
        FUN_0049e9c0(0);
        return (int)sVar4;
    }
    return 0;
}
