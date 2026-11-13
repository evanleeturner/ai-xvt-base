/* SPDX-License-Identifier: MIT */
/* z_xvt 004979e0 */
void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);
unsigned int __cdecl FUN_0042b250(int param_1);

extern short DAT_006d9d50;
extern int DAT_006d4988;
extern int DAT_006d9d56;
extern short DAT_006da9f0;
extern short DAT_00732e40;
extern int DAT_00732e54;

void __cdecl FUN_004979e0(short param_1, int param_2)
{
  unsigned short *psVar1;
  unsigned int uVar2;
  unsigned short uVar3;
  int iVar4;
  int v1f, v23, v27;
  int v_i4_4, v_i4_8, v_i4_c;
  int v_base_1f;
  unsigned short v_0d;

  iVar4 = (unsigned int)DAT_006d9d50 * 0x23 + DAT_006d4988;
  
  __builtin_memcpy(&v1f, (void *)(DAT_006d9d56 + 0x1f), 4);
  __builtin_memcpy(&v23, (void *)(DAT_006d9d56 + 0x23), 4);
  __builtin_memcpy(&v27, (void *)(DAT_006d9d56 + 0x27), 4);
  __builtin_memcpy(&v_i4_4, (void *)(iVar4 + 4), 4);
  __builtin_memcpy(&v_i4_8, (void *)(iVar4 + 8), 4);
  __builtin_memcpy(&v_i4_c, (void *)(iVar4 + 0xc), 4);
  
  FUN_00418cd0(v1f - v_i4_4,
               v23 - v_i4_8,
               v27 - v_i4_c);
               
  *(short *)(DAT_006d9d56 + 0x32) = DAT_006da9f0 + param_1;
  uVar2 = FUN_0042b250(DAT_00732e54);
  psVar1 = (unsigned short *)((unsigned int)DAT_006d9d50 * 0x23 + 0x10 + DAT_006d4988);
  uVar3 = *psVar1 - *(short *)(DAT_006d9d56 + 0x32);
  if (0x7fff < uVar3) {
    uVar3 = -uVar3;
  }
  if (uVar3 < 0x301) {
    *psVar1 = *(short *)(DAT_006d9d56 + 0x32);
    __builtin_memcpy(&v_base_1f, (void *)((unsigned int)DAT_006d9d50 * 0x23 + 0x1f + DAT_006d4988), 4);
    *(unsigned char *)(v_base_1f + 0x90) = 1;
    iVar4 = v_base_1f;
    *(unsigned char *)(iVar4 + 0x89) = *(unsigned char *)(iVar4 + 0x90);
    *(unsigned char *)(DAT_00732e54 + 0xab) = 3;
  }
  else {
    *(unsigned char *)(DAT_00732e54 + 0xab) = 2;
    *(unsigned short *)(DAT_00732e54 + 0xac) = ((unsigned short)(uVar2 >> 1) & 0x7fff) + 0x4000;
  }
  if (param_2 != 0) {
    *(unsigned short *)(DAT_006d9d56 + 0x2e) = DAT_00732e40;
    *(unsigned short *)(DAT_00732e54 + 0x9e) = 0xffff;
    *(unsigned char *)(DAT_00732e54 + 0x9d) = 0;
    __builtin_memcpy(&v_0d, (void *)(DAT_00732e54 + 0xd), 2);
    if (v_0d < *(unsigned short *)(DAT_006d9d56 + 0x2e)) {
      *(unsigned char *)(DAT_00732e54 + 0x9c) = 2;
    }
    else {
      *(unsigned char *)(DAT_00732e54 + 0x9c) = 1;
    }
    *(unsigned char *)(DAT_00732e54 + 0x96) = 0;
    *(unsigned char *)(DAT_00732e54 + 0x97) = 0;
  }
  return;
}
