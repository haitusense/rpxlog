## correspondence table for header

| txt header    | MIR                 | SDR           | WIR |
| :--           | :--                 | :--           | :-- |    
| Cust_LotId    | ?                   |               | |
| Customer_Id   | ?                   |               | |
| Device        | ?                   |               | |
|               | part_typ            |               | |
|               | exec_typ            |               | |
|               | exec_ver            |               | |
| ProberCard_Id |                     | card_id       | |
| Test_Program  | job_nam / job_rev   |               | |
| Tester_Id     | node_nam / tstr_typ |               | |
| Flow_ID       | flow_id             |               | |
| HSTS_LOT      | lot_id              |               | |
|               | sblot_id            |               | |
| Operator_Id   | oper_nam            |               | |
|               |                     | hand_typ      | |
| Prober_Id     |                     | hand_id       | |
| temperature   | tst_temp            |               | |
| Process       |                     |               | |
|               |                     | load_typ      | |
| LoadBoard_Id  |                     | load_id       | |
| Handle_Id     |                     | (not hand_id) | |
| Wafer_Id      |                     |               | wafer_id |
| Wafer_No      | ?                   |               | |
| start_time    | setup_t / start_t   |               | start_t |
| end_time      | -                   |               | |
|               | stat_num            |               | |
|               | burn_tim            |               | |

WCR なぜ二個ある？
PIR = 319

DTRは区別ができるように書き方を工夫の必要ある
DTR { text_dat: "00   0   PASS  Temp_AF  Reg[252-253-254]\t\t0\t\t0\t\t0" }
