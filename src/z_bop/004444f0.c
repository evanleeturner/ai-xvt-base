/* SPDX-License-Identifier: MIT */
/* z_bop 004444f0 */
void __cdecl FUN_004444f0(int);

extern unsigned int _DAT_00556941;
extern unsigned int _DAT_00556945;
extern unsigned int _DAT_00556965;
extern unsigned int DAT_00556934;
extern unsigned int DAT_00556935;
extern unsigned int DAT_00556936;
extern unsigned int DAT_0055693d;
extern unsigned int DAT_00556959[];

void __cdecl FUN_004444f0(int param_1)
{
  double dVar1;
  double dVar2;
  unsigned int uVar3;
  unsigned char *pbVar4;
  unsigned long long local_8;
  unsigned long long dValBits;
  unsigned char field_2;
  unsigned int field_6;
  unsigned int field_10;
  unsigned int child_ptr;

  __builtin_memcpy(&field_2, (void *)(param_1 + 2), 1);
  if (field_2 != '\0') {
    uVar3 = 0;
    do {
      if ((1 << ((unsigned char)uVar3 & 0x1f) & (unsigned int)field_2) != 0) {
        __builtin_memcpy(&child_ptr, (void *)(param_1 + 0x32 + uVar3 * 4), 4);
        FUN_004444f0(child_ptr);
      }
      uVar3 = uVar3 + 1;
    } while (uVar3 < 8);
  }
  
  __builtin_memcpy(&dVar2, &_DAT_00556941, 8);
  
  __builtin_memcpy(&field_10, (void *)(param_1 + 10), 4);
  if (field_10 != 0) {
    __builtin_memcpy(&field_6, (void *)(param_1 + 6), 4);
    pbVar4 = (unsigned char *)(field_6 * 9 + DAT_0055693d);
    
    local_8 = (unsigned long long)DAT_00556959[(unsigned int)pbVar4[1] - DAT_00556935];
    dVar2 = (double)local_8;
    
    local_8 = (unsigned long long)DAT_00556959[(unsigned int)*pbVar4 - DAT_00556934];
    dVar1 = (double)local_8;
    
    local_8 = (unsigned long long)DAT_00556959[(unsigned int)pbVar4[2] - DAT_00556936];
    dVar1 = (double)local_8 + dVar1 + dVar2;
    
    __builtin_memcpy(&dVar2, &_DAT_00556941, 8);
    if (dVar1 < dVar2) {
      unsigned short field_6_ushort;
      __builtin_memcpy(&field_6_ushort, (void *)(param_1 + 6), 2);
      _DAT_00556965 = (unsigned int)field_6_ushort;
      dVar2 = dVar1;
    }
  }
  
  __builtin_memcpy(&dValBits, &dVar2, 8);
  _DAT_00556945 = (unsigned int)(dValBits >> 32);
  _DAT_00556941 = (unsigned int)dValBits;
  return;
}
