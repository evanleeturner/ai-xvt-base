/* SPDX-License-Identifier: MIT */
/* z_bop 00444cb0 */
extern int DAT_0055696d;
extern int DAT_0055695d;
extern int DAT_00556961;
extern int DAT_0055692c;
extern void *DAT_00556928;
extern void *DAT_00556959;
extern int DAT_00556930;

extern void *FUN_00444d90(int, int, int, int, int, int);
extern void *_malloc(unsigned int);
extern void FUN_004438b0(void);
extern void _exit(int);

void __cdecl FUN_00444cb0(int param_1)
{
  void *puVar1;
  int iVar2;
  
  DAT_0055696d = 0;
  DAT_0055695d = 0;
  DAT_00556961 = 0;
  if (param_1 > 8) {
    param_1 = 8;
  }
  if (param_1 < 2) {
    param_1 = 2;
  }
  DAT_0055692c = param_1;
  DAT_00556928 = FUN_00444d90(0, 0, 0, 0x80, 0x80, 0x80);
  DAT_00556959 = _malloc(0x7fc);
  if ((DAT_00556928 != (void *)0) && (DAT_00556959 != (void *)0)) {
    void *tmp_ptr = DAT_00556928;
    __builtin_memcpy((void *)((char *)DAT_00556928 + 0x2e), &tmp_ptr, 4);
    puVar1 = DAT_00556928;
    int tmp_int = 0;
    __builtin_memcpy((void *)((char *)DAT_00556928 + 0xe), &tmp_int, 4);
    *(int *)((char *)puVar1 + 0x12) = 0;
    DAT_00556930 = 0;
    DAT_00556959 = (void *)((int)DAT_00556959 + 0x3fc);
    iVar2 = -0xff;
    do {
      *(int *)((int)DAT_00556959 + iVar2 * 4) = iVar2 * iVar2;
      iVar2 = iVar2 + 1;
    } while (iVar2 < 0x100);
    return;
  }
  FUN_004438b0();
  _exit(1);
}
