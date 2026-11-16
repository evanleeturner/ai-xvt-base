/* SPDX-License-Identifier: MIT */
/* z_xvt 004b7343 */
extern unsigned int *DAT_0052f1f4;
extern unsigned int *DAT_0064ee44;
extern unsigned int *DAT_0064ee48;
extern unsigned int *DAT_00742818;
extern unsigned int _DAT_0064ee50;
extern unsigned int _DAT_0064ee54;
extern unsigned int _DAT_0064ee58;
extern unsigned int _DAT_0064ee5c;
extern float _DAT_00520260;

void __cdecl FUN_004b51f0(int param_1, unsigned int param_2, unsigned int param_3);
void __cdecl FUN_0045bbe0(void);
void *__cdecl _memset(void *dst, int val, unsigned int size);

unsigned int __cdecl FUN_004b7343(unsigned int param_1, unsigned int param_2)
{
    int iVar1;
    unsigned int uVar2;
    unsigned int local_84[20];
    unsigned int local_34;
    unsigned int vp[11];
    float local_1c;
    float local_18;
    float local_14;
    float local_10;

    iVar1 = ((int (__stdcall *)(unsigned int *, unsigned int **, unsigned int))
             (*(unsigned int *)(*DAT_0052f1f4 + 0x1c)))(DAT_0052f1f4, &DAT_0064ee44, 0);
    if (iVar1 == 0) {
        iVar1 = ((int (__stdcall *)(unsigned int *, unsigned int *))
                 (*(unsigned int *)(*DAT_00742818 + 0x24)))(DAT_00742818, DAT_0064ee44);
        if (iVar1 == 0) {
            _memset(vp, 0, 0x2c);
            vp[0] = 0x2c;
            vp[2] = 0;
            vp[1] = 0;
            vp[3] = param_1;
            vp[4] = param_2;
            local_1c = (float)param_1 / _DAT_00520260;
            local_18 = (float)param_2 / _DAT_00520260;
            local_14 = (float)param_1 / (local_1c * _DAT_00520260);
            local_10 = (float)param_2 / (local_18 * _DAT_00520260);
            __builtin_memcpy(&vp[5], &local_1c, 4);
            __builtin_memcpy(&vp[6], &local_18, 4);
            __builtin_memcpy(&vp[7], &local_14, 4);
            __builtin_memcpy(&vp[8], &local_10, 4);
            iVar1 = ((int (__stdcall *)(unsigned int *, void *))
                     (*(unsigned int *)(*DAT_0064ee44 + 0x14)))(DAT_0064ee44, vp);
            if (iVar1 == 0) {
                _DAT_0064ee50 = 0;
                _DAT_0064ee54 = 0;
                _DAT_0064ee58 = param_1;
                _DAT_0064ee5c = param_2;
                iVar1 = ((int (__stdcall *)(unsigned int *, unsigned int **, unsigned int))
                         (*(unsigned int *)(*DAT_0052f1f4 + 0x18)))(DAT_0052f1f4, &DAT_0064ee48, 0);
                if (iVar1 == 0) {
                    _memset(local_84, 0, 0x50);
                    local_84[0] = 0x50;
                    local_84[1] = 0;
                    local_84[2] = 0;
                    local_84[3] = 0x3f000000;
                    local_84[19] = 1;
                    ((int (__stdcall *)(unsigned int *, void *))
                     (*(unsigned int *)(*DAT_0064ee48 + 0x10)))(DAT_0064ee48, local_84);
                    ((int (__stdcall *)(unsigned int *, unsigned int *, unsigned int *))
                     (*(unsigned int *)(*DAT_0064ee48 + 0x18)))(DAT_0064ee48, DAT_00742818, &local_34);
                    iVar1 = ((int (__stdcall *)(unsigned int *, unsigned int))
                             (*(unsigned int *)(*DAT_0064ee44 + 0x20)))(DAT_0064ee44, local_34);
                    if (iVar1 == 0) {
                        FUN_0045bbe0();
                        uVar2 = 1;
                    } else {
                        FUN_004b51f0(iVar1, 0x52f210, 0x79);
                        FUN_0045bbe0();
                        uVar2 = 0;
                    }
                } else {
                    FUN_004b51f0(iVar1, 0x52f210, 0x79);
                    FUN_0045bbe0();
                    uVar2 = 0;
                }
            } else {
                FUN_004b51f0(iVar1, 0x52f210, 0x79);
                FUN_0045bbe0();
                uVar2 = 0;
            }
        } else {
            FUN_004b51f0(iVar1, 0x52f210, 0x79);
            FUN_0045bbe0();
            uVar2 = 0;
        }
    } else {
        FUN_004b51f0(iVar1, 0x52f210, 0x79);
        FUN_0045bbe0();
        uVar2 = 0;
    }
    return uVar2;
}
