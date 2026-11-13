/* SPDX-License-Identifier: MIT */
/* z_xvt 00432b70 */
DWORD __stdcall timeGetTime(void);
int * __cdecl FUN_00431d70(int *, undefined4 *);
void __cdecl FUN_00430980(int, int);

int * __cdecl FUN_00432b70(int *param_1, undefined4 *param_2, int param_3)
{
    DWORD start_time;
    DWORD current_time;
    int *piVar3;
    int local_8;
    undefined4 local_4;

    start_time = timeGetTime();
    do {
        current_time = timeGetTime();
        if ((unsigned int)(param_3 * 1000) < (current_time - start_time)) {
            return NULL;
        }
        while ((piVar3 = FUN_00431d70(&local_8, &local_4)) != NULL) {
            if (local_8 != 0) {
                *param_1 = local_8;
                *param_2 = local_4;
                return piVar3;
            }
            FUN_00430980(*piVar3, (int)piVar3);
        }
    } while (1);
}
