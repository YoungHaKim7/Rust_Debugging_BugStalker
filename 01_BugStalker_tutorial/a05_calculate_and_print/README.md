# Result

```bash

$ bs target/debug/a05_calculate_and_print

BugStalker greets

(bs) break calculation_single_value
New breakpoint 1 at 0x0000000000FCE7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7

(bs) r
Hit breakpoint 1 at 0x00555555563CE7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   7     let mut int8 = 1_i8;

(bs) source 10
a05_calculate_and_print::calculation_single_value at /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   1 use std::sync::Mutex;
   2 use std::thread;
   3 use std::time::Duration;
   4
   5 #[allow(clippy::deref_addrof)]
   6 fn calculation_single_value() {
   7     let mut int8 = 1_i8;
   8     int8 += 1;
   9     int8 /= 3;
  10     println!("{int8}");
  11     *&mut int8 = -5;
  12     int8 += 11;
  13     println!("{int8}");
  14 }
  15
  16 fn calculation_four_value() {
  17     let mut a = 1_u64;
  18     let mut b = 2_u64;
  19     let mut c = 3_u64;
  20     let mut d = 4_u64;
  21

(bs) b calculation_four_value
New breakpoint 2 at 0x0000000000FEA7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:17
(bs) r
Restart a program?
(bs y/n) y
Hit breakpoint 3 at 0x00555555563CE7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   7     let mut int8 = 1_i8;
───╯

(bs) watch info
0/4 active watchpoints:
(bs) c
0
6
Hit breakpoint 4 at 0x00555555563EA7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:17
  17     let mut a = 1_u64;
(bs) c
13
1
5
7
v[2] = 4, s.b = 2
Program exit with code: 0

(bs) r
Restart a program?
(bs y/n) y
Hit breakpoint 5 at 0x00555555563CE7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   7     let mut int8 = 1_i8;

(bs) b calculation_global_value
New breakpoint 7 at 0x00000000010104: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:34

(bs) source 10
a05_calculate_and_print::calculation_single_value at /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   1 use std::sync::Mutex;
   2 use std::thread;
   3 use std::time::Duration;
   4
   5 #[allow(clippy::deref_addrof)]
   6 fn calculation_single_value() {
   7     let mut int8 = 1_i8;
   8     int8 += 1;
   9     int8 /= 3;
  10     println!("{int8}");
  11     *&mut int8 = -5;
  12     int8 += 11;
  13     println!("{int8}");
  14 }
  15
  16 fn calculation_four_value() {
  17     let mut a = 1_u64;
  18     let mut b = 2_u64;
  19     let mut c = 3_u64;
  20     let mut d = 4_u64;
  21

(bs) next
   8     int8 += 1;

(bs) next
   9     int8 /= 3;

(bs) next
  10     println!("{int8}");

(bs) next
0
  11     *&mut int8 = -5;

(bs) next
  12     int8 += 11;

(bs) next
  13     println!("{int8}");
(bs) next
6
  14 }
(bs) next

a05_calculate_and_print::main at /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:109
 109     calculation_four_value();

(bs) next
13
 110     calculation_global_value();

(bs) next
1
 111     calculation_global_value_mt();

(bs) watch info
0/4 active watchpoints:
(bs) watch info
remove  r       info

(bs) next
5
 112     calculation_local_value_mt();
(bs) c
7
v[2] = 4, s.b = 2
Program exit with code: 0
(bs) r
Restart a program?
(bs y/n) y
Hit breakpoint 10 at 0x00555555563CE7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:7
   7     let mut int8 = 1_i8;

(bs) watch GLOBAL_1
New watchpoint 1 at 0x005555555C5010, condition: w, watch size: 8b, expression: GLOBAL_1
(bs) next
   8     int8 += 1;
(bs) next
   9     int8 /= 3;
(bs) watch GLOBAL_2
Error: the size of the watch object does not fit into one of the size class (1, 2, 4, 8 bytes), try to specify a field to observ


(bs) c
0
6
Hit breakpoint 9 at 0x00555555563EA7: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:17
  17     let mut a = 1_u64;


(bs) c
13
Hit breakpoint 8 at 0x00555555564104: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:34
  34         GLOBAL_1 -= 1;

(bs) next
Hit watchpoint 1 (w) at 0x00555555564126: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:35
old value: a05_calculate_and_print::GLOBAL_1 = i64(1)
new value: a05_calculate_and_print::GLOBAL_1 = i64(0)

(bs) next
Hit watchpoint 1 (w) at 0x00555555564167: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:36
old value: a05_calculate_and_print::GLOBAL_1 = i64(0)
new value: a05_calculate_and_print::GLOBAL_1 = i64(3)

(bs) next
Hit watchpoint 1 (w) at 0x005555555641C5: /home/01_BugStalker_tutorial/a05_calculate_and_print/src/main.rs:36
old value: a05_calculate_and_print::GLOBAL_1 = i64(3)
new value: a05_calculate_and_print::GLOBAL_1 = i64(1)

```

