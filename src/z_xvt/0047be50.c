/* SPDX-License-Identifier: MIT */
/* z_xvt 0047be50 */
void __cdecl FUN_0047ed40(char *param_1, void *param_2, int param_3, int param_4);
int __cdecl _fgetc(void *stream);
int __cdecl _fclose(void *stream);

extern void *DAT_00732948;
extern char *DAT_00732fec;
extern char **DAT_0071f2f0;
extern int DAT_00525574;

void __cdecl FUN_0047be50(char *param_1, unsigned short param_2, short param_3, unsigned int param_4)
{
    unsigned int uVar1;
    void *_File;
    int iVar2;
    int iVar3;
    unsigned int uVar4;
    short local_2;

    FUN_0047ed40(param_1, &DAT_00525574, 1, 0);
    _File = DAT_00732948;
    local_2 = 0;
    if (param_3 != 0) {
        uVar4 = param_4 & 0xffff;
        do {
            iVar3 = (int)local_2;
            DAT_0071f2f0[param_2] = DAT_00732fec;
            if ((int)uVar4 <= iVar3) {
                param_2 = param_2 + 1;
            }
            iVar2 = _fgetc(_File);
            uVar1 = *(unsigned int*)((char*)_File + 0xC);
            while (((uVar1 & 0x10) == 0 && ((short)iVar2 != 0xff))) {
                if ((int)uVar4 <= iVar3) {
                    *DAT_00732fec = (char)iVar2;
                    DAT_00732fec = DAT_00732fec + 1;
                }
                iVar2 = _fgetc(_File);
                uVar1 = *(unsigned int*)((char*)_File + 0xC);
            }
            if ((int)uVar4 <= iVar3) {
                param_3 = param_3 - 1;
                *DAT_00732fec = 0xff;
                DAT_00732fec = DAT_00732fec + 1;
            }
            local_2 = local_2 + 1;
        } while (param_3 != 0);
    }
    if (((*(unsigned int*)((char*)DAT_00732948 + 0xC)) & 0x20U) == 0) {
        _fclose(DAT_00732948);
    }
    return;
}
