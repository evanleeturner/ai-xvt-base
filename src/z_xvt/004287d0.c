/* SPDX-License-Identifier: MIT */
/* z_xvt 004287d0 */
void * __cdecl FUN_004288b0(unsigned char, unsigned char, int, unsigned char, unsigned char, unsigned char);
void * __cdecl _malloc(unsigned int);
void __cdecl FUN_0047f010(unsigned short);
void __cdecl _exit(int);

extern int DAT_00536f35;
extern int DAT_00536f25;
extern int DAT_00536f29;
extern int DAT_00536ef4;
extern void * DAT_00536ef0;
extern void * DAT_00536f21;
extern int DAT_00536ef8;

void __cdecl FUN_004287d0(int param_1)
{
  void * puVar1;
  int iVar2;
  
  DAT_00536f35 = 0;
  DAT_00536f25 = 0;
  DAT_00536f29 = 0;
  if (8 < param_1) {
    param_1 = 8;
  }
  if (param_1 < 2) {
    param_1 = 2;
  }
  DAT_00536ef4 = param_1;
  DAT_00536ef0 = FUN_004288b0(0, 0, 0, 0x80, 0x80, 0x80);
  DAT_00536f21 = _malloc(0x7fc);
  if (DAT_00536ef0 != 0 && DAT_00536f21 != 0) {
    void *tmp_2e = DAT_00536ef0;
    __builtin_memcpy((char *)DAT_00536ef0 + 0x2e, &tmp_2e, 4);
    puVar1 = DAT_00536ef0;
    int tmp_e = 0;
    __builtin_memcpy((char *)DAT_00536ef0 + 0xe, &tmp_e, 4);
    int tmp_12 = 0;
    __builtin_memcpy((char *)puVar1 + 0x12, &tmp_12, 4);
    DAT_00536ef8 = 0;
    DAT_00536f21 = (void *)((int)DAT_00536f21 + 0x3fc);
    iVar2 = -0xff;
    do {
      *(int *)((int)DAT_00536f21 + iVar2 * 4) = iVar2 * iVar2;
      iVar2 = iVar2 + 1;
    } while (iVar2 < 0x100);
    return;
  }
  FUN_0047f010(0);
  _exit(1);
}
