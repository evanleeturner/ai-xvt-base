/* SPDX-License-Identifier: MIT */
/* z_bop 004ada90 */
extern unsigned short DAT_009ec5fe;
extern int DAT_009a1fe8;
void __stdcall RaiseException(unsigned long, unsigned long, unsigned long, const unsigned long *);

void __cdecl FUN_004ada90(int param_1, int param_2)
{
  unsigned short field_7b;
  unsigned short field_79;
  unsigned int uVar4;
  int iVar5;
  int pObj;
  unsigned __int64 div1_num;
  unsigned __int64 div2_num;
  unsigned __int64 divisor = 0xec;

  div1_num = (unsigned int)DAT_009ec5fe * (unsigned int)param_2;
  if (divisor == 0 || div1_num / divisor > 0xFFFFFFFF) {
    RaiseException(0xC0000094UL, 0, 0, NULL);
  }
  uVar4 = (unsigned int)(div1_num / divisor);

  iVar5 = param_1 * 0x23;
  pObj = *(int *)(DAT_009a1fe8 + 0x1f + iVar5);

  __builtin_memcpy(&field_7b, (void *)(pObj + 0x7b), 2);

  div2_num = (unsigned __int64)(((int)uVar4 * -0xec) + ((unsigned int)DAT_009ec5fe * (unsigned int)param_2)) * 0x10000;
  if (divisor == 0 || div2_num / divisor > 0xFFFFFFFF) {
    RaiseException(0xC0000094UL, 0, 0, NULL);
  }
  unsigned short scaled_rem = (unsigned short)(div2_num / divisor);

  unsigned short new_7b = field_7b - scaled_rem;
  __builtin_memcpy((void *)(pObj + 0x7b), &new_7b, 2);

  if (field_7b < new_7b) {
    __builtin_memcpy(&field_79, (void *)(pObj + 0x79), 2);
    field_79 -= 1;
    __builtin_memcpy((void *)(pObj + 0x79), &field_79, 2);
  }

  __builtin_memcpy(&field_79, (void *)(pObj + 0x79), 2);
  field_79 -= (unsigned short)uVar4;
  __builtin_memcpy((void *)(pObj + 0x79), &field_79, 2);

  __builtin_memcpy(&field_79, (void *)(pObj + 0x79), 2);
  if (0x8000 < field_79) {
    field_79 = 0;
    __builtin_memcpy((void *)(pObj + 0x79), &field_79, 2);
  }
}
