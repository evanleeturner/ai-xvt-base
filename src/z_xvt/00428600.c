/* SPDX-License-Identifier: MIT */
/* z_xvt 00428600 */
extern unsigned int _DAT_00536f0d;
extern unsigned int _DAT_00536f09;
extern unsigned int DAT_00536f05;
extern unsigned int DAT_00536f21;
extern unsigned int DAT_00536efc;
extern unsigned int DAT_00536efd;
extern unsigned int DAT_00536efe;
extern unsigned int _DAT_00536f2d;

void __cdecl FUN_00428600(int param_1);

void __cdecl FUN_00428600(int param_1)
{
  double dVar1;
  double dVar2;
  unsigned int uVar3;
  unsigned char *pbVar4;
  unsigned long long local_8;
  int val_6;
  int val_10;
  unsigned short val_6_short;
  int child_ptr;
  
  if (*(char *)((char*)param_1 + 2) != '\0') {
    uVar3 = 0;
    do {
      if ((1 << ((unsigned char)uVar3 & 0x1f) & (unsigned int)*(unsigned char *)((char*)param_1 + 2)) != 0) {
        __builtin_memcpy(&child_ptr, (char*)param_1 + 0x32 + uVar3 * 4, 4);
        FUN_00428600(child_ptr);
      }
      uVar3 = uVar3 + 1;
    } while (uVar3 < 8);
  }
  
  union { double d; unsigned int parts[2]; } g_double;
  g_double.parts[0] = _DAT_00536f09;
  g_double.parts[1] = _DAT_00536f0d;
  dVar2 = g_double.d;
  
  __builtin_memcpy(&val_10, (char*)param_1 + 10, 4);
  if (val_10 != 0) {
    __builtin_memcpy(&val_6, (char*)param_1 + 6, 4);
    pbVar4 = (unsigned char *)(val_6 * 9 + DAT_00536f05);
    local_8 = (unsigned long long)*(unsigned int *)(DAT_00536f21 + ((unsigned int)*pbVar4 - (unsigned int)DAT_00536efc) * 4);
    dVar2 = (double)local_8;
    local_8 = (unsigned long long)*(unsigned int *)(DAT_00536f21 + ((unsigned int)pbVar4[1] - (unsigned int)DAT_00536efd) * 4);
    dVar1 = (double)local_8;
    local_8 = (unsigned long long)*(unsigned int *)(DAT_00536f21 + ((unsigned int)pbVar4[2] - (unsigned int)DAT_00536efe) * 4);
    dVar1 = (double)local_8 + dVar1 + dVar2;
    
    g_double.parts[0] = _DAT_00536f09;
    g_double.parts[1] = _DAT_00536f0d;
    dVar2 = g_double.d;
    
    if (dVar1 < g_double.d) {
      __builtin_memcpy(&val_6_short, (char*)param_1 + 6, 2);
      _DAT_00536f2d = (unsigned int)val_6_short;
      dVar2 = dVar1;
    }
  }
  
  g_double.d = dVar2;
  _DAT_00536f0d = g_double.parts[1];
  _DAT_00536f09 = g_double.parts[0];
  return;
}
