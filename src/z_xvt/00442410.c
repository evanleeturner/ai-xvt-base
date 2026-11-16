/* SPDX-License-Identifier: MIT */
/* z_xvt 00442410 */
extern unsigned char DAT_006d4a02;
extern int DAT_00709310[];
extern int DAT_007092e8[];
extern unsigned int _DAT_00732a3c;
extern int DAT_006d4980;
extern int DAT_006d4988;
extern int DAT_0071ecf4;
extern unsigned char DAT_0071b730[];
extern unsigned char DAT_0071b73c[];
extern unsigned char DAT_0071b74a[];
extern unsigned char DAT_0070ad48[];
extern unsigned char DAT_0072f508[];
extern unsigned char DAT_007092d9;

void __cdecl FUN_00442720(unsigned short param_1, unsigned char param_2);

void __stdcall FUN_00442410(void)
{
  int iVar1;
  unsigned int uVar2;
  unsigned char bVar3;
  int iVar5;
  unsigned int uVar6;
  int bVar7;
  int local_c;
  int local_8;
  int local_4;
  int i;
  unsigned int p1;
  unsigned int p2;
  unsigned int tmp;
  unsigned int u32;
  unsigned short us;
  unsigned short us2;
  unsigned char b;

  local_4 = 10;
  if (DAT_006d4a02 == 2) {
    iVar5 = 0;
    local_c = 0;
    for (i = 0; i < 8; i++) {
      if (local_c < (int)((unsigned int)DAT_007092e8[i] + (unsigned int)DAT_00709310[i])) {
        local_c = (int)((unsigned int)DAT_007092e8[i] + (unsigned int)DAT_00709310[i]);
        local_4 = iVar5;
      }
      iVar5 = iVar5 + 1;
    }
  }
  if ((int)_DAT_00732a3c < DAT_006d4980) {
    local_8 = (int)(_DAT_00732a3c * 0x23);
    uVar6 = _DAT_00732a3c;
    do {
      iVar5 = (int)((unsigned int)DAT_006d4988 + (unsigned int)local_8);
      if (*(char *)(iVar5 + 3) != 0) {
        iVar1 = (int)((unsigned int)DAT_0071ecf4 * 0x5bd);
        __builtin_memcpy(&u32, &DAT_0071b730[iVar1], 4);
        if (u32 != uVar6) {
          bVar3 = 0;
          __builtin_memcpy(&p1, (unsigned char *)(iVar5 + 0x1f), 4);
          uVar2 = (unsigned int)*(unsigned char *)(p1 + 0x87);
          __builtin_memcpy(&us, &DAT_0071b73c[iVar1], 2);
          if ((DAT_006d4a02 == 2) && ((unsigned int)us == uVar2 && (*(char *)(iVar5 + 2) == 0))) {
            bVar3 = 0x2f;
          }
          else if ((local_4 == 10) ||
                   ((int)((unsigned int)DAT_00709310[uVar2] + (unsigned int)DAT_007092e8[uVar2]) != local_c)) {
            __builtin_memcpy(&tmp, (unsigned char *)(iVar5 + 0x1b), 4);
            if (((int)tmp != -1) && (DAT_0071ecf4 != (int)tmp)) {
              if (DAT_006d4a02 == 2) {
                b = DAT_0070ad48[(unsigned int)*(unsigned char *)((uVar6 & 0xffff) * 0x23 + 0x16 + (unsigned int)DAT_006d4988) * 0x566];
                if ((unsigned int)b == (unsigned int)us) {
                  bVar7 = 0;
                }
                else {
                  bVar7 = DAT_0072f508[(unsigned int)b + (unsigned int)us * 0x1e5] == 0;
                }
                bVar3 = (unsigned char)((((unsigned int)(0u - (unsigned int)(!bVar7))) & 0xa0U) + 0x33);
              }
              else {
                switch (*(unsigned char *)(p1 + 0x86)) {
                case 0:
                  bVar3 = 0x3f;
                  break;
                case 1:
                case 4:
                  bVar3 = 0x37;
                  break;
                case 2:
                  bVar3 = 0x33;
                  break;
                default:
                  bVar3 = 0x3b;
                }
              }
            }
          }
          else {
            bVar3 = 0xd4;
          }
          if (bVar3 != 0) {
            __builtin_memcpy(&tmp, (unsigned char *)(p1 + 0xa7), 4);
            uVar2 = (unsigned int)us;
            if ((DAT_007092d9 == 0) && (*(char *)(tmp + 0x1f0 + uVar2) == 0)) {
              b = DAT_0070ad48[(unsigned int)*(unsigned char *)((uVar6 & 0xffff) * 0x23 + 0x16 + (unsigned int)DAT_006d4988) * 0x566];
              if ((unsigned int)b == uVar2) {
                bVar7 = 0;
              }
              else {
                bVar7 = DAT_0072f508[(unsigned int)b + uVar2 * 0x1e5] == 0;
              }
              if (bVar7) goto LAB_004426e3;
            }
            bVar7 = 0;
            if (!(((unsigned short)uVar6 == 0xffff) || (DAT_006d4980 <= (int)(uVar6 & 0xffff)))) {
              iVar5 = (int)((uVar6 & 0xffff) * 0x23 + (unsigned int)DAT_006d4988);
              __builtin_memcpy(&tmp, (unsigned char *)(iVar5 + 0x1b), 4);
              if ((int)tmp != -1) {
                __builtin_memcpy(&p2, (unsigned char *)(iVar5 + 0x1f), 4);
                __builtin_memcpy(&tmp, (unsigned char *)(p2 + 0xa7), 4);
                if (tmp != 0) {
                  if ((*(unsigned char *)(tmp + 0x1e0) & 1) != 0) {
                    if (*(char *)(tmp + 0x237) != 0) {
                      if (*(char *)(tmp + 0x233) == 3) {
                        __builtin_memcpy(&us2, (unsigned char *)(tmp + 0x238), 2);
                        if (us2 != 0) {
                          bVar7 = 1;
                        }
                      }
                    }
                  }
                }
              }
            }
            __builtin_memcpy(&us2, &DAT_0071b74a[iVar1], 2);
            if ((!bVar7) && ((unsigned int)us2 != uVar6)) {
              FUN_00442720((unsigned short)uVar6, bVar3);
            }
          }
        }
      }
LAB_004426e3:
      uVar6 = uVar6 + 1;
      local_8 = (int)((unsigned int)local_8 + 0x23);
    } while ((int)uVar6 < DAT_006d4980);
  }
  return;
}
