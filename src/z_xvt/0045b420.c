/* SPDX-License-Identifier: MIT */
/* z_xvt 0045b420 */
extern int DAT_00525730;
extern int DAT_005254f8;
extern int DAT_00732a38;
extern int DAT_006d4988;
extern int DAT_0072f46c;
extern int DAT_0072f4ec;
extern int DAT_006db700;
extern int DAT_006db6e0;
extern int DAT_006db70c;
extern int DAT_00703be0;
extern int DAT_00703adc;
extern int DAT_00703c68;
extern int DAT_006db7a8;
extern int DAT_006db79c;
extern int DAT_006db7a0;
extern int DAT_00525540;

unsigned int __cdecl FUN_00412810(int a, int b, int c);
int __cdecl FUN_00413d00(int a, int b, int c, int d, int e, int f);
void __cdecl FUN_004832a0(short a, short b, short c, int d, int e);

void __cdecl FUN_0045b420(int param_1)
{
  unsigned char bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  unsigned int uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int *piVar13;
  int *piVar14;
  int local_14;
  int local_10;
  int t0;
  int t1;
  int t2;
  short s0;
  short s1;
  short s2;

  DAT_00525730 = 0;
  if (DAT_005254f8 != 0) {
    local_10 = 0;
    local_14 = 0;
    iVar2 = *(int *)(param_1 + 4);
    iVar3 = *(int *)(param_1 + 8);
    iVar4 = *(int *)(param_1 + 0xc);
    if (0 < DAT_00732a38) {
      iVar12 = DAT_006d4988;
      piVar13 = &DAT_0072f46c;
      do {
        piVar14 = piVar13;
        if ((*(char *)(iVar12 + 3) != '\0') && (*(char *)(iVar12 + 2) == '\r')) {
          __builtin_memcpy(&t0, (unsigned char *)(iVar12 + 4), 4);
          __builtin_memcpy(&t1, (unsigned char *)(iVar12 + 8), 4);
          __builtin_memcpy(&t2, (unsigned char *)(iVar12 + 0xc), 4);
          iVar10 = (int)((unsigned int)t0 - (unsigned int)iVar2);
          iVar5 = (int)((unsigned int)t1 - (unsigned int)iVar3);
          iVar11 = (int)((unsigned int)t2 - (unsigned int)iVar4);
          uVar6 = FUN_00412810(iVar10, iVar5, iVar11);
          if (uVar6 < 0x4000) {
            __builtin_memcpy(&iVar7, (unsigned char *)(param_1 + 0x1f), 4);
            if (iVar7 == 0) {
              FUN_004832a0(*(short *)(param_1 + 0x14), *(short *)(param_1 + 0x12),
                           *(short *)(param_1 + 0x10), 0, 0);
              iVar9 = FUN_00413d00(iVar10, iVar5, iVar11, DAT_006db700, DAT_006db6e0, DAT_006db70c);
              iVar8 = DAT_00703be0;
              iVar7 = DAT_00703adc;
              piVar13[-3] = iVar9;
              iVar9 = FUN_00413d00(iVar10, iVar5, iVar11, DAT_00703c68, iVar8, iVar7);
              iVar8 = DAT_006db7a8;
              iVar7 = DAT_006db79c;
              piVar13[-2] = (int)(0U - (unsigned int)iVar9);
              iVar9 = DAT_006db7a0;
            }
            else {
              __builtin_memcpy(&s0, (unsigned char *)(iVar7 + 0x97), 2);
              __builtin_memcpy(&s1, (unsigned char *)(iVar7 + 0x99), 2);
              __builtin_memcpy(&s2, (unsigned char *)(iVar7 + 0x9b), 2);
              iVar7 = FUN_00413d00(iVar10, iVar5, iVar11, (int)s0, (int)s1, (int)s2);
              piVar13[-3] = iVar7;
              __builtin_memcpy(&iVar7, (unsigned char *)(param_1 + 0x1f), 4);
              __builtin_memcpy(&s0, (unsigned char *)(iVar7 + 0x91), 2);
              __builtin_memcpy(&s1, (unsigned char *)(iVar7 + 0x93), 2);
              __builtin_memcpy(&s2, (unsigned char *)(iVar7 + 0x95), 2);
              iVar7 = FUN_00413d00(iVar10, iVar5, iVar11, (int)s0, (int)s1, (int)s2);
              piVar13[-2] = (int)(0U - (unsigned int)iVar7);
              __builtin_memcpy(&iVar9, (unsigned char *)(param_1 + 0x1f), 4);
              __builtin_memcpy(&s0, (unsigned char *)(iVar9 + 0xa1), 2);
              __builtin_memcpy(&s1, (unsigned char *)(iVar9 + 0x9f), 2);
              __builtin_memcpy(&s2, (unsigned char *)(iVar9 + 0x9d), 2);
              iVar8 = (int)s0;
              iVar7 = (int)s1;
              iVar9 = (int)s2;
            }
            iVar5 = FUN_00413d00(iVar10, iVar5, iVar11, iVar9, iVar7, iVar8);
            piVar13[-1] = iVar5;
            *piVar13 = 0x10;
            switch (*(unsigned char *)(iVar12 + 3)) {
            case 0x7f:
            case 0x80:
            case 0x81:
            case 0x82:
              switch (*(unsigned char *)(iVar12 + 0x19)) {
              case 2:
                *piVar13 = 0xc0;
                break;
              case 3:
                *piVar13 = 0x140;
                break;
              case 4:
                *piVar13 = 0x1e0;
                break;
              case 5:
              case 6:
              case 7:
              case 8:
                *piVar13 = 0x140;
                break;
              case 9:
                *piVar13 = 0xc0;
                break;
              case 10:
                *piVar13 = 0x60;
                break;
              case 0xb:
                *piVar13 = 0x30;
              }
              __builtin_memcpy(&iVar7, (unsigned char *)(iVar12 + 0x1f), 4);
              if (iVar7 != 0) {
                bVar1 = *(unsigned char *)(iVar7 + 1);
                if (3 < bVar1) {
                  *piVar13 = (int)((unsigned int)((int)(bVar1 + 4) >> 2) * (unsigned int)*piVar13);
                }
              }
              break;
            case 0x83:
            case 0x84:
              switch (*(unsigned char *)(iVar12 + 0x19)) {
              case 2:
                *piVar13 = 0x30;
                break;
              case 3:
                *piVar13 = 0x60;
                break;
              case 4:
                *piVar13 = 0x40;
                break;
              case 5:
                *piVar13 = 0x20;
              }
              break;
            default:
              *piVar13 = (int)((unsigned int)DAT_00525540 + 0xffffff00U);
            }
            piVar14 = piVar13 + 4;
            local_10 = local_10 + 1;
            *piVar13 = (int)((unsigned int)*piVar13 << 3);
            if (piVar14 == &DAT_0072f4ec) break;
          }
        }
        iVar12 = iVar12 + 0x23;
        local_14 = local_14 + 1;
        piVar13 = piVar14;
      } while (local_14 < DAT_00732a38);
    }
    DAT_00525730 = local_10;
  }
  return;
}
