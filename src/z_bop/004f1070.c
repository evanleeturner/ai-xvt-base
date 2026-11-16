/* SPDX-License-Identifier: MIT */
/* z_bop 004f1070 */
COLORREF GetPixel(HDC hdc, int X, int Y);
COLORREF SetPixel(HDC hdc, int X, int Y, COLORREF crColor);

uint __cdecl FUN_004f1070(int *param_1, COLORREF param_2)
{
    int iVar1;
    COLORREF color;
    uint uVar2;
    HDC local_70;
    COLORREF local_6c[9];
    uint *puStack_48 = 0;
    unsigned char bStack_18 = 0;
    
    uVar2 = 0xffffffff;
    color = local_6c[0];
    if ((param_2 != 0xffffffff) &&
        (iVar1 = (*(int (**)(int *, HDC *))(*param_1 + 0x44))(param_1, &local_70), color = local_6c[0], iVar1 == 0)) {
        color = GetPixel(local_70, 0, 0);
        SetPixel(local_70, 0, 0, param_2);
        (*(void (**)(int *, HDC))(*param_1 + 0x68))(param_1, local_70);
    }
    local_6c[0] = 0x6c;
    do {
        iVar1 = (*(int (**)(int *, int, COLORREF *, int, int))(*param_1 + 0x64))(param_1, 0, local_6c, 0, 0);
    } while (iVar1 == -0x7789fde4);
    if (iVar1 == 0) {
        uVar2 = ((1U << (bStack_18 & 0x1f)) - 1U) & *puStack_48;
        (*(void (**)(int *, int))(*param_1 + 0x80))(param_1, 0);
    }
    if ((param_2 != 0xffffffff) &&
        (iVar1 = (*(int (**)(int *, HDC *))(*param_1 + 0x44))(param_1, &local_70), iVar1 == 0)) {
        SetPixel(local_70, 0, 0, color);
        (*(void (**)(int *, HDC))(*param_1 + 0x68))(param_1, local_70);
    }
    return uVar2;
}
