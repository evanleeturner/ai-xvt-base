/* SPDX-License-Identifier: MIT */
/* z_bop 0043e110 */
extern char DAT_009e9670;
extern int DAT_009ecc34;
extern int DAT_009a1fe8;
extern int DAT_00a08374;

void __cdecl FUN_004a95c0(unsigned int param_1);
void FUN_004a9640(char param_1);
void FUN_00440840(int param_1, unsigned short param_2, int param_3);

void __stdcall FUN_0043e110(void)
{
    int iVar1;
    unsigned short uVar2;
    char cVar3;
    int local_ptr_a9;
    unsigned short local_ushort_bd;
    short local_short_bf;
    int base_ptr;
    int ptr_a9;

    base_ptr = *(int *)(*(int *)((int)&DAT_009e9670 + DAT_009ecc34 * 0x5bd) * 0x23 + 0x1f + DAT_009a1fe8);
    ptr_a9 = base_ptr + 0xa9;
    __builtin_memcpy(&local_ptr_a9, (void *)ptr_a9, 4);
    if ((*(unsigned char *)(local_ptr_a9 + 0x1e1) & 0x40) != 0) {
        FUN_004a95c0(0x40);
        iVar1 = local_ptr_a9;
        __builtin_memcpy(&local_ushort_bd, (void *)(iVar1 + 0xbd), 2);
        uVar2 = local_ushort_bd / 0x28f;
        __builtin_memcpy(&local_short_bf, (void *)(iVar1 + 0xbf), 2);
        if (local_short_bf == 0) {
            uVar2 = uVar2 * 2;
        }
        if (DAT_00a08374 == 0) {
            cVar3 = 2;
        }
        else {
            cVar3 = 0;
        }
        FUN_004a9640(cVar3);
        FUN_00440840(DAT_00a08374 + 0x29, uVar2, 1);
    }
    return;
}
