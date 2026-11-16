/* SPDX-License-Identifier: MIT */
/* z_xvt 004282d0 */
/* Prototypes for callees */
void __cdecl FUN_00428980(unsigned char *param_1);
unsigned char * __cdecl FUN_004288b0(signed char param_1, char param_2, unsigned char *param_3, int param_4, int param_5, int param_6);
void __cdecl FUN_0047f010(int param_1);
void __cdecl _exit(int param_1);

extern unsigned char *DAT_00536ef0;
extern unsigned int DAT_00536ef4;
extern unsigned int DAT_00536ef8;
extern unsigned char DAT_00536f21[4];   /* packed pointer, unaligned */
extern unsigned char DAT_00536f25[4];   /* packed uint, unaligned */
extern double _DAT_005200e0;

void __cdecl FUN_004282d0(int param_1)
{
  unsigned char *pbVar1;
  unsigned int uVar2;
  double dVar3;
  unsigned char bVar4;
  char cVar5;
  unsigned int uVar6;
  unsigned char *puVar7;
  char cVar8;
  unsigned char bVar9;
  int iVar10;
  unsigned int uVar11;
  char cVar12;
  unsigned char *pbVar13;
  unsigned char *pbVar14;
  unsigned int local_18;
  unsigned int local_4;
  unsigned int u_a;
  unsigned int u_b;
  unsigned int u_c;
  unsigned int cnt;
  unsigned char *parent;
  unsigned char *tbl;
  double d_a;
  double d_b;
  unsigned int t0;
  unsigned int t1;
  unsigned int t2;

  u_a = *(unsigned int *)(param_1 + 0x1034);
  dVar3 = (double)u_a;
  u_b = *(unsigned int *)(param_1 + 0x1030);
  local_4 = 0;
  __builtin_memcpy(&d_a, DAT_00536ef0 + 0xe, 8);
  d_b = (double)u_b * dVar3 * _DAT_005200e0 + d_a;
  __builtin_memcpy(DAT_00536ef0 + 0xe, &d_b, 8);
  __builtin_memcpy(&pbVar14, (unsigned char *)param_1 + 0x1086, 4);
  __builtin_memcpy(&cnt, (unsigned char *)param_1 + 0x108e, 4);
  if (cnt != 0) {
    do {
      pbVar13 = DAT_00536ef0;
      __builtin_memcpy(&u_a, DAT_00536f25, 4);
      if (0x41241 < u_a) {
        pbVar1 = DAT_00536ef0 + 2;
        if (DAT_00536ef0[2] != 0) {
          iVar10 = 0;
          do {
            if (((unsigned int)(1 << ((unsigned char)iVar10 & 0x1f)) & (unsigned int)*pbVar1) != 0) {
              __builtin_memcpy(&puVar7, pbVar13 + iVar10 * 4 + 0x32, 4);
              FUN_00428980(puVar7);
            }
            iVar10 = iVar10 + 1;
          } while (iVar10 < 8);
        }
        if (pbVar13[1] == DAT_00536ef4) {
          __builtin_memcpy(&parent, pbVar13 + 0x2e, 4);
          parent[2] = (unsigned char)(parent[2] & ~(unsigned char)(1 << (*pbVar13 & 0x1f)));
          __builtin_memcpy(&u_a, parent + 10, 4);
          __builtin_memcpy(&u_b, pbVar13 + 10, 4);
          u_a = u_a + u_b;
          __builtin_memcpy(parent + 10, &u_a, 4);
          __builtin_memcpy(&d_a, pbVar13 + 0x16, 8);
          __builtin_memcpy(&d_b, parent + 0x16, 8);
          d_a = d_a + d_b;
          __builtin_memcpy(parent + 0x16, &d_a, 8);
          __builtin_memcpy(&d_a, pbVar13 + 0x1e, 8);
          __builtin_memcpy(&d_b, parent + 0x1e, 8);
          d_a = d_a + d_b;
          __builtin_memcpy(parent + 0x1e, &d_a, 8);
          __builtin_memcpy(&d_a, pbVar13 + 0x26, 8);
          __builtin_memcpy(&d_b, parent + 0x26, 8);
          d_a = d_a + d_b;
          __builtin_memcpy(parent + 0x26, &d_a, 8);
          __builtin_memcpy(&u_a, DAT_00536f25, 4);
          u_a = u_a - 1;
          __builtin_memcpy(DAT_00536f25, &u_a, 4);
        }
        DAT_00536ef4 = DAT_00536ef4 - 1;
      }
      bVar9 = 7;
      local_18 = 1;
      uVar6 = pbVar14[3] + 1;
      pbVar13 = DAT_00536ef0;
      if (DAT_00536ef4 != 0) {
        do {
          bVar4 = (unsigned char)(pbVar14[2] >> (bVar9 & 0x1f));
          uVar2 = ((((unsigned int)(*pbVar14 >> (bVar9 & 0x1f)) & 1) * 2) |
                   ((unsigned int)(pbVar14[1] >> (bVar9 & 0x1f)) & 1)) * 2;
          uVar11 = uVar2 | (bVar4 & 1);
          __builtin_memcpy(&puVar7, pbVar13 + uVar11 * 4 + 0x32, 4);
          if (puVar7 == (unsigned char *)0) {
            pbVar13[2] = (unsigned char)(pbVar13[2] | (unsigned char)(1 << (signed char)uVar11));
            cVar5 = (char)((unsigned int)(1 << ((8U - (unsigned int)(int)(char)local_18) & 0x1f)) >> 1);
            cVar12 = cVar5;
            if ((bVar4 & 1) == 0) {
              cVar12 = (char)-cVar5;
            }
            cVar8 = cVar5;
            if ((uVar2 & 2) == 0) {
              cVar8 = (char)-cVar5;
            }
            if ((uVar2 & 4) == 0) {
              cVar5 = (char)-cVar5;
            }
            puVar7 = FUN_004288b0((signed char)uVar11, (char)local_18, pbVar13,
                                  (int)((unsigned int)pbVar13[3] + (unsigned int)(int)cVar5),
                                  (int)((unsigned int)pbVar13[4] + (unsigned int)(int)cVar8),
                                  (int)((unsigned int)pbVar13[5] + (unsigned int)(int)cVar12));
            __builtin_memcpy(pbVar13 + uVar11 * 4 + 0x32, &puVar7, 4);
            if (puVar7 == (unsigned char *)0) {
              FUN_0047f010(0);
              _exit(1);
            }
            if (DAT_00536ef4 == local_18) {
              DAT_00536ef8 = DAT_00536ef8 + 1;
            }
          }
          __builtin_memcpy(&pbVar13, pbVar13 + uVar11 * 4 + 0x32, 4);
          bVar9 = (unsigned char)(bVar9 - 1);
          local_18 = local_18 + 1;
          __builtin_memcpy(&tbl, DAT_00536f21, 4);
          __builtin_memcpy(&t2, tbl + ((unsigned int)pbVar14[2] - (unsigned int)pbVar13[5]) * 4, 4);
          __builtin_memcpy(&t1, tbl + ((unsigned int)pbVar14[1] - (unsigned int)pbVar13[4]) * 4, 4);
          __builtin_memcpy(&t0, tbl + ((unsigned int)*pbVar14 - (unsigned int)pbVar13[3]) * 4, 4);
          __builtin_memcpy(&d_a, pbVar13 + 0xe, 8);
          d_b = ((double)t2 + (double)t1 + (double)t0) * (double)uVar6 + d_a;
          __builtin_memcpy(pbVar13 + 0xe, &d_b, 8);
        } while (local_18 <= DAT_00536ef4);
      }
      __builtin_memcpy(&u_a, pbVar13 + 10, 4);
      u_a = u_a + uVar6;
      __builtin_memcpy(pbVar13 + 10, &u_a, 4);
      u_b = (unsigned int)*pbVar14 * uVar6;
      __builtin_memcpy(&d_a, pbVar13 + 0x16, 8);
      d_b = (double)u_b + d_a;
      __builtin_memcpy(pbVar13 + 0x16, &d_b, 8);
      u_b = (unsigned int)pbVar14[1] * uVar6;
      __builtin_memcpy(&d_a, pbVar13 + 0x1e, 8);
      d_b = (double)u_b + d_a;
      __builtin_memcpy(pbVar13 + 0x1e, &d_b, 8);
      pbVar1 = pbVar14 + 2;
      pbVar14 = pbVar14 + 6;
      u_c = (unsigned int)*pbVar1 * uVar6;
      __builtin_memcpy(&d_a, pbVar13 + 0x26, 8);
      d_b = (double)u_c + d_a;
      __builtin_memcpy(pbVar13 + 0x26, &d_b, 8);
      local_4 = local_4 + 1;
      __builtin_memcpy(&cnt, (unsigned char *)param_1 + 0x108e, 4);
    } while (local_4 < cnt);
  }
  return;
}
