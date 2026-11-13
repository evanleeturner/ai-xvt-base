/* SPDX-License-Identifier: MIT */
/* z_bop 004820b0 */
extern unsigned short DAT_00a07cec;
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_009e9670[];
extern unsigned short DAT_00521f2e[];
extern unsigned short DAT_009a73a0;
extern unsigned int DAT_009a1ff8;
extern unsigned int DAT_009d1308;
extern unsigned char DAT_009e9683[];

extern unsigned long long __cdecl FUN_00482640(unsigned int, unsigned int, unsigned short, int, int);
extern unsigned int __cdecl FUN_00484f80(unsigned short);
extern void __cdecl FUN_00450650(unsigned short, int);

unsigned short __cdecl FUN_004820b0(int param_1)
{
  unsigned char bVar1;
  unsigned int uVar2;
  unsigned short uVar3;
  unsigned int uVar4;
  unsigned short uVar5;
  unsigned long long uVar6;
  unsigned short local_4;
  unsigned short local_2;
  
  uVar3 = 0xffff;
  local_4 = 0xffff;
  uVar4 = 0xffffffff;
  local_2 = 0xffff;
  uVar5 = DAT_00a07cec;
  if ((int)(unsigned int)DAT_00a07cec < (int)DAT_00a07ce8) {
    do {
      bVar1 = *(unsigned char *)((unsigned int)uVar5 * 0x23 + 3 + DAT_009a1fe8);
      if ((bVar1 != 0) && (*(unsigned int *)((int)&DAT_009e9670 + param_1 * 0x5bd) != (unsigned int)uVar5)) {
        uVar2 = (unsigned int)bVar1;
        if ((DAT_00521f2e[uVar2 * 0x1a] & 1) != 0) {
          uVar6 = FUN_00482640(uVar2 * 0x19,
                               (unsigned int)(uVar2 * 5 >> 8) | DAT_00521f2e[uVar2 * 0x1a],
                               uVar5, 1, param_1);
          if ((short)uVar6 == 0) {
            if (DAT_009a73a0 < uVar3) {
              local_4 = uVar5;
              uVar3 = DAT_009a73a0;
            }
          }
          else if (DAT_009a1ff8 < uVar4) {
            uVar4 = DAT_009a1ff8;
            local_2 = uVar5;
          }
        }
      }
      uVar5 = uVar5 + 1;
    } while ((int)(unsigned int)uVar5 < (int)DAT_00a07ce8);
  }
  uVar2 = DAT_00a07ce8 & 0xffff;
  if ((int)(DAT_00a07ce8 & 0xffff) < (int)(DAT_009d1308 + DAT_00a07ce8)) {
    do {
      bVar1 = *(unsigned char *)(uVar2 * 0x23 + 3 + DAT_009a1fe8);
      uVar5 = (unsigned short)uVar2;
      if (bVar1 != 0) {
        uVar2 = (unsigned int)bVar1;
        if ((DAT_00521f2e[uVar2 * 0x1a] & 1) != 0) {
          uVar6 = FUN_00482640(uVar2 * 0x19,
                               (unsigned int)(uVar2 * 5 >> 8) | DAT_00521f2e[uVar2 * 0x1a],
                               uVar5, 1, param_1);
          if ((short)uVar6 == 0) {
            if (DAT_009a73a0 < uVar3) {
              local_4 = uVar5;
              uVar3 = DAT_009a73a0;
            }
          }
          else if (DAT_009a1ff8 < uVar4) {
            uVar4 = DAT_009a1ff8;
            local_2 = uVar5;
          }
        }
      }
      uVar2 = (unsigned int)(unsigned short)(uVar5 + 1);
    } while ((int)(unsigned int)(unsigned short)(uVar5 + 1) < (int)(DAT_009d1308 + DAT_00a07ce8));
  }
  if (local_2 == 0xffff) {
    if (DAT_009e9683[param_1 * 0x5bd] == '\0') {
      if (0x31 < uVar3) goto LAB_0048226f;
    }
    else if (0xf9 < uVar3) goto LAB_0048226f;
    local_2 = local_4;
  }
LAB_0048226f:
  if ((DAT_009e9683[param_1 * 0x5bd] == '\0') &&
     (uVar4 = FUN_00484f80(local_2), (char)uVar4 == '\x01')) {
    local_2 = 0xffff;
    FUN_00450650(0x101, param_1);
  }
  return local_2;
}
