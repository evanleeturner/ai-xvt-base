/* SPDX-License-Identifier: MIT */
/* z_bop 0049ea10 */
size_t __cdecl _fread(void *dst, size_t size, size_t count, void *stream);
char __cdecl FUN_0049e060(void);
void __cdecl FUN_0049eb60(int param);
extern int _DAT_009a20ac;

size_t __cdecl FUN_0049ea10(void *param_1, size_t param_2, size_t param_3, void *param_4)
{
    size_t _Count = param_3;
    int iVar3 = 0xf;
    size_t sVar2;
    char cVar1;

    while (1) {
        do {
            iVar3 = iVar3 - 1;
            sVar2 = _fread(param_1, param_2, _Count, param_4);
            param_1 = (void *)((int)param_1 + sVar2 * param_2);
            _Count = _Count - sVar2;
            if (_Count == 0) {
                _DAT_009a20ac = 0;
                return param_3;
            }
        } while (iVar3 != 0);

        if (_Count == 0) break;

        while (1) {
            cVar1 = FUN_0049e060();
            if (cVar1 == 'R' || cVar1 == 'r') break;
            if (cVar1 == 'F' || cVar1 == 'f') {
                _DAT_009a20ac = 1;
                FUN_0049eb60(1);
                return 0;
            }
        }
        iVar3 = 5;
    }
    _DAT_009a20ac = 0;
    return param_3;
}
