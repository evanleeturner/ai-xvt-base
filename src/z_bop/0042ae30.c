/* SPDX-License-Identifier: MIT */
/* z_bop 0042ae30 */
extern float _DAT_00555d48;
extern float _DAT_00555d4c;
extern float _DAT_00555d50;
extern float _DAT_00555d54;
extern float _DAT_00555d68;
extern float _DAT_00555d6c;

void __cdecl FUN_0042ad10(void *, void *);

float __cdecl FUN_0042ae30(int param_1, int param_2)
{
    int iVar3 = 0;
    _DAT_00555d50 = 0.0f;
    _DAT_00555d54 = 0.0f;
    _DAT_00555d68 = 0.0f;
    _DAT_00555d6c = 0.0f;
    _DAT_00555d48 = 0.0f;
    _DAT_00555d4c = 0.0f;
    
    if (*(int *)(param_1 + 6) > 0) {
        int iVar2 = 0;
        do {
            int *puVar1 = (int *)(*(int *)(param_1 + 10) + iVar2);
            iVar2 = iVar2 + 4;
            iVar3 = iVar3 + 1;
            FUN_0042ad10((void *)*puVar1, (void *)param_1);
        } while (iVar3 < *(int *)(param_1 + 6));
    }
    
    _DAT_00555d50 = _DAT_00555d50 - _DAT_00555d54;
    _DAT_00555d68 = _DAT_00555d68 - _DAT_00555d6c;
    _DAT_00555d48 = _DAT_00555d48 - _DAT_00555d4c;
    
    if (param_2 != 0) {
        float local_8 = 0.0f;
        if (param_2 == 1) {
            local_8 = _DAT_00555d50;
        }
        if (param_2 == 2) {
            local_8 = _DAT_00555d68;
        }
        if (param_2 == 3) {
            local_8 = _DAT_00555d48;
        }
        return local_8;
    }
    
    if ((_DAT_00555d68 < _DAT_00555d50) && (_DAT_00555d48 < _DAT_00555d50)) {
        return _DAT_00555d50;
    }
    if ((_DAT_00555d50 < _DAT_00555d68) && (_DAT_00555d48 < _DAT_00555d68)) {
        return _DAT_00555d68;
    }
    return _DAT_00555d48;
}
