/* SPDX-License-Identifier: MIT */
/* z_xvt 004288b0 */
void * __cdecl _malloc(unsigned int);

extern unsigned int DAT_00536f25;
extern unsigned int DAT_00536f29;
extern unsigned char *DAT_00536f31;
extern unsigned char *DAT_00536f35;

unsigned char * __cdecl FUN_004288b0(unsigned char param_1, unsigned char param_2, unsigned int param_3, unsigned char param_4, unsigned char param_5, unsigned char param_6)
{
  unsigned int tmp;
  unsigned char *puVar2;
  unsigned char *puVar4;
  int iVar3;
  
  if (DAT_00536f29 == 0) {
    puVar2 = (unsigned char *)_malloc(0x29004);
    if (puVar2 == (unsigned char *)0x0) {
      return (unsigned char *)0x0;
    }
    *(unsigned char **)(puVar2 + 0x29000) = DAT_00536f35;
    DAT_00536f29 = 0x800;
    DAT_00536f31 = puVar2;
    DAT_00536f35 = puVar2;
  }
  puVar2 = DAT_00536f31;
  DAT_00536f25 = DAT_00536f25 + 1;
  DAT_00536f29 = DAT_00536f29 + -1;
  DAT_00536f31 = DAT_00536f31 + 0x52;
  tmp = param_3;
  __builtin_memcpy(puVar2 + 0x2e, &tmp, 4);
  puVar4 = puVar2 + 0x32;
  for (iVar3 = 8; iVar3 != 0; iVar3 = iVar3 + -1) {
    tmp = 0;
    __builtin_memcpy(puVar4, &tmp, 4);
    puVar4 = puVar4 + 4;
  }
  *puVar2 = param_1;
  puVar2[1] = param_2;
  puVar2[2] = 0;
  puVar2[3] = param_4;
  puVar2[4] = param_5;
  puVar2[5] = param_6;
  tmp = 0;
  __builtin_memcpy(puVar2 + 0xe, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x12, &tmp, 4);
  __builtin_memcpy(puVar2 + 10, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x16, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x1a, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x1e, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x22, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x26, &tmp, 4);
  __builtin_memcpy(puVar2 + 0x2a, &tmp, 4);
  return puVar2;
}
