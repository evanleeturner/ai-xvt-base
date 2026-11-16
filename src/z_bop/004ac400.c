/* SPDX-License-Identifier: MIT */
/* z_bop 004ac400 */
int __stdcall FUN_004ac460(void);
void __cdecl FUN_004aafe0(void);

void __cdecl FUN_004ac400(int *param_1)
{
    int local_64[20];
    int iVar2;
    int bVar1;
    
    local_64[0] = 100;
    do {
        iVar2 = ((int (__stdcall *)(int *, int, int, int, int, int *))(*(int **)param_1 + 0x14))(param_1, 0, 0, 0, 0x400, local_64);
        if (iVar2 == 0) {
            return;
        }
        if (iVar2 == (int)0x887601C2) {
            bVar1 = FUN_004ac460();
            if (bVar1 == 0) {
                return;
            }
            FUN_004aafe0();
        }
    } while (iVar2 == (int)0x8876021C);
    return;
}
