/* SPDX-License-Identifier: MIT */
/* z_xvt 004c9600 */
extern unsigned int DAT_0081bc68;
extern char DAT_00523570[];
extern char s_c_train_1TA01BF_TIE_0053231c[];

unsigned int __stdcall FUN_004be6c0(void);
void *_fopen(const char *filename, const char *mode);
int _fclose(void *stream);

unsigned int __stdcall FUN_004c9600(void)
{
    void *_File;
    int iVar1;
    char *pcVar2;
    char *pcVar3;
    unsigned int *puVar4;
    char local_80[21];
    unsigned int local_6b;

    pcVar2 = s_c_train_1TA01BF_TIE_0053231c;
    pcVar3 = local_80;
    for (iVar1 = 5; iVar1 != 0; iVar1 = iVar1 - 1) {
        *(unsigned int *)pcVar3 = *(unsigned int *)pcVar2;
        pcVar2 = pcVar2 + 4;
        pcVar3 = pcVar3 + 4;
    }
    *pcVar3 = *pcVar2;
    puVar4 = &local_6b;
    for (iVar1 = 0x1a; iVar1 != 0; iVar1 = iVar1 - 1) {
        *puVar4 = 0;
        puVar4 = puVar4 + 1;
    }
    *(unsigned short *)puVar4 = 0;
    *(unsigned char *)((int)puVar4 + 2) = 0;
    local_80[0] = FUN_004be6c0();
    if (local_80[0] != '\0') {
        _File = _fopen(local_80, DAT_00523570);
        if (_File != (void *)0x0) {
            _fclose(_File);
        }
        DAT_0081bc68 = (unsigned int)(_File != (void *)0x0);
        return DAT_0081bc68;
    }
    return 0;
}
