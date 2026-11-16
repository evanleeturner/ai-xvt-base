/* SPDX-License-Identifier: MIT */
/* z_xvt 004c01d0 */
extern void *DAT_0081b6f4;
extern char DAT_0081bcf0;
extern void **DAT_0081b848;

int __cdecl __strcmpi(const char *, const char *);
void __cdecl FID_conflict__remove(char *);
void __cdecl FUN_004c0090(int *);

int __stdcall FUN_004c01d0(void)
{
    void **puVar2;
    int local_4;
    int iVar1;

    if (DAT_0081b6f4 != NULL && DAT_0081bcf0 != '\0' && DAT_0081b848 != NULL)
    {
        puVar2 = (void **)*DAT_0081b848;
        local_4 = 0;
        iVar1 = (int)DAT_0081b6f4;
        while (1)
        {
            if (iVar1 == 0 || (int)DAT_0081b848[1] <= local_4)
                goto LAB_004c0269;
            iVar1 = __strcmpi((char *)((int)DAT_0081b6f4 + local_4 * 0xe), &DAT_0081bcf0);
            if (iVar1 == 0)
                break;
            puVar2 = puVar2[1];
            local_4 = local_4 + 1;
            iVar1 = local_4 * 0xe + (int)DAT_0081b6f4;
        }
        FID_conflict__remove((char *)*puVar2);
    }
LAB_004c0269:
    puVar2 = (void **)&DAT_0081bcf0;
    for (iVar1 = 0xf7ce; iVar1 != 0; iVar1 = iVar1 - 1)
    {
        *(int *)puVar2 = 0;
        puVar2 = puVar2 + 1;
    }
    *(unsigned short *)puVar2 = 0;
    local_4 = 0;
    FUN_004c0090(&local_4);
    return 1;
}
