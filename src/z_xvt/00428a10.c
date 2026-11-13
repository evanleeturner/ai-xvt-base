/* SPDX-License-Identifier: MIT */
/* z_xvt 00428a10 */
extern double _DAT_00536f11;
extern int DAT_00536f25;
extern int DAT_00536ef8;
extern unsigned int DAT_00536f1d;
extern unsigned int DAT_00536f19;

void __cdecl FUN_00428a10(byte *param_1);

void __cdecl FUN_00428a10(byte *param_1)
{
  int iVar1;
  unsigned int uVar2;
  double d_tmp;
  int i_tmp;
  byte b_tmp;
  unsigned int ui_tmp;
  byte *p_tmp;
  int i_param_10;
  double d_param_16;
  double d_iVar1_16;
  double d_param_1e;
  double d_iVar1_1e;
  double d_param_26;
  double d_iVar1_26;
  double d_concat;

  if (param_1[2] != 0) {
    uVar2 = 0;
    do {
      __builtin_memcpy(&p_tmp, param_1 + uVar2 * 4 + 0x32, 4);
      if ((1 << ((byte)uVar2 & 0x1f) & (unsigned int)param_1[2]) != 0) {
        FUN_00428a10(p_tmp);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < 8);
  }

  __builtin_memcpy(&d_tmp, param_1 + 0xe, 8);
  if (d_tmp <= _DAT_00536f11) {
    __builtin_memcpy(&iVar1, param_1 + 0x2e, 4);
    
    __builtin_memcpy(&b_tmp, (byte *)iVar1 + 2, 1);
    b_tmp = b_tmp & ~('\x01' << (*param_1 & 0x1f));
    __builtin_memcpy((byte *)iVar1 + 2, &b_tmp, 1);
    
    __builtin_memcpy(&i_tmp, (byte *)iVar1 + 10, 4);
    __builtin_memcpy(&i_param_10, param_1 + 10, 4);
    i_tmp = i_tmp + i_param_10;
    __builtin_memcpy((byte *)iVar1 + 10, &i_tmp, 4);
    
    __builtin_memcpy(&d_param_16, param_1 + 0x16, 8);
    __builtin_memcpy(&d_iVar1_16, (byte *)iVar1 + 0x16, 8);
    d_iVar1_16 = d_param_16 + d_iVar1_16;
    __builtin_memcpy((byte *)iVar1 + 0x16, &d_iVar1_16, 8);
    
    __builtin_memcpy(&d_param_1e, param_1 + 0x1e, 8);
    __builtin_memcpy(&d_iVar1_1e, (byte *)iVar1 + 0x1e, 8);
    d_iVar1_1e = d_param_1e + d_iVar1_1e;
    __builtin_memcpy((byte *)iVar1 + 0x1e, &d_iVar1_1e, 8);
    
    __builtin_memcpy(&d_param_26, param_1 + 0x26, 8);
    __builtin_memcpy(&d_iVar1_26, (byte *)iVar1 + 0x26, 8);
    d_iVar1_26 = d_param_26 + d_iVar1_26;
    __builtin_memcpy((byte *)iVar1 + 0x26, &d_iVar1_26, 8);
    
    DAT_00536f25 = DAT_00536f25 + -1;
    return;
  }

  __builtin_memcpy(&i_tmp, param_1 + 10, 4);
  if (i_tmp != 0) {
    DAT_00536ef8 = DAT_00536ef8 + 1;
  }

  __builtin_memcpy(&d_tmp, param_1 + 0xe, 8);
  __builtin_memcpy(&d_concat, &DAT_00536f19, 8);
  if (d_tmp < d_concat) {
    __builtin_memcpy(&ui_tmp, param_1 + 0x12, 4);
    DAT_00536f1d = ui_tmp;
    __builtin_memcpy(&ui_tmp, param_1 + 0xe, 4);
    DAT_00536f19 = ui_tmp;
  }
  return;
}
