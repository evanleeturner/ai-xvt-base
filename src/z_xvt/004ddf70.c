/* SPDX-License-Identifier: MIT */
/* z_xvt 004ddf70 */
extern int DAT_00759afc;
extern int *DAT_00759aec;
extern char DAT_00759645;
extern int DAT_00758830;

extern int __cdecl FUN_004dee40(char *param_1);
extern void __stdcall FUN_004d4d90(void);
extern int * __cdecl FUN_00417a40(int *param_1, char *param_2, int param_3);
extern char * __cdecl _strncpy(char *dest, const char *src, unsigned int n);
extern int __cdecl FUN_004ded00(char *param_1);
extern int __stdcall FUN_004d4d00(void);

bool __cdecl FUN_004ddf70(char *param_1, char *param_2, int param_3)
{
  char cVar1;
  int iVar2;
  char acStack_110[191];
  char buf_40[0x40];
  int iStack_10;
  int *local_8;

  if (*param_1 == '\0') {
    return 0;
  }
  if (*param_2 == '\0') {
    return 0;
  }
  if (0x7f < DAT_00759afc) {
    return 0;
  }
  if (DAT_00759aec == (int *)0x0) {
    return 0;
  }
  iVar2 = FUN_004dee40(param_2);
  cVar1 = DAT_00759645;
  if (iVar2 != -1) {
    return 1;
  }
  FUN_004d4d90();
  local_8 = FUN_00417a40(DAT_00759aec, param_1, param_3);
  if (local_8 != (int *)0x0) {
    (*(void (__stdcall **)(int *, int))(*local_8 + 0x34))(local_8, 0);
    _strncpy(buf_40, param_2, 0x40);
    _strncpy(acStack_110, param_1, 0x100);
    iStack_10 = FUN_004ded00(buf_40);
    if (cVar1 != '\0') {
      DAT_00758830 = FUN_004d4d00();
    }
    return (bool)('\x01' - (iStack_10 == 0));
  }
  if (cVar1 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return 0;
}
