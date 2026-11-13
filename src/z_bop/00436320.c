/* SPDX-License-Identifier: MIT */
/* z_bop 00436320 */
extern unsigned int DAT_009ec47c;
extern unsigned int DAT_009a1fe8;
extern unsigned int DAT_00521f1c;
extern unsigned int DAT_009ecc34;
extern unsigned int DAT_009a8e34;
extern unsigned int DAT_00a07ce8;
extern unsigned int DAT_009d1308;

void __cdecl FUN_00436310(void);
int __cdecl FUN_00436470(unsigned int param_1, unsigned int param_2, unsigned int param_3);
int __cdecl FUN_0044ff10(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_004362a0(unsigned int param_1, unsigned int param_2);

void __stdcall FUN_00436320(void)
{
  unsigned int uVar5 = 0;
  int iVar6;
  unsigned int uVar3;
  int iVar4;
  unsigned char bVar1;
  unsigned int tmp;

  FUN_00436310();
  if (DAT_009ec47c != 0) {
    iVar6 = 0;
    do {
      uVar3 = (unsigned int)*(unsigned char *)((char *)DAT_009a1fe8 + iVar6 + 3);
      if (uVar3 != 0) {
        unsigned char type_val = *(unsigned char *)((char *)DAT_009a1fe8 + iVar6 + 2);
        switch (type_val) {
          case 0:
          case 1:
          case 2:
          case 3:
          case 4:
          case 5:
            __builtin_memcpy(&tmp, (char *)&DAT_00521f1c + uVar3 * 0x1a, 4);
            iVar4 = FUN_00436470(uVar5, tmp, DAT_009ecc34);
            break;
          case 6:
          case 7:
          case 0xb:
          case 0xd:
            __builtin_memcpy(&tmp, (char *)&DAT_00521f1c + uVar3 * 0x1a, 4);
            iVar4 = FUN_0044ff10(uVar5, tmp);
            break;
          default:
            iVar4 = 0;
            break;
        }
        if (iVar4 != 0) {
          FUN_004362a0(uVar5, DAT_009a8e34);
        }
      }
      iVar6 += 0x23;
      uVar5 += 1;
    } while (uVar5 < DAT_009ec47c);
  }
  if (DAT_00a07ce8 < DAT_009d1308 + DAT_00a07ce8) {
    iVar6 = DAT_00a07ce8 * 0x23;
    uVar5 = DAT_00a07ce8;
    do {
      uVar3 = (unsigned int)*(unsigned char *)((char *)DAT_009a1fe8 + iVar6 + 3);
      if (uVar3 != 0 && (bVar1 = *(unsigned char *)((char *)DAT_009a1fe8 + iVar6 + 2), bVar1 > 7) && bVar1 < 10) {
        __builtin_memcpy(&tmp, (char *)&DAT_00521f1c + uVar3 * 0x1a, 4);
        if (FUN_00436470(uVar5, tmp, DAT_009ecc34)) {
          FUN_004362a0(uVar5, DAT_009a8e34);
        }
      }
      iVar6 += 0x23;
      uVar5 += 1;
    } while (uVar5 < DAT_009d1308 + DAT_00a07ce8);
  }
  return;
}
